use std::marker::PhantomData;
use crate::{
    aabb::{AABB, AabbF, BoundingBox},
    hittable::{Hittable, HitRecord},
    ray::Ray,
    rectangle::Axis,
    util::{self},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct TranslateHittable {
    instance: Box<dyn Hittable>,
    offset: Vec3,
}

impl TranslateHittable {
    pub fn new(instance: &Box<dyn Hittable>, offset: &Vec3) -> TranslateHittable {
        TranslateHittable{
        instance: instance.box_clone(),
        offset: offset.clone(),
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
            Some(mut gothit) => {gothit.p += self.offset; Some(gothit)},
            None => None,
       }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        match self.instance.bounding_box(t0, t1) {
            Some(bb) => {Some(BoundingBox::AabbF(AabbF::new(
                            bb.min() + self.offset,
                            bb.max() + self.offset,
                            )))},
            None => None,
        }
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
        write!(f,
               "RotateHittable: sin_theta: {} cos_theta: {}, rotate_around: {}\nbbox: {}",
               &self.sin_theta, &self.cos_theta, &self.rotate_around, &self.bbox.unwrap_or_default())
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
    pub fn new( instance: &Box<dyn Hittable>) -> 
        RotateHittableBuilder<util::No> 
        {
            RotateHittableBuilder::new(instance)
        }

    #[allow(dead_code)]
    fn rotate(&self, torotate: &Vec3) -> Vec3 {
        let rotated = match self.rotate_around {
            Axis::X => {
                let torotate = *torotate;
                let mut rotated = torotate.clone();
                rotated.y = (self.cos_theta * torotate.y) - (self.sin_theta * torotate.z);
                rotated.z = (self.sin_theta * torotate.y) + (self.cos_theta * torotate.z);
                rotated
            },
            Axis::Y => {
                let torotate = *torotate;
                let mut rotated = torotate.clone();
                rotated.x = (self.cos_theta * torotate.x) + (self.sin_theta * torotate.z);
                rotated.z = (-1. * self.sin_theta * torotate.x) + (self.cos_theta * torotate.z);
                rotated
            },
            Axis::Z => {
                let torotate = *torotate;
                let mut rotated = torotate.clone();
                rotated.x = (self.cos_theta * torotate.x) - (self.sin_theta * torotate.y);
                rotated.y = (self.sin_theta * torotate.x) + (self.cos_theta * torotate.y);

                rotated
            },
        };
        rotated
    }

    // Note: to unrotate just use -theta instead.
    // cos(-theta) = cos(theta) and sin(-theta) = -sin(theta)
    // so one won't require a change, and the other we just change the sign
    fn unrotate(&self, unrotate: &Vec3) -> Vec3 {
        let unrotated = match self.rotate_around {
            Axis::X => {
                let unrotate = *unrotate;
                let mut unrotated = unrotate.clone();
                unrotated.y = (self.cos_theta * unrotate.y) - (-1. * self.sin_theta * unrotate.z);
                unrotated.z = (-1. * self.sin_theta * unrotate.y) + (self.cos_theta * unrotate.z);
                unrotated

            },
            Axis::Y => {
                let unrotate = *unrotate;
                let mut unrotated = unrotate.clone();
                unrotated.x = (self.cos_theta * unrotate.x) + (-1. * self.sin_theta * unrotate.z);
                unrotated.z = (self.sin_theta * unrotate.x) + (self.cos_theta * unrotate.z);
                unrotated

            },
            Axis::Z => {
                let unrotate = *unrotate;
                let mut unrotated = unrotate.clone();
                unrotated.x = (self.cos_theta * unrotate.x) - (-1. * self.sin_theta * unrotate.y);
                unrotated.y = (-1. * self.sin_theta * unrotate.x) + (self.cos_theta * unrotate.y);
                unrotated

            },
        };
        unrotated
    }
    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "RotateHittable: sin_theta: {} cos_theta: {}, rotate_around: {}\nbbox: {}",
               &self.sin_theta, &self.cos_theta, &self.rotate_around, &self.bbox.unwrap_or_default())

    }

}

