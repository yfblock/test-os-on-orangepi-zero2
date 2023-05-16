use core::fmt;

struct QEMUOutput;

impl fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }

        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Return a reference to the console.
pub fn console() -> impl core::fmt::Write {
    QEMUOutput {}
}