use super::hittable::{HitRecord, TextureCoord};
use super::ray::Ray;
use super::textures::{ConstantTexture, NoneTexture, Texture, TextureType};
use super::util::{random_in_unit_sphere, reflect, refract};
use super::vec3::{dot, unit_vector, Color, Vec3};
use super::vect;
use rand::Rng;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn albedo(&self) -> TextureType;
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn box_clone(&self) -> Box<MaterialType>;
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        vect!(0, 0, 0)
    } // return black as default
}

#[derive(Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    Metal(Metal),
    DiffuseLight(DiffuseLight),
    Nothing(NoneMaterial),
}

impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::Nothing(NoneMaterial)
    }
}

impl std::fmt::Display for MaterialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

#[derive(Copy, Clone, Default)]
pub struct NoneMaterial;

impl Material for NoneMaterial {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        return None;
    }
    fn albedo(&self) -> TextureType {
        TextureType::Nothing(NoneTexture)
    }
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoneMaterial is empty.")
    }
    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::Nothing(*self))
    }
}

impl Material for MaterialType {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            MaterialType::Lambertian(innertype) => {
                return innertype.scatter(ray_in, rec);
            }
            MaterialType::Dielectric(innertype) => {
                return innertype.scatter(ray_in, rec);
            }
            MaterialType::Metal(innertype) => {
                return innertype.scatter(ray_in, rec);
            }
            MaterialType::DiffuseLight(innertype) => {
                return innertype.scatter(ray_in, rec);
            }
            MaterialType::Nothing(_innertype) => {
                return None;
            }
        }
    }
    fn albedo(&self) -> TextureType {
        match self {
            MaterialType::Lambertian(innertype) => return innertype.albedo(),
            MaterialType::Dielectric(innertype) => return innertype.albedo(),
            MaterialType::Metal(innertype) => return innertype.albedo(),
            MaterialType::DiffuseLight(innertype) => return innertype.albedo(),
            MaterialType::Nothing(_innertype) => TextureType::Nothing(NoneTexture),
        }
    }
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaterialType::Lambertian(innertype) => return innertype.inner_fmt(f),
            MaterialType::Dielectric(innertype) => return innertype.inner_fmt(f),
            MaterialType::Metal(innertype) => return innertype.inner_fmt(f),
            MaterialType::DiffuseLight(innertype) => return innertype.inner_fmt(f),
            MaterialType::Nothing(innertype) => innertype.inner_fmt(f),
        }
    }
    fn box_clone(&self) -> Box<MaterialType> {
        match self {
            MaterialType::Lambertian(innertype) => {
                return Box::new(MaterialType::Lambertian(innertype.clone()));
            }
            MaterialType::Dielectric(innertype) => {
                return Box::new(MaterialType::Dielectric(innertype.clone()));
            }
            MaterialType::Metal(innertype) => {
                return Box::new(MaterialType::Metal(innertype.clone()));
            }
            MaterialType::DiffuseLight(innertype) => {
                return Box::new(MaterialType::DiffuseLight(innertype.clone()));
            }
            MaterialType::Nothing(_innertype) => Box::new(MaterialType::Nothing(NoneMaterial)),
        }
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        match self {
            MaterialType::Lambertian(innertype) => return innertype.emitted(u, v, p),
            MaterialType::Dielectric(innertype) => return innertype.emitted(u, v, p),
            MaterialType::Metal(innertype) => return innertype.emitted(u, v, p),
            MaterialType::DiffuseLight(innertype) => return innertype.emitted(u, v, p),
            MaterialType::Nothing(innertype) => innertype.emitted(u, v, p),
        }
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }
}
// we want the material trait to be supported for references AND non-references
//impl<'a, T> Material for &'a T where T: Material { }
//impl<'a, T> Material for &'a mut T where T: Material {}

macro_rules! mat_display {
    ($klass:ty) => {
        #[allow(dead_code)]
        impl std::fmt::Display for $klass {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner_fmt(f)
            }
        }
    };
}

mat_display!(dyn Material);
// because the material can be None, we'll also add a specialization for an option
// can't do this because of the orphan rule
//impl std::fmt::Display for std::option::Option<&dyn Material> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        match self.is_some() {
//            true => return self.as_ref().inner_fmt(f),
//            false => return write!(f, "None"),
//        }
//    }
//}

