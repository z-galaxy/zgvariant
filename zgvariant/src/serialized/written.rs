use std::ops::Deref;

use crate::serialized::Context;

/// Represents the return value of [`crate::to_writer`] function.
///
/// It mainly contains the size of serialized bytes in the GVariant format.
#[derive(Debug)]
pub struct Written {
    size: usize,
    context: Context,
}

impl Written {
    /// Create a new `Written` instance.
    pub fn new(size: usize, context: Context) -> Self {
        Self { size, context }
    }

    /// The size of the serialized bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    /// The encoding context.
    pub fn context(&self) -> Context {
        self.context
    }
}

impl Deref for Written {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.size
    }
}
