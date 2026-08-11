// Node arm of the four-language App Store Server Library benchmark comparison.
//
// Emits one JSON object per line on stdout and nothing else:
//   {"lib":"node","case":"verify_notification","iterations":500,"ns_per_op":41230.5}
//
// `ns_per_op` is the MEDIAN of 500 individually-timed iterations, after 50
// warmup runs. Median rather than mean because benchmark noise is
// one-directional: an interrupt or a GC pause can only make a sample slower, so
// the distribution has a hard floor and a long right tail, and a single outlier
// moves the mean but not the median.
//
// Diagnostics (skipped cases, the anti-DCE sink) go to stderr.

import { createRequire } from "node:module";
import { readFileSync } from "node:fs";

const require = createRequire(import.meta.url);
const {
  SignedDataVerifier,
  Environment,
  ReceiptUtility,
  PromotionalOfferSignatureCreator,
} = require("@apple/app-store-server-library");

const WARMUP = 50;
const ITERATIONS = 500;

const text = (p) => readFileSync(new URL(`../data/${p}`, import.meta.url), "utf8");
const bytes = (p) => readFileSync(new URL(`../data/${p}`, import.meta.url));

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
// only so the timing loop is uniform. The extra await on an already-resolved
// value is a real but tiny cost borne equally by those cases.
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

/** Standard median: sort, and average the two middle samples when n is even. */
function median(samples) {
  const sorted = [...samples].sort((a, b) => a - b);
  const mid = sorted.length >> 1;
  return sorted.length % 2 === 0 ? (sorted[mid - 1] + sorted[mid]) / 2 : sorted[mid];
}

let sink = 0; // keeps the JIT from eliminating the work

for (const [name, fn] of Object.entries(CASES)) {
  // Run once first. A case that throws is skipped entirely rather than being
  // timed: a failing call is fast, and a fast wrong number is worse than a
  // missing one.
  try {
    sink += (await fn()) ? 1 : 0;
  } catch (e) {
    console.error(`case ${name} failed: ${e.message}; not reporting a figure for it`);
    continue;
  }

  for (let i = 0; i < WARMUP; i++) sink += (await fn()) ? 1 : 0;

  // Time each iteration separately so the reported figure can be a median.
  // hrtime's resolution is 41 ns against a smallest case of ~2.6 us, so
  // per-iteration timing costs nothing in accuracy.
  const samples = new Array(ITERATIONS);
  for (let i = 0; i < ITERATIONS; i++) {
    const start = process.hrtime.bigint();
    sink += (await fn()) ? 1 : 0;
    samples[i] = Number(process.hrtime.bigint() - start);
  }
  const nsPerOp = median(samples);

  console.log(
    JSON.stringify({
      lib: "node",
      case: name,
      iterations: ITERATIONS,
      ns_per_op: Number(nsPerOp.toFixed(1)),
    }),
  );
}

console.error(`sink=${sink}`);
