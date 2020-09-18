use std::ffi::CString;
use std::ptr;

use ffi::*;
use std::os::raw::{c_char, c_void};

pub struct LogStream {
    raw: AiLogStream,
    attached: bool
}

impl LogStream {
    pub fn file(filename: &str) -> Option<LogStream> {
        let cstr = CString::new(filename).unwrap();
        let stream = unsafe { aiGetPredefinedLogStream(AiDefaultLogStream::File, cstr.as_ptr()) };
        if stream.callback.is_some() {
            Some(LogStream { raw: stream, attached: false })
        } else {
            None
        }
    }

    pub fn stdout() -> LogStream {
        let stream = unsafe { aiGetPredefinedLogStream(AiDefaultLogStream::StdOut, ptr::null()) };
        LogStream { raw: stream, attached: false }
    }

    pub fn stderr() -> LogStream {
        let stream = unsafe { aiGetPredefinedLogStream(AiDefaultLogStream::StdErr, ptr::null()) };
        LogStream { raw: stream, attached: false }
    }

    #[cfg(windows)]
    pub fn debug() -> LogStream {
        let stream = unsafe { aiGetPredefinedLogStream(AiDefaultLogStream::Debugger, ptr::null()) };
        LogStream { raw: stream, attached: false }
    }

    pub fn callback(cb: unsafe extern "system" fn(*const c_char, *mut c_char)) -> LogStream {
        LogStream {
            raw: AiLogStream {
                callback: Some(cb),
                user: ptr::null::<c_void>() as *mut c_void
            },
            attached: false
        }
    }

    pub fn attached(&self) -> bool { self.attached }

    pub fn attach(&mut self) {
        if !self.attached { unsafe { aiAttachLogStream(&self.raw) } }
    }

    pub fn detach(&mut self) {
        if self.attached { unsafe { aiDetachLogStream(&self.raw); } }
    }

    pub fn set_verbose_logging(state: bool) {
        unsafe { aiEnableVerboseLogging(if state { AI_TRUE } else { AI_FALSE }) }
    }
}

impl Drop for LogStream {
    fn drop(&mut self) {
        self.detach()
    }
}
