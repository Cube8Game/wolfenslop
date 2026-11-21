use crate::ray::Ray;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Ray out of bounds: {:?}", .0)]
    RayOutOfBounds(Ray),
}

pub type Result<T> = std::result::Result<T, Error>;