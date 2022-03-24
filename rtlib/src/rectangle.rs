use super::{
    aabb::{AabbF, BoundingBox},
    hittable::{HitRecord, Hittable, TextureCoord},
    materials::{MaterialType, NoneMaterial},
    ray::Ray,
    vect,
};

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Default for Axis {
    fn default() -> Self {
        Axis::X
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: String = match &self {
            Axis::X => "Axis::X".into(),
            Axis::Y => "Axis::Y".into(),
            Axis::Z => "Axis::Z".into(),
        };
        write!(f, "{}", value)
    }
}

// NOTE: for an XY Rectangle, you align on Z axis
// for XZ Rectangle, you align on Y axis
// for YZ Rectangle, you align on Z axis.
// So, only 2 points are ever needed, and then the 'k'
// plane/axis where it's aligned to.
#[derive(Clone)]
pub struct Rect {
    material: MaterialType,
    axis0_min: f64,
    axis0_max: f64,
    axis1_min: f64,
    axis1_max: f64,
    k: f64,
    aligned_axis: Axis,
}

impl Default for Rect {
    fn default() -> Rect {
        Rect {
            material: MaterialType::Nothing(NoneMaterial),
            axis0_min: f64::default(),
            axis0_max: f64::default(),
            axis1_min: f64::default(),
            axis1_max: f64::default(),
            k: f64::default(),
            aligned_axis: Axis::default(),
        }
    }
}

impl Rect {
    pub fn new(
        axis0_min: f64,
        axis0_max: f64,
        axis1_min: f64,
        axis1_max: f64,
        k: f64,
        mat: &MaterialType,
        aligned_axis: Axis,
    ) -> Self {
        Rect {
            material: mat.clone(),
            axis0_min,
            axis0_max,
            axis1_min,
            axis1_max,
            k,
            aligned_axis,
        }
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rect: axis0_min: {} axis0_max: {} axis1_min: {} axis1_max: {}",
            &self.axis0_min, &self.axis0_max, &self.axis1_min, &self.axis1_max
        )?;
        write!(
            f,
            " k: {} , aligned_axis: {}, material: {}",
            &self.k, &self.aligned_axis, &self.material
        )
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

impl Hittable for Rect {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        //eprintln!("Rect({})::hit({:?}, {}, {})", &self, &r, &tmin, &tmax);
        let t: f64;
        let axis0: f64;
        let axis1: f64;
        match self.aligned_axis {
            // XY Rect
            Axis::Z => {
                t = (self.k - r.origin().z) / r.direction().z;
                if t < tmin || t > tmax {
                    return None;
                }
                axis0 = r.origin().x + t * r.direction().x;
                axis1 = r.origin().y + t * r.direction().y;
            }
            // XZ Rect
            Axis::Y => {
                t = (self.k - r.origin().y) / r.direction().y;
                if t < tmin || t > tmax {
                    return None;
                }
                axis0 = r.origin().x + t * r.direction().x;
                axis1 = r.origin().z + t * r.direction().z;
            }
            // YZ Rect
            Axis::X => {
                t = (self.k - r.origin().x) / r.direction().x;
                if t < tmin || t > tmax {
                    return None;
                }
                axis0 = r.origin().y + t * r.direction().y;
                axis1 = r.origin().z + t * r.direction().z;
            }
        }
        if (axis0 < self.axis0_min)
            || (axis0 > self.axis0_max)
            || (axis1 < self.axis1_min)
            || (axis1 > self.axis1_max)
        {
            return None;
        }

        let mut hitrec = HitRecord::new(r.point_at_parameter(t), t, self.material.clone());

