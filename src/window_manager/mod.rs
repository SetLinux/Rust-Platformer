extern crate glfw;
use glfw::Context;
pub struct Window {
    pub glwin: glfw::Window,
    pub events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}
impl Window {
    pub fn new(
        win: glfw::Window,
        evt: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    ) -> Window {
        Window {
            glwin: win,
            events: evt,
        }
    }
}
pub fn create_window(width: u32, height: u32, name: String) -> (glfw::Glfw, Window) {
    let mut glfwer = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfwer.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfwer.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

      glfwer.window_hint(glfw::WindowHint::Samples(Some(2)));

    let (mut window, events) = glfwer
        .create_window(width, height, &name, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    //Yes i shadowed the old windows with this one :)
    window.set_key_polling(true);
    let window = Window::new(window, events);

    (glfwer, window)
}
pub fn make_context_current(window: &mut Window) {
    window.glwin.make_current();
    gl::load_with(|s| window.glwin.get_proc_address(s) as *const _);
}
pub fn get_mouse_position(window : &mut Window,mv : &na::Matrix4<f32>) -> (f32,f32) {
    unsafe {
        let (xpos,ypos) = window.glwin.get_cursor_pos();
        (xpos as f32,1000.0 - ypos as f32)
    }

}