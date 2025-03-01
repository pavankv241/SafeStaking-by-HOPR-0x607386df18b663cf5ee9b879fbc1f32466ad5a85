pub mod db;
pub mod errors;
pub mod traits;

#[cfg(feature = "wasm")]
pub mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[allow(dead_code)]
    #[wasm_bindgen]
    pub fn core_ethereum_db_initialize_crate() {
        // When the `console_error_panic_hook` feature is enabled, we can call the
        // `set_panic_hook` function at least once during initialization, and then
        // we will get better error messages if our code ever panics.
        //
        // For more details see
        // https://github.com/rustwasm/console_error_panic_hook#readme
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
    }

    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    // #[cfg(feature = "wee_alloc")]
    // #[global_allocator]
    // static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
}
