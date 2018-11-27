
//
// This file was auto-generated, do not edit directly.
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

pub mod zwlr_screencopy_manager_v1 {
    //! manager to inform clients and begin capturing
    //!
    //! This object is a manager which offers requests to start capturing from a
    //! source.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// capture an output
        ///
        /// Capture the next frame of an entire output.
        CaptureOutput {frame: Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>, overlay_cursor: i32, output: Proxy<super::wl_output::WlOutput>, },
        /// capture an output's region
        ///
        /// Capture the next frame of an output's region.
        /// 
        /// The region is given in output logical coordinates, see
        /// xdg_output.logical_size. The region will be clipped to the output's
        /// extents.
        CaptureOutputRegion {frame: Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>, overlay_cursor: i32, output: Proxy<super::wl_output::WlOutput>, x: i32, y: i32, width: i32, height: i32, },
        /// destroy the manager
        ///
        /// All objects created by the manager will still remain valid, until their
        /// appropriate destroy request has been called.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "capture_output",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Int,
                    super::ArgumentType::Object,
                ]
            },
            super::MessageDesc {
                name: "capture_output_region",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Int,
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ]
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[
                ]
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        fn opcode(&self) -> u16 {
            match *self {
                Request::CaptureOutput { .. } => 0,
                Request::CaptureOutputRegion { .. } => 1,
                Request::Destroy => 2,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>(version, meta.child())),
                1 => Some(Object::from_interface::<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>(version, meta.child())),
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }

        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::CaptureOutput { frame, overlay_cursor, output, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::NewId(frame.id()),
                        Argument::Int(overlay_cursor),
                        Argument::Object(output.id()),
                    ]
                },
                Request::CaptureOutputRegion { frame, overlay_cursor, output, x, y, width, height, } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::NewId(frame.id()),
                        Argument::Int(overlay_cursor),
                        Argument::Object(output.id()),
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ]
                },
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: vec![
                    ]
                },
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::CaptureOutput { frame, overlay_cursor, output, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = frame.c_ptr() as *mut _;
                    _args_array[1].i = overlay_cursor;
                    _args_array[2].o = output.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::CaptureOutputRegion { frame, overlay_cursor, output, x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 7] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = frame.c_ptr() as *mut _;
                    _args_array[1].i = overlay_cursor;
                    _args_array[2].o = output.c_ptr() as *mut _;
                    _args_array[3].i = x;
                    _args_array[4].i = y;
                    _args_array[5].i = width;
                    _args_array[6].i = height;
                    f(1, &mut _args_array)
                },
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        fn opcode(&self) -> u16 {
            match *self {
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                _ => Err(()),
            }
        }

        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwlrScreencopyManagerV1;

    impl Interface for ZwlrScreencopyManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_screencopy_manager_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_screencopy_manager_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// capture an output
        ///
        /// Capture the next frame of an entire output.
        fn capture_output<F>(&self, overlay_cursor: i32, output: &Proxy<super::wl_output::WlOutput>, implementor: F) ->Result<Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>, ()>
            where F: FnOnce(NewProxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>) -> Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>;
        /// capture an output's region
        ///
        /// Capture the next frame of an output's region.
        /// 
        /// The region is given in output logical coordinates, see
        /// xdg_output.logical_size. The region will be clipped to the output's
        /// extents.
        fn capture_output_region<F>(&self, overlay_cursor: i32, output: &Proxy<super::wl_output::WlOutput>, x: i32, y: i32, width: i32, height: i32, implementor: F) ->Result<Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>, ()>
            where F: FnOnce(NewProxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>) -> Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>;
        /// destroy the manager
        ///
        /// All objects created by the manager will still remain valid, until their
        /// appropriate destroy request has been called.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwlrScreencopyManagerV1> {
        fn capture_output<F>(&self, overlay_cursor: i32, output: &Proxy<super::wl_output::WlOutput>, implementor: F) ->Result<Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>, ()>
            where F: FnOnce(NewProxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>) -> Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>
        {
            let msg = Request::CaptureOutput {
                frame: self.child_placeholder(),
                overlay_cursor: overlay_cursor,
                output: output.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }

        fn capture_output_region<F>(&self, overlay_cursor: i32, output: &Proxy<super::wl_output::WlOutput>, x: i32, y: i32, width: i32, height: i32, implementor: F) ->Result<Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>, ()>
            where F: FnOnce(NewProxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>) -> Proxy<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>
        {
            let msg = Request::CaptureOutputRegion {
                frame: self.child_placeholder(),
                overlay_cursor: overlay_cursor,
                output: output.clone(),
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send_constructor(msg, implementor, None)
        }

        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zwlr_screencopy_frame_v1 {
    //! a frame ready for copy
    //!
    //! This object represents a single frame.
    //! 
    //! When created, a "buffer" event will be sent. The client will then be able
    //! to send a "copy" request. If the capture is successful, the compositor
    //! will send a "flags" followed by a "ready" event.
    //! 
    //! If the capture failed, the "failed" event is sent. This can happen anytime
    //! before the "ready" event.
    //! 
    //! Once either a "ready" or a "failed" event is received, the client should
    //! destroy the frame.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// the object has already been used to copy a wl_buffer
        AlreadyUsed = 0,
        /// buffer attributes are invalid
        InvalidBuffer = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyUsed),
                1 => Some(Error::InvalidBuffer),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    bitflags! {
        pub struct Flags: u32 {
            /// contents are y-inverted
            const YInvert = 1;
        }
    }
    impl Flags {
        pub fn from_raw(n: u32) -> Option<Flags> {
            Some(Flags::from_bits_truncate(n))

        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }

    pub enum Request {
        /// copy the frame
        ///
        /// Copy the frame to the supplied buffer. The buffer must have a the
        /// correct size, see zwlr_screencopy_frame_v1.buffer. The buffer needs to
        /// have a supported format.
        /// 
        /// If the frame is successfully copied, a "flags" and a "ready" events are
        /// sent. Otherwise, a "failed" event is sent.
        Copy {buffer: Proxy<super::wl_buffer::WlBuffer>, },
        /// delete this object, used or not
        ///
        /// Destroys the frame. This request can be sent at any time by the client.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "copy",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                ]
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[
                ]
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        fn opcode(&self) -> u16 {
            match *self {
                Request::Copy { .. } => 0,
                Request::Destroy => 1,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }

        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Copy { buffer, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::Object(buffer.id()),
                    ]
                },
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                    ]
                },
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Copy { buffer, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = buffer.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// buffer information
        ///
        /// Provides information about the frame's buffer. This event is sent once
        /// as soon as the frame is created.
        /// 
        /// The client should then create a buffer with the provided attributes, and
        /// send a "copy" request.
        Buffer {format: u32, width: u32, height: u32, stride: u32, },
        /// frame flags
        ///
        /// Provides flags about the frame. This event is sent once before the
        /// "ready" event.
        Flags {flags: Flags, },
        /// indicates frame is available for reading
        ///
        /// Called as soon as the frame is copied, indicating it is available
        /// for reading. This event includes the time at which presentation happened
        /// at.
        /// 
        /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
        /// each component being an unsigned 32-bit value. Whole seconds are in
        /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
        /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
        /// for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part
        /// may have an arbitrary offset at start.
        /// 
        /// After receiving this event, the client should destroy the object.
        Ready {tv_sec_hi: u32, tv_sec_lo: u32, tv_nsec: u32, },
        /// frame copy failed
        ///
        /// This event indicates that the attempted frame copy has failed.
        /// 
        /// After receiving this event, the client should destroy the object.
        Failed,
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "buffer",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "flags",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "ready",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "failed",
                since: 1,
                signature: &[
                ]
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        fn opcode(&self) -> u16 {
            match *self {
                Event::Buffer { .. } => 0,
                Event::Flags { .. } => 1,
                Event::Ready { .. } => 2,
                Event::Failed => 3,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Buffer {
                        format: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        width: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        height: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        stride: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Flags {
                        flags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Flags::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Ready {
                        tv_sec_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        tv_sec_lo: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        tv_nsec: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                3 => {
                    Ok(Event::Failed)
                },
                _ => Err(()),
            }
        }

        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Buffer {
                        format: _args[0].u,
                        width: _args[1].u,
                        height: _args[2].u,
                        stride: _args[3].u,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Flags {
                        flags: Flags::from_raw(_args[0].u).ok_or(())?,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Ready {
                        tv_sec_hi: _args[0].u,
                        tv_sec_lo: _args[1].u,
                        tv_nsec: _args[2].u,
                }) },
                3 => {
                    Ok(Event::Failed) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwlrScreencopyFrameV1;

    impl Interface for ZwlrScreencopyFrameV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_screencopy_frame_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_screencopy_frame_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// copy the frame
        ///
        /// Copy the frame to the supplied buffer. The buffer must have a the
        /// correct size, see zwlr_screencopy_frame_v1.buffer. The buffer needs to
        /// have a supported format.
        /// 
        /// If the frame is successfully copied, a "flags" and a "ready" events are
        /// sent. Otherwise, a "failed" event is sent.
        fn copy(&self, buffer: &Proxy<super::wl_buffer::WlBuffer>) ->();
        /// delete this object, used or not
        ///
        /// Destroys the frame. This request can be sent at any time by the client.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwlrScreencopyFrameV1> {
        fn copy(&self, buffer: &Proxy<super::wl_buffer::WlBuffer>) ->()
        {
            let msg = Request::Copy {
                buffer: buffer.clone(),
            };
            self.send(msg);
        }

        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