pub struct RotateHittableBuilder<RotateHittableBuilderComplete>
where
    RotateHittableBuilderComplete: util::ToAssign
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
    RotateHittableBuilderInitialized: util::ToAssign
{
    pub fn new( instance: &Box<dyn Hittable>) -> RotateHittableBuilder<RotateHittableBuilderInitialized> {
         RotateHittableBuilder {
            completed: PhantomData {},
            instance: instance.box_clone(),
            sin_theta: f64::default(),
            cos_theta: f64::default(),
            rotate_around: Axis::default(),
            bbox: None,
         }
    }

    pub fn with_rotate_around_y(self, angle: f64) -> RotateHittableBuilder<util::Yes> {
        let radians = (std::f64::consts::PI / 180.) * angle;
        let mut ret_self = RotateHittableBuilder{
                completed: PhantomData{},
                bbox: self.instance.bounding_box(0., 1.),
                instance: self.instance,
                sin_theta: radians.sin(),
                cos_theta: radians.cos(),
                rotate_around: Axis::Y,
            };

        if ret_self.bbox.is_none() { 
            return ret_self;
        }
        // otherwise we have a bounding box to rotate.    
        let bbox = ret_self.bbox.unwrap();
        eprintln!("bbox={}", bbox);
        let mut min: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
        let mut max: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x: f64 = i as f64 * &bbox.max().x + (1. - i as f64) * &bbox.min().x;
                    let y: f64 = j as f64 * &bbox.max().y + (1. - j as f64) * &bbox.min().y;
                    let z: f64 = k as f64 * &bbox.max().z + (1. - k as f64) * &bbox.min().z;

                    let newx: f64 = (ret_self.cos_theta * x) + (ret_self.sin_theta * z);
                    // no Y as we're rotating around Y
                    let newz: f64 = (-1. * ret_self.sin_theta * x ) + (ret_self.sin_theta * z);
                    eprintln!("ijk({},{},{}) newx:newz ({},{})", &i, &j, &k, &newx, &newz);

                    let tester: [f64; 3] = [newx, y, newz];
                    for c in 0..3 {
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                            eprintln!(
                                "index: {} tester[c]={} > max[c]={} setting to max[c]",
                                c, &tester[c], &max[c]);
                        }
                        if tester[c] < min[c] {
                            min[c] = tester[c];
                            eprintln!(
                                "index: {} tester[c]={} < min[c]={} setting to min[c]",
                                c, &tester[c], &max[c]);
                        }
                    }
                }
            }
        }

        ret_self.bbox = Some(BoundingBox::AabbF(AabbF::new(min.into(), max.into())));
        ret_self
    }

    pub fn with_rotate_around_x(self, angle: f64) -> RotateHittableBuilder<util::Yes> {
        let radians = (std::f64::consts::PI / 180.) * angle;
        let mut ret_self = RotateHittableBuilder{
                completed: PhantomData{},
                bbox: self.instance.bounding_box(0., 1.),
                instance: self.instance,
                sin_theta: radians.sin(),
                cos_theta: radians.cos(),
                rotate_around: Axis::X,
            };

        if ret_self.bbox.is_none() { 
            return ret_self;
        }
        // otherwise we have a bounding box to rotate.    
        let bbox = ret_self.bbox.unwrap();
        let mut min: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
        let mut max: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x: f64 = i as f64 * &bbox.max().x + (1. - i as f64) * &bbox.min().x;
                    let y: f64 = j as f64 * &bbox.max().y + (1. - j as f64) * &bbox.min().y;
                    let z: f64 = k as f64 * &bbox.max().z + (1. - k as f64) * &bbox.min().z;

                    // no newx because rotating around x
                    let newy: f64 = (ret_self.cos_theta * y) - (ret_self.sin_theta * z);
                    let newz: f64 = (ret_self.sin_theta * y ) + (ret_self.cos_theta * z);

                    let tester: [f64; 3] = [x, newy, newz];
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

        ret_self.bbox = Some(BoundingBox::AabbF(AabbF::new(min.into(), max.into())));
        ret_self
    }

    pub fn with_rotate_around_z(self, angle: f64) -> RotateHittableBuilder<util::Yes> {
        let radians = (std::f64::consts::PI / 180.) * angle;
        let mut ret_self = RotateHittableBuilder{
                completed: PhantomData{},
                bbox: self.instance.bounding_box(0., 1.),
                instance: self.instance,
                sin_theta: radians.sin(),
                cos_theta: radians.cos(),
                rotate_around: Axis::Z,
            };

        if ret_self.bbox.is_none() { 
            return ret_self;
        }
        // otherwise we have a bounding box to rotate.    
        let bbox = ret_self.bbox.unwrap();
        let mut min: [f64; 3] = [f64::MAX, f64::MAX, f64::MAX];
        let mut max: [f64; 3] = [f64::MIN, f64::MIN, f64::MIN];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x: f64 = i as f64 * &bbox.max().x + (1. - i as f64) * &bbox.min().x;
                    let y: f64 = j as f64 * &bbox.max().y + (1. - j as f64) * &bbox.min().y;
                    let z: f64 = k as f64 * &bbox.max(). z + (1. - k as f64) * &bbox.min().z;

                    let newx: f64 = (ret_self.cos_theta * x) - (ret_self.sin_theta * y);
                    let newy: f64 = (ret_self.sin_theta * x ) + (ret_self.cos_theta * y);
                    // no Z as we're rotating around Z
                    let tester: [f64; 3] = [newx, newy, z];
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

        ret_self.bbox = Some(BoundingBox::AabbF(AabbF::new(min.into(), max.into())));
        ret_self
    }
}

