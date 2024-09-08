use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Hash)]
#[repr(u32)]
pub enum PrimitiveType {
    None = 0,
    Sphere,
    Cuboid,
    Cylinder,
    Wedge,
    Pyramid,
}

impl PrimitiveType {
    pub fn iter() -> impl Iterator<Item = PrimitiveType> {
        (0..)
            .map(PrimitiveType::try_from)
            .take_while(Result::is_ok)
            .map(Result::unwrap)
    }
}

impl Default for PrimitiveType {
    fn default() -> Self {
        PrimitiveType::None
    }
}
