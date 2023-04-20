use raytracer::color::*;
use raytracer::sampling::*;
use raytracer::structs::*;
use wasm_bindgen::prelude::*;

pub mod scene;
use scene::*;

// setup memory to be read from JS
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
    t: Float,
) {
    let (scene, camera) = render_scene(
        camera_x, camera_y, camera_z, x_rot, y_rot, z_rot, fov, width, height, t,
    );

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
