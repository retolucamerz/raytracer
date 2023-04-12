use raytracer::camera::*;
use raytracer::color::*;
use raytracer::primitives::*;
use raytracer::sampling::*;
use raytracer::scene::*;
use raytracer::structs::*;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

const MAX_WIDTH: usize = 2 * 1920;
const MAX_HEIGHT: usize = 2 * 1080;

const OUTPUT_BUFFER_SIZE: usize = MAX_WIDTH * MAX_HEIGHT * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

// Function to return a pointer to our buffer in wasm memory
#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = OUTPUT_BUFFER.as_ptr();
    }

    return pointer;
}

#[wasm_bindgen]
pub fn generate_image(
    width: usize,
    height: usize,
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
    camera_x: Float,
    camera_y: Float,
    camera_z: Float,
    x_rot: Float,
    y_rot: Float,
    z_rot: Float,
    fov: Float,
    supersampling: bool,
) {
    let t: Float = 0.;
    let light: LightSource = LightSource {
        pos: Vec3 {
            x: 5. * t.cos(),
            y: 4. * t.sin(),
            z: 0.,
        },
        color: Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        },
    };

    let reflective_options = ShadingOptions {
        base_color: COLOR_GREEN,
        ambiant_part: 0.2,
        diffuse_part: 0.4,
        specular_part: 0.,
        specular_coefficient: 4.,
        reflective_part: 0.4,
        refraction_index: 0.,
    };
    let big_sphere: Sphere = Sphere {
        center: Vec3 {
            x: 0.,
            y: 0.,
            z: 5.,
        },
        radius: 1.,
        options: ShadingOptions {
            base_color: COLOR_BLUE,
            ..OPTIONS
        },
    };
    let small_sphere: Sphere = Sphere {
        center: Vec3 {
            x: 0.,
            y: 0.,
            z: 4. + 0.5 * t.sin(),
        },
        radius: 0.3,
        options: ShadingOptions {
            base_color: COLOR_RED,
            ..reflective_options
        },
    };
    let sphere1: Sphere = Sphere {
        center: Vec3 {
            x: 1.15 * t.cos(),
            y: 1.15 * t.sin(),
            z: 4.1,
        },
        radius: 0.3,
        options: reflective_options,
    };
    let sphere2: Sphere = Sphere {
        center: Vec3 {
            x: 1.15 * (t + 2. * PI / 3.).cos(),
            y: 1.15 * (t + 2. * PI / 3.).sin(),
            z: 4.1,
        },
        radius: 0.3,
        options: reflective_options,
    };
    let sphere3: Sphere = Sphere {
        center: Vec3 {
            x: 1.15 * (t + 4. * PI / 3.).cos(),
            y: 1.15 * (t + 4. * PI / 3.).sin(),
            z: 4.1,
        },
        radius: 0.3,
        options: reflective_options,
    };
    let checkerboard1: Checkerboard = Checkerboard {
        axis: Axis::ZAxis,
        pos: Vec3 {
            x: 0.,
            y: 0.,
            z: 5.,
        },
        radius: 2.,
        grid_size: 0.25,
        options: WHITE_OPTIONS,
    };
    let checkerboard2: Checkerboard = Checkerboard {
        axis: Axis::ZAxis,
        pos: Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        radius: 2.,
        grid_size: 0.25,
        options: WHITE_OPTIONS,
    };

    let scene: Scene = Scene {
        lights: vec![light],
        spheres: vec![big_sphere, small_sphere, sphere1, sphere2, sphere3],
        boards: vec![checkerboard1, checkerboard2],
    };

    let origin = Vec3 {
        x: camera_x,
        y: camera_y,
        z: 0.5 + camera_z,
    };
    let x_direction = (Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    })
    .rotate(x_rot, y_rot, z_rot);
    let y_direction = (Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    })
    .rotate(x_rot, y_rot, z_rot);
    let camera = Camera::create(origin, x_direction, y_direction, fov, width, height);

    let sample_grid: &[(Float, Float)] = if supersampling {
        &SAMPLE_GRID_5
    } else {
        &SAMPLE_GRID_1
    };

    let width_ = (width as Float) / 2.;
    let height_ = (height as Float) / 2.;
    for y in start_y..end_y {
        for x in start_x..end_x {
            let mut c: Color = COLOR_ZERO;

            for (dx, dy) in sample_grid {
                let sample_x = (dx + x as Float - width_) / width_;
                let sample_y = (dy + y as Float - height_) / height_;
                let ray = &camera.grid_ray(sample_x, sample_y);

                c += scene.shade(ray, 1);
            }
            c = c.scale(255. / (sample_grid.len() as Float));

            let square_number: usize = y * width + x;
            let square_rgba_index: usize = square_number * 4;
            unsafe {
                OUTPUT_BUFFER[square_rgba_index + 0] = c.r as u8; // Red
                OUTPUT_BUFFER[square_rgba_index + 1] = c.g as u8; // Green
                OUTPUT_BUFFER[square_rgba_index + 2] = c.b as u8; // Blue
                OUTPUT_BUFFER[square_rgba_index + 3] = 255;
            }
        }
    }
}
