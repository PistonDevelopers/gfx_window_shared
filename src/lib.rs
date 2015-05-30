//! Gfx output implementations using shared references to an underlying Piston
//! window with OpenGL support.

extern crate gfx;
extern crate gfx_device_gl;
extern crate window;

pub use sync::{ SyncOutput, SyncSuccess, init_sync };
pub use shared::{ SharedOutput, SharedSuccess, init_shared };

mod sync;
mod shared;
