use glm::Vec4;
use imgui_glfw_rs::glfw::Window;
use imgui_glfw_rs::imgui::{Context, EditableColor, im_str, ImGuiWindowFlags, StyleColor, Ui};
use imgui_glfw_rs::ImguiGLFW;
use crate::history::History;

pub struct Gui{
    pub imgui: Context,
    pub imgui_glfw: ImguiGLFW
}

impl Gui {
    pub fn new(p_window: &mut Window) -> Self {
        let mut imgui = Context::create();
        let mut imgui_glfw = ImguiGLFW::new(&mut imgui, p_window);

        let style = imgui.style_mut();
        if let Some(color) = style.colors.get_mut(StyleColor::WindowBg as usize) {
            color[3] = 1.0;
        }

        Gui {
            imgui,
            imgui_glfw,
        }
    }

    fn begin(&mut self, p_window: &mut Window) -> Ui{
        self.imgui_glfw.frame(p_window, &mut self.imgui)
    }

    fn end(&mut self, frame: Ui, p_window: &mut Window) {
        self.imgui_glfw.draw(frame, p_window);
    }

    pub fn show_gui(&mut self, p_window: &mut Window, color: &mut Vec4, first_click: &mut bool, mut brush_size: &mut i32, history: &mut History) -> bool {
        let frame = self.imgui_glfw.frame(p_window, &mut self.imgui);

        let color_edit = EditableColor::Float4(color.as_array_mut());
        let mut is_hovered = false;
        frame.window(im_str!("Color Picker"))
            .flags(ImGuiWindowFlags::NoCollapse)
            .position([0.0, 0.0], imgui_glfw_rs::imgui::Condition::FirstUseEver)
            .size([300.0, 110.0], imgui_glfw_rs::imgui::Condition::FirstUseEver)
            .build(|| {
                frame.color_picker(im_str!("X"), color_edit).build();

                if frame.is_window_hovered() || frame.is_window_focused() {
                    is_hovered = true;
                    *first_click = false;
                }

                frame.drag_int(im_str!("Brush Size"), &mut brush_size).build();
                frame.slider_int(im_str!("History Size"), &mut history.max_undos, 1, 25).build();
            });


        self.imgui_glfw.draw(frame, p_window);
        is_hovered
    }
}