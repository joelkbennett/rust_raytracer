use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    for row in 1..scene.height {
        for col in 1..scene.width {
            let r = (255.99 * (col as f64 / scene.width as f64)) as u8;
            let g = (255.99 * (row as f64 / scene.height as f64)) as u8;
            let b = (255.99 * 0.2) as u8;
            let pixel_color = color::Color{r, g, b};
            add_color(&mut output, &pixel_color);
        }
    }
}

fn create_blue_gradient(scene: &scene::Scene, mut output: &mut String) {
    for col in 1..scene.height {
        for row in 1..scene.width {
            let u = row as f64 / scene.width as f64;
            let v = col as f64 / scene.height as f64;
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

    let camera = camera::Camera{
        origin: Point { x: 0.0, y: 0.0, z: 0.0 },
        lower_left: Point{ x: -2.0, y: -1.0, z: -1.0 },
        horizontal: Point { x: 4.0, y: 0.0, z: 0.0 },
        vertical: Point { x: 0.0, y: 2.0, z: 0.0 },
    };

    let sphere = sphere::Sphere{
        center: Point { x: 0.0, y: 0.0, z: -2.0 },
        radius: 0.5,
    };

    let scene = scene::Scene{
        width: 1200,
        height: 600,
        camera,
        objects: vec![sphere],
    };

    let mut file_contents = format!("P3\n{} {}\n255\n", scene.width, scene.height);
    let path = Path::new(FILENAME);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(e) => panic!("Couldn't create {}: {}", display, e.description()),
        Ok(file) => file,
    };

    // create_rainbow_gradient(&scene, &mut file_contents);
    create_blue_gradient(&scene, &mut file_contents);

    match file.write_all(file_contents.as_bytes()) {
        Err(e) => panic!("Couldn't write to {}: {}", display, e.description()),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}
