use crate::ctypes::{c_int, c_off};
use crate::{fs::opened_file::Fd, result::Result};
use crate::{process::current_process, syscalls::SyscallHandler};

pub enum WhenceValues {
    SEEK_SET = 0,  // seek relative to beginning of file
    SEEK_CUR = 1,  // seek relative to current file position
    SEEK_END = 2,  // seek relative to end of file
    SEEK_DATA = 3, // seek to the next data
    SEEK_HOLE = 4, // seek to the next hole
}

impl<'a> SyscallHandler<'a> {
    pub fn sys_lseek(&mut self, fd: Fd, offset: c_off, whence: c_int) -> Result<isize> {
        Ok(0)
    }
}
