use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_DETACH;

#[repr(transparent)]
pub struct HINSTANCE(pub isize);

mod loader;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => (),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    true
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllRegisterServer()  {
    loader::do_load();
}