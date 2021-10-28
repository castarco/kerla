use crate::ctypes::{c_int, c_off};
use crate::{fs::opened_file::Fd, result::Result};
use crate::{process::current_process, syscalls::SyscallHandler};

const SEEK_SET: c_int = 0;
const SEEK_CUR: c_int = 0;
const SEEK_END: c_int = 0;
const SEEK_DATA: c_int = 0;
const SEEK_HOLE: c_int = 0;

impl<'a> SyscallHandler<'a> {
    pub fn sys_lseek(&mut self, fd: Fd, offset: c_off, whence: c_int) -> Result<isize> {
        Ok(0)
    }
}
