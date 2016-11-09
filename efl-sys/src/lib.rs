extern crate libc;

use libc::{c_void, c_int, c_char, c_float};//, c_ulong, c_long, c_uint, c_uchar, size_t};

//#[repr(C)]
enum Ui {}

//pub type LoginSuccessCb = extern fn(data : *const Ui, success : bool);

extern "C" {
    fn efl_init();
    fn efl_run();

    fn login_visible_set(b :bool);
    fn loading_visible_set(b :bool);
    fn chat_visible_set(b :bool);

    fn login_success(ob : *const Ui, b : bool);
    fn ui_new(on_request_login_cb : *const c_void, rust_data : *const c_void) -> *const Ui;
}

pub struct UiCon
{
    ui : *const Ui,
}

impl UiCon
{
    pub fn new(
        login_cb : *const c_void,
        core : *const c_void) -> UiCon
    {
        UiCon {
            ui : unsafe { ui_new(login_cb as *const _, core) }
        }
    }

    pub fn on_success(&self, success : bool)
    {
        unsafe { login_success(self.ui, success); }
    }

    pub fn set_login_visible(&self, visible : bool)
    {
        unsafe { login_visible_set(visible); }
    }

    pub fn set_loading_visible(&self, visible : bool)
    {
        unsafe { loading_visible_set(visible); }
    }

    pub fn set_chat_visible(&self, visible : bool)
    {
        unsafe { chat_visible_set(visible); }
    }

}

pub fn app_init() {
    unsafe { efl_init(); }
}

pub fn app_run() {
    unsafe { efl_run(); }
}




