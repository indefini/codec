extern crate libc;

use libc::{c_void, c_int, c_char, c_float};//, c_ulong, c_long, c_uint, c_uchar, size_t};
use std::ffi::CString;

//#[repr(C)]
enum Ui {}
enum Ecore_Animator {}

//pub type LoginSuccessCb = extern fn(data : *const Ui, success : bool);
pub type SimpleDataCb = extern fn(data : *const c_void);
pub type AnimatorCallback = extern fn(data : *const c_void) -> bool;

extern "C" {
    fn efl_init();
    fn efl_run();
    fn ecore_animator_add(cb : AnimatorCallback, data : *const c_void);

    fn ecore_thread_main_loop_begin();
    fn ecore_thread_main_loop_end();
    fn ecore_main_loop_thread_safe_call_async(cb : SimpleDataCb, data : *const c_void);

    fn login_visible_set(b :bool);
    fn loading_visible_set(b :bool);
    fn chat_visible_set(b :bool);

    fn loading_text_set(t : *const c_char);

    fn login_success(b : bool);
    fn ui_new(on_request_login_cb : *const c_void, rust_data : *const c_void) -> *const Ui;
}


pub struct UiCon
{
    //ui : *const Ui,
}

impl UiCon
{
    pub fn new(
        login_cb : *const c_void,
        core : *const c_void) -> UiCon
    {
            unsafe { ui_new(login_cb as *const _, core) };
        UiCon {
        //    ui : unsafe { ui_new(login_cb as *const _, core) }
        }
    }

    pub fn on_success(&self, success : bool)
    {
        unsafe { login_success(success); }
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

    pub fn set_loading_text(&self, text : &str)
    {
        unsafe { loading_text_set(CString::new(text).unwrap().as_ptr()); }
    }

}

pub fn app_init() {
    unsafe { efl_init(); }
}

pub fn app_run() {
    unsafe { efl_run(); }
}

use std::any::Any;

pub fn add_anim_fn<F>(f : F) where F : Fn() -> bool
{
    let fp = &f as *const _ as *mut c_void;

    unsafe {
        ecore_animator_add(do_thing_wrapper::<F>, fp);
    }


    extern fn do_thing_wrapper<F>(f : *const c_void) -> bool
    //extern fn do_thing_wrapper<F>(f : AnimatorCallback, data : *const c_void)
      where F: Fn() -> bool {
    //let opt_closure = closure as *mut Option<F>;
    let opt_closure = unsafe { (f as *const F).as_ref() };
    unsafe {
      //(*opt_closure).take().unwrap()(data as Box<Any>);
        return opt_closure.unwrap()();
    }
  }
}

pub fn add_async<F>(f : F) where F : Fn()
{
    let fp = &f as *const _ as *mut c_void;
    unsafe { ecore_main_loop_thread_safe_call_async(
            wrapper::<F>, fp);
    }

    extern fn wrapper<F>(f : *const c_void)
      where F: Fn() {
    let opt_closure = unsafe { (f as *const F).as_ref() };
    unsafe {
        return opt_closure.unwrap()();
    }
  }
}


pub fn main_loop_begin() {
    unsafe { ecore_thread_main_loop_begin(); }
}

pub fn main_loop_end() {
    unsafe { ecore_thread_main_loop_end(); }
}

pub fn set_login_visible(visible : bool)
{
    unsafe { login_visible_set(visible); }
}

pub fn set_loading_visible(visible : bool)
{
    unsafe { loading_visible_set(visible); }
}

pub fn set_chat_visible(visible : bool)
{
    unsafe { chat_visible_set(visible); }
}

