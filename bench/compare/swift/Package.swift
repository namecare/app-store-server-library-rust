// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "compare-runner",
    platforms: [.macOS(.v13)],
    dependencies: [
        // Apple's published Swift library, tracked at the tip of `main` — the
        // same policy as the other arms, which resolve from their own
        // registries (npm `@apple/app-store-server-library`, PyPI
        // `app-store-server-library`).
        //
        // A branch dependency rather than a version range: this suite compares
        // the current state of each official library, and pinning Swift to a
        // release while Node and Python float would silently compare different
        // points in their histories. `Package.resolved` records the exact
        // commit each run measured, so a result is still reproducible after
        // the fact.
        .package(
            url: "https://github.com/apple/app-store-server-library-swift.git",
            branch: "main"
        )
    ],
    targets: [
        .executableTarget(
            name: "runner",
            dependencies: [
                .product(name: "AppStoreServerLibrary", package: "app-store-server-library-swift")
            ]
            // Optimisation comes from `swift build -c release` (which implies
            // -O). `.unsafeFlags` is rejected by SwiftPM for some dependency
            // graphs, so we rely on the build configuration instead.
        )
    ]
)
