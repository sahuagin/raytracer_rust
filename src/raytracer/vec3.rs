mod vec3 {
    pub use Vec3 as Point3;
    pub use Vec3 as Color;
    
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[allow(unused_imports, dead_code)]
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
    
    #[allow(unused_imports, dead_code)]
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.dot(v2)
    }

      // this is how you would do it if there were internal members
      // that didn't also implement trait Default
      // impl Default for Vec3 {
      //     fn default() -> Self {
      //         Self { x: 0.0, y: 0.0, z: 0.0}
      //     }
      // }
  
    impl Vec3 {
        #[allow(unused_imports, dead_code)]
        pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
            Vec3 {x,y,z}
        }
        
        #[allow(unused_imports, dead_code)]
        pub fn length(&self) -> f64 {
            self.length_squared().sqrt()
        }
        
        #[allow(unused_imports, dead_code)]
        pub fn length_squared(&self) -> f64 {
            self.x*self.x+self.y*self.y+self.z*self.z
        }
        
        #[allow(unused_imports, dead_code)]
        pub fn dot(&self, rhs: &Self) -> f64 {
            self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
        }
        
        #[allow(unused_imports, dead_code)]
        pub fn cross(&self, rhs: &Self) -> Self {
            Vec3::new( self.y*rhs.z - self.z*rhs.y,
                      -(self.x*rhs.z - self.z*rhs.x),
                      self.x*rhs.y - self.y*rhs.x)
        }
        
        #[allow(unused_imports, dead_code)]
        pub fn unit(&self) -> Self {
            *self / self.length()
        }   
        
       
    }

}
use std::fmt;
use std::ops;
pub use vec3::Vec3;
pub use vec3::Color;
pub use vec3::Point3;
pub use vec3::unit_vector;
pub use vec3::dot;

impl ops::Add<Vec3> for vec3::Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self{
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }   
}

impl ops::Add<&Vec3> for vec3::Vec3 {
    type Output = Self;
    
    fn add(self, rhs: &Self) -> Self{
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }   
}

impl ops::AddAssign<Vec3> for vec3::Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}


impl ops::AddAssign<&Vec3> for vec3::Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for vec3::Vec3 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self{
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }   
}

impl ops::Sub<&Vec3> for vec3::Vec3 {
    type Output = Self;
    
    fn sub(self, rhs: &Self) -> Self{
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }   
}

impl ops::SubAssign<Vec3> for vec3::Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl ops::SubAssign<&Vec3> for vec3::Vec3 {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl ops::MulAssign<f64> for vec3::Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<f64> for vec3::Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs, 
        }
    }

}

impl ops::Mul<vec3::Vec3> for f64 {
    type Output = vec3::Vec3;
    fn mul(self, rhs: vec3::Vec3) -> vec3::Vec3 {
        vec3::Vec3 {
            x: rhs.x * self, 
            y: rhs.y * self, 
            z: rhs.z * self, 
        }
    }
    
}
impl ops::Div<f64> for vec3::Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ops::DivAssign<f64> for vec3::Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// NOTE: This isn't how I'd normally want to represent pretty printed output,
// but these will be printed out into a ppm file, so it needs to be in this format.
impl fmt::Display for vec3::Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]

#[test]
fn test_default_construction(){
    let vec = Vec3::default();

    assert_eq!(vec.x, 0_f64);
    assert_eq!(vec.y, 0_f64);
    assert_eq!(vec.z, 0_f64);
}

#[test]
fn test_constructor() {
    let vec = Vec3::new(3.0, 4.0, 5.0);

    assert_eq!(vec.x, 3_f64);
    assert_eq!(vec.y, 4_f64);
    assert_eq!(vec.z, 5_f64);
}

#[test]
fn test_equality() {
    let vec = Vec3::new(3.0, 4.0, 5.0);
    let vec2 = Vec3::new(3.0, 4.0, 5.0);
    let vec3 = Vec3::new(5.0, 4.0, 3.0);
    
    assert_eq!(vec, vec2);
    assert_ne!(vec, vec3);
}

