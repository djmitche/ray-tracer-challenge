use crate::Space;

/// World space is the space in which individual objects are positioned.
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct World;

impl Space for World {}

/// Object space is the standardized space in which an object is defined.
///
/// Each object has its own object space, but all are represented by the
/// same type.
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Object;

impl Space for Object {}

/// Camera space is the standardized space in which the camera operates.
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Camera;

impl Space for Camera {}
