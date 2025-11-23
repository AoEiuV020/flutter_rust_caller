pub mod business;
pub mod call;
pub mod ffi;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export modules for public use
pub use business::*;
pub use call::{call, execute};
pub use ffi::{rust_call, rust_free_string};

#[cfg(target_arch = "wasm32")]
pub use wasm::{rust_call_wasm, rust_call_async_wasm, wasm_ready};
