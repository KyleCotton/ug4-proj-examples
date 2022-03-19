pub mod mutex_tree;
pub mod original_tree;
pub mod standard_tree;
pub mod macro_tree;

#[cfg(feature = "mutex")]
pub use crate::mutex_tree::RustyTree;

#[cfg(feature = "original")]
pub use crate::original_tree::RustyTree;

#[cfg(feature = "standard")]
pub use crate::standard_tree::RustyTree;

#[cfg(feature = "macro")]
pub use crate::macro_tree::RustyTree;

#[cfg(not(any(feature = "mutex", feature = "original", feature = "standard", feature = "macro")))]
compile_error!("One of the following features must be enabled: [mutex|original|standard|macro]");
