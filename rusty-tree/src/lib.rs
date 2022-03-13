#[cfg(feature = "standard")]
mod standard_tree;
#[cfg(feature = "standard")]
pub use crate::tree::RustyTree;

#[cfg(feature = "macro")]
mod macro_tree;
#[cfg(feature = "macro")]
pub use crate::macro_tree::RustyTree;

// #[cfg(not(any(feature = "standard", feature = "macro")))]
// compile_error!("Either feature \"standard\" or \"macro\" must be enabled for this crate.");

mod original_tree;
pub use crate::original_tree::RustyTree;
