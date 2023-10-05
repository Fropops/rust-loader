#![cfg_attr(
    all(
      target_os = "windows",
      feature = "no_console",
    ),
    windows_subsystem = "windows"
  )]  

mod loader;

fn main() {
    loader::do_load();
}