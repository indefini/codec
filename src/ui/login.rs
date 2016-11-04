#[repr(C)]
struct Eo;

pub type LoginSuccessCb = extern fn(data : *const Eo, success : bool);
extern {
    fn login_success(ob : *const Eo, b : bool)
    fn login_widget_new(on_request_login_cb : *const c_void) -> *const Eo;
}

trait Login
{
    //From core to ui
    fn on_success(&self, success : bool);
}

struct LoginWidget
{
    eo : *const Eo
}

impl LoginWidget
{
    fn new() -> LoginWidget {
        LoginWidget :  {
            eo : unsafe { login_widget_new(request_login_from_ui) };
        }
    }
}

impl Login for LoginWidget {
    
    fn on_success(&self, success : bool)
    {
        unsafe { login_success(self.eo, success); }
    }

}

extern fn request_login_from_ui(
    data : *const c_void,
    user : *const c_char,
    pass : *const c_char);
{
    //TODO
    //let core : &Core = unsafe {mem::transmute(data) };
    //core.request_login_from_ui(user, pass);
}


