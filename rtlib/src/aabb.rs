use super::vect;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::{MaterialType, NoneMaterial};
use crate::ray::Ray;
use crate::util::{ffmax, ffmin};
///! aabb: or Axis-Aligned Bounding Box
///
///
use crate::vec3::Vec3;
use std::{mem, cmp::Ordering};

#[derive(Clone, Copy, Debug)]
pub enum BoundingBox {
    Aabb(Aabb),
    AabbF(AabbF),
    Empty,
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox::Empty
    }
}

impl BoundingBox {
    pub fn get(&self) -> Option<Box<dyn AABB>> {
        match self {
            BoundingBox::Aabb(x) => Some(Box::new(x.clone())),
            BoundingBox::AabbF(x) => Some(Box::new(x.clone())),
            BoundingBox::Empty => None,
        }
    }

    pub fn expand_to_contain(
        bb0: Option<BoundingBox>,
        bb1: Option<BoundingBox>,
    ) -> Option<BoundingBox> {
        // have to check the corners: because the bb is strictly defined as
        // an axis aligned bb with the corners defined to contain everything inside
        // that means that we should only have to compare the lower corners to themselves
        // and the upper corners to themselves and take the lesser/greater of them.
        let mut bbmin: Option<Vec3>;
        let mut bbmax: Option<Vec3>;

        if bb0.is_some() == true {
            let bbfirst = &bb0.unwrap() as &dyn AABB;
            bbmin = Some(bbfirst.min());
            bbmax = Some(bbfirst.max());
            if bb1.is_some() {
                let tmpmin = bbmin.unwrap();
                let tmpmax = bbmax.unwrap();
                let bbsecond = &bb1.unwrap() as &dyn AABB;
                // we have both, so we can compare
                // we should have gotten Vec3 from the min() calls
                let x = bbsecond.min().x.minimum(tmpmin.x);
                let y = bbsecond.min().y.minimum(tmpmin.y);
                let z = bbsecond.min().z.minimum(tmpmin.z);

                bbmin.replace(vect!(x, y, z));
                let x = bbsecond.max().x.maximum(tmpmax.x);
                let y = bbsecond.max().y.maximum(tmpmax.y);
                let z = bbsecond.max().z.maximum(tmpmax.z);
                bbmax.replace(vect!(x, y, z));
            }
            // else if here we have the first as optional, but not the second
            // that means that the assignments we have are what we want
            // could just return. If it was the first answer, then we've set them
            // to their new values
            return Some(BoundingBox::AabbF(AabbF::new(
                bbmin.unwrap(),
                bbmax.unwrap(),
            )));
        } else if bb1.is_some() {
            // then bb0 is none, and the answer is just bb1
            return bb1.clone();
        } else {
            //both are None, return Empty
            return Some(BoundingBox::Empty);
        }
    }

    pub fn cmp_by_x(lhs: & dyn Hittable, rhs: & dyn Hittable) -> Option<Ordering> {
        let box_left = lhs.bounding_box(0., 0.);
        let box_right = rhs.bounding_box(0., 0.);

        if box_left.is_some() {
            if box_right.is_some() {
                return (box_left.unwrap().min().x -
                             box_right.unwrap().min().x).partial_cmp(&0.0);
            }
            else {
                // have lhs, don't have rhs
                return Some(Ordering::Greater);
            }
            
        } else {
            //lhs is None
            if box_right.is_some() {
                return Some(Ordering::Less);
            }
            else {
                // neither has a value
                eprintln!("no bounding box in bvhnode compare!");
                return None;
            }
        }
    }

    pub fn cmp_by_y(lhs: & dyn Hittable, rhs: & dyn Hittable) -> Option<Ordering> {
        let box_left = lhs.bounding_box(0., 0.);
        let box_right = rhs.bounding_box(0., 0.);

        if box_left.is_some() {
            if box_right.is_some() {
                return (box_left.unwrap().min().y -
                             box_right.unwrap().min().y).partial_cmp(&0.0);
            }
            else {
                // have lhs, don't have rhs
                return Some(Ordering::Greater);
            }
            
        } else {
            //lhs is None
            if box_right.is_some() {
                return Some(Ordering::Less);
            }
            else {
                // neither has a value
                eprintln!("no bounding box in bvhnode compare!");
                return None;
            }
        }
    }

