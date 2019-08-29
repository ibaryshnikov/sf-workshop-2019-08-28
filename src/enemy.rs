use web_sys::CanvasRenderingContext2d;

#[derive(Clone)]
pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub is_alive: bool,
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            width: 20.0,
            height: 20.0,
            is_alive: true,
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let x = self.x - self.width / 2.0;
        let y = self.y - self.height / 2.0;
        ctx.fill_rect(x, y, self.width, self.height);
    }
}

pub fn get_initial_enemies() -> Vec<Enemy> {
    #[rustfmt::skip]
    let positions = vec![
        (140, 60), (180, 60), (220, 60), (260, 60),
        (160, 140), (200, 140), (240, 140),
    ];
    let mut result = vec![];
    for (x, y) in positions {
        result.push(Enemy::new(f64::from(x), f64::from(y)));
    }
    result
}
