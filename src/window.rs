use std::sync::mpsc::Receiver;
use device_query::Keycode;
use glm::*;
use imgui_glfw_rs::{glfw, ImguiGLFW};
use crate::input::Input;
use imgui_glfw_rs::glfw::*;
use imgui_glfw_rs::glfw::OpenGlProfileHint::Core;
use crate::open_gl::viewport;

pub struct Win {
    edit: bool,
    pub p_glfw: Glfw,
    pub p_window: Window,
    pub events: Receiver<(f64, WindowEvent)>
}

impl Win {
    pub fn new() -> Self {
        let mut p_glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("[ERROR]: Failed to initialize Window!");

        p_glfw.window_hint(WindowHint::Floating(true));
        p_glfw.window_hint(WindowHint::Decorated(false));
        p_glfw.window_hint(WindowHint::TransparentFramebuffer(true));
        p_glfw.window_hint(WindowHint::Visible(false));

        p_glfw.window_hint(WindowHint::ContextVersion(4, 6));
        p_glfw.window_hint(WindowHint::OpenGlProfile(Core));
        p_glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        let (mut p_window, events) = p_glfw.with_primary_monitor(|glfw, monitor| {
            let area = monitor.as_ref().unwrap().get_workarea();
            glfw.create_window(area.2 as u32, area.3 as u32, "Overdraw", WindowMode::Windowed)
        }).expect("[ERROR]: Failed to create Window!");

        p_window.make_current();
        p_window.set_all_polling(true);
        p_window.set_focus_on_show(true);

        // Rendering
        gl::load_with(|proc_addr| p_glfw.get_proc_address_raw(proc_addr));
        gl::Viewport::load_with(|proc_addr| { p_glfw.get_proc_address_raw(proc_addr) });

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Win {
            edit: false,
            p_glfw,
            p_window,
            events
        }
    }

    pub fn edit(&mut self, input: &Input) -> bool {
        if input.get_key(Keycode::LControl) && input.get_key(Keycode::LAlt){
            if input.get_key_down(Keycode::A) {
                if self.edit == false {
                    self.show();
                    self.focus();
                    self.edit = true;
                } else {
                    self.hide();
                    self.edit = false;
                }
            }
        }
        self.edit
    }
    pub fn get_pos_x(&self) -> u32 {
        unsafe {
            self.p_window.get_pos().0 as u32
        }
    }
    pub fn get_pos_y(&self) -> u32 {
        unsafe {
            self.p_window.get_pos().1 as u32
        }
    }
    pub fn get_width(&self) -> u32 {
        unsafe {
            self.p_window.get_size().0 as u32
        }
    }
    pub fn get_height(&self) -> u32 {
        unsafe {
            self.p_window.get_size().1 as u32
        }
    }

    pub fn get_pos(&self) -> Vec2 {
        vec2(self.p_window.get_pos().0 as f32, self.p_window.get_pos().1 as f32)
    }
    pub fn get_size(&self) -> Vec2 {
        vec2(self.p_window.get_size().0 as f32, self.p_window.get_size().1 as f32)
    }

    pub fn open(&self) -> bool {
        !self.p_window.should_close()
    }

    pub fn poll_events(&mut self, input: &mut Input, mut imgui: &mut imgui_glfw_rs::imgui::Context, imgui_glfw: &mut ImguiGLFW) {
        self.p_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            imgui_glfw.handle_event(&mut imgui, &event);
            match event {
                WindowEvent::Pos(_, _) => {}
                WindowEvent::Size(_, _) => {}
                WindowEvent::Close => {}
                WindowEvent::Refresh => {}
                WindowEvent::Focus(_) => {}
                WindowEvent::Iconify(_) => {}
                WindowEvent::FramebufferSize(x, y) => viewport(0, 0, x as u32, y as u32),
                WindowEvent::MouseButton(_, _, _) => {}
                WindowEvent::CursorPos(_, _) => {}
                WindowEvent::CursorEnter(_) => {}
                WindowEvent::Scroll(_, _) => {}
                WindowEvent::Key(_, _, _, _) => {}
                WindowEvent::Char(_) => {}
                WindowEvent::CharModifiers(_, _) => {}
                WindowEvent::FileDrop(_) => {}
                WindowEvent::Maximize(_) => {}
                WindowEvent::ContentScale(_, _) => {}
            }
        }

        input.set_cursor_pos(*Vec2::from_array( &[self.p_window.get_cursor_pos().0 as f32, self.p_window.get_cursor_pos().1 as f32]));
    }

    pub fn show(&mut self) {
        self.p_window.show();
    }

    pub fn hide(&mut self) {
        self.p_window.hide();
    }

    pub fn focus(&mut self) {
        self.p_window.focus();
    }

    pub fn swap_buffers(&mut self) {
        self.p_window.swap_buffers();
    }
}