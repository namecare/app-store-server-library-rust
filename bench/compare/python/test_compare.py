"""Python arm of the cross-language comparison, under pytest-benchmark.

Each arm of this suite runs its own language's idiomatic benchmark harness —
Divan in Rust, XCTest ``measure`` in Swift, tinybench in Node, pytest-benchmark
here. Stage 3 captures each harness's native output; Stage 4 (``render.py``)
owns all four formats and normalises them into one table.

What Stage 4 reads
------------------
``--benchmark-json`` writes pytest-benchmark's own report, which carries both a
``median`` in its ``stats`` block and the full raw ``data`` array. Stage 4 takes
the median from the raw samples, the same way it does for Swift and Node.

Iteration count is pinned, not adaptive
---------------------------------------
Every harness in this suite is adaptive by default and each needs a different
explicit opt-out. pytest-benchmark's is ``benchmark.pedantic()``: the ordinary
``benchmark(fn)`` call auto-calibrates both round count and inner iterations,
so only ``pedantic`` can pin exactly 50 warmup and 500 timed rounds to match
the other three arms.

Note this library is fully synchronous, unlike Node and Swift — there is no
event loop in these numbers.
"""
import pathlib
import uuid

import pytest

from appstoreserverlibrary.models.Environment import Environment
from appstoreserverlibrary.promotional_offer import PromotionalOfferSignatureCreator
from appstoreserverlibrary.receipt_utility import ReceiptUtility
from appstoreserverlibrary.signed_data_verifier import SignedDataVerifier

WARMUP, ITERATIONS = 50, 500

# `data/` is shared by all four arms: byte-identical inputs are what make a
# ratio between two cells of the table mean anything.
DATA = pathlib.Path(__file__).parent.parent / "data"


def text(rel):
    return (DATA / rel).read_text()


root_ca = (DATA / "certs/testCA.der").read_bytes()
# enable_online_checks = False, second positional argument in this library, so
# the chain is verified on every iteration rather than served from a cache.
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
# iteration — verified to cost tens of microseconds, not the ~1us a no-op would.
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


@pytest.mark.parametrize("case", list(CASES), ids=list(CASES))
def test_case(benchmark, case):
    """Times one case, named so Stage 4 can recover the shared case name.

    A case that raises on its trial run is skipped rather than reported: a
    failing call is fast, and a fast wrong number is worse than a missing one.
    """
    fn = CASES[case]
    try:
        fn()
    except Exception as e:  # noqa: BLE001 — any failure means "do not report a figure"
        pytest.skip(f"case {case} failed: {e}; not reporting a figure for it")

    # `pedantic` rather than `benchmark(fn)`: it is the only way to pin the
    # round count instead of letting pytest-benchmark auto-calibrate.
    # rounds=500 timed samples of 1 iteration each, after 50 warmup rounds.
    benchmark.pedantic(fn, rounds=ITERATIONS, iterations=1, warmup_rounds=WARMUP)
