use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode, MouseButton, MouseState};
use glm::{vec2, Vec2};

pub struct Input {
    pub device_state: DeviceState,
    keys: Vec<Keycode>,
    prev_keys: Vec<Keycode>,

    mouse: MouseState,
    prev_mouse: MouseState,
    prev_rel_cursor_pos: Vec2,
    rel_cursor_pos: Vec2
}

impl Input {
    pub fn new() -> Input {
        let device_state = DeviceState::new();

        Input {
            device_state,
            keys: Vec::new(),
            prev_keys: Vec::new(),

            mouse: Default::default(),
            prev_mouse: Default::default(),
            prev_rel_cursor_pos: vec2(0.0, 0.0),
            rel_cursor_pos: vec2(0.0, 0.0)
        }
    }
    pub fn poll(&mut self) {
        self.prev_keys = self.keys.clone();
        self.keys = self.device_state.get_keys();

        self.prev_mouse = self.mouse.clone();
        self.mouse = self.device_state.get_mouse();
    }
    pub fn get_key(&self, key: Keycode) -> bool {
        self.keys.contains(&key)
    }
    pub fn get_key_down(&self, key: Keycode) -> bool {
        let key_state = self.keys.contains(&key);
        let prev_key_state = self.prev_keys.contains(&key);
        if key_state && !prev_key_state {
            return true;
        }
        return false;
    }
    pub fn get_key_up(&self, key: Keycode) -> bool {
        let key_state = self.keys.contains(&key);
        let prev_key_state = self.prev_keys.contains(&key);
        if !key_state && prev_key_state {
            return true;
        }
        return false;
    }

    pub fn get_button(&self, button: usize) -> bool{
        self.mouse.button_pressed[button]
    }
    pub fn get_button_down(&self, button: usize) -> bool{
        let button_state = if self.mouse.button_pressed.len() > button {
            self.mouse.button_pressed[button]
        } else {
            false
        };

        let prev_button_state = if self.prev_mouse.button_pressed.len() > button {
            self.prev_mouse.button_pressed[button]
        } else {
            false
        };

        if button_state && !prev_button_state {
            return true;
        }
        return false;
    }
    pub fn get_button_up(&self, button: usize) -> bool{
        let button_state = self.mouse.button_pressed[button];
        let prev_button_state = self.prev_mouse.button_pressed[button];

        if !button_state && prev_button_state {
            return true;
        }
        return false;
    }
}

impl Input {
    pub fn set_cursor_pos(&mut self, pos: Vec2) {
        self.prev_rel_cursor_pos = self.rel_cursor_pos;
        self.rel_cursor_pos = vec2(pos.x, pos.y);
    }
    pub fn get_cursor_pos(&self) -> Vec2 {
        self.rel_cursor_pos
    }

    pub fn get_prev_cursor_pos(&self) -> Vec2 {
        self.prev_rel_cursor_pos
    }
}