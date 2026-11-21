pub fn point_in_aabb(point: (f32, f32), aabb: ((f32, f32), (f32, f32))) -> bool {
    point.0 >= aabb.0.0 &&
    point.0 <= aabb.1.0 &&
    point.1 >= aabb.0.1 &&
    point.1 <= aabb.1.1
}