// Node arm of the four-language App Store Server Library benchmark comparison,
// under tinybench.
//
// Each arm of this suite runs its own language's idiomatic benchmark harness —
// Divan in Rust, XCTest `measure` in Swift, tinybench here, pytest-benchmark in
// Python. Stage 3 captures each harness's native output; Stage 4
// (`render.py`) owns all four formats and normalises them into one table.
//
// ## What Stage 4 reads
//
// This arm writes tinybench's own results to stdout as JSON, including the full
// raw sample array per case. `render.py` takes the median from those samples,
// the same way it does for Swift — so both arms report a median computed from
// raw data rather than a harness-chosen summary statistic.
//
// tinybench reports latencies in MILLISECONDS; the JSON below converts to
// nanoseconds so every arm's artifact speaks the same unit.
//
// ## Iteration count is pinned, not adaptive
//
// Every harness in this suite is adaptive by default and each needs a different
// explicit opt-out. tinybench's is subtle: a task runs until BOTH its iteration
// count AND its time budget are satisfied, so `iterations: 500` alone would
// keep going past 500 for fast cases until 1000 ms had elapsed. Setting
// `time: 0` and `warmupTime: 0` is what actually pins the window to exactly 500
// samples after 50 warmup runs, matching the other three arms.
//
// `retainSamples: true` is required to see the raw array at all — tinybench
// discards samples by default.

import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { Bench } from "tinybench";

const require = createRequire(import.meta.url);
const {
  SignedDataVerifier,
  Environment,
  ReceiptUtility,
  PromotionalOfferSignatureCreator,
} = require("@apple/app-store-server-library");

const WARMUP = 50;
const ITERATIONS = 500;

const text = (p) => readFileSync(new URL(`../../resources/${p}`, import.meta.url), "utf8");
const bytes = (p) => readFileSync(new URL(`../../resources/${p}`, import.meta.url));

// Root CA must stay a raw Buffer: the verifier wants DER bytes, not a string.
const rootCa = bytes("certs/testCA.der");

// NOTE: enableOnlineChecks is the SECOND positional parameter here, unlike the
// Swift and Rust arms where it comes last. It must be false so that no
// certificate-chain caching or network revocation check distorts the timings.
const verifier = new SignedDataVerifier(
  [rootCa],
  false,
  Environment.SANDBOX,
  "com.example",
  1234,
);

const receipts = new ReceiptUtility();
const signer = new PromotionalOfferSignatureCreator(
  text("certs/testSigningKey.p8"),
  "L256SYR32L",
  "com.test.app",
);

const inputs = {
  notification: text("signed/testNotification"),
  transaction: text("signed/transactionInfo"),
  renewalInfo: text("signed/renewalInfo"),
  receipt: text("receipts/xcode-app-receipt-with-transaction"),
  receiptLegacy: text("receipts/xcode-app-receipt-legacy"),
};

// The receipt and signing cases are synchronous; they are wrapped in `async`
// only so the loop is uniform. The extra await on an already-resolved value is
// a real but tiny cost borne equally by those cases.
const CASES = {
  verify_notification: () => verifier.verifyAndDecodeNotification(inputs.notification),
  verify_transaction: () => verifier.verifyAndDecodeTransaction(inputs.transaction),
  verify_renewal_info: () => verifier.verifyAndDecodeRenewalInfo(inputs.renewalInfo),
  receipt_app: async () => receipts.extractTransactionIdFromAppReceipt(inputs.receipt),
  receipt_app_legacy: async () => receipts.extractTransactionIdFromAppReceipt(inputs.receiptLegacy),
  sign_promotional_offer: async () =>
    signer.createSignature(
      "com.test.product",
      "com.test.offer",
      "6b9f1f4a-1a1e-4b0e-9b0e-1a1e4b0e9b0e",
      "3db5c98d-8acf-4e29-831e-8e1f82f9f6e9",
      12345,
    ),
};

// A case that throws is skipped entirely rather than timed: a failing call is
// fast, and a fast wrong number is worse than a missing one. This is what lets
// `receipt_app` render honestly as "unsupported" — Node's DER-only ASN.1 parser
// genuinely cannot read the BER app receipt — rather than as a fast timing.
const supported = {};
for (const [name, fn] of Object.entries(CASES)) {
  try {
    await fn();
    supported[name] = fn;
  } catch (e) {
    console.error(`case ${name} failed: ${e.message}; not reporting a figure for it`);
  }
}

const bench = new Bench({
  iterations: ITERATIONS,
  warmupIterations: WARMUP,
  // Both budgets must be zero or the counts above become minimums, not exact
  // counts — see the note at the top of this file.
  time: 0,
  warmupTime: 0,
  // tinybench discards raw samples unless asked; Stage 4 takes the median from
  // them.
  retainSamples: true,
});

for (const [name, fn] of Object.entries(supported)) bench.add(name, fn);

await bench.run();

// tinybench's own result objects, emitted as this arm's raw artifact. `result`
// is a discriminated union in 6.x, so a task that did not reach `completed`
// reports no figure rather than a wrong one.
const output = {
  lib: "node",
  harness: `tinybench ${ITERATIONS}x${WARMUP}`,
  cases: {},
};

for (const task of bench.tasks) {
  const result = task.result;
  if (!result || result.state !== "completed") {
    console.error(`case ${task.name} did not complete; not reporting a figure for it`);
    continue;
  }
  const msToNs = (ms) => ms * 1e6;
  output.cases[task.name] = {
    // Raw per-iteration samples, in nanoseconds, for Stage 4 to reduce.
    samples_ns: (result.latency.samples ?? []).map(msToNs),
    // tinybench's own p50, kept alongside as a cross-check on our median.
    p50_ns: msToNs(result.latency.p50),
    mean_ns: msToNs(result.latency.mean),
    samples_count: result.latency.samplesCount ?? result.latency.samples?.length ?? 0,
  };
}

console.log(JSON.stringify(output, null, 2));
