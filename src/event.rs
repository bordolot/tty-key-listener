use std::os::raw::{c_int, c_uint, c_ulong};

pub(crate) type Event = crate::ffi::EpollEvent;
pub(crate) type Events = Vec<Event>;

pub(crate) const TERMINAL_EVENT_ID: c_ulong = 1;
pub(crate) const EVENT_TYPES: c_uint = (crate::ffi::EPOLLIN | crate::ffi::EPOLLRDHUP) as u32;

pub(crate) const MAX_EVENTS: usize = 100;
pub(crate) const DEF_TIMEOUT: c_int = c_int::max_value();

pub(crate) fn identify_event(buffer: &[u8]) -> KeyEvent {
    if buffer.len() == 3 {
        if buffer[0] == 27 && buffer[1] == 91 && buffer[2] == 65 {
            return KeyEvent::ArrowUp;
        }
        if buffer[0] == 27 && buffer[1] == 91 && buffer[2] == 66 {
            return KeyEvent::ArrowDown;
        }
        if buffer[0] == 27 && buffer[1] == 91 && buffer[2] == 67 {
            return KeyEvent::ArrowRight;
        }
        if buffer[0] == 27 && buffer[1] == 91 && buffer[2] == 68 {
            return KeyEvent::ArrowLeft;
        }
    }
    if buffer[0].is_ascii() {
        return KeyEvent::KeyChar(buffer[0] as char);
    }
    KeyEvent::Undefined
}

pub enum KeyEvent {
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
    KeyChar(char),
    Undefined,
}
