use std::cell::RefCell;
use std::ops::Drop;
use std::rc::Rc;

use js_sys::Date;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, Window};

use crate::bullet::{Bullet, CollisionKind};
use crate::dom_helpers::*;
use crate::enemy::{get_initial_enemies, Enemy};
use crate::ship::Ship;

#[derive(Debug)]
enum KeyDirection {
    Up,
    Down,
}

#[derive(Default)]
struct Callbacks {
    onkeydown: Option<Closure<dyn FnMut(KeyboardEvent)>>,
    onkeyup: Option<Closure<dyn FnMut(KeyboardEvent)>>,
}

pub struct Scene {
    width: u16,
    height: u16,
    window: Window,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    callbacks: Callbacks,
    ship: Ship,
    last_bullet_fired: f64,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    last_updated: f64,
    pub state_changed: bool,
}

impl Scene {
    pub fn new() -> Result<Scene, JsValue> {
        let window = get_window()?;
        let document = get_document(&window)?;
        let body = get_body(&document)?;

        body.style().set_property("background", "silver")?;

        let width = 400;
        let height = 400;

        let canvas = create_canvas(&document)?;

        canvas.set_width(u32::from(width));
        canvas.set_height(u32::from(height));
        canvas.style().set_property("background", "black")?;
        canvas.style().set_property("display", "block")?;
        canvas.style().set_property("margin", "auto")?;

        body.append_child(&canvas)?;

        let ctx = get_context(&canvas)?;

        Ok(Scene {
            width,
            height,
            window,
            canvas,
            ctx,
            callbacks: Callbacks::default(),
            ship: Ship::new(200.0, f64::from(width)),
            last_bullet_fired: 0.0,
            bullets: vec![],
            enemies: get_initial_enemies(),
            last_updated: 0.0,
            state_changed: true,
        })
    }

    pub fn add_listeners(&mut self, self_ref: Rc<RefCell<Scene>>) {
        console_log!("Adding listeners");
        self.set_keydown(self_ref.clone());
        self.set_keyup(self_ref);
    }

    pub fn remove_listeners(&mut self) {
        self.window.set_onkeydown(None);
        self.window.set_onkeyup(None);
        self.callbacks.onkeydown = None;
        self.callbacks.onkeyup = None;
    }

    fn set_keydown(&mut self, self_ref: Rc<RefCell<Scene>>) {
        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            self_ref
                .borrow_mut()
                .process_key_code(&e.code(), KeyDirection::Down);
        }) as Box<dyn FnMut(KeyboardEvent)>);
        self.window
            .set_onkeydown(Some(closure.as_ref().unchecked_ref()));
        self.callbacks.onkeydown = Some(closure);
    }
    fn set_keyup(&mut self, self_ref: Rc<RefCell<Scene>>) {
        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            self_ref
                .borrow_mut()
                .process_key_code(&e.code(), KeyDirection::Up);
        }) as Box<dyn FnMut(KeyboardEvent)>);
        self.window
            .set_onkeyup(Some(closure.as_ref().unchecked_ref()));
        self.callbacks.onkeyup = Some(closure);
    }

    fn process_key_code(&mut self, code: &str, direction: KeyDirection) {
        use KeyDirection::*;

        match (code, direction) {
            ("ArrowLeft", Down) => self.ship.direction = -1,
            ("ArrowRight", Down) => self.ship.direction = 1,
            ("ArrowLeft", Up) | ("ArrowRight", Up) => self.ship.direction = 0,
            ("Space", Down) => self.fire_bullet(),
            _ => (),
        }
    }

    fn fire_bullet(&mut self) {
        let now = Date::now();
        let diff = now - self.last_bullet_fired;
        if diff.abs() < 300.0 {
            return;
        }
        self.last_bullet_fired = now;
        self.bullets.push(self.ship.fire_bullet());
    }

    pub fn update_state(&mut self) {
        let now = Date::now();
        let delta_time = now - self.last_updated;
        self.last_updated = now;
        self.move_ship(delta_time);
        self.move_bullets(delta_time);
        self.bullets = self
            .bullets
            .clone()
            .into_iter()
            .filter(|bullet| {
                use CollisionKind::*;
                match bullet.check_collision(&mut self.enemies) {
                    EndOfMap | Hit => false,
                    NotYet => true,
                }
            })
            .collect();
        self.enemies = self
            .enemies
            .clone()
            .into_iter()
            .filter(|enemy| enemy.is_alive)
            .collect();
    }

    fn move_bullets(&mut self, delta_time: f64) {
        if self.bullets.is_empty() {
            return;
        }
        for bullet in &mut self.bullets {
            bullet.change_position(delta_time);
        }
        self.state_changed = true;
    }

    fn move_ship(&mut self, delta_time: f64) {
        // 0 means not moving
        if self.ship.direction == 0 {
            return;
        }
        self.ship.change_position(delta_time);
        self.state_changed = true;
    }

    pub fn draw(&self) {
        if !self.state_changed {
            return;
        }
        // clear screen
        self.ctx
            .clear_rect(0.0, 0.0, f64::from(self.width), f64::from(self.height));

        // let's draw in white, with black background
        self.ctx.set_fill_style(&JsValue::from("white"));

        // draw ship
        self.ship.draw(&self.ctx);
        // draw bullets
        for bullet in &self.bullets {
            bullet.draw(&self.ctx);
        }
        for enemy in &self.enemies {
            enemy.draw(&self.ctx);
        }
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        self.canvas.remove();
    }
}
