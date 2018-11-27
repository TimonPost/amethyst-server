//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2018 Simon Ser

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice (including the next
    paragraph) shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
    THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
*/


use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;

static mut types_null: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];

// zwlr_screencopy_manager_v1

static mut zwlr_screencopy_manager_v1_requests_capture_output_types: [*const wl_interface; 3] = [
    unsafe { &zwlr_screencopy_frame_v1_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    unsafe { &wl_output_interface as *const wl_interface },
];
static mut zwlr_screencopy_manager_v1_requests_capture_output_region_types: [*const wl_interface; 7] = [
    unsafe { &zwlr_screencopy_frame_v1_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    unsafe { &wl_output_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut zwlr_screencopy_manager_v1_requests: [wl_message; 3] = [
    wl_message { name: b"capture_output\0" as *const u8 as *const c_char, signature: b"nio\0" as *const u8 as *const c_char, types: unsafe { &zwlr_screencopy_manager_v1_requests_capture_output_types as *const _ } },
    wl_message { name: b"capture_output_region\0" as *const u8 as *const c_char, signature: b"nioiiii\0" as *const u8 as *const c_char, types: unsafe { &zwlr_screencopy_manager_v1_requests_capture_output_region_types as *const _ } },
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut zwlr_screencopy_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_screencopy_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &zwlr_screencopy_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// zwlr_screencopy_frame_v1

static mut zwlr_screencopy_frame_v1_requests_copy_types: [*const wl_interface; 1] = [
    unsafe { &wl_buffer_interface as *const wl_interface },
];
pub static mut zwlr_screencopy_frame_v1_requests: [wl_message; 2] = [
    wl_message { name: b"copy\0" as *const u8 as *const c_char, signature: b"o\0" as *const u8 as *const c_char, types: unsafe { &zwlr_screencopy_frame_v1_requests_copy_types as *const _ } },
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut zwlr_screencopy_frame_v1_events: [wl_message; 4] = [
    wl_message { name: b"buffer\0" as *const u8 as *const c_char, signature: b"uuuu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"flags\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"ready\0" as *const u8 as *const c_char, signature: b"uuu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"failed\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut zwlr_screencopy_frame_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_screencopy_frame_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwlr_screencopy_frame_v1_requests as *const _ },
    event_count: 4,
    events: unsafe { &zwlr_screencopy_frame_v1_events as *const _ },
};

