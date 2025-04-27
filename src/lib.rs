pub mod helper;
pub mod segments;
pub mod edi835;
pub mod edi999;
pub mod edi270;
pub mod edi271;
pub mod error;
pub mod transaction_processor;
pub mod segment_config;
pub mod loop_processor;

// Re-export commonly used items
pub use error::{EdiError, EdiResult};
pub use transaction_processor::TransactionSet;

// Re-export transaction set controllers
pub use edi835::controller::Edi835;
pub use edi999::controller::Edi999;
pub use edi270::controller::Edi270;
pub use edi271::controller::Edi271;

// Re-export transaction set functions
pub use edi835::controller::{get_835, write_835};
pub use edi999::controller::{get_999, write_999};
pub use edi270::controller::{get_270, write_270};
pub use edi271::controller::{get_271, write_271};