impl<RotateHittableBuilderInitialized> RotateHittableBuilder<RotateHittableBuilderInitialized>
where
    RotateHittableBuilderInitialized: util::Assigned
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
        eprintln!("r.direction: {:?} unrotated to direction: {:?}", &r.direction(), &direction);
        let origin: Vec3 = self.unrotate(&r.origin());
        eprintln!("r.origin: {:?} unrotated to origin: {:?}", &r.origin(), &origin);
        let rotated_r = Ray::new(
            &origin,
            &direction,
            Some(r.time()));
        eprintln!("Ray: {:?} unrotated_Ray: {:?}", r, rotated_r);

        match self.instance.hit(&rotated_r, t_min, t_max) {
            Some(hitrec) => {
                eprintln!("Got a hit! {:?}", &hitrec);
                let p: Vec3 = self.rotate(&hitrec.p);
                let normal: Vec3 = self.rotate(&hitrec.normal);
                Some(HitRecord{
                    t: hitrec.t,
                    p: p,
                    normal: normal,
                    material: hitrec.material,
                    texture_coord: hitrec.texture_coord,
                    front_face: hitrec.front_face,
                })
            },
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

#[cfg(test)]
mod test {
    use super::{TranslateHittable, RotateHittable};
    use crate::{
        aabb::{AabbF, BoundingBox},
        cube::Cube,
        hittable::{Hittable, HitRecord, TextureCoord},
        materials::{DiffuseLight, MaterialType},
        ray::Ray,
        textures::{TextureType, ConstantTexture},
        vec3::Vec3,
        vect,
    };

    #[test]
    fn test_translate_cube() {
        let p0 = vect!(-1,-1,-1);
        let p1 = vect!(1,1,1);
        let cube_initial = Cube::new(
            &p0,
            &p1,
            &MaterialType::default());

        let t_offset = vect!(2, 3, 4);
        let translated_cube = TranslateHittable::new(
            &cube_initial.box_clone(),
            &t_offset,
            );

        let t_bb = translated_cube.bounding_box(0.0, 1.0);
        let ans_bb = Some(BoundingBox::AabbF(AabbF::new(p0 + t_offset, p1 + t_offset)));

        assert_eq!(t_bb.unwrap(), ans_bb.unwrap());


    }

    #[test]
    fn test_translated_cube_hit() {
        let p0 = vect!(-1,-1,-1);
        let p1 = vect!(1,1,1);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        let t_offset = vect!(2,3,4);
        let t_c0 = TranslateHittable::new(
            &c0.box_clone(),
            &t_offset,
            );

        // from the front
        let r = Ray::new(
            //origin gets translated
            &(vect!(0, 0, 2) + t_offset),
            // direction stays the same
            &vect!(0, 0, -1),
            // give it enough length/time? that it hits
            Some(10.0)
            );

        let mut hr_ans = HitRecord::new(
            vect!(0, 0, 1) + t_offset,
            1.0,
            MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));
        hr_ans.normal = vect!(0,0,1);
        hr_ans.texture_coord = Some(TextureCoord{u: 0.5, v: 0.5});

        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));

        // from the back
        let r = Ray::new(
            &(vect!(0, 0, -2) + t_offset),
            &vect!(0, 0, 1),
            Some(10.0)
            );

        hr_ans.normal = vect!(0, 0, -1);
        hr_ans.p = vect!(0, 0, -1) + t_offset;

        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 

        // from the right side
        let r = Ray::new(
            &(vect!(2, 0, 0) + t_offset),
            &vect!(-1, 0, 0),
            Some(10.0)
            );

        hr_ans.normal = vect!(1, 0, 0);
        hr_ans.p = vect!(1, 0, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the left side
        let r = Ray::new(
            &(vect!(-2, 0, 0) + t_offset),
            &vect!(1, 0, 0),
            Some(10.0)
            );

        hr_ans.normal = vect!(-1, 0, 0);
        hr_ans.p = vect!(-1, 0, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the top
        let r = Ray::new(
            &(vect!(0, 2, 0) + t_offset),
            &vect!(0, -1, 0),
            Some(10.0)
            );

        hr_ans.normal = vect!(0, 1, 0);
        hr_ans.p = vect!(0, 1, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the bottom
        let r = Ray::new(
            &(vect!(0, -2, 0) + t_offset),
            &vect!(0, 1, 0),
            Some(10.0)
            );

        hr_ans.normal = vect!(0, -1, 0);
        hr_ans.p = vect!(0, -1, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 

    }

    #[test]
    fn test_rotate_builder() {
        let p0 = vect!(-1,-1,-1);
        let p1 = vect!(1,1,1);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        let rb = RotateHittable::new(&c0.box_clone()).with_rotate_around_x(90.).build();
        //eprintln!("RotateHittable: {:?}", rb);

        let bb = BoundingBox::AabbF(AabbF::new(p0, p1));
        assert_eq!(rb.bounding_box(0.0, 1.0), Some(bb));
    }

    #[test]
    fn test_rotate_x() {
        let angle0: f64 = 180.;
        let p0 = vect!(0,0,0);
        let p1 = vect!(2,3,4);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        let rb = RotateHittable::new(&c0.box_clone()).with_rotate_around_x(angle0).build();
        //eprintln!("Cube: {:?}", c0);
        //eprintln!("RotateHittable: {:?}", rb);

        let rotate_x = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                invec.x,
                radians.cos() * invec.y - radians.sin() * invec.z,
                radians.sin() * invec.y + radians.cos() * invec.z,
            )
        };

        let _unrotate_x = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                invec.x,
                radians.cos() * invec.y + radians.sin() * invec.z,
                -1. * radians.sin() * invec.y + radians.cos() * invec.z,
            )
        };


        let rot_p0 = rotate_x(p0, angle0);
        let rot_p1 = rotate_x(p1, angle0);
        //eprintln!("rot_p0: {}", &rot_p0);
        //eprintln!("rot_p1: {}", &rot_p1);

        let rot_min = vect!(
            rot_p0.x.min(rot_p1.x),
            rot_p0.y.min(rot_p1.y),
            rot_p0.z.min(rot_p1.z)
        );
        //eprintln!("rot_min: {}", &rot_min);

        let rot_max = vect!(
            rot_p0.x.max(rot_p1.x),
            rot_p0.y.max(rot_p1.y),
            rot_p0.z.max(rot_p1.z)
            );
        //eprintln!("rot_max: {}", &rot_max);


        let bb = BoundingBox::AabbF(AabbF::new(rot_min, rot_max));
        assert_eq!(rb.bounding_box(0.0, 1.0).as_ref(), Some(&bb));

        // from the front
        let r = Ray::new(
            &rotate_x(vect!(1, 1, 6), angle0),
            &rotate_x(vect!(0, 0, -1), angle0),
            // give it enough length/time? that it hits
            Some(10.0)
            );

        //eprintln!("Rotated_Ray: {:?}", &r);

        let pat = vect!(1, -1, -4);
        //let unrot_pat = unrotate_x(pat, 180.);
        let mut hr_ans = HitRecord::new(
            pat,
            2.0,
            MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));
        // right, since we've rotated 180, all of the non-x axis are flipped
        // so since we were aiming at -1, that would be rotated on the way in, 
        // (because it's explicitly rotated in the test before testing for a hit)
        // rather than get a (0,0,1) that we'd get normally, we get the (0,0,-1)
        // because it's rotated 180
        hr_ans.normal = vect!(0,0,-1);
        hr_ans.texture_coord = Some(TextureCoord{u: 0.5, v: 1.0/3.0});
        //eprintln!("pat: {} unrot_pat: {}", &pat, &unrot_pat);

        let hr = rb.hit(&r, 0.0, 10.0);
        //eprintln!("hr={:?}", hr);
        assert_eq!(hr, Some(hr_ans.clone()));
    }

    #[test]
    fn test_rotate_y() {
        let angle0: f64 = 180.;
        let p0 = vect!(0,0,0);
        let p1 = vect!(2,3,4);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        let rb = RotateHittable::new(&c0.box_clone()).with_rotate_around_y(angle0).build();
        eprintln!("Cube: {:?}", c0);
        eprintln!("RotateHittable: {:?}", rb);

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


        let rot_p0 = rotate_y(p0, angle0);
        let rot_p1 = rotate_y(p1, angle0);
        eprintln!("rot_p0: {}", &rot_p0);
        eprintln!("rot_p1: {}", &rot_p1);

        let rot_min = vect!(
            rot_p0.x.min(rot_p1.x),
            rot_p0.y.min(rot_p1.y),
            rot_p0.z.min(rot_p1.z)
        );
        eprintln!("rot_min: {}", &rot_min);

        let rot_max = vect!(
            rot_p0.x.max(rot_p1.x),
            rot_p0.y.max(rot_p1.y),
            rot_p0.z.max(rot_p1.z)
            );
        eprintln!("rot_max: {}", &rot_max);


        let bb = BoundingBox::AabbF(AabbF::new(rot_min, rot_max));
        assert_eq!(rb.bounding_box(0.0, 1.0).as_ref(), Some(&bb));

        // from the front
        let r = Ray::new(
            &rotate_y(vect!(1, 1, 6), angle0),
            &rotate_y(vect!(0, 0, -1), angle0),
            // give it enough length/time? that it hits
            Some(10.0)
            );

        eprintln!("Rotated_Ray: {:?}", &r);

        let pat = vect!(1, -1, -4);
        let unrot_pat = unrotate_y(pat, angle0);
        let mut hr_ans = HitRecord::new(
            vect!(1, -1, -4),
            2.0,
            MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));
        hr_ans.normal = vect!(0,0,-1);
        hr_ans.texture_coord = Some(TextureCoord{u: 0.5, v: 1.0/3.0});
        eprintln!("pat: {} unrot_pat: {}", &pat, &unrot_pat);

        let hr = rb.hit(&r, 0.0, 10.0);
        eprintln!("hr={:?}", hr);
        assert_eq!(hr, Some(hr_ans.clone()));
    }

    #[test]
    fn test_rotate_z() {
        let angle0: f64 = 180.;
        let p0 = vect!(0,0,0);
        let p1 = vect!(2,3,4);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        let rb = RotateHittable::new(&c0.box_clone()).with_rotate_around_z(angle0).build();
        //eprintln!("Cube: {:?}", c0);
        //eprintln!("RotateHittable: {:?}", rb);

        let rotate_z = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                radians.cos() * invec.x - radians.sin() * invec.y,
                radians.sin() * invec.x + radians.cos() * invec.y,
                invec.z,
            )
        };

        let unrotate_y = |invec: Vec3, angle: f64| {
            let radians = std::f64::consts::PI / 180. * angle;
            Vec3::new(
                radians.cos() * invec.x - -1. * radians.sin() * invec.y,
                -1. * radians.sin() * invec.x + radians.cos() * invec.y,
                invec.z,
            )
        };


        let rot_p0 = rotate_z(p0, angle0);
        let rot_p1 = rotate_z(p1, angle0);
        //eprintln!("rot_p0: {}", &rot_p0);
        //eprintln!("rot_p1: {}", &rot_p1);

        let rot_min = vect!(
            rot_p0.x.min(rot_p1.x),
            rot_p0.y.min(rot_p1.y),
            rot_p0.z.min(rot_p1.z)
        );
        //eprintln!("rot_min: {}", &rot_min);

        let rot_max = vect!(
            rot_p0.x.max(rot_p1.x),
            rot_p0.y.max(rot_p1.y),
            rot_p0.z.max(rot_p1.z)
            );
        //eprintln!("rot_max: {}", &rot_max);


        let bb = BoundingBox::AabbF(AabbF::new(rot_min, rot_max));
        assert_eq!(rb.bounding_box(0.0, 1.0).as_ref(), Some(&bb));

        // from the front
        let r = Ray::new(
            &rotate_z(vect!(1, 1, 6), angle0),
            &rotate_z(vect!(0, 0, -1), angle0),
            // give it enough length/time? that it hits
            Some(10.0)
            );

        eprintln!("Rotated_Ray: {:?}", &r);

        let pat = vect!(1, -1, -4);
        let unrot_pat = unrotate_y(pat, angle0);
        let mut hr_ans = HitRecord::new(
            pat,
            2.0,
            MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));
        hr_ans.normal = vect!(0,0,-1);
        hr_ans.texture_coord = Some(TextureCoord{u: 0.5, v: 1.0/3.0});
        eprintln!("pat: {} unrot_pat: {}", &pat, &unrot_pat);

        let hr = rb.hit(&r, 0.0, 10.0);
        eprintln!("hr={:?}", hr);
        assert_eq!(hr, Some(hr_ans.clone()));
    }
}

