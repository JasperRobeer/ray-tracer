//! Representation of a ray

use vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin:    Vector3,
    direction: Vector3,
}

impl Ray {
   fn new(origin: Vector3, direction: Vector3) -> Ray {
       Ray {
           origin: origin,
           direction: direction
       }
   }
}
