extern crate image;
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use std::ops::{Add, Sub, Mul, Neg};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

#[derive(Clone, Copy)]
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

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color{
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.r) * 255.0) as u8,
            (gamma_encode(self.g) * 255.0) as u8,
            (gamma_encode(self.b) * 255.0) as u8,
            255,
        )
    }
}

#[derive(Clone, Copy)]
pub struct Sphere{
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct Scene{
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray{
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
}

#[derive(Clone, Copy)]
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

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

}



pub trait Intersectable{
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64>{
        let l: Vector3 = self.center - ray.origin;
        let adj2 = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj2 * adj2);
        let radius2 = self.radius * self.radius;
        if d2 > radius2{
            return None
        }
        let len = (radius2 - d2).sqrt();
        
        let inter_len1 = (adj2 - len);
        let inter_len2 = (adj2 + len);
        

        if inter_len1 < inter_len2{
            Some(inter_len1)
        } else{
            Some(inter_len2)
        }
    }
}

pub fn render(scene: &[Scene]) -> DynamicImage {
    println!("{}",scene[0].width);
    let mut image = DynamicImage::new_rgb8(scene[0].width, scene[0].height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene[0].width {
        for y in 0..scene[0].height {
            let mut inter = Option::None;
            let mut color = Color{r: 0.0, g: 0.0, b: 0.0};
            for s in scene{
                let ray = Ray::create_prime(x, y, &s);
                let inter_tmp = s.sphere.intersect(&ray);
                if inter_tmp != None{
                    if inter != None{
                        if inter_tmp < inter{
                            color = s.sphere.color;
                            inter = inter_tmp;
                        }
                    } 
                    else{
                        color = s.sphere.color;
                        inter = inter_tmp;
                    }

                }
            }
            image.put_pixel(x, y, color.to_rgba());
        }
    }
    image
}

fn main() {
let scene = vec![Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere{
            center: Point{
                x: 0.0,
                y: 0.0,
                z: -3.0,
            },
            radius: 1.0,
            color: Color{
                r: 0.2,
                g: 1.0,
                b: 0.2,
            },
        },
    },
    Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere{
            center: Point{
                x: 2.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color{
                r: 1.0,
                g: 0.0,
                b: 0.4,
            },
        },
    },
    Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere{
            center: Point{
                x: 4.0,
                y: 0.0,
                z: -7.0,
            },
            radius: 1.0,
            color: Color{
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        },
    }];
  /*
let scene = vec![Scene {
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
                r: 0.2,
                g: 1.0,
                b: 0.2,
            },
        },
    }];
    */
    let img:DynamicImage = render(&scene);
    img.save("test.png").unwrap();
}
