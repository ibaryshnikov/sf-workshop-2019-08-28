use web_sys::CanvasRenderingContext2d;

use crate::bullet::Bullet;

pub struct Ship {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    pub direction: i8,
    speed: f64,
    scene_width: f64,
}

impl Ship {
    pub fn new(x: f64, scene_width: f64) -> Self {
        Self {
            x,
            y: 380.0,
            width: 40.0,
            height: 40.0,
            direction: 0,
            speed: 0.2,
            scene_width,
        }
    }

    pub fn fire_bullet(&mut self) -> Bullet {
        Bullet::new(self.x, self.y - self.height / 2.0)
    }

    // returns true if the position was changed
    pub fn change_position(&mut self, delta_time: f64) {
        let new_x = self.x + f64::from(self.direction) * self.speed * delta_time;
        let half_width = self.width / 2.0;
        self.x = if new_x < half_width {
            half_width
        } else if new_x > self.scene_width - half_width {
            self.scene_width - half_width
        } else {
            new_x
        };
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let x = self.x - self.width / 2.0;
        let y = self.y - self.height / 2.0;
        ctx.fill_rect(x, y, self.width, self.height);
    }
}
