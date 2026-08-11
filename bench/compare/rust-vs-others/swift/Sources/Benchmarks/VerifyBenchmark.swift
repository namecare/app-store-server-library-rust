//===----------------------------------------------------------------------===//
//
// The signed-payload verification cases: JWS signature check plus X.509 chain
// validation, over the fixtures every language arm shares.
//
//===----------------------------------------------------------------------===//

import Benchmark
import Fixtures

/// Times one `async` verification.
///
/// `startMeasurement` is called after setup so only the library call is timed,
/// and exactly one call is made per sample — not the
/// `for _ in benchmark.scaledIterations` loop swift-certificates uses — so this
/// column stays comparable with the Rust, Node and Python arms.
///
/// The measured window includes executor scheduling and actor hops: Swift's
/// verify methods are `async` and its `ChainVerifier` is an actor, so those are
/// costs a real caller pays on every call.
///
/// A failing call is fast, and a fast wrong number is worse than a missing one,
/// so a case that stops verifying fails the run rather than reporting a figure.
@inline(__always)
private func benchmarkVerify(
    _ benchmark: Benchmark,
    _ fixture: Fixture,
    _ compareCase: CompareCase
) async {
    benchmark.startMeasurement()
    let verified = await fixture.runAsync(compareCase)
    benchmark.stopMeasurement()

    precondition(verified, "\(compareCase.rawValue) failed to verify; refusing to report a figure")
    blackHole(verified)
}

public func verifyNotification(_ benchmark: Benchmark, _ fixture: Fixture) async {
    await benchmarkVerify(benchmark, fixture, .verifyNotification)
}

public func verifyTransaction(_ benchmark: Benchmark, _ fixture: Fixture) async {
    await benchmarkVerify(benchmark, fixture, .verifyTransaction)
}

public func verifyRenewalInfo(_ benchmark: Benchmark, _ fixture: Fixture) async {
    await benchmarkVerify(benchmark, fixture, .verifyRenewalInfo)
}
