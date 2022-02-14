use crate::prelude::*;
#[allow(unused_imports)]
use crate::prelude::Hitters;
#[allow(unused_imports)]
use crate::textures::{Texture, ConstantTexture};
#[allow(unused_macros, unused_imports)]
use crate::materials::{MaterialType, Metal};
#[allow(unused_imports)]
use crate::prelude::Ray;
#[allow(unused_imports)]
use crate::vec3::Vec3;
#[allow(unused_macros, unused_imports)]
#[macro_export]
macro_rules! color_to_texture{
    ($col:expr) => {
        $crate::textures::TextureType::ConstantTexture(
            $crate::textures::ConstantTexture::new(0.0, 0.0, $col))
    }
}

macro_rules! texture_display{
    ($klass:ty) => {
        #[allow(dead_code)]
        impl std::fmt::Display for $klass {
            fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner_fmt(f)
            }
        }
    }
}

texture_display!(dyn Texture);
texture_display!(ConstantTexture);

#[allow(unused_macros, unused_imports)]
use std::fmt;
#[allow(unused_macros, unused_imports)]
use std::ops;
#[allow(unused_imports)]

#[allow(unused_macros)]
macro_rules! vect{
($x: expr, $y: expr, $z: expr) => {
    $crate::vec3::Vec3::new($x, $y, $z)
}
}
#[allow(unused_macros)]
pub(crate) use vect;

#[macro_export]
#[allow(unused_macros)]
macro_rules! color{
($x: expr, $y: expr, $z: expr) => {
    vect!($x, $y, $z)
}
}
#[allow(unused_imports)]
pub(crate) use color;



#[allow(unused_macros)]
#[macro_export]
macro_rules! sphere{
    ($c:expr, $r:expr, $mat:expr) => {
        Sphere::new($c, $r, Some($mat) )
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! ray{
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
    }
}

#[macro_export]
macro_rules! wrap_hitter {
    ($klass:item, $($p:expr),*) => {
        $crate::hittable::Hitters::$klass(
            $klass::new(
                $( $p, )*))
    }
}
 
