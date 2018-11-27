
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2017 Red Hat Inc.

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

pub mod zxdg_output_manager_v1 {
    //! manage xdg_output objects
    //!
    //! A global factory interface for xdg_output objects.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the xdg_output_manager object
        ///
        /// Using this request a client can tell the server that it is not
        /// going to use the xdg_output_manager object anymore.
        /// 
        /// Any objects already created through this instance are not affected.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// create an xdg output from a wl_output
        ///
        /// This creates a new xdg_output object for the given wl_output.
        GetXdgOutput {id: Proxy<super::zxdg_output_v1::ZxdgOutputV1>, output: Proxy<super::wl_output::WlOutput>, },
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "get_xdg_output",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Object,
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
                Request::Destroy => 0,
                Request::GetXdgOutput { .. } => 1,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<super::zxdg_output_v1::ZxdgOutputV1>(version, meta.child())),
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
                Request::GetXdgOutput { id, output, } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::NewId(id.id()),
                        Argument::Object(output.id()),
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
                Request::GetXdgOutput { id, output, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = output.c_ptr() as *mut _;
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


    pub struct ZxdgOutputManagerV1;

    impl Interface for ZxdgOutputManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_output_manager_v1";
        const VERSION: u32 = 2;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_output_manager_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// destroy the xdg_output_manager object
        ///
        /// Using this request a client can tell the server that it is not
        /// going to use the xdg_output_manager object anymore.
        /// 
        /// Any objects already created through this instance are not affected.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// create an xdg output from a wl_output
        ///
        /// This creates a new xdg_output object for the given wl_output.
        fn get_xdg_output<F>(&self, output: &Proxy<super::wl_output::WlOutput>, implementor: F) ->Result<Proxy<super::zxdg_output_v1::ZxdgOutputV1>, ()>
            where F: FnOnce(NewProxy<super::zxdg_output_v1::ZxdgOutputV1>) -> Proxy<super::zxdg_output_v1::ZxdgOutputV1>;
    }

    impl RequestsTrait for Proxy<ZxdgOutputManagerV1> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn get_xdg_output<F>(&self, output: &Proxy<super::wl_output::WlOutput>, implementor: F) ->Result<Proxy<super::zxdg_output_v1::ZxdgOutputV1>, ()>
            where F: FnOnce(NewProxy<super::zxdg_output_v1::ZxdgOutputV1>) -> Proxy<super::zxdg_output_v1::ZxdgOutputV1>
        {
            let msg = Request::GetXdgOutput {
                id: self.child_placeholder(),
                output: output.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }

    }
}

