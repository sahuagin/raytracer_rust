//#![feature(associated_type_bounds)]
use num_traits::float;
#[allow(unused_imports)]
pub use Vec3 as Point3;
pub use Vec3 as Color;
//use std::error::Error;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq)]
//pub struct Vec3T<T, X: Into<T>, Y: Into<T>, Z: Into<T>>
//where T: From<X> + From<Y> + From<Z> {
//pub struct Vec3T<T: float::Float +
//                    std::ops::MulAssign +
//                    std::ops::Mul +
//                    std::ops::Add +
//                    std::ops::Sub +
//                    std::ops::Div +
//                    std::ops::AddAssign +
//                    std::ops::SubAssign +
//                    std::ops::DivAssign> {
pub struct Vec3T<T: float::Float + std::fmt::Display> {
    pub x: T,
    pub y: T,
    pub z: T,
}

trait Vector3Trait {
    type Base;
}

pub type Vec3 = Vec3T<f64>;
// I want to use this in the std::op::Mul so that I can do the 'float * vec' version
// as well as the 'vec3 * float' version we already have
impl Vector3Trait for Vec3 {
    type Base = f64;
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
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

impl<
        T: float::Float
            + std::ops::MulAssign
            + std::ops::Mul
            + std::ops::Add
            + std::ops::Sub
            + std::ops::Div
            + std::ops::AddAssign
            + std::ops::SubAssign
            + std::ops::DivAssign
            + std::fmt::Display,
    > Vec3T<T>
where
    f64: From<T>,
{
    #[allow(unused_imports, dead_code)]
    pub fn new<X: Into<T>, Y: Into<T>, Z: Into<T>>(x: X, y: Y, z: Z) -> Vec3T<T> {
        Vec3T::<T> {
            x: X::into(x),
            y: Y::into(y),
            z: Z::into(z),
        }
    }

    #[allow(unused_imports, dead_code)]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    #[allow(unused_imports, dead_code)]
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[allow(unused_imports, dead_code)]
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[allow(unused_imports, dead_code)]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            -(self.x * rhs.z - self.z * rhs.x),
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[allow(unused_imports, dead_code)]
    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn normalize(&self) -> Self {
        self.unit()
    }

    pub fn get(&self, i: usize) -> T {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    3_i32, i
                );
            }
        }
    }

    // We want to get the corners of a box. in that case, we want the smallest, and
    // largest items in each axis independently
    pub fn min(&self, rhs: &Self) -> Self {
        let x = if self.x < rhs.x { self.x } else { rhs.x };
        let y = if self.y < rhs.y { self.y } else { rhs.y };
        let z = if self.z < rhs.z { self.z } else { rhs.z };
        Self::new(x, y, z)
    }

    pub fn max(&self, rhs: &Self) -> Self {
        let x = if self.x > rhs.x { self.x } else { rhs.x };
        let y = if self.y > rhs.y { self.y } else { rhs.y };
        let z = if self.z > rhs.z { self.z } else { rhs.z };
        Self::new(x, y, z)
    }

    pub fn partial_cmp_by_x(&self, rhs: &Self) -> Option<Ordering>{
        self.x.partial_cmp(&rhs.x)
    }

    pub fn partial_cmp_by_y(&self, rhs: &Self) -> Option<Ordering> {
        self.y.partial_cmp(&rhs.y)
    }

    pub fn partial_cmp_by_z(&self, rhs: &Self) -> Option<Ordering> {
        self.z.partial_cmp(&rhs.z)
    }
}

#[allow(unused_macros)]
macro_rules! impl_binop{
(VEC, $op_trait: ident, $fn_name: ident, $op:tt, $target: ident, $rhs: ident) => {
    impl std::ops::$op_trait<$rhs> for $target {
        type Output = $target;

        fn $fn_name(self, rhs: $rhs) -> Self::Output {
            $target {
                x: self.x $op rhs.x,
                y: self.y $op rhs.y,
                z: self.z $op rhs.z,
            }
        }
    }
};

(SCALAR, $op_trait: ident, $fn_name: ident, $op:tt, $target: ident, $rhs: ident) => {
    impl std::ops::$op_trait<$rhs> for $target {
        type Output = $target;

        fn $fn_name(self, rhs: $rhs) -> Self::Output {
            $target {
                x: self.x $op rhs,
                y: self.y $op rhs,
                z: self.z $op rhs,
            }
        }
    }

    impl std::ops::$op_trait<$target> for $rhs {
        type Output = $target;

        fn $fn_name(self, rhs: $target) -> Self::Output {
            $target {
                x: rhs.x $op self,
                y: rhs.y $op self,
                z: rhs.z $op self,
            }
        }
    }
};
}

