extern crate image;

use image::Luma;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

pub fn paint() {
    let img = image::open("self.png").unwrap();
    let mut gray = img.to_luma();
    let white = Luma([255]);
    draw_hollow_rect_mut(&mut gray, Rect::at(60, 10).of_size(20, 20), white);
    gray.save("gray.png").unwrap();
}