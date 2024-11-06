#[allow(dead_code)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
pub struct Yuv {
    y: f32,
    u: f32,
    v: f32,
}

#[allow(dead_code)]
trait Color {
    fn to_yuv(&self) -> Yuv;
}

impl Color for Rgb {
    fn to_yuv(&self) -> Yuv {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let y = 0.299 * r + 0.587 * g + 0.114 * b;
        let u = -0.14713 * r - 0.288862 * g + 0.436 * b;
        let v = 0.615 * r - 0.51498 * g - 0.10001 * b;

        Yuv { y, u, v }
    }
}
