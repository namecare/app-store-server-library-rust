use std::time::{SystemTime, UNIX_EPOCH};
use base64::engine::general_purpose::STANDARD;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use app_store_server_library::chain_verifier::ChainVerifier;
use app_store_server_library::primitives::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
use base64::Engine;

pub fn signed_payload() -> String {
    std::env::var("SIGNED_PAYLOAD").expect("SIGNED_PAYLOAD must be set")
}

pub fn apple_root_cert() -> String {
    std::env::var("APPLE_ROOT_BASE64_ENCODED").expect("APPLE_ROOT_BASE64_ENCODED must be set")
}

fn text_chain_verification() {


}

fn criterion_benchmark(c: &mut Criterion) {
    dotenv::dotenv().ok();

    let payload = signed_payload();
    let token = payload.as_str();
    let header = jsonwebtoken::decode_header(token).expect("Expect header");

    let Some(x5c) = header.x5c else {
        return;
    };

    let root_cert = apple_root_cert();
    let root_cert_der = STANDARD.decode(root_cert).expect("Expect bytes");

    let verifier = ChainVerifier::new(false, vec![root_cert_der]);

    let effective_date =  SystemTime::now();
    let since_the_epoch = effective_date
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");  // Define the effective date

    c.bench_function("verify", |b| b.iter(|| {
        let pub_key = verifier.verify(&x5c,  Some(since_the_epoch.as_secs())).expect("Expect pub key");
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);