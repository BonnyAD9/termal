use std::{
    fs,
    io::{self, ErrorKind},
    mem,
    os::fd::{AsRawFd, IntoRawFd, RawFd},
    ptr,
    sync::{Mutex, MutexGuard},
    time::{Duration, Instant},
};

use libc::{
    POLLERR, POLLIN, TCSANOW, TIOCGWINSZ, cfmakeraw, ioctl, poll, pollfd,
    ppoll, tcgetattr, tcsetattr, termios as Termios, time_t, timespec,
    winsize,
};

use crate::{Error, Result, raw::TermSize};

static ORIGINAL_TERMINAL_MODE: Mutex<Option<Termios>> = Mutex::new(None);

/// The maximum time you can wait for stdin. Larger values will return error.
///
/// This is unix implementation and thus the maximum is `time_t::MAX` seconds
/// which is `i32::MAX` on 32 bit platforms and `i64::MAX` on 64 bit platforms.
pub const MAX_STDIN_WAIT: Duration = Duration::from_secs(time_t::MAX as u64);

fn get_original_terminal_mode() -> MutexGuard<'static, Option<Termios>> {
    ORIGINAL_TERMINAL_MODE
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

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

/// Check if raw mode on linux is enabled.
pub fn is_raw_mode_enabled() -> bool {
    get_original_terminal_mode().is_some()
}

/// Enable raw mode on linux.
pub(crate) fn enable_raw_mode() -> Result<()> {
    let mut orig_mode = get_original_terminal_mode();

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

/// Disable raw mode on linux.
pub(crate) fn disable_raw_mode() -> Result<()> {
    let mut orig_mode = get_original_terminal_mode();

    if let Some(orig_mode_ios) = orig_mode.as_ref() {
        let tty = TtyFd::get()?;
        set_terminal_attr(tty.as_raw_fd(), orig_mode_ios)?;
        *orig_mode = None;
    }

    Ok(())
}

/// Get the window size on linux.
pub(crate) fn window_size() -> Result<TermSize> {
    let tty = TtyFd::get()?;
    let mut size = winsize {
        ws_col: 0,
        ws_row: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    Ok(
        to_io_result(unsafe { ioctl(tty.fd, TIOCGWINSZ, &mut size) })
            .map(|_| size.into())?,
    )
}

/// Wait for stdin input on linux with the given timeout. If zero returns
/// immidietly whether there is available input.
pub(crate) fn wait_for_stdin(timeout: Duration) -> Result<bool> {
    if timeout == Duration::MAX {
        return infinite_wait_for_stdin();
    }

    if timeout > MAX_STDIN_WAIT {
        return Err(Error::IntConvert);
    }

    let end = Instant::now() + timeout;

    let mut pdfs = pollfd {
        fd: libc::STDIN_FILENO,
        events: POLLIN,
        revents: 0,
    };

    loop {
        let wait = end.saturating_duration_since(Instant::now());
        let wait = timespec {
            tv_sec: wait.as_secs() as time_t,
            tv_nsec: wait.subsec_nanos().into(),
        };
        let r = unsafe { ppoll(&mut pdfs, 1, &wait, ptr::null()) };

        match r {
            0 => return Ok(false),
            1 => {
                return if (pdfs.revents & POLLERR) == POLLERR {
                    Err(Error::WaitAbandoned)
                } else {
                    Ok(true)
                };
            }
            -1 => {
                let err = io::Error::last_os_error();
                if err.kind() == ErrorKind::Interrupted {
                    continue;
                }
                return Err(err.into());
            }
            _ => return Err(Error::WaitAbandoned),
        }
    }
}

fn infinite_wait_for_stdin() -> Result<bool> {
    let mut pdfs = pollfd {
        fd: libc::STDIN_FILENO,
        events: POLLIN,
        revents: 0,
    };

    loop {
        let r = unsafe { poll(&mut pdfs, 1, -1) };

        match r {
            // Shouldn't happen
            0 => return Ok(false),
            1 => {
                return if (pdfs.revents & POLLERR) == POLLERR {
                    Err(Error::WaitAbandoned)
                } else {
                    Ok(true)
                };
            }
            -1 => {
                let err = io::Error::last_os_error();
                if err.kind() == ErrorKind::Interrupted {
                    continue;
                }
                return Err(err.into());
            }
            _ => return Err(Error::WaitAbandoned),
        }
    }
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