#[derive(Clone)]
pub struct Lambertian {
    albedo: TextureType,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(texture: &TextureType) -> Self {
        Lambertian {
            albedo: texture.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(&rec.p, &(target - &rec.p), None);
        //let attenuation = self.albedo.value(0.0, 0.0, &rec.p);
        let u: f64 = rec.texture_coord.unwrap_or(TextureCoord::default()).u;
        let v: f64 = rec.texture_coord.unwrap_or(TextureCoord::default()).v;
        let attenuation = self.albedo.value(u, v, &rec.p);
        return Some((attenuation, scattered));
    }

    fn albedo(&self) -> TextureType {
        self.albedo.clone()
    }

    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lambertian::albedo: ")?;
        self.albedo().inner_fmt(f)
    }

    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::Lambertian(self.clone()))
    }
}

mat_display!(Lambertian);

#[derive(Clone)]
pub struct Metal {
    albedo: TextureType,
    fuzz: f64,
}

impl Metal {
    // NOTE: Default fuzz is 1
    pub fn new(a: TextureType, mut fuzz: f64) -> Self {
        if fuzz > 1.0 {
            fuzz = 1.0
        };
        Metal {
            albedo: a,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = &reflect(&unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = Ray::new(
            &rec.p,
            &(*reflected + self.fuzz * random_in_unit_sphere()),
            None,
        );
        let attenuation = self.albedo().value(0.0, 0.0, reflected);

        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        return None;
    }

    fn albedo(&self) -> TextureType {
        self.albedo.clone()
    }

    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Metal::fuzz: {} albedo: ", self.fuzz)?;
        return self.albedo().inner_fmt(f);
    }
    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::Metal(self.clone()))
    }
}
mat_display!(Metal);

#[derive(Clone)]
pub struct Dielectric {
    #[allow(dead_code)]
    albedo: TextureType,
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(albedo: &Color, refractive_index: f64) -> Self {
        Dielectric {
            albedo: TextureType::ConstantTexture(ConstantTexture::new(albedo)),
            ref_idx: refractive_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut outward_normal = rec.normal;
        let reflected = reflect(&ray_in.direction(), &rec.normal);
        let mut ni_over_nt: f64 = self.ref_idx;
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let cosine: f64;
        let reflect_prob: f64;
        let mut refracted = Color::new(0.0, 0.0, 0.0);
        let scattered: Ray;

        if dot(&ray_in.direction(), &outward_normal) > 0.0 {
            // otherwise it goes into the object
            outward_normal *= -1.0;
            cosine =
                self.ref_idx * dot(&ray_in.direction(), &rec.normal) / ray_in.direction().length();
        } else {
            // otherwise, it's the inverse refractive index
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(&ray_in.direction(), &rec.normal) / ray_in.direction().length()
        }
        if let Some(tmp_refracted) = refract(&ray_in.direction(), outward_normal, ni_over_nt) {
            refracted = tmp_refracted;
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            scattered = Ray::new(&rec.p, &reflected, None);
        } else {
            scattered = Ray::new(&rec.p, &refracted, None);
        }
        return Some((attenuation, scattered));
    }

    fn albedo(&self) -> TextureType {
        self.albedo.clone()
    }

    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dielectric::refraction_index: {} albedo: ", self.ref_idx)?;
        return self.albedo().inner_fmt(f);
    }

    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::Dielectric(self.clone()))
    }
}
//mat_display!(Metal);

#[derive(Clone)]
pub struct DiffuseLight {
    albedo: TextureType,
}

impl DiffuseLight {
    // NOTE: Default fuzz is 1
    pub fn new(a: TextureType) -> Self {
        DiffuseLight { albedo: a }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        return None;
    }

    fn albedo(&self) -> TextureType {
        self.albedo.clone()
    }

    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiffuseLight: ")?;
        return self.albedo().inner_fmt(f);
    }
    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::DiffuseLight(self.clone()))
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.albedo.value(u, v, p)
    }
}
mat_display!(DiffuseLight);

pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let mut r0: f64 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}

#[cfg(test)]
mod test {
    use super::super::color_to_texture;
    #[allow(unused_imports)]
    use super::{Color, DiffuseLight, Material, MaterialType, NoneMaterial};
    use crate::vect;

    #[test]
    fn test_base_material() {
        let mat = MaterialType::Nothing(NoneMaterial::default());
        let ans_color = vect!(0, 0, 0);

        assert_eq!(mat.emitted(0., 0., &vect!(1, 2, 3)), ans_color);
    }

    #[test]
    fn test_diffuse_light() {
        let light_color = Color::new(1.0, 0.5, 0.2);
        let light = DiffuseLight::new(color_to_texture!(&light_color));
        let result = light.emitted(0.0, 0.0, &light_color);

        assert_eq!(result, light_color);
    }
}
