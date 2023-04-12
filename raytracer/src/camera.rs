use super::structs::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Camera {
    pub origin: Vec3,
    pub x_direction: Vec3,
    pub y_direction: Vec3,
    pub direction: Vec3, // should be x.cross(y)
}

impl Camera {
    pub fn create(
        origin: Vec3,
        x_direction: Vec3,
        y_direction: Vec3,
        fov: Float,
        width: usize,
        height: usize,
    ) -> Camera {
        let x_norm = x_direction.normalize();
        let y_norm = y_direction.normalize();
        let direction = x_direction.cross(y_direction);

        let width_ = (width as Float) / 2.;
        let height_ = (height as Float) / 2.;
        let (x_scale, y_scale) = if width > height {
            let tmp = (fov / 2.).tan();
            (tmp, height_ * tmp / width_)
        } else {
            let tmp = (fov / 2.).tan();
            (width_ * tmp / height_, tmp)
        };

        Camera {
            origin,
            x_direction: x_norm.scale(x_scale),
            y_direction: y_norm.scale(y_scale),
            direction,
        }
    }

    pub fn grid_ray(&self, x: Float, y: Float) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction + self.x_direction.scale(x) + self.y_direction.scale(y),
        }
    }
}
