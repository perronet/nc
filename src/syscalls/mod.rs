// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod types;
pub use types::*;

#[cfg(not(nightly))]
#[path = "c.rs"]
mod internal;

#[cfg(all(nightly, target_arch = "aarch64"))]
#[path = "syscall_aarch64.rs"]
mod internal;

#[cfg(all(nightly, target_arch = "arm"))]
#[path = "syscall_arm.rs"]
mod internal;

#[cfg(all(nightly, target_arch = "mips"))]
#[path = "syscall_mips.rs"]
mod internal;

#[cfg(all(nightly, target_arch = "mips64"))]
#[path = "syscall_mips64.rs"]
mod internal;

#[cfg(all(nightly, target_arch = "powerpc64"))]
#[path = "syscall_powerpc64.rs"]
mod internal;

#[cfg(all(nightly, target_arch = "s390x"))]
#[path = "syscall_s390x.rs"]
mod internal;

#[cfg(all(nightly, target_arch = "x86"))]
#[path = "syscall_x86.rs"]
mod internal;

#[cfg(all(nightly, target_arch = "x86_64"))]
#[path = "syscall_x86_64.rs"]
mod internal;

pub use internal::*;
