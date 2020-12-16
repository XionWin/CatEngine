
pub fn hsl2rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let p2 = if l <= 0.5 {
        l * (1f32 + s)
    } else {
        l + s - l * s
    };

    let p1 = 2f32 * l - p2;

    let (double_r, double_g, double_b) = if s == 0f32 {
        (l, l, l)
    } else {
        (
            qqh2rgb(p1, p2, h + 120f32),
            qqh2rgb(p1, p2, h),
            qqh2rgb(p1, p2, h - 120f32),
        )
    };

    (
        ((double_r * 255f32) % 255f32) as u8,
        ((double_g * 255f32) % 255f32) as u8,
        ((double_b * 255f32) % 255f32) as u8,
    )
}

fn qqh2rgb(q1: f32, q2: f32, hue: f32) -> f32
{
    let mut hue = hue;
    hue = loop {
        match hue {
            h if h > 360f32  =>{hue -= 360f32},
            h if h < 0f32  =>{hue += 360f32},
            _ => break hue
        }
    };

    match hue {
        h if h< 60f32 => {q1 + (q2 - q1) * hue / 60f32},
        h if h< 180f32 => {q2},
        h if h< 240f32 => {q1 + (q2 - q1) * (240f32 - hue) / 60f32},
        _ => 0f32,
    }
}