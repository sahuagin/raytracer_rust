use criterion::{criterion_group, criterion_main, Criterion};
use rtlib::aabb::AabbF;
use rtlib::hittable::Hittable;
use rtlib::prelude::Vec3;
use rtlib::ray::Ray;

fn bench_hit_aabb_f() {
    let hitbox = AabbF::new(Vec3::new(0, 0, 0), Vec3::new(256, 256, 256));
    let testray1 = Ray::new(&Vec3::new(0, 0, 0.9), &Vec3::new(128, 128, -512), None);
    let testray2 = Ray::new(&Vec3::new(0, 0, -0.9), &Vec3::new(128, 128, -512), None);

    assert!(hitbox.hit(&testray1, f64::MIN, f64::MAX).is_some());
    assert!(hitbox.hit(&testray2, f64::MIN, f64::MAX).is_none());
}

fn bench_aabb(c: &mut Criterion) {
    c.bench_function("test_hit_aabb", |b| b.iter( bench_hit_aabb_f ));
}

criterion_group!(benches, bench_aabb);
criterion_main!(benches);
