//===----------------------------------------------------------------------===//
//
// The Swift arm of the app-store-server-library cross-language benchmark
// comparison. See ../../../../README.md.
//
// Style follows apple/swift-certificates' CertificatesBenchmark: this file
// registers the benchmarks and holds the shared configuration, and each
// benchmark's body lives in its own file next to it.
//
//===----------------------------------------------------------------------===//

import Benchmark
import Fixtures

/// Matches every other arm of the comparison: 50 warmup iterations, then 500
/// timed samples.
let warmupIterations = 50
let iterations = 500

let benchmarks: @Sendable () -> Void = {
    Benchmark.defaultConfiguration = .init(
        // Wall clock only. This suite compares time, not allocations, and
        // requesting malloc metrics would make jemalloc a runtime concern on top
        // of the build-time one.
        metrics: [.wallClock],
        // Deliberately NOT `scalingFactor: .kilo`. swift-certificates scales up
        // and wraps `for _ in benchmark.scaledIterations` around its work, which
        // times a loop per sample. Every other arm here times ONE call per
        // sample, and a table whose cells are only comparable if measured the
        // same way cannot afford that difference.
        warmupIterations: warmupIterations,
        // The run ends at whichever of maxDuration/maxIterations is reached
        // first, so this stays far larger than 500 iterations could ever take —
        // that is what makes the iteration count the binding constraint. Same
        // idiom swift-certificates uses.
        maxDuration: .seconds(10_000_000),
        maxIterations: iterations
    )

    // A fixture that fails to build is a hard error: a benchmark that silently
    // measures nothing is worse than one that does not run.
    let fixture: Fixture
    do {
        fixture = try Fixture()
    } catch {
        fatalError("fixture setup failed: \(error)")
    }

    Benchmark("verify_notification") { benchmark in
        await verifyNotification(benchmark, fixture)
    }

    Benchmark("verify_transaction") { benchmark in
        await verifyTransaction(benchmark, fixture)
    }

    Benchmark("verify_renewal_info") { benchmark in
        await verifyRenewalInfo(benchmark, fixture)
    }

    Benchmark("receipt_app") { benchmark in
        receiptApp(benchmark, fixture)
    }

    Benchmark("receipt_app_legacy") { benchmark in
        receiptAppLegacy(benchmark, fixture)
    }

    Benchmark("sign_promotional_offer") { benchmark in
        signPromotionalOffer(benchmark, fixture)
    }
}
