use criterion::{black_box, criterion_group, criterion_main, Criterion};
use referencing::{Draft, Registry};
use serde_json::json;

fn bench_anchor_lookup(c: &mut Criterion) {
    let data = json!({
      "definitions": {
        "foo": {
          "id": "#foo",
          "foo": "bar"
        }
      }
    });
    let resource = Draft::Draft4.create_resource(data);
    let registry =
        Registry::try_new("http://example.com/", resource).expect("Invalid registry input");

    let mut group = c.benchmark_group("Anchor Lookup");

    // Benchmark lookup of existing anchor
    group.bench_function("lookup", |b| {
        b.iter(|| {
            let resolver = registry
                .try_resolver("http://example.com/")
                .expect("Invalid base URI");
            let _resolved = resolver.lookup(black_box("#foo"));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_anchor_lookup);
criterion_main!(benches);