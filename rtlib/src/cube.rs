use crate::{
    aabb::{BoundingBox, AabbF},
    hittable::{FlipNormal, Hitters, Hittable, HitRecord},
    hitlist::{HitList},
    materials::MaterialType,
    ray::Ray,
    rectangle::{Axis, Rect},
    vec3::Vec3,
};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Cube {
    walls: HitList,
    pmin: Vec3,
    pmax: Vec3,
}

impl Cube {
    pub fn new(pt0: &Vec3, pt1: &Vec3, material: &MaterialType) -> Cube {
        let mut hl = HitList::new();
        // front
        hl.add(
            Hitters::Rect(
                Rect::new(
                    pt0.x, pt1.x, pt0.y, pt1.y, pt1.z,
                    &material, Axis::Z)));
        // flip the front to get the back
        hl.add(
            Hitters::FlipNormal(
                FlipNormal::new(
                    &Rect::new(
                        pt0.x, pt1.x, pt0.y, pt1.y, pt0.z,
                        &material, Axis::Z
                        ).box_clone())));

        // top
        hl.add(
            Hitters::Rect(
                Rect::new(
                    pt0.x, pt1.x, pt0.z, pt1.z, pt1.y,
                    &material, Axis::Y)));
        // flip for bottom
        hl.add(
            Hitters::FlipNormal(
                FlipNormal::new(
                    &Rect::new(
                        pt0.x, pt1.x, pt0.z, pt1.z, pt0.y,
                        &material, Axis::Y
                        ).box_clone())));
        // side
        hl.add(
            Hitters::Rect(
                Rect::new(
                    pt0.y, pt1.y, pt0.z, pt1.z, pt1.x,
                    &material, Axis::X)));
        // flip for other side
        hl.add(
            Hitters::FlipNormal(
                FlipNormal::new(
                    &Rect::new(
                        pt0.y, pt1.y, pt0.z, pt1.z, pt0.x,
                        &material, Axis::X
                        ).box_clone())));
        Cube {walls: hl, pmin: *pt0, pmax: *pt1 }
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cube: pmin: {} pmax: {} walls:\n", &self.pmin, &self.pmax)?;
        for (i, w) in self.walls.list.iter().enumerate() {
            write!(f, "{}: {}", &i, &w)?;
        }
        write!(f, "  End Cube")
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        eprintln!("Cube::hit({:?}, {}, {})", &r, &tmin, &tmax);
        self.walls.hit(r, tmin, tmax)
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::AabbF(AabbF{minimum: self.pmin, maximum: self.pmax}))
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

impl std::fmt::Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cube")
            .field("pmin", &self.pmin)
            .field("pmax", &self.pmax)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::{Cube};
    use crate::{
        hittable::{HitRecord, Hittable, TextureCoord},
        materials::{DiffuseLight, MaterialType},
        ray::Ray,
        textures::{ConstantTexture, TextureType},
        vect,
    };
    
    #[test]
    fn test_cube_construction() {
        let p0 = vect!(-1,-1,-1);
        let p1 = vect!(1,1,1);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::default());

        assert_eq!(c0.pmin, p0);
        assert_eq!(c0.pmax, p1);
    }

    #[test]
    fn test_cube_hit() {
        let p0 = vect!(-1,-1,-1);
        let p1 = vect!(1,1,1);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        // from the front
        let r = Ray::new(
            &vect!(0, 0, 2),
            &vect!(0, 0, -1),
            None
            );

        let mut hr_ans = HitRecord::new(
            vect!(0, 0, 1),
            1.0,
            MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));
        hr_ans.normal = vect!(0,0,1);
        hr_ans.texture_coord = Some(TextureCoord{u: 0.5, v: 0.5});


        let hr = c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));

        // from the back
        let r = Ray::new(
            &vect!(0, 0, -2),
            &vect!(0, 0, 1),
            None
            );

        hr_ans.normal = vect!(0, 0, -1);
        hr_ans.p = vect!(0, 0, -1);
        let hr = c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 

        // from the right side
        let r = Ray::new(
            &vect!(2, 0, 0),
            &vect!(-1, 0, 0),
            None
            );

        hr_ans.normal = vect!(1, 0, 0);
        hr_ans.p = vect!(1, 0, 0);
        let hr = c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the left side
        let r = Ray::new(
            &vect!(-2, 0, 0),
            &vect!(1, 0, 0),
            None
            );

        hr_ans.normal = vect!(-1, 0, 0);
        hr_ans.p = vect!(-1, 0, 0);
        let hr = c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the top
        let r = Ray::new(
            &vect!(0, 2, 0),
            &vect!(0, -1, 0),
            None
            );

        hr_ans.normal = vect!(0, 1, 0);
        hr_ans.p = vect!(0, 1, 0);
        let hr = c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the bottom
        let r = Ray::new(
            &vect!(0, -2, 0),
            &vect!(0, 1, 0),
            None
            );

        hr_ans.normal = vect!(0, -1, 0);
        hr_ans.p = vect!(0, -1, 0);
        let hr = c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
    }
}
