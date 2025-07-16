use wasm_bindgen::prelude::*;

pub mod clib {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(clippy::all)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use clib::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn js_console_log(s: &str);
}

pub struct JsLog;

impl log::Log for JsLog {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool { true }
    fn flush(&self) { }
    fn log(&self, record: &log::Record<'_>) {
        js_console_log(&format!("{}:{} -- {}",
            record.level(),
            record.target(),
            record.args()));

    }
}

// Maximum alginment not less than usize
const MALLOC_HEADER_SIZE: usize = 8;

#[no_mangle]
pub unsafe extern "C" fn _malloc_r(re: *mut _reent, len: usize) -> *mut u8 {
    use std::alloc::*;

    let len2 = len + MALLOC_HEADER_SIZE;
    let ptr = alloc(Layout::from_size_align(len2 , MALLOC_HEADER_SIZE).unwrap());
    if ptr.is_null() {
        (*re)._errno = ENOMEM as _;
        ptr
    } else {
        (ptr as *mut usize).write(len2);
        ptr.offset(MALLOC_HEADER_SIZE as isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn _free_r(_: *mut _reent, ptr: *mut u8) {
    use std::alloc::*;
    if ptr.is_null() {
        return;
    }
    let ptr = ptr.offset(-(MALLOC_HEADER_SIZE as isize));
    let len2 = (ptr as *mut usize).read();
    dealloc(ptr, Layout::from_size_align(len2, MALLOC_HEADER_SIZE).unwrap());
}
#[no_mangle]
pub unsafe extern "C" fn _realloc_r(re: *mut _reent, ptr: *mut u8, new_len: usize) -> *mut u8 {
    use std::alloc::*;
    if ptr.is_null() {
        return _malloc_r(re, new_len);
    }
    let ptr = ptr.offset(-(MALLOC_HEADER_SIZE as isize));
    let len2 = (ptr as *mut usize).read();
    realloc(ptr, Layout::from_size_align(len2, MALLOC_HEADER_SIZE).unwrap(), new_len)
}
#[no_mangle]
pub unsafe extern "C" fn _calloc_r(re: *mut _reent, nmemb: usize, len: usize) -> *mut u8 {
    let real_len = match nmemb.checked_mul(len) {
        Some(x) => x,
        None => {
            (*re)._errno = ENOMEM as _;
            return std::ptr::null_mut();
        }
    };
    let ptr = _malloc_r(re, real_len);
    ptr.write_bytes(0, real_len);
    ptr
}

// Wasm32 never exits
#[no_mangle]
pub unsafe extern "C" fn _exit(_e: i32) {
    log::debug!("_exit({_e})");
}
#[no_mangle]
pub unsafe extern "C" fn __cxa_atexit(_: i32, _: i32, _: i32) -> i32 {
    log::debug!("_cxa_atexit()");
    0
}
#[no_mangle]
pub unsafe extern "C" fn _read_r(re: *mut _reent, _fd: i32, _buf: *mut u8, _len: usize) -> isize {
    log::debug!("_read({_fd}, , {_len})");
    (*re)._errno = EINVAL as _;
    -1
}
#[no_mangle]
pub unsafe extern "C" fn _lseek_r(re: *mut _reent, _fd: i32, _offs: isize, _whence: i32) -> i32 {
    log::debug!("_lseek({_fd}, {_offs}, {_whence})");
    (*re)._errno = ESPIPE as _;
    -1
}

static mut STDOUT: Vec<u8> = Vec::new();
static mut STDERR: Vec<u8> = Vec::new();

#[no_mangle]
pub unsafe extern "C" fn _write_r(re: *mut _reent, fd: i32, buf: *const u8, len: usize) -> isize {
    log::debug!("_write_r({fd}, , {len})");
    let mut s = std::slice::from_raw_parts(buf, len);

    //Wasm is single threaded, so no harm here
    #[allow(static_mut_refs)]
    let f = match fd {
        1 => &mut STDOUT,
        2 => &mut STDERR,
        _ => {
            (*re)._errno = EIO as _;
            return -1;
        }
    };
    while !s.is_empty() {
        match s.iter().position(|c| *c == b'\n') {
            None => {
                f.extend(s);
                break;
            }
            Some(eol) => {
                f.extend(&s[.. eol]);
                match fd {
                    1 => log::info!("{}", String::from_utf8_lossy(&f)),
                    2 => log::warn!("{}", String::from_utf8_lossy(&f)),
                    _ => unreachable!(),
                }
                f.clear();
                s = &s[eol + 1 ..];
            }
        }
    }
    len as isize
}
#[no_mangle]
pub unsafe extern "C" fn _close_r(_re: *mut _reent, _fd: i32) -> i32 {
    log::debug!("_close({_fd})");
    0
}
#[no_mangle]
pub unsafe extern "C" fn _isatty_r(re: *mut _reent, _fd: i32) -> i32 {
    log::debug!("_isatty({_fd})");
    (*re)._errno = ENOTTY as _;
    0
}
#[no_mangle]
pub unsafe extern "C" fn _fstat_r(re: *mut _reent, _fd: i32, _buf: i32) -> i32 {
    log::debug!("_fstat({_fd})");
    (*re)._errno = EIO as _;
    -1
}
#[no_mangle]
pub unsafe extern "C" fn _getpid_r(_re: *mut _reent) -> i32 {
    log::debug!("_getpid()");
    // pid=1 is special but we are not so
    2
}
#[no_mangle]
pub unsafe extern "C" fn _kill_r(re: *mut _reent, _pid: i32, _sig: i32) -> i32 {
    log::debug!("_kill({_pid}, {_sig})");
    (*re)._errno = EPERM as _;
    -1
}
#[no_mangle]
pub unsafe extern "C" fn _open_r(re: *mut _reent, _path: *const u8, _flags: i32, _mode: i32) -> i32 {
    log::debug!("_open(...)");
    (*re)._errno = ENOENT as _;
    -1
}
