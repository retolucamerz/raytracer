use wasm_bindgen::prelude::*;

use super::color::*;
use super::structs::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ShadingOptions {
    pub base_color: Color,
    pub ambiant_part: Float,
    pub diffuse_part: Float,
    pub specular_part: Float,
    pub specular_coefficient: Float,
    pub reflective_part: Float,
    pub refraction_index: Float,
}

pub type Intersection = Option<(Float, Vec3, Vec3, ShadingOptions)>;

pub trait Primitive {
    // computes the intersection with the primitive and
    // returns Some(distance, intersection point, normal vec, shading options) if successful, otherwise None
    fn intersect(&self, ray: &Ray) -> Intersection;
}

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
    pub options: ShadingOptions,
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let oc = ray.origin - self.center;

        let a: Float = ray.direction.dot(ray.direction);
        let b: Float = 2. * ray.direction.dot(oc);
        let c: Float = oc.dot(oc) - self.radius * self.radius;

        let d: Float = b * b - 4. * a * c;

        if d < 0. {
            return None;
        }

        let t: Float = (-b - d.sqrt()) / (2. * a);
        let intersection: Vec3 = ray.at(t);
        let normal = (intersection - self.center).normalize();

        Some((t, intersection, normal, self.options))
    }
}

pub enum Axis {
    XAxis,
    YAxis,
    ZAxis,
}

pub struct Checkerboard {
    pub axis: Axis,
    pub pos: Vec3,
    pub radius: Float,
    pub grid_size: Float,
    pub options: ShadingOptions,
}

impl Primitive for Checkerboard {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let ax_pos = match self.axis {
            Axis::XAxis => self.pos.x,
            Axis::YAxis => self.pos.y,
            Axis::ZAxis => self.pos.z,
        };

        // i = o + t d => t = (i - o) / d
        let t = match self.axis {
            Axis::XAxis => (ax_pos - ray.origin.x) / ray.direction.x,
            Axis::YAxis => (ax_pos - ray.origin.y) / ray.direction.y,
            Axis::ZAxis => (ax_pos - ray.origin.z) / ray.direction.z,
        };
        if t < 0. || t > 1e12 {
            return None;
        };

        let normal = match self.axis {
            Axis::XAxis => Vec3 {
                x: if ray.origin.x > ax_pos { 1. } else { -1. },
                y: 0.,
                z: 0.,
            },
            Axis::YAxis => Vec3 {
                x: 0.,
                y: if ray.origin.y > ax_pos { 1. } else { -1. },
                z: 0.,
            },
            Axis::ZAxis => Vec3 {
                x: 0.,
                y: 0.,
                z: if ray.origin.z > ax_pos { 1. } else { -1. },
            },
        };

        let intersection_point = ray.at(t);
        if (intersection_point - self.pos).norm() > self.radius {
            return None;
        };

        let d = match self.axis {
            Axis::XAxis => {
                (intersection_point.y / self.grid_size).floor() as i32
                    + (intersection_point.z / self.grid_size).floor() as i32
            }
            Axis::YAxis => {
                (intersection_point.z / self.grid_size).floor() as i32
                    + (intersection_point.z / self.grid_size).floor() as i32
            }
            Axis::ZAxis => {
                (intersection_point.x / self.grid_size).floor() as i32
                    + (intersection_point.y / self.grid_size).floor() as i32
            }
        };
        let base_color = if d % 2 == 0 {
            COLOR_BLACK
        } else {
            self.options.base_color
        };

        Some((
            t,
            intersection_point,
            normal,
            ShadingOptions {
                base_color,
                ..self.options
            },
        ))
    }
}
