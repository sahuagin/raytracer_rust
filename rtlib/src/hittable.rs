use super::aabb::BoundingBox;
use super::bvh::{Bvh, BvhNode};
use super::cube::Cube;
use super::hitlist::HitList;
#[allow(unused_imports)]
use super::materials::{Material, MaterialType};
use super::ray::Ray;
use super::rectangle::Rect;
use super::sphere::{MovingSphere, Sphere};
use super::vec3::{Point3, Vec3};
use std::{cmp::PartialEq, fmt};

#[allow(unused_imports, dead_code)]
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn box_clone(&self) -> Box<dyn Hittable>;

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox>;

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl Hittable for Hitters {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hitters::HitList(x) => x.hit(r, t_min, t_max),
            Hitters::Sphere(x) => x.hit(r, t_min, t_max),
            Hitters::MovingSphere(x) => x.hit(r, t_min, t_max),
            Hitters::BoundingBox(x) => x.hit(r, t_min, t_max),
            Hitters::Cube(x) => x.hit(r, t_min, t_max),
            Hitters::FlipNormal(x) => x.hit(r, t_min, t_max),
            Hitters::BVolumeHierarchy(x) | Hitters::BvhNode(x) => x.hit(r, t_min, t_max),
            Hitters::Rect(x) => x.hit(r, t_min, t_max),
            Hitters::Custom(x) => x.hit(r, t_min, t_max),
            Hitters::Nothing(_x) => None,
        }
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        match self {
            Hitters::HitList(x) => x.bounding_box(t0, t1),
            Hitters::Sphere(x) => x.bounding_box(t0, t1),
            Hitters::MovingSphere(x) => x.bounding_box(t0, t1),
            Hitters::BoundingBox(x) => x.bounding_box(t0, t1),
            Hitters::Cube(x) => x.bounding_box(t0, t1),
            Hitters::BVolumeHierarchy(x) | Hitters::BvhNode(x) => x.bounding_box(t0, t1),
            Hitters::FlipNormal(x) => x.bounding_box(t0, t1),
            Hitters::Rect(x) => x.bounding_box(t0, t1),
            Hitters::Custom(x) => x.bounding_box(t0, t1),
            Hitters::Nothing(_x) => None,
        }
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hitters::HitList(x) => x.inner_fmt(f),
            Hitters::Sphere(x) => x.inner_fmt(f),
            Hitters::MovingSphere(x) => x.inner_fmt(f),
            Hitters::BoundingBox(x) => x.inner_fmt(f),
            Hitters::Cube(x) => x.inner_fmt(f),
            Hitters::BVolumeHierarchy(x) | Hitters::BvhNode(x) => x.inner_fmt(f),
            Hitters::FlipNormal(x) => x.inner_fmt(f),
            Hitters::Rect(x) => x.inner_fmt(f),
            Hitters::Custom(x) => x.hitter_fmt(f),
            Hitters::Nothing(_x) => write!(f, "Hitter::Nothing"),
        }
    }
}

impl std::fmt::Display for Hitters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.hitter_fmt(f)
    }
}

// create a tuple struct. The hittable will be at self.0
#[derive(Clone)]
pub struct FlipNormal(Box<dyn Hittable>);

impl FlipNormal {
    pub fn new(hit: &dyn Hittable) -> Self {
        FlipNormal(hit.box_clone())
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlipNormal of ")?;
        self.0.as_ref().hitter_fmt(f)
    }
}

impl Hittable for FlipNormal {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let hr = self.0.hit(r, tmin, tmax);
        match hr {
            Some(mut x) => {
                x.normal *= -1.0;
                Some(x)
            }
            None => None,
        }
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        self.0.bounding_box(t0, t1)
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.hitter_fmt(f)
    }
}

#[derive(Clone)]
pub struct Custom(Box<dyn Hittable>);

impl Custom {
    pub fn new(hit: &dyn Hittable) -> Self {
        Custom(hit.box_clone())
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom: ")?;
        self.0.as_ref().hitter_fmt(f)
    }
}

impl Hittable for Custom {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.0.hit(r, tmin, tmax)
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        self.0.bounding_box(t0, t1)
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.hitter_fmt(f)
    }
}

