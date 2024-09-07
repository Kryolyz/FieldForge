
// use strum::{EnumIter, AsRefStr};
use strum_macros::{EnumIter, AsRefStr};

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr, PartialEq)]
pub enum PrimitiveType {
    None,
    Sphere,
    Cuboid,
    Cylinder,
    Wedge,
    Pyramid,
}