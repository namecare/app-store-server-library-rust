// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "runner",
    platforms: [.macOS(.v13)],
    products: [
        .library(name: "Fixtures", targets: ["Fixtures"])
    ],
    dependencies: [
        .package(url: "https://github.com/ordo-one/package-benchmark.git", from: "1.22.0"),
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
