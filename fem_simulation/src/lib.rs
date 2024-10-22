mod fem;
mod config;
mod io;
mod linear_solvers;
pub mod wasm; // Expose the wasm module

// Conditionally compile for WebAssembly
#[cfg(target_arch = "wasm32")]
pub use crate::wasm::*;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;

fn use_ipctk_function() {
    unsafe {
        // Call an ipctk function
        let result = ipctk_function();  // Replace with actual function
        // Handle the result
    }
}