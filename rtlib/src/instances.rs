use crate::{
    aabb::{AabbF, BoundingBox, AABB},
    hittable::{HitRecord, Hittable},
    ray::Ray,
    rectangle::Axis,
    util::{self},
    vec3::Vec3,
    vect,
};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct TranslateHittable {
    instance: Box<dyn Hittable>,
    offset: Vec3,
}

impl TranslateHittable {
    pub fn new(instance: &dyn Hittable, offset: &Vec3) -> TranslateHittable {
        TranslateHittable {
            instance: instance.box_clone(),
            offset: *offset,
        }
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TranslateHittable: offset: {} of ", &self.offset)?;
        self.instance.hitter_fmt(f)
    }
}

impl Hittable for TranslateHittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(&(r.origin() - self.offset), &r.direction(), Some(r.time()));
        match self.instance.hit(&moved_ray, t_min, t_max) {
            Some(mut gothit) => {
                gothit.p += self.offset;
                Some(gothit)
            }
            None => None,
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        self.instance.bounding_box(t0, t1).map(|bb| BoundingBox::AabbF(AabbF::new(
                bb.min() + self.offset,
                bb.max() + self.offset,
            )))
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

#[derive(Clone)]
pub struct RotateHittable {
    instance: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    rotate_around: Axis,
    bbox: Option<BoundingBox>,
}

impl std::fmt::Display for RotateHittable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RotateHittable: sin_theta: {} cos_theta: {}, rotate_around: {}\nbbox: {}",
            &self.sin_theta,
            &self.cos_theta,
            &self.rotate_around,
            &self.bbox.unwrap_or_default()
        )
    }
}

impl std::fmt::Debug for RotateHittable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RotateHittable")
            .field("instance", &String::from("NoDebug"))
            .field("sin_theta", &self.sin_theta)
            .field("cos_theta", &self.cos_theta)
            .field("rotate_around", &self.rotate_around)
            .field("bbox", &self.bbox)
            .finish()
    }
}

impl RotateHittable {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(instance: &dyn Hittable) -> RotateHittableBuilder<util::No> {
        RotateHittableBuilder::new(&instance.box_clone())
    }

    #[allow(dead_code)]
    fn rotate(&self, torotate: &Vec3) -> Vec3 {
        match self.rotate_around {
            Axis::X => {
                let torotate = *torotate;
                let mut rotated = torotate;
                rotated.y = (self.cos_theta * torotate.y) - (self.sin_theta * torotate.z);
                rotated.z = (self.sin_theta * torotate.y) + (self.cos_theta * torotate.z);
                rotated
            }
            Axis::Y => {
                let torotate = *torotate;
                let mut rotated = torotate;
                rotated.x = (self.cos_theta * torotate.x) + (self.sin_theta * torotate.z);
                rotated.z = (-1. * self.sin_theta * torotate.x) + (self.cos_theta * torotate.z);
                rotated
            }
            Axis::Z => {
                let torotate = *torotate;
                let mut rotated = torotate;
                rotated.x = (self.cos_theta * torotate.x) - (self.sin_theta * torotate.y);
                rotated.y = (self.sin_theta * torotate.x) + (self.cos_theta * torotate.y);

                rotated
            }
        }
    }

    // Note: to unrotate just use -theta instead.
    // cos(-theta) = cos(theta) and sin(-theta) = -sin(theta)
    // so one won't require a change, and the other we just change the sign
    fn unrotate(&self, unrotate: &Vec3) -> Vec3 {
        match self.rotate_around {
            Axis::X => {
                let unrotate = *unrotate;
                let mut unrotated = unrotate;
                unrotated.y = (self.cos_theta * unrotate.y) - (-1. * self.sin_theta * unrotate.z);
                unrotated.z = (-1. * self.sin_theta * unrotate.y) + (self.cos_theta * unrotate.z);
                unrotated
            }
            Axis::Y => {
                let unrotate = *unrotate;
                let mut unrotated = unrotate;
                unrotated.x = (self.cos_theta * unrotate.x) + (-1. * self.sin_theta * unrotate.z);
                unrotated.z = (self.sin_theta * unrotate.x) + (self.cos_theta * unrotate.z);
                unrotated
            }
            Axis::Z => {
                let unrotate = *unrotate;
                let mut unrotated = unrotate;
                unrotated.x = (self.cos_theta * unrotate.x) - (-1. * self.sin_theta * unrotate.y);
                unrotated.y = (-1. * self.sin_theta * unrotate.x) + (self.cos_theta * unrotate.y);
                unrotated
            }
        }
    }
    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RotateHittable: sin_theta: {} cos_theta: {}, rotate_around: {}\nbbox: {}",
            &self.sin_theta,
            &self.cos_theta,
            &self.rotate_around,
            &self.bbox.unwrap_or_default()
        )
    }
}

