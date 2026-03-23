pub mod event;
pub mod ffi;
pub mod termios;

use event::{DEF_TIMEOUT, EVENT_TYPES, Event, Events, MAX_EVENTS, TERMINAL_EVENT_ID};

use std::io::{Error, Result};
use std::os::fd::RawFd;

// use std::io::Result as Result;

pub struct TtyListener {
    tty_fd: RawFd,
    epoll_fd: RawFd,
    events: Events,
}

impl TtyListener {
    pub fn new() -> Result<Self> {
        Ok(TtyListener {
            tty_fd: termios::get_raw_fd()?,
            epoll_fd: create_epoll()?,
            events: Events::with_capacity(MAX_EVENTS),
        })
    }

    pub fn bind_terminal_with_epoll(&self) -> Result<i32> {
        let mut event: Event = Event {
            events: EVENT_TYPES,
            u64: TERMINAL_EVENT_ID,
        };
        let res;
        unsafe {
            res = ffi::epoll_ctl(self.epoll_fd, ffi::EPOLL_CTL_ADD, self.tty_fd, &mut event);
            if res == -1 {
                return Err(Error::last_os_error());
            }
        }
        Ok(res)
    }

    pub fn wait_for_next_event(&mut self) -> Result<i32> {
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
        Ok(events_len)
    }

    pub fn read_events(&self) -> Result<()> {
        let mut tty_buffer = [0u8; termios::TTY_BUFFER_SIZE];

        let result = unsafe {
            ffi::read(
                self.tty_fd,
                tty_buffer.as_mut_ptr() as *mut std::os::raw::c_void,
                termios::TTY_BUFFER_SIZE,
            )
        };

        if result < 0 {
            return Err(Error::last_os_error());
        }
        println!("let_size {:?}\r", result);
        println!("tty_buffer {:?}\r", tty_buffer.get(0));
        println!("tty_buffer {:?}\r", tty_buffer.get(1));
        println!("tty_buffer {:?}\r", tty_buffer.get(2));

        Ok(())
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