#[test]
fn test_add() {
   let vec = Vec3::new(3.0, 4.0, 5.0);
   let vec1 = Vec3::new(1.0, 0.0, 0.0);
   let vec2 = Vec3::new(0.0, 1.0, 0.0);
   let vec3 = Vec3::new(0.0, 0.0, 1.0);
   let vec4 = Vec3::new(1.0, 1.0, 1.0);
   
   assert_eq!(vec+&vec, Vec3::new(6.0, 8.0, 10.0));
   assert_eq!(vec+vec, Vec3::new(6.0, 8.0, 10.0));
   assert_eq!(vec+vec1, Vec3::new(4.0, 4.0, 5.0));
   assert_eq!(vec+vec2, Vec3::new(3.0, 5.0, 5.0));
   assert_eq!(vec+vec3, Vec3::new(3.0, 4.0, 6.0));
   assert_eq!(vec+vec4, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn test_add_assign() {
   let mut vec = Vec3::new(3.0, 4.0, 5.0);
   let vec1 = Vec3::new(1.0, 0.0, 0.0);
   let vec2 = Vec3::new(0.0, 1.0, 0.0);
   let vec3 = Vec3::new(0.0, 0.0, 1.0);
   let vec4 = Vec3::new(1.0, 1.0, 1.0);
   
   let vec_dup = vec.clone();
   vec += &vec.clone();
   assert_eq!(vec, Vec3::new(6.0, 8.0, 10.0));
   vec = vec_dup.clone();
   vec += vec;
   assert_eq!(vec, Vec3::new(6.0, 8.0, 10.0));
   vec = vec_dup.clone();
   vec += &vec1;
   assert_eq!(vec, Vec3::new(4.0, 4.0, 5.0));
   vec = vec_dup.clone();
   vec += vec1;
   assert_eq!(vec, Vec3::new(4.0, 4.0, 5.0));
   vec = vec_dup.clone();
   vec += &vec2;
   assert_eq!(vec, Vec3::new(3.0, 5.0, 5.0));
   vec = vec_dup.clone();
   vec += vec2;
   assert_eq!(vec, Vec3::new(3.0, 5.0, 5.0));
   vec = vec_dup.clone();
   vec += &vec3;
   assert_eq!(vec, Vec3::new(3.0, 4.0, 6.0));
   vec = vec_dup.clone();
   vec += vec3;
   assert_eq!(vec, Vec3::new(3.0, 4.0, 6.0));
   vec = vec_dup.clone();
   vec += &vec4;
   assert_eq!(vec, Vec3::new(4.0, 5.0, 6.0));
   vec = vec_dup.clone();
   vec += vec4;
   assert_eq!(vec, Vec3::new(4.0, 5.0, 6.0));
     
}

#[test]
fn test_mul_assign() {
   let mut vec = Vec3::new(3.0, 4.0, 5.0);
   
   let vec_dup = vec.clone();
   vec *= 1.0;
   assert_eq!(vec, Vec3::new(3.0, 4.0, 5.0));
   vec = vec_dup.clone();
   vec *= 0.0;
   assert_eq!(vec, Vec3::new(0.0, 0.0, 0.0));
   vec = vec_dup.clone();
   vec *= 0.5;
   assert_eq!(vec, Vec3::new(1.5, 2.0, 2.5));
   vec = vec_dup.clone();
   vec *= 0.3;
   assert_eq!(vec, Vec3::new(3.0*0.3, 4.0*0.3, 5.0*0.3));
   vec = vec_dup.clone();
   vec *= 0.3;
   assert_eq!(vec, Vec3::new(0.25*4.0*3.0*0.3, 4.0*0.3, 5.0*0.3));
     
}

#[test]
fn test_div_assign() {
   let mut vec = Vec3::new(3.0, 4.0, 5.0);
   
   let vec_dup = vec.clone();
   vec /= 1.0;
   assert_eq!(vec, Vec3::new(3.0, 4.0, 5.0));
   vec = vec_dup.clone();
   vec /= 0.5;
   assert_eq!(vec, Vec3::new(6.0, 8.0, 10.0));
   vec = vec_dup.clone();
   vec /= 0.3;
   assert_eq!(vec, Vec3::new(3.0/0.3, 4.0/0.3, 5.0/0.3));
   vec = vec_dup.clone();
   vec /= 0.3;
   assert_eq!(vec, Vec3::new(0.25*4.0*3.0/0.3, 4.0/0.3, 5.0/0.3));
     
}

#[test]
fn test_display() {
    let vec = Vec3::new(3.0, 4.0, 5.0);

    assert_eq!(format!("The point is: {}", vec), "The point is: 3 4 5");
}

#[test]
fn test_sub() {
   let vec = Vec3::new(3.0, 4.0, 5.0);
   let vec1 = Vec3::new(1.0, 0.0, 0.0);
   let vec2 = Vec3::new(0.0, 1.0, 0.0);
   let vec3 = Vec3::new(0.0, 0.0, 1.0);
   let vec4 = Vec3::new(1.0, 1.0, 1.0);
   
   assert_eq!(vec-&vec, Vec3::new(0.0, 0.0, 0.0));
   assert_eq!(vec-vec, Vec3::new(0.0, 0.0, 0.0));
   assert_eq!(vec-vec1, Vec3::new(2.0, 4.0, 5.0));
   assert_eq!(vec-vec2, Vec3::new(3.0, 3.0, 5.0));
   assert_eq!(vec-vec3, Vec3::new(3.0, 4.0, 4.0));
   assert_eq!(vec-vec4, Vec3::new(2.0, 3.0, 4.0));
}

#[test]
fn test_sub_assign() {
   let mut vec = Vec3::new(3.0, 4.0, 5.0);
   let vec1 = Vec3::new(1.0, 0.0, 0.0);
   let vec2 = Vec3::new(0.0, 1.0, 0.0);
   let vec3 = Vec3::new(0.0, 0.0, 1.0);
   let vec4 = Vec3::new(1.0, 1.0, 1.0);
   
   let vec_dup = vec.clone();
   vec -= &vec.clone();
   assert_eq!(vec, Vec3::new(0.0, 0.0, 0.0));
   vec = vec_dup.clone();
   vec -= vec;
   assert_eq!(vec, Vec3::new(0.0, 0.0, 0.0));
   vec = vec_dup.clone();
   vec -= &vec1;
   assert_eq!(vec, Vec3::new(2.0, 4.0, 5.0));
   vec = vec_dup.clone();
   vec -= vec1;
   assert_eq!(vec, Vec3::new(2.0, 4.0, 5.0));
   vec = vec_dup.clone();
   vec -= &vec2;
   assert_eq!(vec, Vec3::new(3.0, 3.0, 5.0));
   vec = vec_dup.clone();
   vec -= vec2;
   assert_eq!(vec, Vec3::new(3.0, 3.0, 5.0));
   vec = vec_dup.clone();
   vec -= &vec3;
   assert_eq!(vec, Vec3::new(3.0, 4.0, 4.0));
   vec = vec_dup.clone();
   vec -= vec3;
   assert_eq!(vec, Vec3::new(3.0, 4.0, 4.0));
   vec = vec_dup.clone();
   vec -= &vec4;
   assert_eq!(vec, Vec3::new(2.0, 3.0, 4.0));
   vec = vec_dup.clone();
   vec -= vec4;
   assert_eq!(vec, Vec3::new(2.0, 3.0, 4.0));
     
}

#[test]
fn test_length() {
   let vec = Vec3::new(3.2, 4.0, 5.0);
   let vec2 = Vec3::new(15.0, -5.0, 42.0);

   assert_eq!(vec.length(), ((3.2*3.2) as f64 + (4.0*4.0) as f64 + (5.0*5.0) as f64).sqrt());
   assert_eq!(vec2.length(), (15.0*15.0 as f64 + -5.0*-5.0 as f64 + 42.0*42.0 as f64).sqrt());
}

#[test]
fn test_squared_length() {
   let vec = Vec3::new(3.2, 4.0, 5.0);
   let vec2 = Vec3::new(15.0, -5.0, 42.0);
    
   assert_eq!(vec.length_squared(), (3.2*3.2) as f64 + (4.0*4.0) as f64 + (5.0*5.0) as f64);
   assert_eq!(vec2.length_squared(), 15.0*15.0 as f64 + -5.0*-5.0 as f64 + 42.0*42.0 as f64);

}

#[test]
fn test_dot_product() {
    let vec = Vec3::new(1.0, 2.0, 3.0);
    let vec2 = Vec3::new(3.0, 2.0, 1.0);
    
    assert_eq!(vec.dot(&vec2), 1.0*3.0 + 2.0*2.0 + 3.0*1.0);
}

#[test]
fn test_dot_double() {
    let vec = Vec3::new(1.0, 2.0, 3.0);
    let vec2 = Vec3::new(3.0, 2.0, 1.0);
    
    assert_eq!(dot(&vec, &vec2), 1.0*3.0 + 2.0*2.0 + 3.0*1.0);
}

#[test]
fn test_cross_product() {
    let vec = Vec3::new(1.0, 2.0, 43.0);
    let vec2 = Vec3::new(32.3, 6.7, 10.4);

    assert_eq!(vec.cross(&vec2), Vec3::new(2.0*10.4 - 43.0*6.7,
                                          -(1.0*10.4 - 43.0*32.3),
                                          1.0*6.7 - 2.0*32.3));
}

#[test]
fn test_class_unit() {
    let vec = Vec3::new(1.0, 2.0, 43.0);

    assert_eq!(vec.unit(), vec / vec.length());
}

#[ignore]
#[test]
fn test_unit() {
    let vec = Vec3::new(1.0, 2.0, 43.0);

    assert_eq!(vec3::unit_vector(&vec), vec / vec.length());
}

#[test]
fn test_mul() {
    let vec = Vec3::new(1.7, 100.3, 2.23);
    let mlt = 77.8;
    
    assert_eq!(vec * mlt, Vec3::new(1.7*mlt, 100.3*mlt, 2.23*mlt));
}

#[test]
fn test_mul_commutative() {
    let vec = Vec3::new(1.7, 100.3, 2.23);
    let mlt = 77.8;
    
    assert_eq!(mlt * vec, Vec3::new(1.7*mlt, 100.3*mlt, 2.23*mlt));

}

#[test]
fn test_div() {
    let v = Vec3::new(1.7, 100.3, 2.23);
    let dv = 77.8;

    assert_eq!(v / dv, Vec3::new(v.x/dv, v.y/dv, v.z/dv));
}