pub struct RotateHittableBuilder<RotateHittableBuilderComplete>
where
    RotateHittableBuilderComplete: util::ToAssign,
{
    completed: PhantomData<RotateHittableBuilderComplete>,
    instance: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    rotate_around: Axis,
    bbox: Option<BoundingBox>,
}

impl<RotateHittableBuilderInitialized> RotateHittableBuilder<RotateHittableBuilderInitialized>
where
    RotateHittableBuilderInitialized: util::ToAssign,
{
    pub fn new(
        instance: &dyn Hittable,
    ) -> RotateHittableBuilder<RotateHittableBuilderInitialized> {
        RotateHittableBuilder {
            completed: PhantomData {},
            instance: instance.box_clone(),
            sin_theta: f64::default(),
            cos_theta: f64::default(),
            rotate_around: Axis::default(),
            bbox: None,
        }
    }

    pub fn with_rotate_around_angle(
        self,
        angle: f64,
        axis: Axis,
    ) -> RotateHittableBuilder<util::Yes> {
        let radians = (std::f64::consts::PI / 180.) * angle;
        let sin_theta: f64 = radians.sin();
        let cos_theta: f64 = radians.cos();
        let mut ret_self = RotateHittableBuilder {
            completed: PhantomData {},
            bbox: self.instance.bounding_box(0., 1.),
            instance: self.instance,
            sin_theta,
            cos_theta,
            rotate_around: axis,
        };

        if ret_self.bbox.is_none() {
            return ret_self;
        }

        let rotate_by: Box<dyn Fn(&Vec3) -> Vec3> = match axis {
            Axis::Y => {
                let rotate_y = |invec: &Vec3| {
                    Vec3::new(
                        cos_theta * invec.x + sin_theta * invec.z,
                        invec.y,
                        -1. * sin_theta * invec.x + cos_theta * invec.z,
                    )
                };
                Box::new(rotate_y)
            }
            Axis::X => {
                let rotate_x = |invec: &Vec3| {
                    Vec3::new(
                        invec.x,
                        cos_theta * invec.y - sin_theta * invec.z,
                        sin_theta * invec.y + cos_theta * invec.z,
                    )
                };
                Box::new(rotate_x)
            }
            Axis::Z => {
                let rotate_z = |invec: &Vec3| {
                    Vec3::new(
                        cos_theta * invec.x - sin_theta * invec.y,
                        sin_theta * invec.x + cos_theta * invec.y,
                        invec.z,
                    )
                };
                Box::new(rotate_z)
            }
        };

        // we tested that this was not none earlier
        let bbox = ret_self.bbox.unwrap();
        // otherwise we have a bounding box to rotate.
        let mut min: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
        let mut max: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];

        // NOTE: Ok. So, if we rotate around an axis by 90, we can transpose
        // 2 axis. If it's by 180, we just negate the 2 axis. However, if it's
        // 45 degrees, the pointy bits can stick out farther that the rotated
        // former min/max corners. The widest that the block could possibly be
        // would be the diaganol/distance between the to extreem points. Rather
        // Than figure out the perfect boundary cube, the underneath is an
        // algorithm that calculates every point at every corner, as well as a rough
        // estimate at the widest it could be at it's extreem. Then, each of the points
        // is rotated, and compared for min max.
        for i in 0..2 {
            let x: f64 = i as f64 * bbox.max().x + (1. - i as f64) * bbox.min().x;
            for j in 0..2 {
                let y: f64 = j as f64 * bbox.max().y + (1. - j as f64) * bbox.min().y;
                for k in 0..2 {
                    let z: f64 = k as f64 * bbox.max().z + (1. - k as f64) * bbox.min().y;
                    let tester: [f64; 3] = [x, y, z];
                    let tester: Vec3 = tester.into();
                    let tester: [f64; 3] = rotate_by(&tester).into();
                    for c in 0..tester.len() {
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                        }
                        if tester[c] < min[c] {
                            min[c] = tester[c];
                        }
                    }
                }
            }
        }

        ret_self.bbox = Some(BoundingBox::AabbF(AabbF::new(min.into(), max.into())));
        ret_self
    }

    pub fn with_rotate_around_y(self, angle: f64) -> RotateHittableBuilder<util::Yes> {
        self.with_rotate_around_angle(angle, Axis::Y)
    }

    pub fn with_rotate_around_x(self, angle: f64) -> RotateHittableBuilder<util::Yes> {
        self.with_rotate_around_angle(angle, Axis::X)
    }

    pub fn with_rotate_around_z(self, angle: f64) -> RotateHittableBuilder<util::Yes> {
        self.with_rotate_around_angle(angle, Axis::Z)
    }
}

