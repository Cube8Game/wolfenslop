use crate::{error::{Error, Result}, helpers::point_in_aabb, ray::{Ray, RayCollisionResult}, tile::{SimpleTile, Tile}};

pub struct EmptyTile {
    position: (f32, f32)
}

impl Tile for EmptyTile {
    fn cast_ray(&self, incoming_ray: Ray) -> Result<RayCollisionResult> {
        let aabb = (self.position, (self.position.0 + 1.0, self.position.1 + 1.0));
        if !point_in_aabb(incoming_ray.get_position(), aabb) {
            return Err(Error::RayOutOfBounds(incoming_ray));
        }
        if let Some(ray) = incoming_ray.intersection_vertical(aabb.0.0) {
            let ray_y = ray.get_position().1;
            if aabb.0.1 <= ray_y && ray_y <= aabb.1.1 {
                return Ok(RayCollisionResult::Continue(ray));
            }
        }
        if let Some(ray) = incoming_ray.intersection_vertical(aabb.1.0) {
            let ray_y = ray.get_position().1;
            if aabb.0.1 <= ray_y && ray_y <= aabb.1.1 {
                return Ok(RayCollisionResult::Continue(ray));
            }
        }
        if let Some(ray) = incoming_ray.intersection_horizontal(aabb.0.1) {
            let ray_x = ray.get_position().0;
            if aabb.0.0 <= ray_x && ray_x <= aabb.1.0 {
                return Ok(RayCollisionResult::Continue(ray));
            }
        }
        if let Some(ray) = incoming_ray.intersection_horizontal(aabb.1.1) {
            let ray_x = ray.get_position().0;
            if aabb.0.0 <= ray_x && ray_x <= aabb.1.0 {
                return Ok(RayCollisionResult::Continue(ray));
            }
        }
        return Err(Error::RayOutOfBounds(incoming_ray));
    }
}

impl SimpleTile for EmptyTile {
    fn get_position(&self) -> (f32, f32) {
        self.position
    }
}

impl EmptyTile {
    pub fn new(position: (f32, f32)) -> Self {
        Self { position }
    }
}


pub struct FullTile {
    position: (f32, f32)
}

impl Tile for FullTile {
    fn cast_ray(&self, incoming_ray: Ray) -> Result<RayCollisionResult> {
        let aabb = (self.position, (self.position.0 + 1.0, self.position.1 + 1.0));
        if !point_in_aabb(incoming_ray.get_position(), aabb) {
            return Err(Error::RayOutOfBounds(incoming_ray));
        }
        let position = incoming_ray.get_position();
        Ok(RayCollisionResult::Hit(position.0, position.1))
    }
}

impl SimpleTile for FullTile {
    fn get_position(&self) -> (f32, f32) {
        self.position
    }
}

impl FullTile {
    pub fn new(position: (f32, f32)) -> Self {
        Self { position }
    }
}