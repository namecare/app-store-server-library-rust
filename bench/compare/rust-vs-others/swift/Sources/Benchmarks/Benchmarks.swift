import Benchmark
import Fixtures

let warmupIterations = 50
let iterations = 500

let benchmarks: @Sendable () -> Void = {
    Benchmark.defaultConfiguration = .init(
        metrics: [.wallClock],
        warmupIterations: warmupIterations,
        maxDuration: .seconds(10_000_000),
        maxIterations: iterations
    )

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