    pub fn cmp_by_z(lhs: & dyn Hittable, rhs: & dyn Hittable) -> Option<Ordering> {
        let box_left = lhs.bounding_box(0., 0.);
        let box_right = rhs.bounding_box(0., 0.);

        if box_left.is_some() {
            if box_right.is_some() {
                return (box_left.unwrap().min().z -
                             box_right.unwrap().min().z).partial_cmp(&0.0);
            }
            else {
                // have lhs, don't have rhs
                return Some(Ordering::Greater);
            }
            
        } else {
            //lhs is None
            if box_right.is_some() {
                return Some(Ordering::Less);
            }
            else {
                // neither has a value
                eprintln!("no bounding box in bvhnode compare!");
                return None;
            }
        }
    }

    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundingBox::Aabb(x) => {
                return write!(f, "Aabb::BoundingBox: min(): {} max(): {}", x.min(), x.max());
            },
            BoundingBox::AabbF(x) => {
                return write!(f, "AabbF::BoundingBox: min(): {} max(): {}", x.min(), x.max());
            },
            _ => { return write!(f, "BoundingBox::Empty is empty.");},
        }
    }


}

impl PartialEq for BoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.min() == other.min() && self.max() == other.max() 
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(&other)
    }
}

impl std::fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.inner_fmt(f);
    }
}

impl AABB for BoundingBox {
    fn min(&self) -> Vec3 {
        match self {
            BoundingBox::Aabb(x) => x.min(),
            BoundingBox::AabbF(x) => x.min(),
            BoundingBox::Empty => {
                panic!("BoundingBox is Empty!")
            }
        }
    }
    fn max(&self) -> Vec3 {
        match self {
            BoundingBox::Aabb(x) => x.max(),
            BoundingBox::AabbF(x) => x.max(),
            BoundingBox::Empty => {
                panic!("BoundingBox is Empty!")
            }
        }
    }
}

impl Hittable for BoundingBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            BoundingBox::Aabb(x) => x.hit(r, t_min, t_max),
            BoundingBox::AabbF(x) => x.hit(r, t_min, t_max),
            BoundingBox::Empty => None,
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        match self {
            BoundingBox::Aabb(x) => x.bounding_box(t0, t1),
            BoundingBox::AabbF(x) => x.bounding_box(t0, t1),
            BoundingBox::Empty => None,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct AabbF {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

pub trait AABB {
    fn min(&self) -> Vec3;
    fn max(&self) -> Vec3;
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            minimum: min,
            maximum: max,
        }
    }
}

impl AabbF {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            minimum: min,
            maximum: max,
        }
    }
}

impl AABB for Aabb {
    fn min(&self) -> Vec3 {
        self.minimum
    }
    fn max(&self) -> Vec3 {
        self.maximum
    }
}
impl AABB for AabbF {
    fn min(&self) -> Vec3 {
        self.minimum
    }
    fn max(&self) -> Vec3 {
        self.maximum
    }
}

impl Hittable for Aabb {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let t0 = ffmin(
                (self.min().get(a) - r.origin().get(a)) / r.direction().get(a),
                (self.max().get(a) - r.origin().get(a)) / r.direction().get(a),
            );
            //(self.minimum.get(a) - r.origin().get(a)) / r.direction().get(a),
            //(self.maximum.get(a) - r.origin().get(a)) / r.direction().get(a));
            let t1 = ffmax(
                (self.min().get(a) - r.origin().get(a)) / r.direction().get(a),
                (self.max().get(a) - r.origin().get(a)) / r.direction().get(a),
            );
            //(self.minimum.get(a) - r.origin().get(a)) / r.direction().get(a),
            //(self.maximum.get(a) - r.origin().get(a)) / r.direction().get(a));
            t_min = ffmax(t0, t_min);
            t_max = ffmin(t1, t_max);
            if t_max <= t_min {
                return None;
            }
        }
        Some(HitRecord {
            t: 0.0,
            p: vect!(0, 0, 0),
            normal: vect!(0, 0, 0),
            material: MaterialType::Nothing(NoneMaterial),
            texture_coord: None,
            front_face: false,
        })
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::Aabb(self.clone()))
    }
}

impl Hittable for AabbF {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let inv_d: f64 = 1.0_f64 / r.direction().get(a);
            let mut t0 = (self.min().get(a) - r.origin().get(a)) * inv_d;
            let mut t1 = (self.max().get(a) - r.origin().get(a)) * inv_d;
            //let mut t0 = (self.minimum.get(a) - r.origin().get(a)) * inv_d;
            //let mut t1 = (self.maximum.get(a) - r.origin().get(a)) * inv_d;
            if inv_d < 0. {
                mem::swap(&mut t0, &mut t1);
            }
            if t0 > t_min {
                t_min = t0;
            }
            if t1 < t_max {
                t_max = t1;
            }

