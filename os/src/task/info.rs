//pub struct CloneFlags;


//use lazy_static::*;

pub struct utsname {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}

impl utsname {
    pub fn new() -> Self {
        Self {
            sysname: utsname::str2u8("WowOS/Linux"),
            nodename: utsname::str2u8("WowOS/untuntu"),
            release: utsname::str2u8("WowOS/20230526"),
            version: utsname::str2u8("WowOS/5.26"),
            machine: utsname::str2u8("WowOS/riscv64"),
            domainname: utsname::str2u8("WowOS/test"),
        }
    }

    pub fn str2u8(str: &str) -> [u8; 65] {
        let mut arr: [u8; 65] = [0; 65];
        let cstr = str.as_bytes();
        let len = str.len();
        for i in 0..len{
            arr[i] = cstr[i];
        }
        arr
    }

    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *const u8, size) }
    }
}




//use bitflags::bitflags;
bitflags!{
    pub struct CloneFlags: usize{
        const SIGCHLD = 17;
        const CSIGNAL	    =	0x000000ff;	
        const CLONE_VM	    =   0x00000100;
        const CLONE_FS      =	0x00000200;	
        const CLONE_FILES   =	0x00000400;
        const CLONE_SIGHAND =	0x00000800;	
        const CLONE_PIDFD	=   0x00001000;	
        const CLONE_PTRACE	=   0x00002000;
        const CLONE_VFORK	=   0x00004000;
        const CLONE_PARENT	=   0x00008000;
        const CLONE_THREAD	=   0x00010000;
        const CLONE_NEWNS	=   0x00020000;
        const CLONE_SYSVSEM =	0x00040000;
        const CLONE_SETTLS	=   0x00080000;	
        const CLONE_PARENT_SETTID	=   0x00100000;
        const CLONE_CHILD_CLEARTID	=   0x00200000;
        const CLONE_DETACHED		=   0x00400000;
        const CLONE_UNTRACED	    =	0x00800000;	
        const CLONE_CHILD_SETTID	=   0x01000000;
        const CLONE_NEWCGROUP	    =	0x02000000;	
        const CLONE_NEWUTS	=	0x04000000;	
        const CLONE_NEWIPC	=	0x08000000;
        const CLONE_NEWUSER	=	0x10000000;	
        const CLONE_NEWPID	=	0x20000000;	
        const CLONE_NEWNET	=	0x40000000;	
        const CLONE_IO		=   0x80000000;
    }
}