//impl_binop!(VEC, Add, add, +, Vec3, Vec3);
//impl_binop!(VEC, Sub, sub, -, Vec3, Vec3);
//impl_binop!(VEC, Mul, mul, *, Vec3, Vec3);
//impl_binop!(VEC, Div, div, /, Vec3, Vec3);

//impl_binop!(SCALAR, Mul, mul, *, Vec3, f64);
//impl_binop!(SCALAR, Div, div, /, Vec3, f64);

// this one doesn't really make sense for a vector
//impl std::ops::Div<Vec3> for f64 {
//    type Output = Vec3;
//
//    fn div(self, rhs: Vec3) -> Self::Output {
//        Vec3 {
//            x: rhs.x / self,
//            y: rhs.y / self,
//            z: rhs.z / self,
//        }
//    }
//}
//
impl<T: float::Float + std::fmt::Display> std::ops::Div<T> for Vec3T<T> {
    type Output = Vec3T<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::Add<T> for Vec3T<T> {
    type Output = Vec3T<T>;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::Add<Vec3T<T>> for Vec3T<T> {
    type Output = Vec3T<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::AddAssign<Vec3T<T>> for Vec3T<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::Sub<T> for Vec3T<T> {
    type Output = Vec3T<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::Sub<Vec3T<T>> for Vec3T<T> {
    type Output = Vec3T<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::SubAssign<Vec3T<T>> for Vec3T<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::Mul<T> for Vec3T<T> {
    type Output = Vec3T<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3T<f64>> for f64 {
    type Output = Vec3T<f64>;
    fn mul(self, rhs: Vec3T<f64>) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::Mul<Vec3T<T>> for Vec3T<T> {
    type Output = Vec3T<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::MulAssign<Vec3T<T>> for Vec3T<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: float::Float + std::fmt::Display> std::ops::MulAssign<T> for Vec3T<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[allow(unused_imports)]
//pub(crate) use impl_binop;

impl<T: float::Float + std::ops::Add + std::fmt::Display> std::ops::Add<&Vec3T<T>> for Vec3T<T> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

//impl<T: float::Float +
//        std::ops::AddAssign +
//        std::fmt::Display
//    > std::ops::AddAssign<Vec3T<T>> for Vec3T<T> {
//    fn add_assign(&mut self, rhs: Self) {
//       *self = Self {
//            x: self.x + rhs.x,
//            y: self.y + rhs.y,
//            z: self.z + rhs.z,
//        }
//    }
//}

impl<T: float::Float + std::ops::AddAssign + std::fmt::Display> std::ops::AddAssign<&Vec3T<T>>
    for Vec3T<T>
{
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: float::Float + std::ops::Sub + std::fmt::Display> std::ops::Sub<&Vec3T<T>> for Vec3T<T> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
//
//impl<T: float::Float + std::ops::SubAssign> std::ops::SubAssign<Vec3T<T>> for Vec3T<T> {
//    fn sub_assign(&mut self, rhs: Self) {
//        *self = Self {
//            x: self.x - rhs.x,
//            y: self.y - rhs.y,
//            z: self.z - rhs.z,
//        }
//    }
//}
//
//
//impl<T: float::Float> std::ops::SubAssign<&Vec3T<T>> for Vec3T<T> {
//fn sub_assign(&mut self, rhs: &Self) {
//    *self = Self {
//        x: self.x - rhs.x,
//        y: self.y - rhs.y,
//        z: self.z - rhs.z,
//    }
//}
//}
//
//
//impl<T: float::Float> std::ops::MulAssign<T> for Vec3T<T> {
//fn mul_assign(&mut self, rhs: T) {
//    self.x *= rhs;
//    self.y *= rhs;
//    self.z *= rhs;
//}
//}
//
//impl<T: float::Float +
//        std::ops::DivAssign +
//        std::fmt::Display
//    > std::ops::DivAssign<T> for Vec3T<T> {
//    fn div_assign(&mut self, rhs: T) {
//        self.x /= rhs;
//        self.y /= rhs;
//        self.z /= rhs;
//    }
//}
//
impl<T: float::Float + std::ops::DivAssign + std::fmt::Display> std::ops::DivAssign<T>
    for Vec3T<T>
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: float::Float + std::ops::DivAssign + std::fmt::Display> std::ops::DivAssign<&Vec3T<T>>
    for Vec3T<T>
{
    fn div_assign(&mut self, rhs: &Vec3T<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// NOTE: This isn't how I'd normally want to represent pretty printed output,
// but these will be printed out into a ppm file, so it needs to be in this format.
impl<T: float::Float + std::fmt::Display> std::fmt::Display for Vec3T<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]

mod test {
    use crate::vec3::{dot, unit_vector, Vec3};
    #[test]
    fn test_default_construction() {
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

        assert_eq!(vec + &vec, Vec3::new(6.0, 8.0, 10.0));
        assert_eq!(vec + vec, Vec3::new(6.0, 8.0, 10.0));
        assert_eq!(vec + vec1, Vec3::new(4.0, 4.0, 5.0));
        assert_eq!(vec + vec2, Vec3::new(3.0, 5.0, 5.0));
        assert_eq!(vec + vec3, Vec3::new(3.0, 4.0, 6.0));
        assert_eq!(vec + vec4, Vec3::new(4.0, 5.0, 6.0));
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
        assert_eq!(vec, Vec3::new(3.0 * 0.3, 4.0 * 0.3, 5.0 * 0.3));
        vec = vec_dup.clone();
        vec *= 0.3;
        assert_eq!(vec, Vec3::new(0.25 * 4.0 * 3.0 * 0.3, 4.0 * 0.3, 5.0 * 0.3));
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
        assert_eq!(vec, Vec3::new(3.0 / 0.3, 4.0 / 0.3, 5.0 / 0.3));
        vec = vec_dup.clone();
        vec /= 0.3;
        assert_eq!(vec, Vec3::new(0.25 * 4.0 * 3.0 / 0.3, 4.0 / 0.3, 5.0 / 0.3));
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

        assert_eq!(vec - &vec, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(vec - vec, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(vec - vec1, Vec3::new(2.0, 4.0, 5.0));
        assert_eq!(vec - vec2, Vec3::new(3.0, 3.0, 5.0));
        assert_eq!(vec - vec3, Vec3::new(3.0, 4.0, 4.0));
        assert_eq!(vec - vec4, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut vec = Vec3::new(3.0, 4.0, 5.0);
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        let vec3 = Vec3::new(0.0, 0.0, 1.0);
        let vec4 = Vec3::new(1.0, 1.0, 1.0);

        let vec_dup = vec.clone();
        vec -= vec.clone();
        assert_eq!(vec, Vec3::new(0.0, 0.0, 0.0));
        vec = vec_dup.clone();
        vec -= vec;
        assert_eq!(vec, Vec3::new(0.0, 0.0, 0.0));
        vec = vec_dup.clone();
        vec -= vec1;
        assert_eq!(vec, Vec3::new(2.0, 4.0, 5.0));
        vec = vec_dup.clone();
        vec -= vec1;
        assert_eq!(vec, Vec3::new(2.0, 4.0, 5.0));
        vec = vec_dup.clone();
        vec -= vec2;
        assert_eq!(vec, Vec3::new(3.0, 3.0, 5.0));
        vec = vec_dup.clone();
        vec -= vec2;
        assert_eq!(vec, Vec3::new(3.0, 3.0, 5.0));
        vec = vec_dup.clone();
        vec -= vec3;
        assert_eq!(vec, Vec3::new(3.0, 4.0, 4.0));
        vec = vec_dup.clone();
        vec -= vec3;
        assert_eq!(vec, Vec3::new(3.0, 4.0, 4.0));
        vec = vec_dup.clone();
        vec -= vec4;
        assert_eq!(vec, Vec3::new(2.0, 3.0, 4.0));
        vec = vec_dup.clone();
        vec -= vec4;
        assert_eq!(vec, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_length() {
        let vec = Vec3::new(3.2, 4.0, 5.0);
        let vec2 = Vec3::new(15.0, -5.0, 42.0);

        assert_eq!(
            vec.length(),
            ((3.2 * 3.2) as f64 + (4.0 * 4.0) as f64 + (5.0 * 5.0) as f64).sqrt()
        );
        assert_eq!(
            vec2.length(),
            (15.0 * 15.0 as f64 + -5.0 * -5.0 as f64 + 42.0 * 42.0 as f64).sqrt()
        );
    }

    #[test]
    fn test_squared_length() {
        let vec = Vec3::new(3.2, 4.0, 5.0);
        let vec2 = Vec3::new(15.0, -5.0, 42.0);

        assert_eq!(
            vec.length_squared(),
            (3.2 * 3.2) as f64 + (4.0 * 4.0) as f64 + (5.0 * 5.0) as f64
        );
        assert_eq!(
            vec2.length_squared(),
            15.0 * 15.0 as f64 + -5.0 * -5.0 as f64 + 42.0 * 42.0 as f64
        );
    }

    #[test]
    fn test_dot_product() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(3.0, 2.0, 1.0);

        assert_eq!(vec.dot(&vec2), 1.0 * 3.0 + 2.0 * 2.0 + 3.0 * 1.0);
    }

    #[test]
    fn test_dot_double() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(3.0, 2.0, 1.0);

        assert_eq!(dot(&vec, &vec2), 1.0 * 3.0 + 2.0 * 2.0 + 3.0 * 1.0);
    }

    #[test]
    fn test_cross_product() {
        let vec = Vec3::new(1.0, 2.0, 43.0);
        let vec2 = Vec3::new(32.3, 6.7, 10.4);

        assert_eq!(
            vec.cross(&vec2),
            Vec3::new(
                2.0 * 10.4 - 43.0 * 6.7,
                -(1.0 * 10.4 - 43.0 * 32.3),
                1.0 * 6.7 - 2.0 * 32.3
            )
        );
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

        assert_eq!(unit_vector(&vec), vec / vec.length());
    }

    #[test]
    fn test_mul() {
        let vec = Vec3::new(1.7, 100.3, 2.23);
        let mlt = 77.8;

        assert_eq!(vec * mlt, Vec3::new(1.7 * mlt, 100.3 * mlt, 2.23 * mlt));
    }

    #[test]
    fn test_mul_commutative() {
        let vec = Vec3::new(1.7, 100.3, 2.23);
        let mlt = 77.8;

        assert_eq!(mlt * vec, Vec3::new(1.7 * mlt, 100.3 * mlt, 2.23 * mlt));
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(1.7, 100.3, 2.23);
        let dv = 77.8;

        assert_eq!(v / dv, Vec3::new(v.x / dv, v.y / dv, v.z / dv));
    }

    #[test]
    fn test_mul_vectors() {
        let vec = Vec3::new(1.0, 2.0, -3.0);
        let vec2 = Vec3::new(5.0, 6.0, 7.0);
        let ans = Vec3::new(5.0, 12.0, -21.0);

        assert_eq!(vec * vec2, ans);
    }

    #[test]
    fn test_new_with_0_instead_of_0f() {
        let vec = Vec3::new(1.0, 0, 1);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 0.0);
        assert_eq!(vec.z, 1.0);
    }

    #[test]
    fn test_min_vectors() {
        let vec1 = Vec3::new(1.0, 2.0, -3.0);
        let vec2 = Vec3::new(5.0, 6.0, 7.0);
        let ans_min = Vec3::new(1.0, 2.0, -3.0);
        let ans_max = Vec3::new(5.0, 6.0, 7.0);
        assert_eq!(vec1.min(&vec2), ans_min, "minimum failed");
        assert_eq!(vec1.max(&vec2), ans_max, "maximum failed");
        assert_eq!(vec2.min(&vec1), ans_min, "minimum failed");
        assert_eq!(vec2.max(&vec1), ans_max, "maximum failed");
    }

    #[test]
    fn test_max_vectors() {}
}
