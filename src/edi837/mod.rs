pub mod interchangecontrol;
pub mod interchangecontroltrailer;
pub mod table1;
pub mod loop2000a;
pub mod loop2000b;
pub mod loop2000c;
pub mod loop2010aa;
pub mod loop2010ab;
pub mod loop2010ac;
pub mod loop2010ba;
pub mod loop2010bb;
pub mod loop2010ca;
pub mod loop2300;
pub mod loop2400;
pub mod controller;

pub use controller::{Edi837P, Edi837I, Edi837D, get_837p, write_837p, get_837i, write_837i, get_837d, write_837d};
