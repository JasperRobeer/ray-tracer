//! Camera which observes the scene

use std::f32;
use vector3::*;
use ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin:         Vector3,
    corner:         Vector3,
    horizontal:     Vector3,
    vertical:       Vector3,
    u:              Vector3,
    v:              Vector3,
    w:              Vector3,
    lens_radius:    f32,
}

impl Camera {
    fn new(origin: Vector3, look_at: Vector3, up: Vector3,
           fov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta       = fov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width  = aspect * half_height;
        let w = normalize(origin - look_at);
        let u = normalize(cross(up, w));
        let v = cross(w, u);
        let corner = origin - half_width * focus_dist * u
                            - half_height * focus_dist * v
                            - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical   = 2.0 * half_height * focus_dist * v;

        Camera {
            origin:      origin,
            corner:      corner,
            horizontal:  horizontal,
            vertical:    vertical,
            u:           u,
            v:           v,
            w:           w,
            lens_radius: aperture / 2.0
        }
    }

    /// Produces a ray that is 'transmitted' from the camera.
    fn get_ray(&self, s: f32, t: f32) -> Ray {
        panic!("not implemented")
    }
}
