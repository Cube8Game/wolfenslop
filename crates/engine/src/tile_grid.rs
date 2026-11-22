use bevy::{math::FloatPow, platform::collections::HashMap, prelude::*};
use raycaster::{basic_tiles::EmptyTile, ray::{Ray, RayCollisionResult}, tile::Tile};
use raycaster::error::{Error as RayCasterError, Result as RayCasterResult};

#[derive(Component, Default)]
pub struct SimpleTileGrid {
    tiles: HashMap<(i32, i32), Box<dyn Tile>>,
}

impl SimpleTileGrid {
    pub fn empty(size: (i32, i32)) -> Self {
        let mut result = Self::default();
        for x in 0..size.0 {
            for y in 0..size.1 {
                let position = (x as f32, y as f32);
                result.tiles.insert((x, y), Box::new(EmptyTile::new(position)));
            }
        };
        result
    }

    pub fn bordered(size: (i32, i32)) -> Self {
        let mut result = Self::empty(size);

        // Bottom wall
        for x in 0..size.0 {
            let position = (x as f32, 0.0);
            result.tiles.insert((x, 0), Box::new(EmptyTile::new(position)));
        }

        // Top wall
        for x in 0..size.0 {
            let position = (x as f32, (size.1 - 1) as f32);
            result.tiles.insert((x, size.1 - 1), Box::new(EmptyTile::new(position)));
        }

        // Left wall
        for y in 0..size.1 {
            let position = (0.0, y as f32);
            result.tiles.insert((0, y), Box::new(EmptyTile::new(position)));
        }

        // Right wall
        for y in 0..size.1 {
            let position = ((size.0 - 1) as f32, y as f32);
            result.tiles.insert((size.0 - 1, y), Box::new(EmptyTile::new(position)));
        }

        result
    }
}

impl SimpleTileGrid {
    fn cast_ray_single_tile(&self, incoming_ray: Ray) -> RayCasterResult<RayCollisionResult> {
        let ray_position = incoming_ray.get_position();
        let tile_coords = (
            ray_position.0.floor() as i32 - if incoming_ray.get_direction_vector().0.is_sign_negative() && ray_position.0.fract() == 0.0 {1} else {0}, 
            ray_position.1.floor() as i32 - if incoming_ray.get_direction_vector().1.is_sign_negative() && ray_position.1.fract() == 0.0 {1} else {0},
        );
        let tile = self.tiles.get(&tile_coords);
        if let Some(tile) = tile {
            tile.cast_ray(incoming_ray)
        } else {
            Err(RayCasterError::RayOutOfBounds(incoming_ray))
        }
    }

    pub fn cast_ray(&self, incoming_ray: Ray, max_tiles: usize) -> (f32, f32) {
        let mut ray = incoming_ray;
        for _ in 0..max_tiles {
            match self.cast_ray_single_tile(ray) {
                Ok(collision) => {
                    match collision {
                        RayCollisionResult::Continue(new_ray) => {
                            ray = new_ray;
                        }
                        RayCollisionResult::Hit(x, y) => {
                            return (x, y);
                        }
                    }
                }
                Err(e) => {
                    match e {
                        RayCasterError::RayOutOfBounds(ray) => {
                            return ray.get_position();
                        }
                    }
                }
            }
        };
        ray.get_position()
    }

    // Helper functions
    fn distance(v1: (f32, f32), v2: (f32, f32)) -> f32 {
        ((v1.0-v2.0).squared() + (v1.1-v2.1).squared()).sqrt()
    }

    fn get_angle_offset(fov: f32, ratio: f32) -> f32 {
        let max_horizontal_deviation = (fov * 0.5).tan();
        let ratio_normalized = ratio * 2.0 - 1.0; // -1.0 to 1.0
        let horizontal_deviation = ratio_normalized * max_horizontal_deviation;
        horizontal_deviation.atan()
    }

    pub fn cast_ray_vertical_distance(&self, ray_angle_offset: f32, ray_position: (f32, f32), direction: f32, max_tiles: usize) -> f32 {
        let ray = Ray::from_radians(direction + ray_angle_offset, ray_position);
        let actual_distance = Self::distance(
                self.cast_ray(ray, max_tiles), 
                ray_position
            ); 
        actual_distance * ray_angle_offset.cos()
    }
}

impl SimpleTileGrid {
    pub fn raycast<const S: usize>(&self, position: (f32, f32), direction: f32, fov: f32) -> [f32; S] {
        let mut result: [f32; S] = [0.0; S];
        for i in 0..S {
            let ray_angle_offset = Self::get_angle_offset(fov, ((i as f32) + 0.5) / (S as f32));
            result[i] = self.cast_ray_vertical_distance(ray_angle_offset, position, direction, 256) // TODO: Replace magic number with some function argument
        }
        result
    }
}
