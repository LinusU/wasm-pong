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

    pub fn bounce_inside_box(&mut self) {
        let right = (640 - self.w) as f64;
        let bottom = (480 - self.h) as f64;

        if self.x < 0.0 && self.dx < 0.0 {
            self.x = 0.0;
            self.dx *= -1.0;
        }

        if self.x > right && self.dx > 0.0 {
            self.x = right;
            self.dx *= -1.0;
        }

        if self.y < 0.0 && self.dy < 0.0 {
            self.y = 0.0;
            self.dy *= -1.0;
        }

        if self.y > bottom && self.dy > 0.0 {
            self.y = bottom;
            self.dy *= -1.0;
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
