use std::ffi::{CString, NulError};
use crate::SymlinkError::{StringsInvalid, WindowsErrorResult};

fn main() {

    let from = std::env::args().nth(1);
    let to = std::env::args().nth(2);

    match (from, to) {
        (Some(from), Some(to)) => {
            match add_symlink_manually(from, to) {
                Err(StringsInvalid) => {
                    println!("Error: You've somehow provided strings with null characters. How did you do this.")
                }
                Err(WindowsErrorResult(error_code)) =>{
                    println!("Error: {}", error_code)
                }
                Ok(_) => {
                    println!("Success!")
                }
            }
        }

        (_, _) => {
            println!("Incorrect input arguments provided.");
        }
    }
}

enum SymlinkError {
    StringsInvalid,
    WindowsErrorResult(u32)
}
impl From<NulError> for SymlinkError{
    fn from(_value: NulError) -> Self {
        StringsInvalid
    }
}
fn add_symlink_manually(from: String, to: String) -> Result<u8, SymlinkError> {
    let from = CString::new(from)?;
    let to = CString::new(to)?;

    match unsafe {
        winapi::um::winbase::CreateSymbolicLinkA( from.as_ptr(),to.as_ptr(), 0x1) as u8
    } {
        0 => {
            let result = unsafe {
                winapi::um::errhandlingapi::GetLastError() as u32
            };
            Err(WindowsErrorResult(result))
        }
        x => {
            Ok(x)
        }
    }
}