use raytracer::camera::*;
use raytracer::color::*;
use raytracer::primitives::*;
use raytracer::scene::*;
use raytracer::structs::*;
use std::f32::consts::PI;

// ROTATING_SHPERES
pub fn render_scene(
    camera_x: Float,
    camera_y: Float,
    camera_z: Float,
    x_rot: Float,
    y_rot: Float,
    z_rot: Float,
    fov: Float,
    width: usize,
    height: usize,
    t: Float,
) -> (Scene, Camera) {
    let t = t / 2.;
    let default_x_rot: Float = 0.9;
    let origin = Vec3 {
        x: 0.,
        y: 2.5,
        z: 3.,
    } + Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    }
    .scale(camera_x)
        + Vec3 {
            x: 0.,
            y: 1.,
            z: 0.,
        }
        .scale(camera_y)
        + Vec3 {
            x: 0.,
            y: 0.,
            z: 1.,
        }
        .scale(camera_z);

    let x_direction = (Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    })
    .rotate(x_rot, y_rot, z_rot)
    .rotate(default_x_rot, 0., 0.);
    let y_direction = (Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    })
    .rotate(x_rot, y_rot, z_rot)
    .rotate(default_x_rot, 0., 0.);
    let camera = Camera::create(origin, x_direction, y_direction, fov, width, height);

    let u = (0.5 * t + 1.5).sin();
    let light: LightSource = LightSource {
        pos: Vec3 {
            x: 5. * u.cos(),
            y: 4. * u.sin(),
            z: 1.8 * u,
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
    let scene = Scene {
        lights: vec![light],
        spheres: vec![big_sphere, sphere1, sphere2, sphere3],
        boards: vec![checkerboard1, checkerboard2],
    };

    (scene, camera)
}
