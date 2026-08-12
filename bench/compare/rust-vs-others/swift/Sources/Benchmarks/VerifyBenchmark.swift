import Benchmark
import Fixtures

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
