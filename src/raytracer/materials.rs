use crate::{HitRecord, Color, reflect, unit_vector, dot};
use crate::raytracer::ray::Ray;
pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn albedo(&self) -> Color;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(a: &Color) -> Self {
        Lambertian {
            albedo: *a,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = &reflect(&unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo;
        match dot(&scattered.direction(), &rec.normal) > 0.0 {
            true =>  {return Some((attenuation, scattered));},
        false => {return None;}
        };
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(a: &Color) -> Self {
        Metal {
            albedo: *a,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = &reflect(&unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo;
        match dot(&scattered.direction(), &rec.normal) > 0.90 {
            true => { return Some((attenuation, scattered));},
            false => {return None;},
        }
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }
}