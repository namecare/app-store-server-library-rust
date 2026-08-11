// The cases the Swift arm runs, over the fixtures every language arm shares.
//
// Defined here rather than in the benchmark target so the setup exists once and
// the harness only times it — the same split the Rust arm makes between
// `rust/src/lib.rs` and `rust/benches/compare.rs`.

import Foundation
import AppStoreServerLibrary

/// The canonical case names, shared with every other language arm.
public enum CompareCase: String, CaseIterable, Sendable {
    case verifyNotification = "verify_notification"
    case verifyTransaction = "verify_transaction"
    case verifyRenewalInfo = "verify_renewal_info"
    case receiptApp = "receipt_app"
    case receiptAppLegacy = "receipt_app_legacy"
    case signPromotionalOffer = "sign_promotional_offer"

    /// Whether the underlying library API is `async`.
    ///
    /// The three verify methods are; the receipt and signing calls are plain
    /// synchronous functions. The benchmark target uses this to avoid putting an
    /// async bridge around work that does not need one.
    public var isAsync: Bool {
        switch self {
        case .verifyNotification, .verifyTransaction, .verifyRenewalInfo: return true
        case .receiptApp, .receiptAppLegacy, .signPromotionalOffer: return false
        }
    }
}

/// Built once and reused for every iteration, so the measured work is the
/// library call and not verifier construction — the same setup split every
/// other arm makes.
///
/// `@unchecked Sendable` because every stored property is a `let` written once
/// during construction and only read thereafter. The `unchecked` is needed only
/// because the library does not declare `SignedDataVerifier` and
/// `PromotionalOfferSignatureCreator` as `Sendable`; nothing here mutates them.
public struct Fixture: @unchecked Sendable {
    let verifier: SignedDataVerifier
    let signer: PromotionalOfferSignatureCreator
    let notification: String
    let transaction: String
    let renewalInfo: String
    let receipt: String
    let receiptLegacy: String
    let nonce: UUID

    /// Resolved from `#filePath` rather than the cwd, so the figures do not
    /// depend on where the benchmark was launched from.
    /// #filePath = <repo>/bench/compare/rust-vs-others/swift/Sources/Fixtures/Fixtures.swift
    /// data dir  = <repo>/bench/compare/resources
    ///
    /// `resources/` is shared by both suites: byte-identical inputs are what make a
    /// comparison between two cells of the table mean anything.
    static let dataDirectory = URL(fileURLWithPath: #filePath)
        .deletingLastPathComponent()   // .../Fixtures
        .deletingLastPathComponent()   // .../Sources
        .deletingLastPathComponent()   // .../swift
        .deletingLastPathComponent()   // .../rust-vs-others
        .deletingLastPathComponent()   // .../compare
        .appendingPathComponent("resources")

    static func dataString(_ relative: String) throws -> String {
        try String(contentsOf: dataDirectory.appendingPathComponent(relative), encoding: .utf8)
            .trimmingCharacters(in: .whitespacesAndNewlines)
    }

    static func dataBytes(_ relative: String) throws -> Data {
        try Data(contentsOf: dataDirectory.appendingPathComponent(relative))
    }

    public init() throws {
        // Same verifier configuration as every other arm: same root, same bundle
        // id, same environment, and online checks OFF so the chain is actually
        // verified on every iteration rather than served from a cache.
        verifier = try SignedDataVerifier(
            rootCertificates: [Fixture.dataBytes("certs/testCA.der")],
            bundleId: "com.example",
            appAppleId: 1234,
            environment: .sandbox,
            enableOnlineChecks: false
        )
        signer = try PromotionalOfferSignatureCreator(
            privateKey: Fixture.dataString("certs/testSigningKey.p8"),
            keyId: "L256SYR32L",
            bundleId: "com.test.app"
        )
        notification = try Fixture.dataString("signed/testNotification")
        transaction = try Fixture.dataString("signed/transactionInfo")
        renewalInfo = try Fixture.dataString("signed/renewalInfo")
        receipt = try Fixture.dataString("receipts/xcode-app-receipt-with-transaction")
        receiptLegacy = try Fixture.dataString("receipts/xcode-app-receipt-legacy")
        nonce = UUID(uuidString: "3db5c98d-8acf-4e29-831e-8e1f82f9f6e9")!
    }

    /// Runs one synchronous case. Returns false if the call did not succeed.
    ///
    /// A failing call is fast, and a fast wrong number is worse than a missing
    /// one — the caller refuses to report a figure rather than timing a no-op.
    public func runSync(_ benchmarkCase: CompareCase) -> Bool {
        switch benchmarkCase {
        case .receiptApp:
            return ReceiptUtility.extractTransactionId(appReceipt: receipt) != nil
        case .receiptAppLegacy:
            return ReceiptUtility.extractTransactionId(appReceipt: receiptLegacy) != nil
        case .signPromotionalOffer:
            let signature = try? signer.createSignature(
                productIdentifier: "com.test.product",
                subscriptionOfferID: "com.test.offer",
                appAccountToken: "6b9f1f4a-1a1e-4b0e-9b0e-1a1e4b0e9b0e",
                nonce: nonce,
                timestamp: 12345
            )
            return signature != nil
        case .verifyNotification, .verifyTransaction, .verifyRenewalInfo:
            preconditionFailure("\(benchmarkCase.rawValue) is async; call runAsync")
        }
    }

    /// Runs one `async` case. Returns false if verification did not succeed.
    public func runAsync(_ benchmarkCase: CompareCase) async -> Bool {
        switch benchmarkCase {
        case .verifyNotification:
            if case .valid = await verifier.verifyAndDecodeNotification(signedPayload: notification) {
                return true
            }
            return false
        case .verifyTransaction:
            if case .valid = await verifier.verifyAndDecodeTransaction(signedTransaction: transaction) {
                return true
            }
            return false
        case .verifyRenewalInfo:
            if case .valid = await verifier.verifyAndDecodeRenewalInfo(signedRenewalInfo: renewalInfo) {
                return true
            }
            return false
        case .receiptApp, .receiptAppLegacy, .signPromotionalOffer:
            preconditionFailure("\(benchmarkCase.rawValue) is synchronous; call runSync")
        }
    }
}
