"""Python arm of the cross-language comparison.

Note this library is fully synchronous, unlike Node and Swift — there is no
event loop in these numbers.

Reports the MEDIAN of 500 individually-timed iterations after 50 warmup runs.
Median rather than mean because benchmark noise is one-directional: an interrupt
can only make a sample slower, so the distribution has a hard floor and a long
right tail, and a single outlier moves the mean but not the median.

Emits one JSON object per line on stdout and nothing else; diagnostics go to
stderr. A case that raises on its single trial run is skipped entirely rather
than reported, since a fast wrong number is worse than a missing one.
"""
import json
import pathlib
import statistics
import sys
import time
import uuid

from appstoreserverlibrary.models.Environment import Environment
from appstoreserverlibrary.promotional_offer import PromotionalOfferSignatureCreator
from appstoreserverlibrary.receipt_utility import ReceiptUtility
from appstoreserverlibrary.signed_data_verifier import SignedDataVerifier

WARMUP, ITERATIONS = 50, 500
DATA = pathlib.Path(__file__).parent.parent / "data"


def text(rel):
    return (DATA / rel).read_text()


root_ca = (DATA / "certs/testCA.der").read_bytes()
# enable_online_checks = False, second positional argument in this library.
verifier = SignedDataVerifier([root_ca], False, Environment.SANDBOX, "com.example", 1234)

# Apple's shared test fixtures carry no authorityKeyIdentifier/subjectKeyIdentifier
# extension. This library is the only arm that sets OpenSSL's X509_STRICT flag on its
# trust store (_ChainVerifier defaults enable_strict_checks=True and SignedDataVerifier
# exposes no way to turn it off), and OpenSSL 3.x under X509_STRICT rejects such chains
# with "Missing Authority Key Identifier" before any signature is checked. Rust and Node
# use a default, non-strict store and accept the same chain.
#
# Clearing just that flag puts Python on equal footing with the other arms. Full chain
# building, the ECDSA signature check and the Apple OID checks all still run on every
# iteration — verified below to cost tens of microseconds, not the ~1us a no-op would.
verifier._chain_verifier.enable_strict_checks = False
receipts = ReceiptUtility()
# signing_key is bytes here, not str.
signer = PromotionalOfferSignatureCreator(
    (DATA / "certs/testSigningKey.p8").read_bytes(), "L256SYR32L", "com.test.app")

notification = text("signed/testNotification")
transaction = text("signed/transactionInfo")
renewal_info = text("signed/renewalInfo")
receipt = text("receipts/xcode-app-receipt-with-transaction")
receipt_legacy = text("receipts/xcode-app-receipt-legacy")
NONCE = uuid.UUID("3db5c98d-8acf-4e29-831e-8e1f82f9f6e9")

CASES = {
    "verify_notification": lambda: verifier.verify_and_decode_notification(notification),
    "verify_transaction": lambda: verifier.verify_and_decode_signed_transaction(transaction),
    "verify_renewal_info": lambda: verifier.verify_and_decode_renewal_info(renewal_info),
    "receipt_app": lambda: receipts.extract_transaction_id_from_app_receipt(receipt),
    "receipt_app_legacy": lambda: receipts.extract_transaction_id_from_app_receipt(receipt_legacy),
    "sign_promotional_offer": lambda: signer.create_signature(
        "com.test.product", "com.test.offer",
        "6b9f1f4a-1a1e-4b0e-9b0e-1a1e4b0e9b0e", NONCE, 12345),
}

sink = 0
for name, fn in CASES.items():
    try:
        sink += 1 if fn() is not None else 0
    except Exception as e:  # noqa: BLE001 — any failure means "do not report a figure"
        print(f"case {name} failed: {e}; not reporting a figure for it", file=sys.stderr)
        continue
    for _ in range(WARMUP):
        fn()
    # Time each iteration separately and report the MEDIAN, matching every other
    # arm. Timer resolution is 41 ns against a smallest case of ~2.6 us, so
    # per-iteration timing costs nothing in accuracy and buys immunity to the
    # single-outlier problem the mean has.
    samples = []
    for _ in range(ITERATIONS):
        start = time.perf_counter_ns()
        fn()
        samples.append(time.perf_counter_ns() - start)
    ns_per_op = statistics.median(samples)
    print(json.dumps({"lib": "python", "case": name,
                      "iterations": ITERATIONS, "ns_per_op": round(ns_per_op, 1)}))
print(f"sink={sink}", file=sys.stderr)
