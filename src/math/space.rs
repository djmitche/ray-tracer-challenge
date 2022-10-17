/// A space is a ZST used to tag a value as relating to a coordinate space.
pub trait Space: Default + PartialEq + Clone + Copy + std::fmt::Debug {}