            if t_max <= t_min {
                return None;
            }
        }
        Some(HitRecord {
            t: 0.0,
            p: vect!(0, 0, 0),
            normal: vect!(0, 0, 0),
            material: MaterialType::Nothing(NoneMaterial),
            texture_coord: None,
            front_face: false,
        })
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::AabbF(self.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::vect;
    use super::Hittable;
    use super::Ray;
    use super::{Aabb, AabbF, BoundingBox, AABB};
    use std::cmp::Ordering;

    #[test]
    fn test_hit_aabb() {
        let hitbox = Aabb::new(vect!(0, 0, 0), vect!(256, 256, 256));
        let testray1 = Ray::new(&vect!(0, 0, 0.9), &vect!(128, 128, -512), None);
        let testray2 = Ray::new(&vect!(0, 0, -0.9), &vect!(128, 128, -512), None);

        assert!(hitbox.hit(&testray1, f64::MIN, f64::MAX).is_none() == false);
        assert!(hitbox.hit(&testray2, f64::MIN, f64::MAX).is_none() == true);
    }

    #[test]
    fn test_hit_aabb_f() {
        let hitbox = AabbF::new(vect!(0, 0, 0), vect!(256, 256, 256));
        let testray1 = Ray::new(&vect!(0, 0, 0.9), &vect!(128, 128, -512), None);
        let testray2 = Ray::new(&vect!(0, 0, -0.9), &vect!(128, 128, -512), None);

        assert!(hitbox.hit(&testray1, f64::MIN, f64::MAX).is_none() == false);
        assert!(hitbox.hit(&testray2, f64::MIN, f64::MAX).is_none() == true);
    }

    #[test]

    fn test_get_aabb() {
        let hb1 = BoundingBox::Empty;
        let hb2 = BoundingBox::default();
        let hb3 = BoundingBox::Aabb(Aabb::new(vect!(0, 0, 0), vect!(1, 1, 1)));
        let hb4 = BoundingBox::AabbF(AabbF::new(vect!(0, 0, 0), vect!(1, 1, 1)));

        assert_eq!(hb1.get().is_none(), true);
        assert_eq!(hb2.get().is_none(), true);
        assert_eq!(hb3.min(), vect!(0, 0, 0));
        assert_eq!(hb3.max(), vect!(1, 1, 1));
        assert_eq!(hb4.min(), vect!(0, 0, 0));
        assert_eq!(hb4.max(), vect!(1, 1, 1));
    }

    #[test]
    fn test_cmp_by_x() {
        let hb1 = BoundingBox::Empty;
        let hb2 = BoundingBox::default(); // this is also empty
        let hb3 = BoundingBox::Aabb(Aabb::new(vect!(0, 0, 0), vect!(1, 1, 1)));
        let hb4 = BoundingBox::AabbF(AabbF::new(vect!(1, 1, 1), vect!(2, 2, 2)));
        
        // should not compare. may change to equals instead
        assert_eq!(BoundingBox::cmp_by_x(&hb1, &hb1), None);
        assert_eq!(BoundingBox::cmp_by_y(&hb1, &hb1), None);
        assert_eq!(BoundingBox::cmp_by_z(&hb1, &hb1), None);

        // these compare default(Empty) vs Empty
        assert_eq!(BoundingBox::cmp_by_x(&hb1, &hb2), None);
        assert_eq!(BoundingBox::cmp_by_y(&hb2, &hb1), None);
        assert_eq!(BoundingBox::cmp_by_z(&hb2, &hb1), None);

        // compare something vs empty
        // first x,y,z with ordering 2,3
        assert_eq!(BoundingBox::cmp_by_x(&hb2, &hb3), Some(Ordering::Less));
        assert_eq!(BoundingBox::cmp_by_y(&hb2, &hb3), Some(Ordering::Less));
        assert_eq!(BoundingBox::cmp_by_z(&hb2, &hb3), Some(Ordering::Less));
        // then x,y,z with ordering 3,2
        assert_eq!(BoundingBox::cmp_by_x(&hb3, &hb2), Some(Ordering::Greater));
        assert_eq!(BoundingBox::cmp_by_y(&hb3, &hb2), Some(Ordering::Greater));
        assert_eq!(BoundingBox::cmp_by_z(&hb3, &hb2), Some(Ordering::Greater));

        // then 2 values
        assert_eq!(BoundingBox::cmp_by_x(&hb3, &hb4), Some(Ordering::Less));
        assert_eq!(BoundingBox::cmp_by_y(&hb3, &hb4), Some(Ordering::Less));
        assert_eq!(BoundingBox::cmp_by_z(&hb3, &hb4), Some(Ordering::Less));
        // and flip their order
        assert_eq!(BoundingBox::cmp_by_x(&hb4, &hb3), Some(Ordering::Greater));
        assert_eq!(BoundingBox::cmp_by_y(&hb4, &hb3), Some(Ordering::Greater));
        assert_eq!(BoundingBox::cmp_by_z(&hb4, &hb3), Some(Ordering::Greater));
        // and equal
        assert_eq!(BoundingBox::cmp_by_z(&hb3, &hb3), Some(Ordering::Equal));
        assert_eq!(BoundingBox::cmp_by_z(&hb4, &hb4), Some(Ordering::Equal));

    }
}
