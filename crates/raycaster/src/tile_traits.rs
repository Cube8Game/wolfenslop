use crate::{basic_tiles::EmptyTile, error::Result, ray::{Ray, RayCollisionResult}, tile::{SimpleTile, Tile}};

pub trait TileDefaultEmpty {
    fn cast_ray_or_empty(&self, incoming_ray: Ray) -> Option<RayCollisionResult>;
}

impl<T: TileDefaultEmpty + SimpleTile + Send + Sync + 'static> Tile for T {
    fn cast_ray(&self, incoming_ray: Ray) -> Result<RayCollisionResult> {
        if let Some(ray) = self.cast_ray_or_empty(incoming_ray) {
            return Ok(ray);
        }
        let fallback = EmptyTile::new(self.get_position());
        fallback.cast_ray(incoming_ray)
    }
}