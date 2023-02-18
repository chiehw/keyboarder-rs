use std::rc::Rc;

pub struct WinConnection {}

impl WinConnection {
    pub fn create_new() -> anyhow::Result<Rc<WinConnection>> {
        anyhow::Ok(Rc::new(Self {}))
    }
}
