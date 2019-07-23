extern crate image;
use image::DynamicImage;

pub struct Point{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Sphere{
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene{
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

pub fn render(scene: &Scene) -> DynamicImage{
    DynamicImage::new_rgb8(scene.width, scene.height)
}

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere{
            center: Point{
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color{
                r: 0.4,
                g: 1.0,
                b: 0.4,
            },
        },
    };

    let img:DynamicImage = render(&scene);
    img.save("test.png").unwrap();
}
