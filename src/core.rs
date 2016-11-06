use room;
use efl;
use libc::{c_void, c_int, c_char, c_float};
use std::ffi::CStr;
use std::borrow::Cow;

pub struct App
{
    core : Box<Core>,
    login : efl::LoginWidget
}

impl App {
    pub fn new() -> App {
        let core = Box::new(Core::new());
        let login = efl::LoginWidget::new(
            request_login_from_ui as *const c_void,
            &*core as *const _ as *const c_void);

        App {
            core : core,
            login : login
        }
    }
}


struct Core
{
    //access_token : String,
    //rooms : room::Rooms,
}

impl Core
{
    fn new() -> Core
    {
        Core {}
    }

    fn request_login_from_ui(&self, user : &str, pass : &str)
    {
        println!("core : there was a request to login {}, {}", user, pass);

    }
}

extern fn request_login_from_ui(
    data : *const c_void,
    user : *const c_char,
    pass : *const c_char)
{
    //TODO
    //let core : *const Core = unsafe {mem::transmute(data) };
    let core : *const Core = data as *const Core; 
    let core = unsafe { &*core };
    core.request_login_from_ui("logtest", &*get_str(pass));//user, pass);
    
}

fn get_string(str : *const c_char) -> String {
    unsafe {
    //    CStr::from_ptr(str).to_string_lossy().into_owned()
    }
    get_str(str).into_owned()
}

fn get_str<'a>(str : *const c_char) -> Cow<'a,str> {
    unsafe {
        CStr::from_ptr(str).to_string_lossy()
    }
}


