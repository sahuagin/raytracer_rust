pub mod raytracer;
use rand::Rng;
use self::raytracer::ray::{Ray};
use self::raytracer::vec3::{Vec3, Color, unit_vector, dot};
use self::raytracer::materials::Material;

#[allow(unused_imports, dead_code)]
pub fn color(ray: &Ray, world: &HitList, depth: i32) -> Color {
    // the 0.001 ignores hits very close to 0, which handles issues with
    // floating point approximation, which generates "shadow acne"
    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        if depth <= 0 {
            return Color::default();
        }
        if let Some((attenuation, sray)) = hit_record
            .material
            .as_ref()
            .unwrap()
            .scatter(&ray, &hit_record) {
                return attenuation * color(&sray, &world, depth-1);
        }
        else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }
    //let unit_direction = ray.direction().normalize();
    let unit_direction = unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

#[allow(unused_imports, dead_code)]
pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4_f64*a*c;
    if discriminant < 0_f64 {
        return -1.0;
    }
    else
    {
        return (-b - discriminant.sqrt()) / (2.0*a);
    }

}

#[allow(unused_imports, dead_code)]
pub trait Hittable {
    fn hit(&self,
            r: &Ray,
            t_min: f64,
            t_max: f64) -> Option<HitRecord>;
    
}

#[allow(unused_imports, dead_code)]
//#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HitRecord<'world> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<&'world dyn Material>,
    pub front_face: bool,
}

#[allow(unused_imports, dead_code)]
impl<'world> HitRecord<'world> {
    pub fn new(p: Point3, t: f64, material: Option<&'world dyn Material>) -> Self {
        HitRecord {
            p,
            normal: p,
            material,
            t,
            front_face: false,
        }
    }
}

#[allow(unused_imports, dead_code)]
#[derive(Default)]
pub struct HitList {
    // we could also call this 'objects' but where's the fun in that?
    pub list: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HitList {
    //pub fn push(&mut self, obj: dyn Hittable) {
    //    self.hitlist.push(Box::new(obj));
    //}
    pub fn new() -> HitList {
        HitList {
            list: Vec::new(),
        }
    }
    
    pub fn add(&mut self, object: impl Hittable + Sync + Send + 'static){
        self.list.push(Box::new(object))
    }
}

impl Hittable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far: f64 = t_max;
        for obj in &self.list {
            if let Some(rec) = obj.hit(&r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec.replace(rec);
            }
        }
        temp_rec
    }
    
}

#[allow(unused_imports, dead_code)]
pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Option<Vec3> = None;
    let mut rng = rand::thread_rng();
    
    loop {
        p.replace(2.0 * Vec3::new(rng.gen::<f64>(),rng.gen::<f64>(),rng.gen::<f64>()  ) - Vec3::new(1.0, 1.0, 1.0));
        if p.unwrap().length_squared() >= 1.0 {
            break;
        }
    }
    
    p.unwrap()
}

#[allow(unused_imports, dead_code)]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (2_f64 * v.dot(n) * *n)
}


#[allow(unused_imports, dead_code)]
pub fn refract(v: &Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit();
    let dt:f64 = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt*(1_f64-dt*dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt*(uv - n*dt) - n * discriminant.sqrt();
        return Some(refracted);
    }
    else {
        return None;
    }
}

#[cfg(test)]

#[test]
fn test_color() {
    let v = Vec3::new(0.0, 0.0, 0.0);
    let v2 = Vec3::new(1.0, 1.0, 1.0);
    let r = Ray::new(&v, &v2);
    let ans = Color { x: 0.21132486540518708, y: 0.21132486540518708, z: 0.21132486540518708 };
    let mut world = HitList::new();
    world.list.push(Box::new(sphere::Sphere::new(&Vec3::new(2.0, 2.0, 2.0), 3.0)));
    let c = color(&r, &world);
    assert_eq!(c, ans );
}

#[allow(unused_imports)]
use crate::raytracer::vec3::Point3;
#[allow(unused_imports)]
use crate::raytracer::sphere;
#[test]
fn test_hitlist() {
    let _ans = true;
    
    // steal test_sphere_hit data
    let pt1 = Point3::new(0.0,0.0,0.0);
    let pt2 = Point3::new(1.0,1.0,1.0);
    let r   = Ray::new(&pt1, &pt2);
    let center = Point3::new(2.0, 2.0, 2.0);
    let radius = 3.0;
    let s   = sphere::Sphere::new(&center, radius);
    let hitrec = Some(HitRecord { t: 0.26794919243112264,
                                p: Vec3 { x: 0.26794919243112264,
                                        y: 0.26794919243112264,
                                        z: 0.26794919243112264 },
                                normal: Vec3 { x: -0.5773502691896258,
                                    y: -0.5773502691896258,
                                    z: -0.5773502691896258
                                }});

    // then, we'll push the sphere into the HitList
    let mut hl = HitList::new();
    hl.list.push(Box::new(s));
    // this should have 2 hits, but we'll return the closest one
    let hit_ans = hl.hit(&r, 0.0, 4.0);
    assert_eq!(hit_ans, hitrec);
    println!("the hitrec i: {:?}", &hitrec);

}