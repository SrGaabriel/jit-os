#![no_std]

pub mod io;

use core::panic::PanicInfo;

pub fn abort(_info: &PanicInfo) -> ! {
    #[cfg(target_os = "windows")]
    unsafe {
        windows::ExitProcess(1);
    }

    #[cfg(not(target_os = "windows"))]
    loop {}
}

#[cfg(target_os = "windows")]
mod windows {
    use core::ffi::c_void;

    type HANDLE = *mut c_void;
    type DWORD = u32;
    type BOOL = i32;

    pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;

    unsafe extern "system" {
        pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
        pub fn WriteConsoleA(
            hConsoleOutput: HANDLE,
            lpBuffer: *const u8,
            nNumberOfCharsToWrite: DWORD,
            lpNumberOfCharsWritten: *mut DWORD,
            lpReserved: *mut c_void,
        ) -> BOOL;
        pub fn ExitProcess(uExitCode: u32) -> !;
    }

    pub fn write_stdout(s: &[u8]) {
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let mut written: DWORD = 0;
            WriteConsoleA(
                handle,
                s.as_ptr(),
                s.len() as DWORD,
                &mut written,
                core::ptr::null_mut(),
            );
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::stdout::print(format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::io::stdout::print(format_args!($($arg)*));
        $crate::io::stdout::print(format_args!("\n"));
    })
}
