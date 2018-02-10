extern crate rgb2xy;
extern crate css_color_parser;

use css_color_parser::Color;

fn main() {
    match std::env::args().nth(1) {
        None => {
            eprintln!("Usage: {} <rgb>", std::env::args().next().unwrap());
        },
        Some(input) => {
            let color: Color = input.parse().unwrap();
            let xy = rgb2xy::rgb2xy(
                f32::from(color.r) / 255.0,
                f32::from(color.g) / 255.0,
                f32::from(color.b) / 255.0
            );
            println!("[{},{}]", xy.0, xy.1);
        }
    }
}
