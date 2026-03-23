use std::os::raw::{c_int, c_uint, c_ulong};

pub type Event = crate::ffi::EpollEvent;
pub type Events = Vec<Event>;

pub const TERMINAL_EVENT_ID: c_ulong = 1;
pub const EVENT_TYPES: c_uint = (crate::ffi::EPOLLIN | crate::ffi::EPOLLRDHUP) as u32;

pub const MAX_EVENTS: usize = 100;
pub const DEF_TIMEOUT: c_int = 3000;
