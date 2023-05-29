//! internal declaration of uniffi-bindgen CLI to stop version mismatches between various
//! versions of uniffi-bindgen. Implemented as suggested
//! [here](https://mozilla.github.io/uniffi-rs/tutorial/foreign_language_bindings.html#creating-the-bindgen-binary)

fn main() {
    uniffi::uniffi_bindgen_main();
}
