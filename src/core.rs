use room;
use efl;
use libc::{c_void, c_int, c_char, c_float};

struct App
{
    core : Box<Core>,
    login : efl::LoginWidget
}

impl App {
    fn new() -> App {
        let core = Box::new(Core::new());
        let login = efl::LoginWidget::new(&*core as *const _ as *const c_void);

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

    }
}

