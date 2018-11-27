//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2015 Giulio camuffo
    Copyright © 2018 Simon Ser

    Permission to use, copy, modify, distribute, and sell this
    software and its documentation for any purpose is hereby granted
    without fee, provided that the above copyright notice appear in
    all copies and that both that copyright notice and this permission
    notice appear in supporting documentation, and that the name of
    the copyright holders not be used in advertising or publicity
    pertaining to distribution of the software without specific,
    written prior permission.  The copyright holders make no
    representations about the suitability of this software for any
    purpose.  It is provided "as is" without express or implied
    warranty.

    THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS
    SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
    FITNESS, IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY
    SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
    WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
    AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
    ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
    THIS SOFTWARE.
*/


use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;

static mut types_null: [*const wl_interface; 1] = [
    NULLPTR as *const wl_interface,
];

// zwlr_gamma_control_manager_v1

static mut zwlr_gamma_control_manager_v1_requests_get_gamma_control_types: [*const wl_interface; 2] = [
    unsafe { &zwlr_gamma_control_v1_interface as *const wl_interface },
    unsafe { &wl_output_interface as *const wl_interface },
];
pub static mut zwlr_gamma_control_manager_v1_requests: [wl_message; 2] = [
    wl_message { name: b"get_gamma_control\0" as *const u8 as *const c_char, signature: b"no\0" as *const u8 as *const c_char, types: unsafe { &zwlr_gamma_control_manager_v1_requests_get_gamma_control_types as *const _ } },
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut zwlr_gamma_control_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_gamma_control_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwlr_gamma_control_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// zwlr_gamma_control_v1

pub static mut zwlr_gamma_control_v1_requests: [wl_message; 2] = [
    wl_message { name: b"set_gamma\0" as *const u8 as *const c_char, signature: b"h\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut zwlr_gamma_control_v1_events: [wl_message; 2] = [
    wl_message { name: b"gamma_size\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"failed\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut zwlr_gamma_control_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_gamma_control_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwlr_gamma_control_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwlr_gamma_control_v1_events as *const _ },
};

