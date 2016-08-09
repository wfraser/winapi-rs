// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
#![cfg(feature = "shared.minwindef+shared.winnt+vc.excpt")]
ENUM!{enum EXCEPTION_DISPOSITION {
    ExceptionContinueExecution,
    ExceptionContinueSearch,
    ExceptionNestedException,
    ExceptionCollidedUnwind,
}}
#[cfg(target_arch = "x86")]
pub type UHALF_PTR = c_ushort;
#[cfg(target_arch = "x86_64")]
pub type UHALF_PTR = c_uint;
// TODO some functions
pub const EXCEPTION_EXECUTE_HANDLER: i32 = 1;
pub const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
pub const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
