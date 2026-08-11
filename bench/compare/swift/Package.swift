// swift-tools-version: 6.0
import PackageDescription

// The Swift arm of the cross-language comparison. One package, three targets:
//
//   Fixtures    the six cases and their setup, shared so both harnesses time
//               exactly the same definition of the work
//   Benchmarks  package-benchmark — this is what feeds the comparison table
//   runnerTests XCTest `measure {}` — a sanity check that does NOT feed it
//
// NOTE: the Benchmarks target needs jemalloc present at BUILD time
// (`brew install jemalloc`). It is not optional: package-benchmark fails to
// compile without `jemalloc/jemalloc.h` even when only `.wallClock` is
// requested and no malloc metrics are collected. `run.sh` checks for it and
// skips the arm with a clear message rather than letting the build fail.
let package = Package(
    name: "runner",
    platforms: [.macOS(.v13)],
    products: [
        .library(name: "Fixtures", targets: ["Fixtures"])
    ],
    dependencies: [
        .package(url: "https://github.com/ordo-one/package-benchmark.git", from: "1.22.0"),
        // Apple's published Swift library, tracked at the tip of `main` — the
        // same policy as the other arms, which resolve from their own
        // registries (npm, PyPI, and this repo by path for Rust).
        //
        // A branch dependency rather than a version range: this suite compares
        // the current state of each official library, and pinning Swift to a
        // release while Node and Python float would silently compare different
        // points in their histories. `Package.resolved` records the exact
        // commit each run measured, so a result stays reproducible after the
        // fact.
        .package(
            url: "https://github.com/apple/app-store-server-library-swift.git",
            branch: "main"
        )
    ],
    targets: [
        .target(
            name: "Fixtures",
            dependencies: [
                .product(name: "AppStoreServerLibrary", package: "app-store-server-library-swift")
            ]
        ),
        .executableTarget(
            name: "Bench",
            dependencies: [
                .product(name: "Benchmark", package: "package-benchmark"),
                "Fixtures",
            ],
            path: "Sources/Benchmarks",
            plugins: [
                .plugin(name: "BenchmarkPlugin", package: "package-benchmark")
            ]
        ),
        .testTarget(
            name: "runnerTests",
            dependencies: ["Fixtures"]
        ),
    ]
)
