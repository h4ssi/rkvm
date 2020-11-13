#[cfg(target_os = "linux")]
mod device_id;

mod event;

#[cfg(target_os = "linux")]
mod glue;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{EventManager, EventWriter};

#[cfg(target_os = "windows")]
pub use windows::{EventManager, EventWriter};

pub use event::{Axis, Button, Direction, Event, Key, KeyKind};
