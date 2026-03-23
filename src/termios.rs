use crate::ffi;
use std::io;
use std::os::unix::io::{IntoRawFd, RawFd};

pub const TTY_BUFFER_SIZE: usize = 1_024;

/// General terminal interface
pub struct Gti {
    tty_fd: RawFd,
    raw: ffi::Termios,
    canon: ffi::Termios,
}

impl Gti {
    pub fn new() -> io::Result<Self> {
        let tty_fd = get_raw_fd()?;
        Ok(Gti {
            tty_fd: tty_fd,
            raw: get_gti(tty_fd)?,
            canon: get_gti(tty_fd)?,
        })
    }

    pub fn turn_terminal_raw_mode_on(&mut self) -> io::Result<()> {
        unsafe { ffi::cfmakeraw(&mut self.raw) }
        if unsafe { ffi::tcsetattr(self.tty_fd, ffi::TCSANOW, &self.raw) } == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub fn turn_terminal_canon_mode_on(&self) -> io::Result<()> {
        if unsafe { ffi::tcsetattr(self.tty_fd, ffi::TCSANOW, &self.canon) } == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

pub fn get_raw_fd() -> io::Result<i32> {
    Ok(std::fs::File::open("/dev/tty")?.into_raw_fd())
}

/// get a general terminal interface
pub fn get_gti(raw_dsc: RawFd) -> io::Result<ffi::Termios> {
    unsafe {
        let mut termios: ffi::Termios = std::mem::zeroed();
        if ffi::tcgetattr(raw_dsc, &mut termios) == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(termios)
    }
}