impl<RotateHittableBuilderInitialized> RotateHittableBuilder<RotateHittableBuilderInitialized>
where
    RotateHittableBuilderInitialized: util::Assigned,
{
    pub fn build(self) -> RotateHittable {
        RotateHittable {
            instance: self.instance,
            sin_theta: self.sin_theta,
            cos_theta: self.cos_theta,
            rotate_around: self.rotate_around,
            bbox: self.bbox,
        }
    }
}

impl Hittable for RotateHittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let direction: Vec3 = self.unrotate(&r.direction());
        //eprintln!("r.direction: {:?} unrotated to direction: {:?}", &r.direction(), &direction);
        let origin: Vec3 = self.unrotate(&r.origin());
        //eprintln!("r.origin: {:?} unrotated to origin: {:?}", &r.origin(), &origin);
        let rotated_r = Ray::new(&origin, &direction, Some(r.time()));
        //eprintln!("Ray: {:?} unrotated_Ray: {:?}", r, rotated_r);

        match self.instance.hit(&rotated_r, t_min, t_max) {
            Some(hitrec) => {
                //eprintln!("Got a hit! {:?}", &hitrec);
                let p: Vec3 = self.rotate(&hitrec.p);
                let normal: Vec3 = self.rotate(&hitrec.normal);
                Some(HitRecord {
                    t: hitrec.t,
                    p,
                    normal,
                    material: hitrec.material,
                    texture_coord: hitrec.texture_coord,
                    front_face: hitrec.front_face,
                })
            }
            None => None,
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        self.bbox
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

#[allow(dead_code)]
fn bench_book_rotate_other(min_in: &Vec3, max_in: &Vec3, angle: f64, axis: Axis) -> (Vec3, Vec3) {
    let radians = (std::f64::consts::PI / 180.) * angle;
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();

    let rotate_by: Box<dyn Fn(&Vec3) -> Vec3> = match axis {
        Axis::Y => {
            let rotate_y = |invec: &Vec3| {
                Vec3::new(
                    cos_theta * invec.x + sin_theta * invec.z,
                    invec.y,
                    -1. * sin_theta * invec.x + cos_theta * invec.z,
                )
            };
            Box::new(rotate_y)
        }
        Axis::X => {
            let rotate_x = |invec: &Vec3| {
                Vec3::new(
                    invec.x,
                    cos_theta * invec.y - sin_theta * invec.z,
                    sin_theta * invec.y + cos_theta * invec.z,
                )
            };
            Box::new(rotate_x)
        }
        Axis::Z => {
            let rotate_z = |invec: &Vec3| {
                Vec3::new(
                    cos_theta * invec.x - sin_theta * invec.y,
                    sin_theta * invec.x + cos_theta * invec.y,
                    invec.z,
                )
            };
            Box::new(rotate_z)
        }
    };
    // otherwise we have a bounding box to rotate.
    let mut min: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
    let mut max: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];
    let rot_min: [f64; 3] = rotate_by(min_in).into();
    let rot_max: [f64; 3] = rotate_by(max_in).into();
    for i in 0..rot_min.len() {
        min[i] = rot_min[i].min(rot_max[i]);
        max[i] = rot_min[i].max(rot_max[i]);
    }
    (Vec3::from(min), Vec3::from(max))
}

