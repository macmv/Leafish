use crate::inventory::{Inventory, InventoryContext, Item};
use crate::render::hud::Hud;
use crate::render::Renderer;
use crate::screen::Screen;
use crate::ui;
use crate::ui::{Container, ImageRef, TextRef};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct InventoryWindow {
    pub elements: Vec<Vec<ImageRef>>,
    pub text_elements: Vec<Vec<TextRef>>,
    pub inventory: Arc<RwLock<dyn Inventory + Sync + Send>>,
    inventory_context: Arc<RwLock<InventoryContext>>,
}

impl Screen for InventoryWindow {
    fn on_active(&mut self, renderer: &mut Renderer, ui_container: &mut Container) {
        self.inventory_context
            .clone()
            .write()
            .inventory
            .replace(self.inventory.clone());
        self.inventory
            .clone()
            .write()
            .init(renderer, ui_container, self);
    }

    fn on_deactive(&mut self, _renderer: &mut Renderer, _ui_container: &mut Container) {
        self.inventory_context.clone().write().inventory = None;
        self.inventory.clone().write().close(self);
        self.clear_elements();
    }

    fn tick(
        &mut self,
        _delta: f64,
        renderer: &mut Renderer,
        ui_container: &mut Container,
    ) -> Option<Box<dyn Screen>> {
        self.inventory
            .clone()
            .write()
            .tick(renderer, ui_container, self);
        None
    }

    fn on_resize(&mut self, renderer: &mut Renderer, ui_container: &mut Container) {
        self.inventory.clone().write().resize(
            renderer.safe_width,
            renderer.safe_height,
            renderer,
            ui_container,
            self,
        );
    }

    fn is_closable(&self) -> bool {
        true
    }

    fn clone_screen(&self) -> Box<dyn Screen> {
        Box::new(self.clone())
    }
}

impl InventoryWindow {
    pub fn new(
        inventory: Arc<RwLock<dyn Inventory + Sync + Send>>,
        inventory_context: Arc<RwLock<InventoryContext>>,
    ) -> Self {
        InventoryWindow {
            elements: vec![],
            text_elements: vec![],
            inventory,
            inventory_context,
        }
    }
}

impl InventoryWindow {
    pub fn draw_item(
        &mut self,
        item: &Item,
        x: f64,
        y: f64,
        elements_idx: usize,
        ui_container: &mut Container,
        renderer: &Renderer,
    ) {
        let icon_scale = Hud::icon_scale(renderer);
        let textures = item.material.item_texture_locations();
        let texture = if let Some(tex) = Renderer::get_texture_optional(&renderer.textures, &*format!("minecraft:{}", textures.0))
        {
            if tex.dummy {
                textures.1
            } else {
                textures.0
            }
        } else {
            textures.1
        };
        let image = ui::ImageBuilder::new()
            .texture_coords((0.0, 0.0, 1.0, 1.0))
            .position(x, y)
            .alignment(ui::VAttach::Middle, ui::HAttach::Center)
            .size(icon_scale * 16.0, icon_scale * 16.0)
            .texture(format!("minecraft:{}", texture))
            .create(ui_container);
        self.elements.get_mut(elements_idx).unwrap().push(image);
    }

    pub fn clear_elements(&mut self) {
        for element in &mut self.elements {
            element.clear();
        }
        self.elements.clear();
        for element in &mut self.text_elements {
            element.clear();
        }
        self.text_elements.clear();
    }
}
