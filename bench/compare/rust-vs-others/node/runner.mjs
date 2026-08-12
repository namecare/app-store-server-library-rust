// Node arm of the four-language App Store Server Library benchmark comparison,
// under tinybench.

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

// The receipt and signing cases are synchronous;
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

// A case that throws is skipped entirely rather than timed
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

// tinybench's own result objects, emitted as this arm's raw artifact.
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
