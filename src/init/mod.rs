mod impls;

/// Struct represents hints to initializer about program
pub struct Hint;

// TODO: pub(crate)
pub struct Stt;

impl Default for Hint {
    fn default() -> Self {
        Hint
    }
}

/// Initializes qqx
pub fn initialize(hint: Hint) {
    unsafe { impls::initialize(hint) }
}
