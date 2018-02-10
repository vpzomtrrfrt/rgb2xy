fn gamma_correction(value: f32) -> f32 {
    if value > 0.04045 {
        ((value + 0.055) / 1.055).powf(2.4)
    }
    else {
        value / 12.92
    }
}

fn rgb2xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    (
        r * 0.664511 + g * 0.154324 + b * 0.162028,
        r * 0.283881 + g * 0.668433 + b * 0.047685,
        r * 0.000088 + g * 0.072310 + b * 0.986039
    )
}

fn xyz2xy(x: f32, y: f32, z: f32) -> (f32, f32) {
    (
        x / (x + y + z),
        y / (x + y + z)
    )
}

pub fn rgb2xy(r: f32, g: f32, b: f32) -> (f32, f32) {
    let xyz = rgb2xyz(
        gamma_correction(r),
        gamma_correction(g),
        gamma_correction(b)
    );
    xyz2xy(xyz.0, xyz.1, xyz.2)
}
