pub struct Color{
    pub r: u32,
    pub g: u32,
    pub b :u32,
}

pub struct Sphere{
    pub center: Point
    pub rdius: f64,
    pub color: Color,
}

pub struct Scene{
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub Sphere:Sphere,
}
