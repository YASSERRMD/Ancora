use std::ffi::CStr;
use std::os::raw::c_char;

use crate::buffer::AncorBuffer;
use crate::error_code::AncorErrorCode;
use crate::handles::AncorRuntime;
use crate::runtime::InnerRuntime;
use crate::tool_callback::AncorToolCallback;
