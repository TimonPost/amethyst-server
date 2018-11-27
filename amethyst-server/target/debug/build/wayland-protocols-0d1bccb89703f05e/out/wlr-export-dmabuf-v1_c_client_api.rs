
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2018 Rostislav Pehlivanov

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

pub mod zwlr_export_dmabuf_manager_v1 {
    //! manager to inform clients and begin capturing
    //!
    //! This object is a manager with which to start capturing from sources.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// capture a frame from an output
        ///
        /// Capture the next frame of a an entire output.
        CaptureOutput {frame: Proxy<super::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1>, overlay_cursor: i32, output: Proxy<super::wl_output::WlOutput>, },
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
                Request::Destroy => 1,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1>(version, meta.child())),
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
                Request::CaptureOutput { frame, overlay_cursor, output, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = frame.c_ptr() as *mut _;
                    _args_array[1].i = overlay_cursor;
                    _args_array[2].o = output.c_ptr() as *mut _;
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


    pub struct ZwlrExportDmabufManagerV1;

    impl Interface for ZwlrExportDmabufManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_export_dmabuf_manager_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_export_dmabuf_manager_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// capture a frame from an output
        ///
        /// Capture the next frame of a an entire output.
        fn capture_output<F>(&self, overlay_cursor: i32, output: &Proxy<super::wl_output::WlOutput>, implementor: F) ->Result<Proxy<super::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1>, ()>
            where F: FnOnce(NewProxy<super::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1>) -> Proxy<super::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1>;
        /// destroy the manager
        ///
        /// All objects created by the manager will still remain valid, until their
        /// appropriate destroy request has been called.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwlrExportDmabufManagerV1> {
        fn capture_output<F>(&self, overlay_cursor: i32, output: &Proxy<super::wl_output::WlOutput>, implementor: F) ->Result<Proxy<super::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1>, ()>
            where F: FnOnce(NewProxy<super::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1>) -> Proxy<super::zwlr_export_dmabuf_frame_v1::ZwlrExportDmabufFrameV1>
        {
            let msg = Request::CaptureOutput {
                frame: self.child_placeholder(),
                overlay_cursor: overlay_cursor,
                output: output.clone(),
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

pub mod zwlr_export_dmabuf_frame_v1 {
    //! a DMA-BUF frame
    //!
    //! This object represents a single DMA-BUF frame.
    //! 
    //! If the capture is successful, the compositor will first send a "frame"
    //! event, followed by one or several "object". When the frame is available
    //! for readout, the "ready" event is sent.
    //! 
    //! If the capture failed, the "cancel" event is sent. This can happen anytime
    //! before the "ready" event.
    //! 
    //! Once either a "ready" or a "cancel" event is received, the client should
    //! destroy the frame. Once an "object" event is received, the client is
    //! responsible for closing the associated file descriptor.
    //! 
    //! All frames are read-only and may not be written into or altered.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// frame flags
    ///
    /// Special flags that should be respected by the client.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Flags {
        /// clients should copy frame before processing
        Transient = 0x1,
    }
    impl Flags {
        pub fn from_raw(n: u32) -> Option<Flags> {
            match n {
                0x1 => Some(Flags::Transient),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// cancel reason
    ///
    /// Indicates reason for cancelling the frame.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum CancelReason {
        /// temporary error, source will produce more frames
        Temporary = 0,
        /// fatal error, source will not produce frames
        Permanent = 1,
        /// temporary error, source will produce more frames
        Resizing = 2,
    }
    impl CancelReason {
        pub fn from_raw(n: u32) -> Option<CancelReason> {
            match n {
                0 => Some(CancelReason::Temporary),
                1 => Some(CancelReason::Permanent),
                2 => Some(CancelReason::Resizing),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// delete this object, used or not
        ///
        /// Unreferences the frame. This request must be called as soon as its no
        /// longer used.
        /// 
        /// It can be called at any time by the client. The client will still have
        /// to close any FDs it has been given.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
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
            }
        }

        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
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
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
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
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// a frame description
        ///
        /// Main event supplying the client with information about the frame. If the
        /// capture didn't fail, this event is always emitted first before any other
        /// events.
        /// 
        /// This event is followed by a number of "object" as specified by the
        /// "num_objects" argument.
        Frame {width: u32, height: u32, offset_x: u32, offset_y: u32, buffer_flags: u32, flags: Flags, format: u32, mod_high: u32, mod_low: u32, num_objects: u32, },
        /// an object description
        ///
        /// Event which serves to supply the client with the file descriptors
        /// containing the data for each object.
        /// 
        /// After receiving this event, the client must always close the file
        /// descriptor as soon as they're done with it and even if the frame fails.
        Object {index: u32, fd: ::std::os::unix::io::RawFd, size: u32, offset: u32, stride: u32, plane_index: u32, },
        /// indicates frame is available for reading
        ///
        /// This event is sent as soon as the frame is presented, indicating it is
        /// available for reading. This event includes the time at which
        /// presentation happened at.
        /// 
        /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
        /// each component being an unsigned 32-bit value. Whole seconds are in
        /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
        /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
        /// for valid timestamps tv_nsec must be in [0, 999999999]. The seconds part
        /// may have an arbitrary offset at start.
        /// 
        /// After receiving this event, the client should destroy this object.
        Ready {tv_sec_hi: u32, tv_sec_lo: u32, tv_nsec: u32, },
        /// indicates the frame is no longer valid
        ///
        /// If the capture failed or if the frame is no longer valid after the
        /// "frame" event has been emitted, this event will be used to inform the
        /// client to scrap the frame.
        /// 
        /// If the failure is temporary, the client may capture again the same
        /// source. If the failure is permanent, any further attempts to capture the
        /// same source will fail again.
        /// 
        /// After receiving this event, the client should destroy this object.
        Cancel {reason: CancelReason, },
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "frame",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "object",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Fd,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
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
                name: "cancel",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
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
                Event::Frame { .. } => 0,
                Event::Object { .. } => 1,
                Event::Ready { .. } => 2,
                Event::Cancel { .. } => 3,
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
                    Ok(Event::Frame {
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
                        offset_x: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        offset_y: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        buffer_flags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        flags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Flags::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                        format: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        mod_high: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        mod_low: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        num_objects: {
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
                    Ok(Event::Object {
                        index: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        fd: {
                            if let Some(Argument::Fd(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        size: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        offset: {
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
                        plane_index: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
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
                    let mut args = msg.args.into_iter();
                    Ok(Event::Cancel {
                        reason: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                CancelReason::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
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
                    let _args = ::std::slice::from_raw_parts(args, 10);
                    Ok(Event::Frame {
                        width: _args[0].u,
                        height: _args[1].u,
                        offset_x: _args[2].u,
                        offset_y: _args[3].u,
                        buffer_flags: _args[4].u,
                        flags: Flags::from_raw(_args[5].u).ok_or(())?,
                        format: _args[6].u,
                        mod_high: _args[7].u,
                        mod_low: _args[8].u,
                        num_objects: _args[9].u,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 6);
                    Ok(Event::Object {
                        index: _args[0].u,
                        fd: _args[1].h,
                        size: _args[2].u,
                        offset: _args[3].u,
                        stride: _args[4].u,
                        plane_index: _args[5].u,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Ready {
                        tv_sec_hi: _args[0].u,
                        tv_sec_lo: _args[1].u,
                        tv_nsec: _args[2].u,
                }) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Cancel {
                        reason: CancelReason::from_raw(_args[0].u).ok_or(())?,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwlrExportDmabufFrameV1;

    impl Interface for ZwlrExportDmabufFrameV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_export_dmabuf_frame_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_export_dmabuf_frame_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// delete this object, used or not
        ///
        /// Unreferences the frame. This request must be called as soon as its no
        /// longer used.
        /// 
        /// It can be called at any time by the client. The client will still have
        /// to close any FDs it has been given.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwlrExportDmabufFrameV1> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

