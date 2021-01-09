pub mod context;
pub mod handler;
pub mod timer;

pub use context::Context;
pub fn init() {
    handler::init();
    timer::init();
    println!("mod trap initialized");
}