use rustix::fd::{AsFd, AsRawFd};

use libc::{c_int, c_void};

use crate::fs::opendir::MustangDir;

#[no_mangle]
unsafe extern "C" fn dirfd(dir: *mut c_void) -> c_int {
    libc!(libc::dirfd(dir.cast()));

    let dir = dir.cast::<MustangDir>();
    (*dir).dir.as_fd().as_raw_fd()
}