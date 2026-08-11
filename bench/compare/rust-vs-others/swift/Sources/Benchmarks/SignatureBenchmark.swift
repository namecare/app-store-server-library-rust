//===----------------------------------------------------------------------===//
//
// The promotional-offer signature case: an ECDSA P-256 signature over the
// offer's fields.
//
//===----------------------------------------------------------------------===//

import Benchmark
import Fixtures

/// Times one promotional-offer signature.
///
/// Synchronous, like the receipt cases — the library's signing API is not
/// `async`, so nothing is bridged.
public func signPromotionalOffer(_ benchmark: Benchmark, _ fixture: Fixture) {
    benchmark.startMeasurement()
    let signed = fixture.runSync(.signPromotionalOffer)
    benchmark.stopMeasurement()

    precondition(signed, "sign_promotional_offer produced no signature; refusing to report a figure")
    blackHole(signed)
}
