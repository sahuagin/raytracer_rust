#![feature(generic_associated_types)]
#![feature(float_minimum_maximum)]
#![feature(allocator_api, slice_ptr_get)]
#![feature(box_into_boxed_slice)]
#![feature(int_roundings)]
#![feature(let_chains)]
#![feature(const_fn_trait_bound)]
pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod cube;
pub mod hitlist;
pub mod hittable;
pub mod instances;
pub mod materials;
pub mod perlin;
pub mod ray;
pub mod rectangle;
pub mod sphere;
pub mod textures;
pub mod util;
pub mod vec3;
pub mod volumes;

pub mod prelude {
    pub use super::aabb::*;
    pub use super::bvh::*;
    pub use super::camera::*;
    pub use super::cube::*;
    pub use super::hitlist::*;
    pub use super::hittable::*;
    pub use super::instances::*;
    pub use super::materials::*;
    pub use super::perlin::*;
    pub use super::ray::*;
    pub use super::rectangle::*;
    pub use super::sphere::*;
    pub use super::textures::*;
    pub use super::util::*;
    pub use super::vec3::*;
    pub use super::volumes::*;
}

#[allow(unused_macros, unused_imports)]
#[macro_export]
macro_rules! color_to_texture {
    ($col:expr) => {
        $crate::textures::TextureType::ConstantTexture($crate::textures::ConstantTexture::new($col))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! vect {
    ($x: expr, $y: expr, $z: expr) => {
        $crate::vec3::Vec3::new($x, $y, $z)
    };
}
#[macro_export]
#[allow(unused_macros)]
macro_rules! color {
    ($x: expr, $y: expr, $z: expr) => {
        vect!($x, $y, $z)
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! sphere {
    ($c:expr, $r:expr, $mat:expr) => {
        Sphere::new($c, $r, Some($mat))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! ray {
    ($pt1: expr, $pt2: expr, $pt3: expr) => {
        Ray::new($pt1, $pt2, Some($pt3))
    };
    ($pt1: expr, $pt2: expr) => {
        $crate::ray::Ray::new($pt1, $pt2, None)
    };
}

#[macro_export]
macro_rules! wrap_material {
    ($klass:ident, $($p0:expr),*) => {
        $crate::materials::MaterialType::$klass(
            $klass::new(
                $( $p0, )*))
    }
}

#[macro_export]
macro_rules! wrap_texture {
    ($klass:ty, $p0:expr) => {
        $crate::textures::TextureType::$klass($p0)
    };
}

#[macro_export]
macro_rules! wrap_hitter {
    ($klass:item, $($p:expr),*) => {
        $crate::hittable::Hitters::$klass(
            $klass::new(
                $( $p, )*))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
