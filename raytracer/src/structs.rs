use std::ops;
use wasm_bindgen::prelude::*;

pub type Float = f32;

#[wasm_bindgen]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: Float) -> Vec3 {
        self.origin + self.direction.scale(t)
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

#[wasm_bindgen]
impl Vec3 {
    pub fn scale(self, s: Float) -> Vec3 {
        Vec3 {
            x: s * self.x,
            y: s * self.y,
            z: s * self.z,
        }
    }

    pub fn norm(self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self.scale(1. / self.norm())
    }

    pub fn dot(self, other: Vec3) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn from_spherical(radius: Float, theta: Float, phi: Float) -> Vec3 {
        Vec3 {
            x: radius * theta.sin() * phi.cos(),
            y: radius * theta.sin() * phi.sin(),
            z: radius * theta.cos(),
        }
    }

    pub fn rotate(self, x_rot: Float, y_rot: Float, z_rot: Float) -> Vec3 {
        // Rotate around x-axis
        let self_x = Vec3 {
            x: self.x,
            y: self.y * x_rot.cos() - self.z * x_rot.sin(),
            z: self.y * x_rot.sin() + self.z * x_rot.cos(),
        };

        // Rotate around y-axis
        let self_xy = Vec3 {
            x: self_x.x * y_rot.cos() + self_x.z * y_rot.sin(),
            y: self_x.y,
            z: -self_x.x * y_rot.sin() + self_x.z * y_rot.cos(),
        };

        // Rotate around z-axis
        let self_xyz = Vec3 {
            x: self_xy.x * z_rot.cos() - self_xy.y * z_rot.sin(),
            y: self_xy.x * z_rot.sin() + self_xy.y * z_rot.cos(),
            z: self_xy.z,
        };

        self_xyz
    }
}

pub const VEC3_ZERO: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: 0.,
};

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
