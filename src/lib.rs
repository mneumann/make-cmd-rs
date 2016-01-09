// Copyright (c) 2015 by Michael Neumann
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

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
