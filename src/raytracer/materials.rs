use crate::{HitRecord, Color, reflect, unit_vector, dot, random_in_unit_sphere};
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
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(&rec.p, &(target-&rec.p));
        //let reflected = &reflect(&unit_vector(&ray_in.direction()), &rec.normal);
        //let scattered = Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo;
        //match dot(&scattered.direction(), &rec.normal) > 0.0 {
        //    true =>  {return Some((attenuation, scattered));},
        //false => {return None;}
        //};
        return Some((attenuation, scattered));
        
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: &Color, f: f64) -> Self {
        Metal {
            albedo: *a,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = &reflect(&unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = Ray::new(&rec.p, &(*reflected + self.fuzz*random_in_unit_sphere()));
        let attenuation = self.albedo;
        
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        return None;
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }
}