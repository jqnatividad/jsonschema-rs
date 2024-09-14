use benchmark::Benchmark;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use serde_json::Value;
use valico::json_schema;

fn bench_compile(c: &mut Criterion, name: &str, schema: &Value) {
    c.bench_function(&format!("valico/{}/compile", name), |b| {
        b.iter(|| {
            let mut scope = json_schema::Scope::new();
            scope
                .compile_and_return(schema.clone(), false)
                .expect("Valid schema");
        })
    });
}

fn bench_validate(c: &mut Criterion, name: &str, schema: &Value, instance: &Value) {
    let mut scope = json_schema::Scope::new();
    let compiled = scope
        .compile_and_return(schema.clone(), false)
        .expect("Valid schema");
    c.bench_with_input(
        BenchmarkId::new(name, "validate"),
        instance,
        |b, instance| {
            b.iter(|| {
                compiled.validate(instance).is_valid();
            })
        },
    );
}

static UNSUPPORTED_BENCHMARKS: &[&str] = &["Open API"];

fn run_benchmarks(c: &mut Criterion) {
    for benchmark in Benchmark::iter() {
        benchmark.run(&mut |name, schema, instances| {
            if !UNSUPPORTED_BENCHMARKS.contains(&name) {
                bench_compile(c, name, schema);
                for instance in instances {
                    let name = format!("valico/{}/{}", name, instance.name);
                    bench_validate(c, &name, schema, &instance.data);
                }
            }
        });
    }
}

criterion_group!(valico, run_benchmarks);
criterion_main!(valico);