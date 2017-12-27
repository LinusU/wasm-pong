extern {
    fn create_canvas(width: u32, height: u32) -> u32;
    fn set_fill_style(id: u32, style: u32);
    fn canvas_fill_rect(id: u32, x: f64, y: f64, w: f64, h: f64);
}

#[derive(Clone, Copy, Debug)]
pub struct Canvas {
    pub id: u32,
    pub width: u32,
    pub height: u32,
}

impl Canvas {
    pub fn init (width: u32, height: u32) -> Canvas {
        let id = unsafe { create_canvas(width, height) };

        Canvas { id, width, height }
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64, color: u32) {
        unsafe {
            set_fill_style(self.id, color);
            canvas_fill_rect(self.id, x, y, w, h);
        }
    }
}
