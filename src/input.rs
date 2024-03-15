use winit::event::{
    ElementState, VirtualKeyCode,
    MouseButton
};
use nalgebra_glm as glm;

#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub code: VirtualKeyCode,
    pub state: ElementState,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseBtn {
    pub button: MouseButton,
    pub state: ElementState,
}

#[derive(Debug, Clone)]
pub struct Input {
    key_states: Vec<Key>,    
    mouse_states: Vec<MouseBtn>,
    mouse_position: glm::Vec2,
}
impl Input {
    pub fn new() -> Self {
        Self { 
            key_states: Vec::new(),
            mouse_states: Vec::new(),
            mouse_position: glm::vec2(0.0, 0.0),
        }
    }

    pub fn set_mouse_state(&mut self, button: MouseButton, state: ElementState) {
        let value = self.mouse_states
            .iter()
            .enumerate()
            .find(|(_, m)| m.button == button);
        
        match value {
            Some((i, _)) => self.mouse_states[i].state = state, 
            None => self.mouse_states.push(MouseBtn { button, state }),
        }
    }
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        if let Some(value) = self.mouse_states
            .iter()
            .find(|m| m.button == button)
        {
            return value.state == ElementState::Pressed 
        }

        false
    }
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
    }
    pub fn get_mouse_position(&self) -> &glm::Vec2 {
        &self.mouse_position
    }

    pub fn set_key_state(&mut self, key_code: VirtualKeyCode, state: ElementState) {
        let value = self.key_states
            .iter()
            .enumerate()
            .find(|(_, k)| {
                k.code == key_code
            });

        match value {
            Some((i, _)) => self.key_states[i].state = state,
            None => self.key_states.push(Key { code: key_code, state }),
        }
    }
    pub fn in_key_pressed(&self, key: VirtualKeyCode) -> bool {
        if let Some(value) = self.key_states
            .iter()
            .find(|k| {
                k.code == key
            }) 
        {
            return value.state == ElementState::Pressed;
        }
        
        false
    }
}