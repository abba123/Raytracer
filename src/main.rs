extern crate image;
use image::DynamicImage;

pub struct Point{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point{
    pub fn zero() -> Point{
        Point{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
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

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray{
    pub fn create_prime(x: u32, y: u32, scene: Scene) -> Ray{
        Ray{
            origin: Point::zero(),
            direction: Vector3::zero(),
        }
    }
}

pub struct Vector3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3{
    pub fn zero() -> Vector3{
        Vector3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64{
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3{
        let l_inv = self.length().recip();
        Vector3{
            x: self.x * l_inv,
            y: self.y * l_inv,
            z: self.z * l_inv,
        }
    }
}

pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
     let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
     let aspect_ratio = (scene.width as f64) / (scene.height as f64);
     let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio ) * fov_adjustment;
     let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;
       

     Ray {
         origin: Point::zero(),
         direction: Vector3 {
                 x: sensor_x,
                 y: sensor_y,
                 z: -1.0,
             }
             .normalize(),
     }
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
