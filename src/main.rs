use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ops::Range;

mod color;
mod point;
mod ray;
mod camera;
mod sphere;
mod scene;
mod image;
mod hittable;

const FILENAME: &str = "rendered_output.ppm";

fn add_color(s: &mut String, color: &color::Color) {
    let color_string = format!("{} {} {}\n", color.r, color.g, color.b);
    s.push_str(&color_string);
}


#[allow(dead_code)]
fn create_rainbow_gradient(scene: &scene::Scene, mut output: &mut String) {
    let row_range = Range{ start: 0, end: scene.height}.rev();
    for row in row_range {
        for col in 0..scene.width {
            let r = (255.99 * (col as f64 / scene.width as f64)) as u8;
            let g = (255.99 * (row as f64 / scene.height as f64)) as u8;
            let b = (255.99 * 0.25) as u8;
            let pixel_color = color::Color{r, g, b};
            add_color(&mut output, &pixel_color);
        }
    }
}

#[allow(dead_code)]
fn create_blue_gradient(scene: &scene::Scene, mut output: &mut String) {
    let row_range = Range{ start: 0, end: scene.height}.rev();
    for row in row_range {
        for col in 0..scene.width {
            let u = col as f64 / scene.width as f64;
            let v = row as f64 / scene.height as f64;
            let direction = scene.camera.lower_left + &(scene.camera.horizontal * u) + &(scene.camera.vertical * v);
            let ray = ray::Ray{ start: scene.camera.origin, direction };
            // assuming a single sphere to start! Given more, we can loop over them
            let pixel_color = ray.color_at_ray(&scene.objects[0]);
            add_color(&mut output, &pixel_color);
        }
    }
}

fn main() {
    use point::Point;

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Point { x: viewport_width, y: 0.0, z: 0.0};
    let vertical =  Point { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left = origin - &(horizontal / &Point { x: 2.0, y: 2.0, z: 2.0 }) - &(vertical / &Point { x: 2.0, y: 2.0, z: 2.0 }) - &Point { x: 0.0, y: 0.0, z: focal_length};

    let camera = camera::Camera{
        origin: origin,
        horizontal: horizontal,
        vertical: vertical,
        lower_left: lower_left,
    };

    // Scene
    let sphere_one = sphere::Sphere{
        center: Point { x: 0.0, y: 0.0, z: -2.0 },
        radius: 0.5,
    };

    let sphere_two = sphere::Sphere{
        center: Point { x: 4.0, y: 4.0, z: -2.0 },
        radius: 0.5,
    };

    let scene = scene::Scene{
        width: image_width,
        height: image_height,
        camera,
        objects: vec![sphere_one, sphere_two],
    };

    let mut file_contents = format!("P3\n{} {}\n255\n", scene.width, scene.height);
    let path = Path::new(FILENAME);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(e) => panic!("Couldn't create {}: {}", display, e),
        Ok(file) => file,
    };

    // create_rainbow_gradient(&scene, &mut file_contents);
    create_blue_gradient(&scene, &mut file_contents);

    match file.write_all(file_contents.as_bytes()) {
        Err(e) => panic!("Couldn't write to {}: {}", display, e),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}
