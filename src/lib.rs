pub mod event;
mod ffi;
mod termios;

use event::{DEF_TIMEOUT, EVENT_TYPES, Event, Events, KeyEvent, MAX_EVENTS, TERMINAL_EVENT_ID};

use std::io::{Error, Result};
use std::os::fd::RawFd;

pub struct TtyListener {
    tty_fd: RawFd,
    epoll_fd: RawFd,
    events: Events,
    gti: termios::Gti,
}

impl TtyListener {
    pub fn new() -> Result<Self> {
        let mut gti = termios::Gti::new()?;
        gti.turn_terminal_raw_mode_on()?;
        let tty_fd = termios::get_raw_fd()?;
        let epoll_fd = create_epoll()?;
        bind_terminal_with_epoll(epoll_fd, tty_fd)?;
        Ok(TtyListener {
            tty_fd: tty_fd,
            epoll_fd: epoll_fd,
            events: Events::with_capacity(MAX_EVENTS),
            gti: gti,
        })
    }

    pub fn clean(&mut self) -> Result<()> {
        self.gti.turn_terminal_canon_mode_on()?;
        Ok(())
    }

    pub fn wait_for_next_event(&mut self) -> Result<KeyEvent> {
        let events_len = unsafe {
            let res = ffi::epoll_wait(
                self.epoll_fd,
                self.events.as_mut_ptr(),
                self.events.capacity() as i32,
                DEF_TIMEOUT,
            );
            if res == -1 {
                return Err(Error::last_os_error());
            }
            res
        };
        unsafe {
            self.events.set_len(events_len as usize);
        }

        if events_len > 0 {
            return Ok(self.read_event()?);
        }

        Ok(KeyEvent::Undefined)
    }

    fn read_event(&self) -> Result<KeyEvent> {
        let mut tty_buffer: [u8; 1024] = [0u8; termios::TTY_BUFFER_SIZE];

        let read_count = unsafe {
            ffi::read(
                self.tty_fd,
                tty_buffer.as_mut_ptr() as *mut std::os::raw::c_void,
                termios::TTY_BUFFER_SIZE,
            )
        };
        if read_count < 0 {
            return Err(Error::last_os_error());
        }
        Ok(event::identify_event(&tty_buffer[0..(read_count as usize)]))
    }

    pub fn get_events(&self) -> &Events {
        &self.events
    }
}

/// Creates an instance of epoll - I/O event notification facility
fn create_epoll() -> Result<RawFd> {
    let ep_fd;
    unsafe {
        ep_fd = ffi::epoll_create1(ffi::EPOLL_CLOEXEC);
        if ep_fd == -1 {
            return Err(Error::last_os_error());
        }
    }
    Ok(ep_fd)
}

fn bind_terminal_with_epoll(epoll_fd: RawFd, tty_fd: RawFd) -> Result<i32> {
    let mut event: Event = Event {
        events: EVENT_TYPES,
        u64: TERMINAL_EVENT_ID,
    };
    let res;
    unsafe {
        res = ffi::epoll_ctl(epoll_fd, ffi::EPOLL_CTL_ADD, tty_fd, &mut event);
        if res == -1 {
            return Err(Error::last_os_error());
        }
    }
    Ok(res)
}