        hitrec.texture_coord = Some(TextureCoord {
            u: (axis0 - self.axis0_min) / (self.axis0_max - self.axis0_min),
            v: (axis1 - self.axis1_min) / (self.axis1_max - self.axis1_min),
        });
        // axis aligned rectangle.
        hitrec.normal = match self.aligned_axis {
            Axis::Z => vect!(0, 0, 1),
            Axis::Y => vect!(0, 1, 0),
            Axis::X => vect!(1, 0, 0),
        };
        Some(hitrec)
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        match self.aligned_axis {
            Axis::Z => Some(BoundingBox::AabbF(AabbF {
                minimum: vect!(self.axis0_min, self.axis1_min, self.k - 0.0001),
                maximum: vect!(self.axis1_max, self.axis1_max, self.k + 0.0001),
            })),
            Axis::Y => Some(BoundingBox::AabbF(AabbF {
                minimum: vect!(self.axis0_min, self.k - 0.0001, self.axis1_min),
                maximum: vect!(self.axis1_max, self.k + 0.0001, self.axis1_max),
            })),
            Axis::X => Some(BoundingBox::AabbF(AabbF {
                minimum: vect!(self.k - 0.0001, self.axis0_min, self.axis1_min),
                maximum: vect!(self.k + 0.0001, self.axis1_max, self.axis1_max),
            })),
        }
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::{Axis, Rect};
    use crate::{
        aabb::{AabbF, BoundingBox},
        hittable::{HitRecord, Hittable, TextureCoord},
        materials::{DiffuseLight, MaterialType},
        ray::Ray,
        textures::{ConstantTexture, TextureType},
        vect,
    };

    #[test]
    fn test_xyrect_creation() {
        let xyrect = Rect::new(
            -1.0,
            1.0,
            -1.0,
            1.0,
            0.0,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
            Axis::Z,
        );

        let bb_ans = BoundingBox::AabbF(AabbF::new(
            vect!(-1, -1, 0.0 - 0.0001),
            vect!(1, 1, 0.0 + 0.0001),
        ));

        assert_eq!(xyrect.bounding_box(0.0, 0.0), Some(bb_ans));
    }

    #[test]
    fn test_xyrect_hit() {
        let xyrect = Rect::new(
            -1.0,
            1.0,
            -1.0,
            1.0,
            0.0,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
            Axis::Z,
        );

        let r = Ray::new(&vect!(0, 0, 1), &vect!(0, 0, -1), None);

        let mut hr_ans = HitRecord::new(
            vect!(0, 0, 0),
            1.0,
            MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );
        hr_ans.normal = vect!(0, 0, 1);
        hr_ans.texture_coord = Some(TextureCoord { u: 0.5, v: 0.5 });

        let hr = xyrect.hit(&r, 0.0, 1.0);

        assert_eq!(hr, Some(hr_ans));
    }

    #[test]
    fn test_xzrect_creation() {
        let xzrect = Rect::new(
            -1.0,
            1.0,
            -1.0,
            1.0,
            0.0,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
            Axis::Y,
        );

        let bb_ans = BoundingBox::AabbF(AabbF::new(
            vect!(-1, 0.0 - 0.0001, -1),
            vect!(1, 0.0 + 0.0001, 1),
        ));

        assert_eq!(xzrect.bounding_box(0.0, 0.0), Some(bb_ans));
    }

    #[test]
    fn test_xzrect_hit() {
        let xzrect = Rect::new(
            -1.0,
            1.0,
            -1.0,
            1.0,
            0.0,
            &MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
            Axis::Y,
        );

        let r = Ray::new(&vect!(0, 1, 0), &vect!(0, -1, 0), None);

        let mut hr_ans = HitRecord::new(
            vect!(0, 0, 0),
            1.0,
            MaterialType::DiffuseLight(DiffuseLight::new(TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(4, 4, 4)),
            ))),
        );
        hr_ans.normal = vect!(0, 1, 0);
        hr_ans.texture_coord = Some(TextureCoord { u: 0.5, v: 0.5 });

        let hr = xzrect.hit(&r, 0.0, 1.0);

        assert_eq!(hr, Some(hr_ans));
    }

    #[test]
    fn test_yzrect_creation() {
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

        let bb_ans = BoundingBox::AabbF(AabbF::new(
            vect!(0.0 - 0.0001, -1, -1),
            vect!(0.0 + 0.0001, 1, 1),
        ));

        assert_eq!(yzrect.bounding_box(0.0, 0.0), Some(bb_ans));
    }

    #[test]
    fn test_yzrect_hit() {
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

        assert_eq!(hr, Some(hr_ans));
    }
}
