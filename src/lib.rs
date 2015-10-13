use std::process::Command;

/// `make` is an alias for `gnu_make`.
pub use gnu_make as make;

#[cfg(any(target_os = "freebsd", target_os = "dragonfly",
      target_os = "netbsd", target_os = "openbsd",
      target_os = "bitrig"))]
pub fn gnu_make() -> Command { Command::new("gmake") }

#[cfg(not(any(target_os = "freebsd", target_os = "dragonfly",
      target_os = "netbsd", target_os = "openbsd",
      target_os = "bitrig")))]
pub fn gnu_make() -> Command { Command::new("make") }

#[cfg(any(target_os = "freebsd", target_os = "dragonfly",
      target_os = "netbsd", target_os = "openbsd",
      target_os = "bitrig"))]
pub fn bsd_make() -> Command { Command::new("make") }
