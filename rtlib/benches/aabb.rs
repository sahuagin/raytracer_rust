use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rtlib::aabb::{Aabb, AabbF};
use rtlib::hittable::Hittable;
use rtlib::prelude::Vec3;
use rtlib::ray::Ray;

fn bench_hit_aabb() {
    let hitbox = Aabb::new(Vec3::new(0, 0, 0), Vec3::new(256, 256, 256));
    let testray1 = Ray::new(&Vec3::new(0, 0, 0.9), &Vec3::new(128, 128, -512), None);
    let testray2 = Ray::new(&Vec3::new(0, 0, -0.9), &Vec3::new(128, 128, -512), None);

    assert!(hitbox.hit(&testray1, f64::MIN, f64::MAX).is_some());
    assert!(hitbox.hit(&testray2, f64::MIN, f64::MAX).is_none());
}

fn bench_hit_aabb_f() {
    let hitbox = AabbF::new(Vec3::new(0, 0, 0), Vec3::new(256, 256, 256));
    let testray1 = Ray::new(&Vec3::new(0, 0, 0.9), &Vec3::new(128, 128, -512), None);
    let testray2 = Ray::new(&Vec3::new(0, 0, -0.9), &Vec3::new(128, 128, -512), None);

    assert!(hitbox.hit(&testray1, f64::MIN, f64::MAX).is_some());
    assert!(hitbox.hit(&testray2, f64::MIN, f64::MAX).is_none());
}

fn bench_aabbs(c: &mut Criterion) {
    let mut group = c.benchmark_group("aabb");
    for i in 0..10 {
        group.bench_function(BenchmarkId::new("Shirley", i), |b| {
            b.iter(bench_hit_aabb)
        });
        group.bench_function(BenchmarkId::new("Pixar", i), |b| {
            b.iter(bench_hit_aabb_f)
        });
    }
}

//fn criterion_benchmark(c: &mut Criterion) {
//    c.bench_function("test_hit_aabb", |b| b.iter(|| bench_hit_aabb()));
//}

criterion_group!(benches, bench_aabbs);
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
