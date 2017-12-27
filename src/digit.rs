use canvas::Canvas;

pub fn draw_digit(canvas: &mut Canvas, digit: u8, x: f64, y: f64, color: u32) {
    match digit {
        0 => {
            canvas.fill_rect(x, y, 48.0, 16.0, color);
            canvas.fill_rect(x, y + 16.0, 16.0, 48.0, color);
            canvas.fill_rect(x + 32.0, y + 16.0, 16.0, 48.0, color);
            canvas.fill_rect(x, y + 64.0, 48.0, 16.0, color);
        },
        1 => {
            canvas.fill_rect(x + 16.0, y, 16.0, 80.0, color);
        },
        2 => {
            canvas.fill_rect(x, y, 48.0, 16.0, color);
            canvas.fill_rect(x + 32.0, y + 16.0, 16.0, 16.0, color);
            canvas.fill_rect(x, y + 32.0, 48.0, 16.0, color);
            canvas.fill_rect(x, y + 48.0, 16.0, 16.0, color);
            canvas.fill_rect(x, y + 64.0, 48.0, 16.0, color);
        },
        3 => {
            canvas.fill_rect(x, y, 48.0, 16.0, color);
            canvas.fill_rect(x + 32.0, y + 16.0, 16.0, 16.0, color);
            canvas.fill_rect(x, y + 32.0, 48.0, 16.0, color);
            canvas.fill_rect(x + 32.0, y + 48.0, 16.0, 16.0, color);
            canvas.fill_rect(x, y + 64.0, 48.0, 16.0, color);
        },
        4 => {
            canvas.fill_rect(x, y, 16.0, 48.0, color);
            canvas.fill_rect(x + 32.0, y, 16.0, 80.0, color);
            canvas.fill_rect(x, y + 32.0, 48.0, 16.0, color);
        },
        5 => {
            canvas.fill_rect(x, y, 48.0, 16.0, color);
            canvas.fill_rect(x, y + 16.0, 16.0, 16.0, color);
            canvas.fill_rect(x, y + 32.0, 48.0, 16.0, color);
            canvas.fill_rect(x + 32.0, y + 48.0, 16.0, 16.0, color);
            canvas.fill_rect(x, y + 64.0, 48.0, 16.0, color);
        },
        6 => {
            canvas.fill_rect(x, y, 16.0, 80.0, color);
            canvas.fill_rect(x + 16.0, y, 32.0, 16.0, color);
            canvas.fill_rect(x + 16.0, y + 32.0, 32.0, 16.0, color);
            canvas.fill_rect(x + 32.0, y + 48.0, 16.0, 16.0, color);
            canvas.fill_rect(x + 16.0, y + 64.0, 32.0, 16.0, color);
        },
        7 => {
            canvas.fill_rect(x, y, 48.0, 16.0, color);
            canvas.fill_rect(x + 32.0, y + 16.0, 16.0, 64.0, color);
        },
        8 => {
            canvas.fill_rect(x, y, 16.0, 80.0, color);
            canvas.fill_rect(x + 32.0, y, 16.0, 80.0, color);
            canvas.fill_rect(x + 16.0, y, 16.0, 16.0, color);
            canvas.fill_rect(x + 16.0, y + 32.0, 16.0, 16.0, color);
            canvas.fill_rect(x + 16.0, y + 64.0, 16.0, 16.0, color);
        },
        9 => {
            canvas.fill_rect(x, y, 16.0, 48.0, color);
            canvas.fill_rect(x + 32.0, y, 16.0, 80.0, color);
            canvas.fill_rect(x + 16.0, y, 16.0, 16.0, color);
            canvas.fill_rect(x + 16.0, y + 32.0, 16.0, 16.0, color);
        },
        _ => {}
    }
}
