use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Benchmark function
fn benchmark_deserialize(c: &mut Criterion) {
    use agil_integration::de::de_str;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Test {
        #[serde(deserialize_with = "de_str")]
        pub data: i32,
    }

    let input = r#"{
        "data": "12311"
    }"#;

    c.bench_function("de_str", |b| {
        b.iter(|| {
            let _: Result<Test, _> = black_box(serde_json::from_str(input));
        })
    });
}

criterion_group!(benches, benchmark_deserialize);
criterion_main!(benches);
