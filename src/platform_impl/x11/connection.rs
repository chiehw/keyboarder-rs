use crate::{
    connection::ConnectionOps,
    platform_impl::Simulator,
    simulate::Simulate,
    types::{KeyEvent, KeyEventBin},
};

use super::keyboard::XKeyboard;
use anyhow::{anyhow, Context};
use filedescriptor::FileDescriptor;
use mio::{unix::SourceFd, Events, Interest, Poll, Token};
use std::{cell::RefCell, io::Read, os::unix::prelude::AsRawFd};

pub struct XConnection {
    pub conn: xcb::Connection,
    pub screen_num: i32,
    pub root: xcb::x::Window,
    pub keyboard: XKeyboard,
    pub simulator: RefCell<Option<Simulator>>,
}

impl XConnection {
    pub fn create_new() -> anyhow::Result<XConnection> {
        let (conn, screen_num) =
            xcb::Connection::connect_with_xlib_display_and_extensions(&[xcb::Extension::Xkb], &[])?;
        let screen = conn
            .get_setup()
            .roots()
            .nth(screen_num as usize)
            .ok_or_else(|| anyhow!("no screen?"))?;
        let root = screen.root();

        let keyboard = XKeyboard::new(&conn)?;

        let conn = XConnection {
            conn,
            screen_num,
            root,
            keyboard,
            simulator: RefCell::new(None),
        };

        anyhow::Ok(conn)
    }

    pub fn conn(&self) -> &xcb::Connection {
        &self.conn
    }
    pub fn run_message_loop(&self, read_fd: &mut FileDescriptor) -> anyhow::Result<()> {
        const TOK_SIMULATE: mio::Token = Token(0xffff_fffc);
        const TOK_XKB: mio::Token = Token(0xffff_fffb);
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(8);

        poll.registry().register(
            &mut SourceFd(&read_fd.as_raw_fd()),
            TOK_SIMULATE,
            Interest::READABLE,
        )?;
        poll.registry().register(
            &mut SourceFd(&self.conn.as_raw_fd()),
            TOK_XKB,
            Interest::READABLE,
        )?;

        loop {
            poll.poll(&mut events, None)
                .map_err(|err| anyhow::anyhow!("polling for events: {:?}", err))?;
            for event in &events {
                match event.token() {
                    TOK_SIMULATE => {
                        let mut buf = vec![0; 64];
                        #[allow(clippy::unused_io_amount)]
                        let num = read_fd.read(&mut buf)?;
                        anyhow::ensure!(num != buf.len(), "buf is too small");

                        let key_event = KeyEventBin::new(buf).to_key_event()?;
                        self.process_key_event_log(&key_event);
                    }
                    TOK_XKB => {
                        self.process_queued_xcb_log();
                    }
                    _ => {}
                }
            }
        }
    }

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

    fn process_key_event_log(&self, key_event: &KeyEvent) {
        if let Err(err) = self.process_key_event(key_event) {
            log::error!("{err:#}");
        }
    }

    fn process_queued_xcb_log(&self) {
        if let Err(err) = self.process_queued_xcb() {
            log::error!("{err:#}");
        }
    }

    fn process_key_event(&self, key_event: &KeyEvent) -> anyhow::Result<()> {
        if let Some(simulator) = self.simulator.borrow_mut().as_mut() {
            simulator.simulate_key_event(key_event);
        }

        Ok(())
    }

    fn process_queued_xcb(&self) -> anyhow::Result<()> {
        if let Some(event) = self
            .conn
            .poll_for_event()
            .context("X11 connection is broken")?
        {
            // key press/release are not processed here.
            // xkbcommon depends on those events in order to:
            //    - update modifiers state
            //    - update keymap/state on keyboard changes
            if matches!(event, xcb::Event::Xkb(_)) {
                self.keyboard.process_xkb_event(&self.conn, &event)?;
            }
        }
        Ok(())
    }
}

impl ConnectionOps for XConnection {}

impl std::ops::Deref for XConnection {
    type Target = xcb::Connection;
    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}
