import Benchmark
import Fixtures

/// Times one receipt parse.
///
/// Synchronous, and deliberately not routed through any async bridge: these are
/// the cheapest cases in the suite, and wrapping a ~20 µs parse in machinery it
/// does not need would measure the machinery.
@inline(__always)
private func benchmarkReceipt(
    _ benchmark: Benchmark,
    _ fixture: Fixture,
    _ compareCase: CompareCase
) {
    benchmark.startMeasurement()
    let extracted = fixture.runSync(compareCase)
    benchmark.stopMeasurement()

    precondition(extracted, "\(compareCase.rawValue) found no transaction id; refusing to report a figure")
    blackHole(extracted)
}

public func receiptApp(_ benchmark: Benchmark, _ fixture: Fixture) {
    benchmarkReceipt(benchmark, fixture, .receiptApp)
}

public func receiptAppLegacy(_ benchmark: Benchmark, _ fixture: Fixture) {
    benchmarkReceipt(benchmark, fixture, .receiptAppLegacy)
}
