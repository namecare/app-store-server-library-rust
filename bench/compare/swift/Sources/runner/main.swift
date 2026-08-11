// Emits the shared JSONL contract the cross-language driver collects.
//
// Same cases, same inputs and the same loop shape as every other arm: one probe
// run, 50 warmup, 500 individually-timed iterations, same output line.
//
// `ns_per_op` is the MEDIAN of those 500 samples. Median rather than mean
// because benchmark noise is one-directional: an interrupt or a scheduling
// hiccup can only make a sample slower, so the distribution has a hard floor and
// a long right tail, and a single outlier moves the mean but not the median.
//
// Note: Swift's verify methods are `async` and its ChainVerifier is an actor,
// so these figures include executor and actor-hop overhead the Rust arm never
// pays. That is a cost a real Swift caller pays, so it belongs in the number.

import Foundation
import AppStoreServerLibrary

let WARMUP = 50
let ITERATIONS = 500

/// The canonical case names, shared with every other language arm.
let CASES = [
    "verify_notification",
    "verify_transaction",
    "verify_renewal_info",
    "receipt_app",
    "receipt_app_legacy",
    "sign_promotional_offer",
]

// Resolve the shared data directory from #filePath rather than the cwd, so the
// binary produces the same numbers no matter where it is launched from.
// #filePath = <repo>/bench/compare/swift/Sources/runner/main.swift
// data dir  = <repo>/bench/compare/data
let dataDirectory = URL(fileURLWithPath: #filePath)
    .deletingLastPathComponent()   // .../Sources/runner
    .deletingLastPathComponent()   // .../Sources
    .deletingLastPathComponent()   // .../swift
    .deletingLastPathComponent()   // .../compare
    .appendingPathComponent("data")

func dataString(_ relative: String) -> String {
    let url = dataDirectory.appendingPathComponent(relative)
    guard let contents = try? String(contentsOf: url, encoding: .utf8) else {
        FileHandle.standardError.write(Data("fatal: cannot read \(url.path)\n".utf8))
        exit(1)
    }
    return contents.trimmingCharacters(in: .whitespacesAndNewlines)
}

func dataBytes(_ relative: String) -> Data {
    let url = dataDirectory.appendingPathComponent(relative)
    guard let contents = try? Data(contentsOf: url) else {
        FileHandle.standardError.write(Data("fatal: cannot read \(url.path)\n".utf8))
        exit(1)
    }
    return contents
}

/// Standard median: sort, and average the two middle samples when n is even.
func median(_ samples: [UInt64]) -> Double {
    precondition(!samples.isEmpty, "median of no samples")
    let sorted = samples.sorted()
    let mid = sorted.count / 2
    if sorted.count % 2 == 0 {
        return (Double(sorted[mid - 1]) + Double(sorted[mid])) / 2
    }
    return Double(sorted[mid])
}

struct Inputs {
    let notification: String
    let transaction: String
    let renewalInfo: String
    let receipt: String
    let receiptLegacy: String
}

@main
struct Runner {
    static func main() async {
        let rootCA = dataBytes("certs/testCA.der")

        let inputs = Inputs(
            notification: dataString("signed/testNotification"),
            transaction: dataString("signed/transactionInfo"),
            renewalInfo: dataString("signed/renewalInfo"),
            receipt: dataString("receipts/xcode-app-receipt-with-transaction"),
            receiptLegacy: dataString("receipts/xcode-app-receipt-legacy")
        )

        // Same verifier configuration as every other arm: same root, same
        // bundle id, same environment, and online checks OFF so the chain is
        // actually verified on every iteration rather than served from cache.
        let verifier: SignedDataVerifier
        do {
            verifier = try SignedDataVerifier(
                rootCertificates: [rootCA],
                bundleId: "com.example",
                appAppleId: 1234,
                environment: .sandbox,
                enableOnlineChecks: false
            )
        } catch {
            FileHandle.standardError.write(Data("fatal: verifier init failed: \(error)\n".utf8))
            exit(1)
        }

        let signer: PromotionalOfferSignatureCreator
        do {
            signer = try PromotionalOfferSignatureCreator(
                privateKey: dataString("certs/testSigningKey.p8"),
                keyId: "L256SYR32L",
                bundleId: "com.test.app"
            )
        } catch {
            FileHandle.standardError.write(Data("fatal: signer init failed: \(error)\n".utf8))
            exit(1)
        }

        let nonce = UUID(uuidString: "3db5c98d-8acf-4e29-831e-8e1f82f9f6e9")!

        // Accumulated so the optimiser cannot eliminate the measured work.
        var sink = 0

        /// Runs one case once. Returns false if the operation failed, so the
        /// runner can refuse to report a number for work that did not happen.
        func runCase(_ name: String) async -> Bool {
            switch name {
            case "verify_notification":
                switch await verifier.verifyAndDecodeNotification(signedPayload: inputs.notification) {
                case .valid(let payload):
                    sink &+= payload.hashValue
                    return true
                case .invalid:
                    return false
                }

            case "verify_transaction":
                switch await verifier.verifyAndDecodeTransaction(signedTransaction: inputs.transaction) {
                case .valid(let payload):
                    sink &+= payload.hashValue
                    return true
                case .invalid:
                    return false
                }

            case "verify_renewal_info":
                switch await verifier.verifyAndDecodeRenewalInfo(signedRenewalInfo: inputs.renewalInfo) {
                case .valid(let payload):
                    sink &+= payload.hashValue
                    return true
                case .invalid:
                    return false
                }

            case "receipt_app":
                guard let id = ReceiptUtility.extractTransactionId(appReceipt: inputs.receipt) else {
                    return false
                }
                sink &+= id.count
                return true

            case "receipt_app_legacy":
                guard let id = ReceiptUtility.extractTransactionId(appReceipt: inputs.receiptLegacy) else {
                    return false
                }
                sink &+= id.count
                return true

            case "sign_promotional_offer":
                do {
                    let signature = try signer.createSignature(
                        productIdentifier: "com.test.product",
                        subscriptionOfferID: "com.test.offer",
                        appAccountToken: "6b9f1f4a-1a1e-4b0e-9b0e-1a1e4b0e9b0e",
                        nonce: nonce,
                        timestamp: 12345
                    )
                    sink &+= signature.count
                    return true
                } catch {
                    return false
                }

            default:
                FileHandle.standardError.write(Data("fatal: unknown case: \(name)\n".utf8))
                exit(1)
            }
        }

        for name in CASES {
            // A failing call is fast, and a fast wrong number is worse than a
            // missing one — so emit no line at all for a case that did not run.
            if await !runCase(name) {
                FileHandle.standardError.write(
                    Data("case \(name) failed; not reporting a figure for it\n".utf8))
                continue
            }

            for _ in 0..<WARMUP {
                _ = await runCase(name)
            }

            // Time each iteration separately so the reported figure is a median.
            var samples: [UInt64] = []
            samples.reserveCapacity(ITERATIONS)
            for _ in 0..<ITERATIONS {
                let start = DispatchTime.now()
                _ = await runCase(name)
                samples.append(DispatchTime.now().uptimeNanoseconds - start.uptimeNanoseconds)
            }

            let nsPerOp = median(samples)
            print(String(format:
                "{\"lib\":\"swift\",\"case\":\"%@\",\"iterations\":%d,\"ns_per_op\":%.1f}",
                name, ITERATIONS, nsPerOp))
        }

        FileHandle.standardError.write(Data("sink: \(sink)\n".utf8))
    }
}
