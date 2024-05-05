use crate::open_gl::texture::*;
use std::collections::*;

pub struct History {
    pub stack: VecDeque<Texture2D>,
    pub max_undos: i32,
    undos: usize
}

impl History {
    pub fn new() -> Self {
        History{ stack: VecDeque::new(), max_undos: 10, undos: 0 }
    }

    pub fn push_back(&mut self, texture: Texture2D) {
        self.stack.push_back(texture);

        if self.stack.len() > self.max_undos as usize + 1 {
            self.stack.pop_front();
        }
    }

    pub fn pop_back(&mut self) {
        self.stack.pop_back();
    }

    pub fn pop_front(&mut self) {
        self.stack.pop_front();
    }

    pub fn get_current(&self) -> &Texture2D {
        let index = glm::max(self.stack.len() as i32 - 1 - self.undos as i32, 0) as usize;
        &self.stack[index]
    }

    pub fn undo(&mut self) {
        self.undos = glm::min(self.undos as i32 + 1, self.stack.len() as i32 - 1) as usize;
    }

    pub fn redo(&mut self) {
        self.undos = glm::max(self.undos as i32 - 1, 0) as usize;
    }

    pub fn pop_undos(&mut self) {
        for _ in 0..self.undos {
            self.stack.pop_back();
        }
        self.undos = 0;
    }
}