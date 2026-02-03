#![no_std]

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = c as u8;
        }
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe {
        for i in 0..n {
            let diff = *s1.add(i) as i32 - *s2.add(i) as i32;
            if diff != 0 {
                return diff;
            }
        }
        0
    }
}

// Windows MSVC C++ exception handler stub
#[unsafe(no_mangle)]
pub extern "C" fn __CxxFrameHandler3() -> ! {
    loop {}
}
