use std::{io, mem::zeroed, ptr::null_mut, time::Duration};

use winapi::{
    shared::{
        minwindef::{BOOL, DWORD},
        winerror::WAIT_TIMEOUT,
    },
    um::{
        consoleapi::{GetConsoleMode, SetConsoleMode},
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        processenv::GetStdHandle,
        winbase::{
            STD_INPUT_HANDLE, WAIT_ABANDONED, WAIT_IO_COMPLETION,
            WAIT_OBJECT_0,
        },
        wincon::{
            CONSOLE_SCREEN_BUFFER_INFO, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT,
            ENABLE_PROCESSED_INPUT, GetConsoleScreenBufferInfo,
        },
        winnt::{
            FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE,
            HANDLE,
        },
        winuser::{
            MWMO_INPUTAVAILABLE, MsgWaitForMultipleObjectsEx, QS_ALLINPUT,
        },
    },
};

use crate::{
    error::{Error, Result},
    raw::TermSize,
};

const NO_RAW_BITS: DWORD =
    ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT;

struct Handle {
    handle: HANDLE,
    close: bool,
}

/// Enables raw mode on windows.
pub fn enable_raw_mode() -> Result<()> {
    let in_buf = Handle::current_in_buf()?;
    in_buf.set_mode(in_buf.get_mode()? & !NO_RAW_BITS)
}

/// Disables raw mode on windows.
pub fn disable_raw_mode() -> Result<()> {
    let in_buf = Handle::current_in_buf()?;
    in_buf.set_mode(in_buf.get_mode()? | NO_RAW_BITS)
}

/// Checks whether raw mode is enabled on windows.
pub fn is_raw_mode_enabled() -> Result<bool> {
    Handle::current_in_buf()?
        .get_mode()
        .map(|m| (m & NO_RAW_BITS) == 0)
}

/// Get the terminal size on windows. The size in pixels is not supported.
pub fn term_size() -> Result<TermSize> {
    Handle::current_out_buf()?.get_info().map(|i| TermSize {
        char_width: (i.srWindow.Right - i.srWindow.Left) as usize,
        char_height: (i.srWindow.Bottom - i.srWindow.Top) as usize,
        // Size in pixels is not supported
        pixel_width: 0,
        pixel_height: 0,
    })
}

/// Wait for stdin on windows with the given timeout. If timeout is zero
/// returns immidietely whether there is data on stdin.
pub fn wait_for_stdin(timeout: Duration) -> Result<bool> {
    let stdin = handle_result(unsafe { GetStdHandle(STD_INPUT_HANDLE) })?;
    let r = unsafe {
        MsgWaitForMultipleObjectsEx(
            1,
            &stdin,
            timeout.as_millis() as DWORD,
            QS_ALLINPUT,
            MWMO_INPUTAVAILABLE,
        )
    };

    const INTERUPT: DWORD = WAIT_OBJECT_0 + 1;
    match r {
        WAIT_OBJECT_0 => Ok(true),
        INTERUPT | WAIT_TIMEOUT | WAIT_IO_COMPLETION => Ok(false),
        WAIT_ABANDONED => Err(Error::WaitAbandoned),
        _ => Err(last_err()),
    }
}

fn result(val: BOOL) -> Result<()> {
    if val == 0 { Err(last_err()) } else { Ok(()) }
}

fn handle_result(val: HANDLE) -> Result<HANDLE> {
    if val == INVALID_HANDLE_VALUE {
        Err(last_err())
    } else {
        Ok(val)
    }
}

fn last_err() -> Error {
    io::Error::last_os_error().into()
}

impl Handle {
    fn exclusive(handle: HANDLE) -> Result<Self> {
        Ok(Self {
            handle: handle_result(handle)?,
            close: true,
        })
    }

    fn current_out_buf() -> Result<Self> {
        Self::file_handle("CONOUT$")
    }

    fn current_in_buf() -> Result<Self> {
        Self::file_handle("CONIN$")
    }

    fn get_info(&self) -> Result<CONSOLE_SCREEN_BUFFER_INFO> {
        let mut res = unsafe { zeroed() };
        result(unsafe { GetConsoleScreenBufferInfo(self.handle, &mut res) })?;
        Ok(res)
    }

    fn get_mode(&self) -> Result<DWORD> {
        let mut mode = 0;
        unsafe { result(GetConsoleMode(self.handle, &mut mode))? };
        Ok(mode)
    }

    fn set_mode(&self, mode: DWORD) -> Result<()> {
        result(unsafe { SetConsoleMode(self.handle, mode) })
    }

    fn file_handle(path: &str) -> Result<Self> {
        let path: Vec<_> = path.encode_utf16().chain([0]).collect();
        let path = path.as_ptr();

        let handle = unsafe {
            CreateFileW(
                path,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut(),
            )
        };

        Self::exclusive(handle)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if self.close && unsafe { CloseHandle(self.handle) != 0 } {
            panic!("Failed to close handle: {}", io::Error::last_os_error());
        }
    }
}
