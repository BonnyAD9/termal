use std::{
    fs, io, mem,
    os::fd::{AsRawFd, IntoRawFd, RawFd},
    sync::Mutex,
};

use libc::{cfmakeraw, ioctl, tcgetattr, tcsetattr, termios as Termios, winsize, TCSANOW, TIOCGWINSZ};

use crate::{error::Result, raw::TermSize};

static ORIGINAL_TERMINAL_MODE: Mutex<Option<Termios>> = Mutex::new(None);

struct TtyFd {
    fd: RawFd,
    close: bool,
}

impl TtyFd {
    fn get() -> Result<Self> {
        let (fd, close) = if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
            (libc::STDIN_FILENO, false)
        } else {
            (
                fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/tty")?
                    .into_raw_fd(),
                true,
            )
        };

        Ok(Self { fd, close })
    }
}

impl AsRawFd for TtyFd {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for TtyFd {
    fn drop(&mut self) {
        if self.close {
            _ = unsafe { libc::close(self.fd) };
        }
    }
}

impl From<winsize> for TermSize {
    fn from(value: winsize) -> Self {
        Self {
            char_width: value.ws_col as usize,
            char_height: value.ws_row as usize,
            pixel_width: value.ws_xpixel as usize,
            pixel_height: value.ws_ypixel as usize,
        }
    }
}

pub fn is_raw_mode_enabled() -> bool {
    ORIGINAL_TERMINAL_MODE.lock().unwrap().is_some()
}

pub(crate) fn enable_raw_mode() -> Result<()> {
    let mut orig_mode = ORIGINAL_TERMINAL_MODE.lock().unwrap();

    if orig_mode.is_some() {
        return Ok(());
    }

    let tty = TtyFd::get()?;
    let fd = tty.as_raw_fd();
    let mut ios = get_terminal_attr(fd)?;
    let orig_mode_ios = ios;

    raw_terminal_attr(&mut ios);
    set_terminal_attr(fd, &ios)?;

    *orig_mode = Some(orig_mode_ios);

    Ok(())
}

pub(crate) fn disable_raw_mode() -> Result<()> {
    let mut orig_mode = ORIGINAL_TERMINAL_MODE.lock().unwrap();

    if let Some(orig_mode_ios) = orig_mode.as_ref() {
        let tty = TtyFd::get()?;
        set_terminal_attr(tty.as_raw_fd(), orig_mode_ios)?;
        *orig_mode = None;
    }

    Ok(())
}

pub(crate) fn window_size() -> Result<TermSize> {
    let tty = TtyFd::get()?;
    let mut size = winsize {
        ws_col: 0,
        ws_row: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    Ok(to_io_result(unsafe { ioctl(tty.fd, TIOCGWINSZ.into(), &mut size) }).map(|_| size.into())?)
}

fn get_terminal_attr(fd: RawFd) -> Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        to_io_result(tcgetattr(fd, &mut termios))?;
        Ok(termios)
    }
}

fn raw_terminal_attr(termios: &mut Termios) {
    unsafe { cfmakeraw(termios) }
}

fn set_terminal_attr(fd: RawFd, termios: &Termios) -> Result<()> {
    to_io_result(unsafe { tcsetattr(fd, TCSANOW, termios) })?;
    Ok(())
}

fn to_io_result(code: i32) -> io::Result<()> {
    if code == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