#[allow(dead_code)]
fn bench_book_rotate_book(min_in: &Vec3, max_in: &Vec3, angle: f64, axis: Axis) -> (Vec3, Vec3) {
    let radians = (std::f64::consts::PI / 180.) * angle;
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();

    let rotate_by: Box<dyn Fn(&Vec3) -> Vec3> = match axis {
        Axis::Y => {
            let rotate_y = |invec: &Vec3| {
                Vec3::new(
                    cos_theta * invec.x + sin_theta * invec.z,
                    invec.y,
                    -1. * sin_theta * invec.x + cos_theta * invec.z,
                )
            };
            Box::new(rotate_y)
        }
        Axis::X => {
            let rotate_x = |invec: &Vec3| {
                Vec3::new(
                    invec.x,
                    cos_theta * invec.y - sin_theta * invec.z,
                    sin_theta * invec.y + cos_theta * invec.z,
                )
            };
            Box::new(rotate_x)
        }
        Axis::Z => {
            let rotate_z = |invec: &Vec3| {
                Vec3::new(
                    cos_theta * invec.x - sin_theta * invec.y,
                    sin_theta * invec.x + cos_theta * invec.y,
                    invec.z,
                )
            };
            Box::new(rotate_z)
        }
    };
    // otherwise we have a bounding box to rotate.
    let mut min: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
    let mut max: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let x: f64 = i as f64 * max_in.x + (1. - i as f64) * min_in.x;
                let y: f64 = j as f64 * max_in.y + (1. - j as f64) * min_in.y;
                let z: f64 = k as f64 * max_in.z + (1. - k as f64) * min_in.z;
                let tester: [f64; 3] = rotate_by(&vect!(x, y, z)).into();
                for c in 0..3 {
                    if tester[c] > max[c] {
                        max[c] = tester[c];
                    }
                    if tester[c] < min[c] {
                        min[c] = tester[c];
                    }
                }
            }
        }
    }
    (Vec3::from(min), Vec3::from(max))
}

#[cfg(test)]
mod test {
    use super::{RotateHittable, TranslateHittable};
    use crate::{
        aabb::{AabbF, BoundingBox},
        cube::Cube,
        hittable::{HitRecord, Hittable, TextureCoord},
        materials::{DiffuseLight, MaterialType},
        ray::Ray,
        textures::{ConstantTexture, TextureType},
        vec3::Vec3,
        vect,
    };

    #[test]
    fn test_translate_cube() {
        let p0 = vect!(-1, -1, -1);
        let p1 = vect!(1, 1, 1);
        let cube_initial = Cube::new(&p0, &p1, &MaterialType::default());

        let t_offset = vect!(2, 3, 4);
        let translated_cube = TranslateHittable::new(&cube_initial.box_clone(), &t_offset);

        let t_bb = translated_cube.bounding_box(0.0, 1.0);
        let ans_bb = Some(BoundingBox::AabbF(AabbF::new(p0 + t_offset, p1 + t_offset)));

        assert_eq!(t_bb.unwrap(), ans_bb.unwrap());
    }

    #[test]
    fn test_translated_cube_hit() {
        let p0 = vect!(-1, -1, -1);
        let p1 = vect!(1, 1, 1);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );

        let t_offset = vect!(2, 3, 4);
        let t_c0 = TranslateHittable::new(&c0.box_clone(), &t_offset);

        // from the front
        let r = Ray::new(
            //origin gets translated
            &(vect!(0, 0, 2) + t_offset),
            // direction stays the same
            &vect!(0, 0, -1),
            // give it enough length/time? that it hits
            Some(10.0),
        );

