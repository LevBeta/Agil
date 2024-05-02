use agil_data::exchange::bybit::subscription::BybitResponse;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_json::serde::from_slice;

fn criterion_benchmark(c: &mut Criterion) {
    let mut input = r#"
                        {
                            "success": true,
                            "ret_msg": "subscribe",
                            "conn_id": "2324d924-aa4d-45b0-a858-7b8be29ab52b",
                            "req_id": "10001",
                            "op": "subscribe"
                        }
                    "#;

    let mut input_x = input.as_bytes().to_vec();
    c.bench_function("deserialize_bybit_response_simd_json", |b| {
        b.iter(|| {
            //let mut x = input.as_bytes().to_vec();
            simd_json::from_slice::<BybitResponse>(black_box(&mut input_x)).unwrap();
        })
    });

    c.bench_function("deserialize_bybit_response_serde_json", |b| {
        b.iter(|| {
            //let mut x = input.as_bytes().to_vec();
            let _: BybitResponse = serde_json::from_str(black_box(input)).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
