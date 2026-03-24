use std::os::raw::{c_int, c_uchar, c_uint, c_void};

pub const NCCS: usize = 32;

pub const EPOLL_CLOEXEC: c_int = 0x80000;

pub const EPOLLIN: c_int = 0x1;
pub const EPOLLPRI: c_int = 0x2;
pub const EPOLLOUT: c_int = 0x4;
pub const EPOLLERR: c_int = 0x8;
pub const EPOLLHUP: c_int = 0x10;
pub const EPOLLRDHUP: c_int = 0x2000;

pub const EPOLL_CTL_ADD: c_int = 1;
pub const EPOLL_CTL_MOD: c_int = 3;
pub const EPOLL_CTL_DEL: c_int = 2;

pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;

#[repr(C)]
#[derive(Clone)]
pub struct Termios {
    pub c_iflag: c_uint,
    pub c_oflag: c_uint,
    pub c_cflag: c_uint,
    pub c_lflag: c_uint,
    pub c_cc: [c_uchar; NCCS],
}

#[repr(C)]
pub struct EpollEvent {
    pub events: u32,
    pub u64: u64,
}

unsafe extern "C" {
    pub fn epoll_create1(flags: c_int) -> c_int;
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut EpollEvent) -> c_int;
    pub fn epoll_wait(
        epfd: c_int,
        events: *mut EpollEvent,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;

    pub fn tcgetattr(fd: c_int, termios: *mut Termios) -> c_int;
    pub fn cfmakeraw(termios: *mut Termios);
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const Termios) -> c_int;

    pub fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;

}
