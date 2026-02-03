use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        #[cfg(target_os = "windows")]
        crate::windows::write_stdout(s.as_bytes());

        #[cfg(not(target_os = "windows"))]
        {
            // TODO: implement for other platforms via syscalls
            let _ = s;
        }

        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}
