// The Swift arm's XCTest `measure {}` benchmarks.
//
// These do NOT feed the cross-language comparison table. `package-benchmark`
// does — see `Sources/Benchmarks/` — because
// XCTest's clock metric is quantised to whole MICROSECONDS: 500 samples of the
// ~18 µs `receipt_app` case produced only 17 distinct values, 284 of them
// landing on exactly 25.0 µs, with a minimum gap between distinct values of
// exactly 1.000 µs. That is ~4% granularity on the cheap cases, and it rounds
// up.
//
// They are kept because they cost nothing to keep and answer a different
// question. The benchmark arm needs jemalloc installed at build time and runs
// through a SwiftPM plugin; this target needs neither. `swift test -c release`
// is therefore a zero-setup check that every case still runs and still
// verifies — useful on a machine where the benchmark arm cannot build, and as a
// second opinion if a table figure ever looks wrong.
//
// The fixtures and the case bodies live in the `Fixtures` target, shared with the
// benchmark target, so both time exactly the same definition of the work.
//
// MUST be run as `swift test -c release`. `swift test` defaults to a debug
// build, and debug figures here would be meaningless.

import XCTest
import Foundation
import Fixtures

final class CompareMeasureTests: XCTestCase {

    /// `nonisolated(unsafe)` because Swift 6 rejects mutable global state by
    /// default. It is sound here: XCTest writes this once in `setUp()` before
    /// any test method runs, and every subsequent access is a read.
    private nonisolated(unsafe) static var sharedFixture: Fixture!

    override class func setUp() {
        super.setUp()
        do {
            sharedFixture = try Fixture()
        } catch {
            // A missing or malformed fixture must fail the run, not silently
            // produce a fast number for work that never happened.
            fatalError("fixture setup failed: \(error)")
        }
    }

    private var fixture: Fixture { Self.sharedFixture }

    /// 500 timed iterations, matching the sample count every other arm uses.
    private func measureOptions() -> XCTMeasureOptions {
        let options = XCTMeasureOptions()
        options.iterationCount = 500
        options.invocationOptions = [.manuallyStop]
        return options
    }

    /// Times a synchronous case, stopping the clock before the assertion so only
    /// the library call is measured.
    ///
    /// A failing call is fast, and a fast wrong number is worse than a missing
    /// one — so a case that stops working fails the test rather than quietly
    /// reporting an impressive timing.
    private func measureSync(
        _ benchmarkCase: CompareCase,
        file: StaticString = #filePath,
        line: UInt = #line
    ) {
        measure(metrics: [XCTClockMetric()], options: measureOptions()) {
            let succeeded = fixture.runSync(benchmarkCase)
            stopMeasuring()
            XCTAssertTrue(succeeded, "\(benchmarkCase.rawValue) failed", file: file, line: line)
        }
    }

    /// Times an `async` case.
    ///
    /// The semaphore wait is inside the timed window on every iteration. That is
    /// deliberate: Swift's verify methods are `async` and its `ChainVerifier` is
    /// an actor, so executor scheduling and actor hops are costs a real Swift
    /// caller pays on every call. The synchronous cases deliberately avoid this
    /// path — the bridge costs ~10 µs, negligible against a ~560 µs
    /// verification but a large fraction of an 18 µs receipt parse.
    private func measureAsync(
        _ benchmarkCase: CompareCase,
        file: StaticString = #filePath,
        line: UInt = #line
    ) {
        measure(metrics: [XCTClockMetric()], options: measureOptions()) {
            let semaphore = DispatchSemaphore(value: 0)
            // A class box rather than a captured `var`: Swift 6 requires the
            // Task's closure to be `Sendable`. Sound because the semaphore
            // strictly orders the write before the read after `wait()`.
            final class Box: @unchecked Sendable { var value = false }
            let box = Box()
            let captured = fixture
            Task {
                box.value = await captured.runAsync(benchmarkCase)
                semaphore.signal()
            }
            semaphore.wait()
            stopMeasuring()
            XCTAssertTrue(box.value, "\(benchmarkCase.rawValue) failed", file: file, line: line)
        }
    }

    func test_verify_notification() { measureAsync(.verifyNotification) }
    func test_verify_transaction() { measureAsync(.verifyTransaction) }
    func test_verify_renewal_info() { measureAsync(.verifyRenewalInfo) }
    func test_receipt_app() { measureSync(.receiptApp) }
    func test_receipt_app_legacy() { measureSync(.receiptAppLegacy) }
    func test_sign_promotional_offer() { measureSync(.signPromotionalOffer) }
}
