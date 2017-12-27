pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub w: u32,
    pub h: u32,
    pub dx: f64,
    pub dy: f64,
}

impl Entity {
    pub fn move_entity(&mut self, dt: f64) {
        self.x += self.dx * dt;
        self.y += self.dy * dt;
    }

    pub fn clamp_vertical(&mut self, min: f64, max: f64) {
        let height = self.h as f64;

        if self.y < min { self.y = min }
        if self.y + height > max { self.y = max - height }
    }

    pub fn bounce_vertical(&mut self, min: f64, max: f64) {
        let height = self.h as f64;
        let bottom = self.y + height;

        if self.y < min && self.dy < 0.0 {
            self.y = min + (min - self.y);
            self.dy = -self.dy;
        }

        if bottom > max && self.dy > 0.0 {
            self.y = max + (max - bottom) - height;
            self.dy = -self.dy;
        }
    }

    pub fn collides_with(&self, other: &Entity) -> bool {
        return
            (other.x < self.x + (self.w as f64)) &&
            (other.x + (other.w as f64) > self.x) &&
            (other.y < self.y + (self.h as f64)) &&
            (other.y + (other.h as f64) > self.y)
    }
}
