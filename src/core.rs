use room;

struct App
{
    core : Box<Core>,
    login : Login
}

impl App {
    fn new() -> App {
        let core : Box::new(Core::new());
        let login :

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

    fn request_login_from_ui(&self, user : &str, pass, &str)
    {

    }
}

