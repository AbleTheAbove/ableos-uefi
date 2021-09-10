// Triple buffer everything
pub enum GModes {}

pub type GCoord = usize;
pub type RGB = (u32, u32, u32);

pub const REFRESH_RATE: u8 = 60;

pub type Resolution = (usize, usize);
pub const DEFAULT_RESOLUTION: Resolution = (1440, 900);

pub type Point = (GCoord, GCoord);

pub struct GOPHandle {}

impl GOPHandle {
    pub fn put_pixel(coords: Point, color: RGB) {}
    pub fn put_line(coords_start: Point, coords_end: Point, thickness: f32, color: RGB) {}
    pub fn put_rect(coords_start: Point, coords_end: Point, color: RGB) {}
    pub fn put_circle(coords: Point, radius: f32) {}
    pub fn paint_cursor(coords: Point) {}
}
