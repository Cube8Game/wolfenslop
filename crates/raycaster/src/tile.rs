use crate::{error::Result, ray::{Ray, RayCollisionResult}};

pub trait Tile: Send + Sync + 'static {
    /// Casts a ray as long as it is in or on the bounds of the tile
    fn cast_ray(&self, incoming_ray: Ray) -> Result<RayCollisionResult>;
}

pub trait SimpleTile {
    fn get_position(&self) -> (f32, f32);
}

pub trait SimpleTileConstructor {
    fn from_position(position: (f32, f32)) -> Self;
}