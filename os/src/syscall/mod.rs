//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAIT4: usize = 260;


//add
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_NANOSLEEP: usize = 101;


mod fs;
mod process;


use fs::*;
use process::*;
/// handle syscall exception with `syscall_id` and other arguments
/// add
pub fn syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    match syscall_id {
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(args[0], args[1], args[2], args[3], args[4]),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8),
        
        //add
        SYSCALL_WAIT4 => sys_waitpid(args[0] as isize, args[1] as *mut i32, args[2] as isize),

        //add
        SYSCALL_GETPPID => sys_getppid(),
        //SYSCALL_UNAME => sys_uname(args[0] as *const u8),
        SYSCALL_NANOSLEEP => sys_nanosleep(args[0] as *mut u8),

        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
