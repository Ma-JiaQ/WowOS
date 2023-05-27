use crate::loader::get_app_data_by_name;
use crate::mm::{translated_refmut, translated_str, translated_byte_buffer};
use crate::task::{
    add_task, current_task, current_user_token, exit_current_and_run_next,
    suspend_current_and_run_next,
};
use crate::timer::get_time_ms;
use alloc::sync::Arc;

//add
pub use crate::task::{CloneFlags,utsname};
use core::arch::asm;
use crate::timer::TimeVal;

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

//add
pub fn sys_getppid() -> isize {
    current_task().unwrap().tgid as isize
}

pub fn sys_fork(flags: usize, stack: usize, _ptid: usize, _ctid: usize, _tls: usize) -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork(false);

    //add
    let flags = CloneFlags::from_bits(flags).unwrap();
    _ = flags;

    if stack != 0 {
        let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
        trap_cx.set_sp(stack);
    }


    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);

    //add
    unsafe { asm!("sfence.vma"); asm!("fence.i"); }

    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, status: *mut i32, options: isize) -> isize {
    if options != 0{
        panic!{"Extended option not support yet..."};
    }
    loop{
        let task = current_task().unwrap();
        // find a child process

        // ---- access current TCB exclusively
        let mut inner = task.inner_exclusive_access();
        
        //failed return-1
        if !inner
            .children
            .iter()
            .any(|p| pid == -1 || pid as usize == p.getpid())
        {
            return -1;
            // ---- release current PCB
        }
        let pair = inner.children.iter().enumerate().find(|(_, p)| {
            // ++++ temporarily access child PCB lock exclusively
            p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.getpid())
            // ++++ release child PCB
        });
        if let Some((idx, _)) = pair {
            let child = inner.children.remove(idx);
            // confirm that child will be deallocated after removing from children list
            assert_eq!(Arc::strong_count(&child), 1);
            let found_pid = child.getpid();
            // ++++ temporarily access child TCB exclusively
            let exit_code = child.inner_exclusive_access().exit_code;

            //add 
            let sstatus = exit_code << 8;
            
            if (status as usize) != 0{
                *translated_refmut(inner.memory_set.token(), status) = sstatus;
            }
            // ++++ release child PCB
            //*translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;
            return found_pid as isize;
        } else {
            drop(inner);
            drop(task);
            suspend_current_and_run_next();
        }
    }
    // ---- release current PCB lock automatically
}

//add 
/*pub fn sys_uname(buf:*const u8) -> isize{
    //取出正在执行的用户地址空间
    let token = current_user_token();
    let uname = utsname::new();
    //以向量的形式返回一组可以在内存空间中直接访问的字节数组切片buf_vec
    let mut buf_vec = translated_byte_buffer(token, buf, core::mem::size_of::<utsname>());
    //抽象缓冲区，使内核可以访问
    let mut usebuffer = UserBuffer::new(buf_vec);
    //将系统信息写入缓冲区usebuffer
    usebuffer.write(uname.as_bytes());
    0   
}*/

//add
pub fn sys_nanosleep(buf:*mut u8) -> isize {
    //获取当前时间
    let start = get_time_ms();
    let token = current_user_token();
    let sleep_time = translated_refmut(token, buf as *mut TimeVal);
    let time = sleep_time.sec*1000 + sleep_time.usec / 1000;
    loop{
        let end = get_time_ms();
        if end - start >= time {
            break;
        }
    };
    0
}