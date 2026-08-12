"""Python arm of the cross-language comparison, under pytest-benchmark."""
import pathlib
import uuid

import pytest

from appstoreserverlibrary.models.Environment import Environment
from appstoreserverlibrary.promotional_offer import PromotionalOfferSignatureCreator
from appstoreserverlibrary.receipt_utility import ReceiptUtility
from appstoreserverlibrary.signed_data_verifier import SignedDataVerifier

WARMUP, ITERATIONS = 50, 500

DATA = pathlib.Path(__file__).parent.parent.parent / "resources"


def text(rel):
    return (DATA / rel).read_text()


root_ca = (DATA / "certs/testCA.der").read_bytes()
verifier = SignedDataVerifier([root_ca], False, Environment.SANDBOX, "com.example", 1234)

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
