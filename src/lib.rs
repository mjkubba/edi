// Library crate: all public items are part of the API even if not used by the binary.
#![allow(dead_code)]

pub mod edi270;
pub mod edi271;
pub mod edi276;
pub mod edi277;
pub mod edi278;
pub mod edi820;
pub mod edi834;
pub mod edi835;
pub mod edi837;
pub mod edi999;
pub mod error;
pub mod helper;
pub mod segments;
pub mod transaction_processor;

// Re-export commonly used items
pub use error::{EdiError, EdiResult};
pub use transaction_processor::TransactionSet;

// Re-export transaction set controllers
pub use edi270::controller::Edi270;
pub use edi271::controller::Edi271;
pub use edi276::controller::Edi276;
pub use edi277::controller::Edi277;
pub use edi278::controller::Edi278;
pub use edi834::controller::Edi834;
pub use edi835::controller::Edi835;
pub use edi837::controller::{Edi837, Edi837D, Edi837I, Edi837P, Edi837Subtype};
pub use edi999::controller::Edi999;

// Re-export transaction set functions
pub use edi270::controller::{get_270, write_270};
pub use edi271::controller::{get_271, write_271};
pub use edi276::controller::{get_276, write_276};
pub use edi277::controller::{get_277, write_277};
pub use edi278::controller::{get_278, write_278};
pub use edi834::controller::{get_834, write_834};
pub use edi835::controller::{get_835, write_835};
pub use edi837::controller::{
    get_837, get_837d, get_837i, get_837p, write_837, write_837d, write_837i, write_837p,
};
pub use edi999::controller::{get_999, write_999};
