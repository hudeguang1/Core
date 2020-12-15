pub mod context;
pub mod handler;

pub use context::Context;
pub fn init() {
    handler::init();
}