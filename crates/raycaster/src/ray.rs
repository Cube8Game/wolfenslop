#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    direction: (f32, f32),
    position: (f32, f32),
}

impl Ray {
    pub fn from_vector(direction: (f32, f32), position: (f32, f32)) -> Self {
        Self { direction, position }
    }

    pub fn from_radians(direction: f32, position: (f32, f32)) -> Self {
        Self { direction: (direction.cos(), direction.sin()), position }
    }
}

impl Ray {
    pub fn get_direction_vector(&self) -> (f32, f32) {
        self.direction
    }

    pub fn get_position(&self) -> (f32, f32) {
        self.position
    }

    pub fn get_gradient(&self) -> f32 {
        let vector = self.get_direction_vector();
        vector.1 / vector.0
    }

    pub fn get_gradient_reciprocal(&self) -> f32 {
        let vector = self.get_direction_vector();
        vector.0 / vector.1
    }
}

impl Ray {
    // TODO: Add checks for edge cases

    pub fn intersection_vertical_bidirectional(&self, x: f32) -> Self {
        let gradient = self.get_gradient();
        let position = self.get_position();
        Self::from_vector(self.get_direction_vector(), (x, x * gradient + position.1 - position.0 * gradient))
    }

    pub fn intersection_vertical(&self, x: f32) -> Option<Self> {
        let incoming_x = self.get_position().0;
        let incoming_direction_x = self.get_direction_vector().0; 
        if 
            incoming_x < x && incoming_direction_x.is_sign_positive() ||
            incoming_x > x && incoming_direction_x.is_sign_negative()
        {
            Some(self.intersection_vertical_bidirectional(x))
        } else {
            None
        }
    }

    pub fn intersection_horizontal_bidirectional(&self, y: f32) -> Self {
        let gradient = self.get_gradient_reciprocal();
        let position = self.get_position();
        Self::from_vector(self.get_direction_vector(), (y * gradient + position.0 - position.1 * gradient, y))
    }

    pub fn intersection_horizontal(&self, y: f32) -> Option<Self> {
        let incoming_y = self.get_position().1;
        let incoming_direction_y = self.get_direction_vector().1; 
        if 
            incoming_y < y && incoming_direction_y.is_sign_positive() ||
            incoming_y > y && incoming_direction_y.is_sign_negative()
        {
            Some(self.intersection_horizontal_bidirectional(y))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RayCollisionResult {
    Continue(Ray),
    Hit(RayHit)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RayHit {
    pub position: (f32, f32),
    pub texture: f32
}

impl RayHit {
    pub fn new(position: (f32, f32), texture: f32) -> Self {
        Self {
            position,
            texture
        }
    }
}