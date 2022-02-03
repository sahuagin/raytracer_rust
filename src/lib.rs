pub mod raytracer;
use self::raytracer::ray::{Ray};
use self::raytracer::vec3::{Vec3, Color, unit_vector, dot};

#[allow(unused_imports, dead_code)]
pub fn color(r: &Ray) -> Color {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(&r.direction());
    let t: f64 = 0.5*(unit_direction.y + 1.0);
    return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
}

#[allow(unused_imports, dead_code)]
pub fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4_f64*a*c;
    discriminant > 0_f64
}

#[cfg(test)]

#[test]
fn test_color() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(77.7, 88.8, 12.34);
    let r = Ray::new(&v, &v2);
    let ans =  Vec3{ x: 0.5628763473545817, y: 0.737725808412749, z: 1.0 };
    let c = color(&r);
    assert_eq!(c, ans );
}

// not implemented
#[ignore]
#[test]
fn test_hit_sphere() {
    let _ans = true;
}