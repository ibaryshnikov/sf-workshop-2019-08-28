use std::cell::RefCell;
use std::ops::Drop;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

#[macro_use]
mod console; // must be imported first to allow us to use macros everywhere

mod bullet;
mod dom_helpers;
mod enemy;
mod scene;
mod ship;

use scene::Scene;

#[wasm_bindgen]
pub struct SceneContainer {
    scene: Rc<RefCell<Scene>>,
}

#[wasm_bindgen]
impl SceneContainer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SceneContainer, JsValue> {
        console_error_panic_hook::set_once();

        let scene = Rc::new(RefCell::new(Scene::new()?));

        scene.borrow_mut().add_listeners(scene.clone());

        Ok(SceneContainer { scene })
    }

    #[wasm_bindgen(js_name = updateState)]
    pub fn update_state(&mut self) {
        self.scene.borrow_mut().update_state();
    }

    pub fn draw(&self) {
        self.scene.borrow().draw();
        self.scene.borrow_mut().state_changed = false;
    }
}

impl Drop for SceneContainer {
    fn drop(&mut self) {
        console_log!(
            "removing listeners, reference count {}",
            Rc::strong_count(&self.scene)
        );
        self.scene.borrow_mut().remove_listeners();
        console_log!("done, reference count {}", Rc::strong_count(&self.scene));
    }
}
