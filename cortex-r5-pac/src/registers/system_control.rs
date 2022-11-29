//! Register definitions for the "system control" coprocessor (CP15)

mod dracr;
mod drbar;
mod drsr;
mod rgnr;
mod sctlr;

pub use dracr::DRACR;
pub use drbar::DRBAR;
pub use drsr::DRSR;
pub use rgnr::RGNR;
pub use sctlr::SCTLR;
