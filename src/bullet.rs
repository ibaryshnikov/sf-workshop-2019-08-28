use web_sys::CanvasRenderingContext2d;

use crate::enemy::Enemy;

#[derive(Debug, PartialEq)]
pub enum CollisionKind {
    NotYet,
    EndOfMap,
    Hit,
}

#[derive(Clone)]
pub struct Bullet {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    direction: i8,
    speed: f64,
}

impl Bullet {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            width: 4.0,
            height: 12.0,
            direction: -1,
            speed: 0.3,
        }
    }
    pub fn change_position(&mut self, delta_time: f64) {
        // bullets always go straight
        // and we only have ship bullets at the moment
        self.y += f64::from(self.direction) * self.speed * delta_time;
    }
    pub fn check_collision(&self, enemies: &mut [Enemy]) -> CollisionKind {
        // goes away from the field of view
        if self.y + self.height < 0.0 {
            return CollisionKind::EndOfMap;
        }
        for enemy in enemies {
            let intersect_x = (self.x - enemy.x).abs() < self.width / 2.0 + enemy.width / 2.0;
            let intersect_y = (self.y - enemy.y).abs() < self.height / 2.0 + enemy.height / 2.0;
            if intersect_x && intersect_y {
                enemy.is_alive = false;
                return CollisionKind::Hit;
            }
        }
        CollisionKind::NotYet
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let x = self.x - self.width / 2.0;
        let y = self.y - self.height / 2.0;
        ctx.fill_rect(x, y, self.width, self.height);
    }
}
