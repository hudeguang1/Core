use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    let err = info.message().unwrap();
    if let Some(location) = info.location() {
        println!("\x1b[1;31mpanic at: '{}:{}, {}'\x1b[0m", location.file(), location.line(), err);
    } else {
        println!("\x1b[1;31mpanic: '{}'\x1b[0m", err);
    }
    loop {}
}