#[derive(Clone, Copy, Default)]
pub struct NoBatter;
impl Hittable for NoBatter {
    fn hit(&self, _r: &Ray, _f_tmin: f64, _f_tmax: f64) -> Option<HitRecord> {
        None
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(*self)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        None
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Swing, batter, batter, batter, batter, Swing, NoBatter!")
    }
}

unsafe impl Send for Hitters {}
unsafe impl Sync for Hitters {}

// all types that implement hittable will be represented in this wrapper
#[derive(Clone)]
pub enum Hitters {
    HitList(HitList),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    BoundingBox(BoundingBox),
    BVolumeHierarchy(Bvh),
    BvhNode(BvhNode),
    Cube(Cube),
    FlipNormal(FlipNormal),
    Rect(Rect),
    Custom(Custom),
    Nothing(NoBatter),
}

#[allow(unused_imports, dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TextureCoord {
    pub u: f64,
    pub v: f64,
}

impl PartialEq for TextureCoord {
    fn eq(&self, other: &Self) -> bool {
        //eprintln!("TextureCoord::PartialEq({}, {})", &self, &other);
        let my_epsilon: f64 = 0.0001_f64;
        if (self.u - other.u).abs() < my_epsilon && (self.v - other.v).abs() < my_epsilon {
            return true;
        }
        false
    }
}

impl std::fmt::Display for TextureCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TextureCoord: u: {} v: {}", &self.u, &self.v)
    }
}

#[allow(unused_imports, dead_code)]
#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: MaterialType,
    pub texture_coord: Option<TextureCoord>,
    pub front_face: bool,
}

#[allow(unused_imports, dead_code)]
impl HitRecord {
    pub fn new(p: Point3, t: f64, material: MaterialType) -> Self {
        HitRecord {
            p,
            normal: p,
            material,
            t,
            texture_coord: Some(TextureCoord {
                u: f64::default(),
                v: f64::default(),
            }),
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, mut outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal *= -1.0;
            outward_normal
        };
    }
}

impl fmt::Debug for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HitRecord")
            .field("t", &self.t)
            .field("p", &self.p)
            .field("normal", &self.normal)
            .field("texture_coord", &self.texture_coord)
            .field("front_face", &self.front_face)
            .finish()
    }
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        if self.t == other.t &&
           self.p == other.p &&
           self.normal == other.normal &&
           //self.material == other.material &&
           self.texture_coord.unwrap_or_default() ==
                other.texture_coord.unwrap_or_default() &&
           self.front_face == other.front_face
        {
            return true;
        }
        false
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.box_clone()
    }
}

impl std::fmt::Display for Box<dyn Hittable> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().hitter_fmt(f)
    }
}

impl std::fmt::Display for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HitRecord: p: {}, normal: {}, material: ",
            self.p, self.normal
        )?;
        self.material.inner_fmt(f)?;
        write!(f, ", t: {}, front_face: {}", self.t, self.front_face)
    }
}

impl Hittable for std::boxed::Box<dyn Hittable> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.as_ref().hit(r, tmin, tmax)
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.as_ref().box_clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        self.as_ref().bounding_box(t0, t1)
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().hitter_fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::{FlipNormal, HitRecord, Hittable, TextureCoord};
    use crate::materials::{DiffuseLight, MaterialType};
    use crate::ray::Ray;
    use crate::rectangle::{Axis, Rect};
    use crate::textures::{ConstantTexture, TextureType};
    use crate::vect;

    #[test]
    fn test_flip_normal_create() {
        let yzrect = Rect::new(
            -1.0,
            1.0,
            -1.0,
            1.0,
            0.0,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
            Axis::X,
        );

        let r = Ray::new(&vect!(1, 0, 0), &vect!(-1, 0, 0), None);

        let mut hr_ans = HitRecord::new(
            vect!(0, 0, 0),
            1.0,
            MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );
        hr_ans.normal = vect!(1, 0, 0);
        hr_ans.texture_coord = Some(TextureCoord { u: 0.5, v: 0.5 });

        let hr = yzrect.hit(&r, 0.0, 1.0);

        let flipnorm = FlipNormal::new(&yzrect);
        let flipped_hr = flipnorm.hit(&r, 0.0, 1.0);

        assert_eq!(hr.unwrap().normal, flipped_hr.unwrap().normal * -1.);
    }
}
