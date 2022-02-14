use crate::vec3::Vec3;
use crate::util::optional_arg;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(a: &Vec3, b: &Vec3, t: Option<f64>) -> Self {
        Self{
            a: *a,
            b: *b,
            time: optional_arg::<f64>(t),
        }
    }
    
    pub fn origin(&self) -> Vec3 {
        self.a
    }
    
    pub fn direction(&self) -> Vec3 {
        self.b
    }
    
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + t*self.b
    }
    
    pub fn at(&self, t: f64) -> Vec3 {
        self.point_at_parameter(t)
    }
    
    pub fn time(&self) -> f64 {
        self.time
    }
}

#[cfg(test)]
mod test {
    use crate::ray;
    use crate::vec3::Vec3;

    #[test]
    fn test_constructor() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(7.3, 9.8, 10.2);
        let r = ray::Ray::new(&a,&b, None);
        
        assert_eq!(r.a, a);
        assert_eq!(r.b, b);
    }

    #[test]
    fn test_origin() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(77.7, 88.8, 12.34);
        let r = ray::Ray::new(&v, &v2, None);
     
        assert_eq!(r.origin(), v);
    }

    #[test]
    fn test_direction() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(77.7, 88.8, 12.34);
        let r = ray::Ray::new(&v, &v2, None);
     
        assert_eq!(r.direction(), v2);

    }

    #[test]
    fn test_point_at_parameter() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(77.7, 88.8, 12.34);
        let r = ray::Ray::new(&v, &v2, None);
        let p = 30.0;
     
        assert_eq!(r.point_at_parameter(p), r.a + r.b*p );
        assert_eq!(r.point_at_parameter(p), r.a + p*r.b );
        
    }
}