        let mut hr_ans = HitRecord::new(
            vect!(0, 0, 1) + t_offset,
            1.0,
            MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );
        hr_ans.normal = vect!(0, 0, 1);
        hr_ans.texture_coord = Some(TextureCoord { u: 0.5, v: 0.5 });

        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));

        // from the back
        let r = Ray::new(&(vect!(0, 0, -2) + t_offset), &vect!(0, 0, 1), Some(10.0));

        hr_ans.normal = vect!(0, 0, -1);
        hr_ans.p = vect!(0, 0, -1) + t_offset;

        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));

        // from the right side
        let r = Ray::new(&(vect!(2, 0, 0) + t_offset), &vect!(-1, 0, 0), Some(10.0));

        hr_ans.normal = vect!(1, 0, 0);
        hr_ans.p = vect!(1, 0, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));

        // from the left side
        let r = Ray::new(&(vect!(-2, 0, 0) + t_offset), &vect!(1, 0, 0), Some(10.0));

        hr_ans.normal = vect!(-1, 0, 0);
        hr_ans.p = vect!(-1, 0, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));

        // from the top
        let r = Ray::new(&(vect!(0, 2, 0) + t_offset), &vect!(0, -1, 0), Some(10.0));

        hr_ans.normal = vect!(0, 1, 0);
        hr_ans.p = vect!(0, 1, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));

        // from the bottom
        let r = Ray::new(&(vect!(0, -2, 0) + t_offset), &vect!(0, 1, 0), Some(10.0));

        hr_ans.normal = vect!(0, -1, 0);
        hr_ans.p = vect!(0, -1, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans));
    }

    #[test]
    fn test_rotate_builder() {
        let p0 = vect!(-1, -1, -1);
        let p1 = vect!(1, 1, 1);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );

        let rb = RotateHittable::new(&c0.box_clone())
            .with_rotate_around_x(90.)
            .build();
        //eprintln!("RotateHittable: {:?}", rb);

        let bb = BoundingBox::AabbF(AabbF::new(p0, p1));
        assert_eq!(rb.bounding_box(0.0, 1.0), Some(bb));
    }

    #[test]
    fn test_rotate_x() {
        let p0 = vect!(0, 0, 0);
        let p1 = vect!(2, 3, 4);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );

        let rotate_x = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                invec.x,
                radians.cos() * invec.y - radians.sin() * invec.z,
                radians.sin() * invec.y + radians.cos() * invec.z,
            )
        };

        let unrotate_x = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                invec.x,
                radians.cos() * invec.y + radians.sin() * invec.z,
                -1. * radians.sin() * invec.y + radians.cos() * invec.z,
            )
        };

        let angles: Vec<f64> = vec![
            0., 45., 90., 135., 180., 225., 360., -45., -135., -180., -225., -360.,
        ];

        let min_ans: Vec<Vec3> = vec![
            vect!(0, 0, 0),
            vect!(0, -2.82842712474619, 0), // 0, 45
            vect!(0, -4, 0),
            vect!(0, -4.949747468305833, -2.82842712474619), // 90, 135
            vect!(0, -3, -4),
            vect!(0, -2.121320343559643, -4.949747468305834), // 180, 225
            vect!(0, 0, 0),
            vect!(0, 0, -2.1213203435596424), // 360, -45
            vect!(0, -2.1213203435596424, -4.949747468305833),
            vect!(0, -3, -4), //-135,-180
            vect!(0, -4.949747468305833, -2.8284271247461907),
            vect!(0, 0, 0), //-225,-360
        ];

        let max_ans: Vec<Vec3> = vec![
            vect!(2, 3, 4),
            vect!(2, 2.121320343559643, 4.949747468305833), // 0, 45
            vect!(2, 0, 3),
            vect!(2, 0, 2.121320343559643), // 90, 135
            vect!(2, 0, 0),
            vect!(2, 2.82842712474619, 0), // 180, 225
            vect!(2, 3, 4),
            vect!(2, 4.949747468305833, 2.8284271247461903), // 360, -45
            vect!(2, 2.8284271247461903, 0),
            vect!(2, 0, 0), // -135, -180
            vect!(2, 0, 2.1213203435596424),
            vect!(2, 3, 4), //-225,-360
        ];

        for (angle0, rot_min, rot_max) in angles
            .iter()
            .zip(min_ans.iter())
            .zip(max_ans.iter())
            .map(|((x, y), z)| (x, y, z))
        {
            let rb = RotateHittable::new(&c0.box_clone())
                .with_rotate_around_x(*angle0)
                .build();

            let bb = BoundingBox::AabbF(AabbF::new(*rot_min, *rot_max));
            assert_eq!(rb.bounding_box(0.0, 1.0).as_ref(), Some(&bb));

            // from the front
            let r = Ray::new(
                &rotate_x(vect!(1, 1, 6), *angle0),
                &rotate_x(vect!(0, 0, -1), *angle0),
                // give it enough length/time? that it hits
                Some(10.0),
            );

            let pat = vect!(1, 1, 4);
            let rot_pat = rotate_x(pat, *angle0);
            let _unrot_pat = unrotate_x(pat, *angle0);
            let mut hr_ans = HitRecord::new(
                rot_pat,
                2.0,
                MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                    ConstantTexture::new(&vect!(4, 4, 4)),
                ))),
            );
            hr_ans.normal = rotate_x(vect!(0, 0, 1), *angle0);
            hr_ans.texture_coord = Some(TextureCoord {
                u: 0.5,
                v: 1.0 / 3.0,
            });

            let hr = rb.hit(&r, 0.0, 10.0);
            assert_eq!(hr, Some(hr_ans.clone()));
        }
    }

    #[test]
    fn test_rotate_y() {
        let p0 = vect!(0, 0, 0);
        let p1 = vect!(2, 3, 4);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );

        let rotate_y = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                radians.cos() * invec.x + radians.sin() * invec.z,
                invec.y,
                -1. * radians.sin() * invec.x + radians.cos() * invec.z,
            )
        };

        let unrotate_y = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                radians.cos() * invec.x - radians.sin() * invec.z,
                invec.y,
                radians.sin() * invec.x + radians.cos() * invec.z,
            )
        };

        let angles: Vec<f64> = vec![
            0., 45., 90., 135., 180., 225., 360., -45., -135., -180., -225., -360.,
        ];

        let min_ans: Vec<Vec3> = vec![
            vect!(0, 0, 0),
            vect!(0, 0, -std::f64::consts::SQRT_2), // 0, 45
            vect!(0, 0, -2),
            vect!(-std::f64::consts::SQRT_2, 0, -4.242640687119285), // 90, 135
            vect!(-2, 0, -4),
            vect!(-4.242640687119286, 0, -2.8284271247461907), // 180, 225
            vect!(0, 0, 0),
            vect!(-2.82842712474619, 0, 0), // 360, -45
            vect!(-4.242640687119286, 0, -2.82842712474619),
            vect!(-2, 0, -4), //-135,-180
            vect!(-std::f64::consts::SQRT_2, 0, -4.242640687119285),
            vect!(0, 0, 0), //-225,-360
        ];

        let max_ans: Vec<Vec3> = vec![
            vect!(2, 3, 4),
            vect!(4.242640687119285, 3, 2.8284271247461903), // 0, 45
            vect!(4, 3, 0),
            vect!(2.8284271247461903, 3, 0), // 90, 135
            vect!(0, 3, 0),
            vect!(0, 3, std::f64::consts::SQRT_2), // 180, 225
            vect!(2, 3, 4),
            vect!(std::f64::consts::SQRT_2, 3, 4.242640687119286), // 360, -45
            vect!(0, 3, std::f64::consts::SQRT_2),
            vect!(0, 3, 0), // -135, -180
            vect!(2.82842712474619, 3, 0),
            vect!(2, 3, 4), //-225,-360
        ];

        for (angle0, rot_min, rot_max) in angles
            .iter()
            .zip(min_ans.iter())
            .zip(max_ans.iter())
            .map(|((x, y), z)| (x, y, z))
        {
            let rb = RotateHittable::new(&c0.box_clone())
                .with_rotate_around_y(*angle0)
                .build();

            let bb = BoundingBox::AabbF(AabbF::new(*rot_min, *rot_max));
            assert_eq!(rb.bounding_box(0.0, 1.0).as_ref(), Some(&bb));

            // from the front
            let r = Ray::new(
                &rotate_y(vect!(1, 1, 6), *angle0),
                &rotate_y(vect!(0, 0, -1), *angle0),
                // give it enough length/time? that it hits
                Some(10.0),
            );

            let pat = vect!(1, 1, 4);
            let rot_pat = rotate_y(pat, *angle0);
            let _unrot_pat = unrotate_y(pat, *angle0);
            let mut hr_ans = HitRecord::new(
                rot_pat,
                2.0,
                MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                    ConstantTexture::new(&vect!(4, 4, 4)),
                ))),
            );
            hr_ans.normal = rotate_y(vect!(0, 0, 1), *angle0);
            hr_ans.texture_coord = Some(TextureCoord {
                u: 0.5,
                v: 1.0 / 3.0,
            });

            let hr = rb.hit(&r, 0.0, 10.0);
            assert_eq!(hr, Some(hr_ans.clone()));
        }
    }

    #[test]
    fn test_rotate_z() {
        let p0 = vect!(0, 0, 0);
        let p1 = vect!(2, 3, 4);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );

        let rotate_z = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                radians.cos() * invec.x - radians.sin() * invec.y,
                radians.sin() * invec.x + radians.cos() * invec.y,
                invec.z,
            )
        };

        let unrotate_z = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                radians.cos() * invec.x - -1. * radians.sin() * invec.y,
                -1. * radians.sin() * invec.x + radians.cos() * invec.y,
                invec.z,
            )
        };

        let angles: Vec<f64> = vec![
            0., 45., 90., 135., 180., 225., 360., -45., -135., -180., -225., -360.,
        ];

        let min_ans: Vec<Vec3> = vec![
            vect!(0, 0, 0),
            vect!(-2.1213203435596424, 0, 0), // 0, 45
            vect!(-3, 0, 0),
            vect!(-3.5355339059327378, -2.1213203435596424, 0), // 90, 135
            vect!(-2, -3, 0),
            vect!(-std::f64::consts::SQRT_2, -3.5355339059327378, 0), // 180, 225
            vect!(0, 0, 0),
            vect!(0, -std::f64::consts::SQRT_2, 0), // 360, -45
            vect!(-std::f64::consts::SQRT_2, -3.5355339059327378, 0),
            vect!(-2, -3, 0), //-135,-180
            vect!(-3.5355339059327378, -2.121320343559643, 0),
            vect!(0, 0, 0), //-225,-360
        ];

        let max_ans: Vec<Vec3> = vec![
            vect!(2, 3, 4),
            vect!(std::f64::consts::SQRT_2, 3.5355339059327378, 4), // 0, 45
            vect!(0, 2, 4),
            vect!(0, std::f64::consts::SQRT_2, 4), // 90, 135
            vect!(0, 0, 4),
            vect!(2.1213203435596424, 0, 4), // 180, 225
            vect!(2, 3, 4),
            vect!(3.5355339059327378, 2.121320343559643, 4), // 360, -45
            vect!(2.121320343559643, 0, 4),
            vect!(0, 0, 4), // -135, -180
            vect!(0, std::f64::consts::SQRT_2, 4),
            vect!(2, 3, 4), //-225,-360
        ];

        for (angle0, rot_min, rot_max) in angles
            .iter()
            .zip(min_ans.iter())
            .zip(max_ans.iter())
            .map(|((x, y), z)| (x, y, z))
        {
            let rb = RotateHittable::new(&c0.box_clone())
                .with_rotate_around_z(*angle0)
                .build();

            let bb = BoundingBox::AabbF(AabbF::new(*rot_min, *rot_max));
            assert_eq!(rb.bounding_box(0.0, 1.0).as_ref(), Some(&bb));

            // from the front
            let r = Ray::new(
                &rotate_z(vect!(1, 1, 6), *angle0),
                &rotate_z(vect!(0, 0, -1), *angle0),
                // give it enough length/time? that it hits
                Some(10.0),
            );

            let pat = vect!(1, 1, 4);
            let rot_pat = rotate_z(pat, *angle0);
            let _unrot_pat = unrotate_z(pat, *angle0);
            let mut hr_ans = HitRecord::new(
                rot_pat,
                2.0,
                MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                    ConstantTexture::new(&vect!(4, 4, 4)),
                ))),
            );
            hr_ans.normal = rotate_z(vect!(0, 0, 1), *angle0);
            hr_ans.texture_coord = Some(TextureCoord {
                u: 0.5,
                v: 1.0 / 3.0,
            });

            let hr = rb.hit(&r, 0.0, 10.0);
            assert_eq!(hr, Some(hr_ans));
        }
    }
}
