pub mod tile;
pub mod ray;

pub mod tile_traits;
pub mod basic_tiles;

pub mod error;
pub mod helpers;

#[cfg(test)]
mod tests {
    use crate::{basic_tiles::{EmptyTile, FullTile}, error::Error, ray::{Ray, RayCollisionResult}, tile::{SimpleTileConstructor, Tile}};

    #[test]
    fn ray_intersection() {
        let ray = Ray::from_vector((1.0, 1.0), (2.0, 4.0));
        assert_eq!(ray.intersection_horizontal(0.0), None);
        assert_eq!(ray.intersection_horizontal(4.0), None);
        assert_eq!(ray.intersection_horizontal(5.0), Some(Ray::from_vector((1.0, 1.0), (3.0, 5.0))));
        assert_eq!(ray.intersection_vertical(0.0), None);
        assert_eq!(ray.intersection_vertical(2.0), None);
        assert_eq!(ray.intersection_vertical(3.0), Some(Ray::from_vector((1.0, 1.0), (3.0, 5.0))));
    }

    #[test]
    fn empty_tile() {
        let tile = EmptyTile::from_position((2.0, 4.0));
        let ray = Ray::from_vector((1.0, 1.0), (1.0, 1.0));
        assert_eq!(tile.cast_ray(ray), Err(Error::RayOutOfBounds(ray)));
        let ray = Ray::from_vector((-1.0, -1.0), (2.5, 4.5));
        assert_eq!(tile.cast_ray(ray), Ok(RayCollisionResult::Continue(Ray::from_vector((-1.0, -1.0), (2.0, 4.0)))));
        let ray = Ray::from_vector((1.0, 1.0), (2.0, 4.0));
        assert_eq!(tile.cast_ray(ray), Ok(RayCollisionResult::Continue(Ray::from_vector((1.0, 1.0), (3.0, 5.0)))));
        let ray = Ray::from_vector((1.0, -1.0), (2.5, 4.5));
        assert_eq!(tile.cast_ray(ray), Ok(RayCollisionResult::Continue(Ray::from_vector((1.0, -1.0), (3.0, 4.0)))));
        let ray = Ray::from_vector((-1.0, 1.0), (2.5, 4.5));
        assert_eq!(tile.cast_ray(ray), Ok(RayCollisionResult::Continue(Ray::from_vector((-1.0, 1.0), (2.0, 5.0)))));
        let ray = Ray::from_vector((1.0, 2.0), (2.0, 4.0));
        assert_eq!(tile.cast_ray(ray), Ok(RayCollisionResult::Continue(Ray::from_vector((1.0, 2.0), (2.5, 5.0)))));
    }

    #[test]
    fn full_tile() {
        let tile = FullTile::from_position((2.0, 4.0));
        let ray = Ray::from_vector((1.0, 1.0), (1.0, 1.0));
        assert_eq!(tile.cast_ray(ray), Err(Error::RayOutOfBounds(ray)));
        let ray = Ray::from_vector((-1.0, -1.0), (2.5, 4.5));
        assert_eq!(tile.cast_ray(ray), Ok(RayCollisionResult::Hit(2.5, 4.5)));
        let ray = Ray::from_vector((1.0, 1.0), (3.0, 5.0));
        assert_eq!(tile.cast_ray(ray), Ok(RayCollisionResult::Hit(3.0, 5.0)));
    }
}
