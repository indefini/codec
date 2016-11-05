extern crate libc;

use libc::{c_void, c_int, c_char, c_float};//, c_ulong, c_long, c_uint, c_uchar, size_t};

//#[repr(C)]
enum Eo {}

pub type LoginSuccessCb = extern fn(data : *const Eo, success : bool);

extern "C" {
    fn login_success(ob : *const Eo, b : bool);
    fn login_new(on_request_login_cb : *const c_void, rust_data : *const c_void) -> *const Eo;
}

pub struct LoginWidget
{
    eo : *const Eo
}

impl LoginWidget
{
    pub fn new(core : *const c_void) -> LoginWidget
    {
        LoginWidget {
            eo : unsafe { login_new(request_login_from_ui as *const _, core) }
        }
    }

    pub fn on_success(&self, success : bool)
    {
        unsafe { login_success(self.eo, success); }
    }

    pub fn set_visible(visible : bool)
    {
        panic!("TODO set visible");
    }

}

extern fn request_login_from_ui(
    data : *const c_void,
    user : *const c_char,
    pass : *const c_char)
{
    //TODO
    //let core : &Core = unsafe {mem::transmute(data) };
    //core.request_login_from_ui(user, pass);
}




