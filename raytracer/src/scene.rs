use super::color::*;
use super::primitives::*;
use super::structs::*;

pub struct LightSource {
    pub pos: Vec3,
    pub color: Color,
}

impl LightSource {
    pub fn diffuse(&self, intersection: &Vec3, normal: &Vec3) -> Float {
        let light_direction = self.pos - *intersection;
        (normal.dot(light_direction) / light_direction.norm()).max(0.)
    }

    pub fn specular(
        &self,
        intersection_point: &Vec3,
        normal: &Vec3,
        camera_pos: &Vec3,
        specular_coefficient: Float,
    ) -> Float {
        let (c, i, n) = (*camera_pos, *intersection_point, *normal);

        let V: Vec3 = (c - i).normalize();
        let L: Vec3 = (self.pos - i).normalize();
        let R: Vec3 = n.scale(2. * n.dot(L)) - L;

        let c = R.dot(V).max(0.);

        c.powf(specular_coefficient)
    }
}

pub const OPTIONS: ShadingOptions = ShadingOptions {
    base_color: COLOR_BLUE,
    ambiant_part: 0.05,
    diffuse_part: 0.75,
    specular_part: 0.2,
    specular_coefficient: 4.,
    reflective_part: 0.,
    refraction_index: 0.,
};

pub struct Scene {
    pub lights: Vec<LightSource>,
    pub spheres: Vec<Sphere>,
    pub boards: Vec<Checkerboard>,
}

const BIAS: Float = 1e-5;

impl Scene {
    pub fn intersect(&self, ray: &Ray) -> Intersection {
        let mut intersection: Intersection = None;
        let mut closest_t: Float = 0.;

        for sphere in &self.spheres {
            let potential_intersection = sphere.intersect(ray);
            match potential_intersection {
                None => continue,
                Some((t, _, _, _)) => {
                    if t < BIAS {
                        continue;
                    }

                    if intersection.is_none() || (t < closest_t) {
                        intersection = potential_intersection;
                        closest_t = t;
                    }
                }
            }
        }

        for board in &self.boards {
            let potential_intersection = board.intersect(ray);
            match potential_intersection {
                None => continue,
                Some((t, _, _, _)) => {
                    if t < BIAS {
                        continue;
                    }

                    if intersection.is_none() || (t < closest_t) {
                        intersection = potential_intersection;
                        closest_t = t;
                    }
                }
            }
        }

        intersection
    }

    pub fn shade(&self, ray: &Ray, recursive: u32) -> Color {
        let (c, _, _, _) = self.shade_with_last_intersect(ray, recursive);
        c
    }

    pub fn shade_with_last_intersect(
        &self,
        ray: &Ray,
        recursive: u32,
    ) -> (Color, Vec3, Float, ShadingOptions) {
        // ToDo: implement refraction

        match self.intersect(ray) {
            Some((t, intersection_point, normal, options)) => {
                let mut light_intensity = 0.;
                let mut specular_color: Color = COLOR_ZERO;
                for light in &self.lights {
                    // skip light if it is not visible
                    let d = (light.pos - intersection_point).norm();
                    let light_ray: Ray = Ray {
                        origin: intersection_point,
                        direction: (light.pos - intersection_point).scale(1. / d),
                    };
                    match self.intersect(&light_ray) {
                        None => (),
                        Some((t, _, _, _)) => {
                            if t + BIAS < d {
                                continue;
                            }
                        }
                    }

                    light_intensity += light.diffuse(&intersection_point, &normal);

                    let specular_intensity = light.specular(
                        &intersection_point,
                        &normal,
                        &ray.origin,
                        options.specular_coefficient,
                    );
                    specular_color += light.color.scale(specular_intensity);
                }
                light_intensity = light_intensity.min(1.);
                let diffuse_color = options
                    .base_color
                    .scale(options.diffuse_part * light_intensity + options.ambiant_part);

                // ToDo: physically correct combination of colors
                specular_color =
                    specular_color.scale(options.specular_part / (self.lights.len() as Float));

                let reflected_color = if options.reflective_part > 1e-5 && recursive > 0 {
                    let c = -ray.direction.normalize();
                    let direction = normal.scale(2. * normal.dot(c)) - c;
                    let reflected_ray = Ray {
                        origin: intersection_point,
                        direction,
                    };

                    self.shade(&reflected_ray, recursive - 1)
                } else {
                    COLOR_ZERO
                }
                .scale(options.reflective_part);

                (
                    diffuse_color + specular_color + reflected_color,
                    normal,
                    t,
                    options,
                )
            }
            None => (COLOR_ZERO, VEC3_ZERO, 0., OPTIONS),
        }
    }
}

pub const WHITE_OPTIONS: ShadingOptions = ShadingOptions {
    base_color: COLOR_WHITE,
    ambiant_part: 0.4,
    diffuse_part: 0.3,
    specular_part: 0.3,
    specular_coefficient: 4.,
    reflective_part: 0.,
    refraction_index: 0.,
};

pub const SPHERE1: Sphere = Sphere {
    center: Vec3 {
        x: 0.,
        y: 0.,
        z: 5.,
    },
    radius: 1.,
    options: OPTIONS,
};
pub const SPHERE2: Sphere = Sphere {
    center: Vec3 {
        x: -0.7,
        y: -0.7,
        z: 4.8,
    },
    radius: 0.5,
    options: OPTIONS,
};
pub const SPHERE3: Sphere = Sphere {
    center: Vec3 {
        x: 0.,
        y: 0.,
        z: 4.8,
    },
    radius: 0.5,
    options: OPTIONS,
};

pub const Z_CHECKERBOARD1: Checkerboard = Checkerboard {
    axis: Axis::ZAxis,
    pos: Vec3 {
        x: 0.,
        y: 0.,
        z: 7.,
    },
    radius: 5.,
    grid_size: 1.,
    options: WHITE_OPTIONS,
};
pub const Z_CHECKERBOARD2: Checkerboard = Checkerboard {
    axis: Axis::ZAxis,
    pos: Vec3 {
        x: 0.,
        y: 0.,
        z: -2.,
    },
    radius: 5.,
    grid_size: 1.,
    options: WHITE_OPTIONS,
};

pub const Y_CHECKERBOARD: Checkerboard = Checkerboard {
    axis: Axis::YAxis,
    pos: Vec3 {
        x: 0.,
        y: -7.,
        z: 0.,
    },
    radius: 5.,
    grid_size: 1.,
    options: WHITE_OPTIONS,
};

pub const LIGHT1: LightSource = LightSource {
    pos: Vec3 {
        x: 5.,
        y: 5.,
        z: 0.,
    },
    color: Color {
        r: 1.,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
};
pub const LIGHT2: LightSource = LightSource {
    pos: Vec3 {
        x: -5.,
        y: -4.,
        z: 2.,
    },
    color: Color {
        r: 1.,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
};