pub mod zxdg_output_v1 {
    //! compositor logical output region
    //!
    //! An xdg_output describes part of the compositor geometry.
    //! 
    //! This typically corresponds to a monitor that displays part of the
    //! compositor space.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the xdg_output object
        ///
        /// Using this request a client can tell the server that it is not
        /// going to use the xdg_output object anymore.
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
        /// position of the output within the global compositor space
        ///
        /// The position event describes the location of the wl_output within
        /// the global compositor space.
        /// 
        /// The logical_position event is sent after creating an xdg_output
        /// (see xdg_output_manager.get_xdg_output) and whenever the location
        /// of the output changes within the global compositor space.
        LogicalPosition {x: i32, y: i32, },
        /// size of the output in the global compositor space
        ///
        /// The logical_size event describes the size of the output in the
        /// global compositor space.
        /// 
        /// For example, a surface without any buffer scale, transformation
        /// nor rotation set, with the size matching the logical_size will
        /// have the same size as the corresponding output when displayed.
        /// 
        /// Most regular Wayland clients should not pay attention to the
        /// logical size and would rather rely on xdg_shell interfaces.
        /// 
        /// Some clients such as Xwayland, however, need this to configure
        /// their surfaces in the global compositor space as the compositor
        /// may apply a different scale from what is advertised by the output
        /// scaling property (to achieve fractional scaling, for example).
        /// 
        /// For example, for a wl_output mode 3840×2160 and a scale factor 2:
        /// 
        /// - A compositor not scaling the surface buffers will advertise a
        /// logical size of 3840×2160,
        /// 
        /// - A compositor automatically scaling the surface buffers will
        /// advertise a logical size of 1920×1080,
        /// 
        /// - A compositor using a fractional scale of 1.5 will advertise a
        /// logical size to 2560×1620.
        /// 
        /// The logical_size event is sent after creating an xdg_output
        /// (see xdg_output_manager.get_xdg_output) and whenever the logical
        /// size of the output changes, either as a result of a change in the
        /// applied scale or because of a change in the corresponding output
        /// mode(see wl_output.mode) or transform (see wl_output.transform).
        LogicalSize {width: i32, height: i32, },
        /// all information about the output have been sent
        ///
        /// This event is sent after all other properties of an xdg_output
        /// have been sent.
        /// 
        /// This allows changes to the xdg_output properties to be seen as
        /// atomic, even if they happen via multiple events.
        Done,
        /// name of this output
        ///
        /// Many compositors will assign names to their outputs, show them to the
        /// user, allow them to be configured by name, etc. The client may wish to
        /// know this name as well to offer the user similar behaviors.
        /// 
        /// The naming convention is compositor defined, but limited to
        /// alphanumeric characters and dashes (-). Each name is unique among all
        /// wl_output globals, but if a wl_output global is destroyed the same name
        /// may be reused later. The names will also remain consistent across
        /// sessions with the same hardware and software configuration.
        /// 
        /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
        /// not assume that the name is a reflection of an underlying DRM
        /// connector, X11 connection, etc.
        /// 
        /// The name event is sent after creating an xdg_output (see
        /// xdg_output_manager.get_xdg_output). This event is only sent once per
        /// xdg_output, and the name does not change over the lifetime of the
        /// wl_output global.
        ///
        /// Only available since version 2 of the interface
        Name {name: String, },
        /// human-readable description of this output
        ///
        /// Many compositors can produce human-readable descriptions of their
        /// outputs.  The client may wish to know this description as well, to
        /// communicate the user for various purposes.
        /// 
        /// The description is a UTF-8 string with no convention defined for its
        /// contents. Examples might include 'Foocorp 11" Display' or 'Virtual X11
        /// output via :1'.
        /// 
        /// The description event is sent after creating an xdg_output (see
        /// xdg_output_manager.get_xdg_output). This event is only sent once per
        /// xdg_output, and the description does not change over the lifetime of
        /// the wl_output global. The description is optional, and may not be sent
        /// at all.
        ///
        /// Only available since version 2 of the interface
        Description {description: String, },
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "logical_position",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ]
            },
            super::MessageDesc {
                name: "logical_size",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ]
            },
            super::MessageDesc {
                name: "done",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "name",
                since: 2,
                signature: &[
                    super::ArgumentType::Str,
                ]
            },
            super::MessageDesc {
                name: "description",
                since: 2,
                signature: &[
                    super::ArgumentType::Str,
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
                Event::LogicalPosition { .. } => 0,
                Event::LogicalSize { .. } => 1,
                Event::Done => 2,
                Event::Name { .. } => 3,
                Event::Description { .. } => 4,
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
                    Ok(Event::LogicalPosition {
                        x: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        y: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::LogicalSize {
                        width: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        height: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                2 => {
                    Ok(Event::Done)
                },
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Name {
                        name: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into());
                                s
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Description {
                        description: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into());
                                s
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
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::LogicalPosition {
                        x: _args[0].i,
                        y: _args[1].i,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::LogicalSize {
                        width: _args[0].i,
                        height: _args[1].i,
                }) },
                2 => {
                    Ok(Event::Done) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Name {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Description {
                        description: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZxdgOutputV1;

    impl Interface for ZxdgOutputV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_output_v1";
        const VERSION: u32 = 2;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_output_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// destroy the xdg_output object
        ///
        /// Using this request a client can tell the server that it is not
        /// going to use the xdg_output object anymore.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZxdgOutputV1> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

