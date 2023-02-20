use super::keyboard::XKeyboard;
use anyhow::{anyhow, Context};
use std::rc::Rc;

pub struct XConnection {
    pub conn: xcb::Connection,
    pub screen_num: i32,
    pub root: xcb::x::Window,
    pub keyboard: XKeyboard,
}

impl XConnection {
    pub fn create_new() -> anyhow::Result<Rc<XConnection>> {
        let (conn, screen_num) =
            xcb::Connection::connect_with_xlib_display_and_extensions(&[xcb::Extension::Xkb], &[])?;
        let screen = conn
            .get_setup()
            .roots()
            .nth(screen_num as usize)
            .ok_or_else(|| anyhow!("no screen?"))?;
        let root = screen.root();

        let keyboard = XKeyboard::new(&conn)?;

        let conn = Rc::new(XConnection {
            conn,
            screen_num,
            root,
            keyboard,
        });

        anyhow::Ok(conn)
    }

    pub fn conn(&self) -> &xcb::Connection {
        &self.conn
    }
}

impl std::ops::Deref for XConnection {
    type Target = xcb::Connection;
    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl XConnection {
    pub(crate) fn send_request_no_reply<R>(&self, req: &R) -> anyhow::Result<()>
    where
        R: xcb::RequestWithoutReply + std::fmt::Debug,
    {
        self.conn
            .send_and_check_request(req)
            .with_context(|| format!("{req:#?}"))
    }

    pub(crate) fn send_request_no_reply_log<R>(&self, req: &R)
    where
        R: xcb::RequestWithoutReply + std::fmt::Debug,
    {
        if let Err(err) = self.send_request_no_reply(req) {
            log::error!("{err:#}");
        }
    }

    #[allow(dead_code)]
    pub(crate) fn send_and_wait_request<R>(
        &self,
        req: &R,
    ) -> anyhow::Result<<<R as xcb::Request>::Cookie as xcb::CookieWithReplyChecked>::Reply>
    where
        R: xcb::Request + std::fmt::Debug,
        R::Cookie: xcb::CookieWithReplyChecked,
    {
        let cookie = self.conn.send_request(req);
        self.conn
            .wait_for_reply(cookie)
            .with_context(|| format!("{req:#?}"))
    }

    pub fn run_message_loop(&self) -> anyhow::Result<()> {
        loop {
            let _event = match self.conn().wait_for_event() {
                Err(xcb::Error::Connection(err)) => {
                    panic!("unexpected I/O error: {}", err);
                }
                Err(xcb::Error::Protocol(xcb::ProtocolError::X(
                    xcb::x::Error::Font(_err),
                    _req_name,
                ))) => {
                    // may be this particular error is fine?
                    continue;
                }
                Err(xcb::Error::Protocol(err)) => {
                    panic!("unexpected protocol error: {:#?}", err);
                }
                Ok(event) => self.process_xcb_event(&event),
            };
        }
    }

    // key press/release are not processed here.
    // xkbcommon depends on those events in order to:
    //    - update modifiers state
    //    - update keymap/state on keyboard changes
    fn process_xcb_event(&self, event: &xcb::Event) -> anyhow::Result<()> {
        if matches!(event, xcb::Event::Xkb(_)) {
            self.keyboard.process_xkb_event(&self.conn, event)?;
        }
        Ok(())
    }
}
