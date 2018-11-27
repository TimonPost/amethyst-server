
//
// This file was auto-generated, do not edit directly.
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

pub mod zwlr_gamma_control_manager_v1 {
    //! manager to create per-output gamma controls
    //!
    //! This interface is a manager that allows creating per-output gamma
    //! controls.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// get a gamma control for an output
        ///
        /// Create a gamma control that can be used to adjust gamma tables for the
        /// provided output.
        GetGammaControl {id: Proxy<super::zwlr_gamma_control_v1::ZwlrGammaControlV1>, output: Proxy<super::wl_output::WlOutput>, },
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
                name: "get_gamma_control",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
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
                Request::GetGammaControl { .. } => 0,
                Request::Destroy => 1,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::zwlr_gamma_control_v1::ZwlrGammaControlV1>(version, meta.child())),
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }

        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetGammaControl { id, output, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::NewId(id.id()),
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
                Request::GetGammaControl { id, output, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = output.c_ptr() as *mut _;
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


    pub struct ZwlrGammaControlManagerV1;

    impl Interface for ZwlrGammaControlManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_gamma_control_manager_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_gamma_control_manager_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// get a gamma control for an output
        ///
        /// Create a gamma control that can be used to adjust gamma tables for the
        /// provided output.
        fn get_gamma_control<F>(&self, output: &Proxy<super::wl_output::WlOutput>, implementor: F) ->Result<Proxy<super::zwlr_gamma_control_v1::ZwlrGammaControlV1>, ()>
            where F: FnOnce(NewProxy<super::zwlr_gamma_control_v1::ZwlrGammaControlV1>) -> Proxy<super::zwlr_gamma_control_v1::ZwlrGammaControlV1>;
        /// destroy the manager
        ///
        /// All objects created by the manager will still remain valid, until their
        /// appropriate destroy request has been called.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwlrGammaControlManagerV1> {
        fn get_gamma_control<F>(&self, output: &Proxy<super::wl_output::WlOutput>, implementor: F) ->Result<Proxy<super::zwlr_gamma_control_v1::ZwlrGammaControlV1>, ()>
            where F: FnOnce(NewProxy<super::zwlr_gamma_control_v1::ZwlrGammaControlV1>) -> Proxy<super::zwlr_gamma_control_v1::ZwlrGammaControlV1>
        {
            let msg = Request::GetGammaControl {
                id: self.child_placeholder(),
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

pub mod zwlr_gamma_control_v1 {
    //! adjust gamma tables for an output
    //!
    //! This interface allows a client to adjust gamma tables for a particular
    //! output.
    //! 
    //! The client will receive the gamma size, and will then be able to set gamma
    //! tables. At any time the compositor can send a failed event indicating that
    //! this object is no longer valid.
    //! 
    //! There can only be at most one gamma control object per output, which
    //! has exclusive access to this particular output. When the gamma control
    //! object is destroyed, the gamma table is restored to its original value.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// invalid gamma tables
        InvalidGamma = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                1 => Some(Error::InvalidGamma),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// set the gamma table
        ///
        /// Set the gamma table. The file descriptor can be memory-mapped to provide
        /// the raw gamma table, which contains successive gamma ramps for the red,
        /// green and blue channels. Each gamma ramp is an array of 16-byte unsigned
        /// integers which has the same length as the gamma size.
        /// 
        /// The file descriptor data must have the same length as three times the
        /// gamma size.
        SetGamma {fd: ::std::os::unix::io::RawFd, },
        /// destroy this control
        ///
        /// Destroys the gamma control object. If the object is still valid, this
        /// restores the original gamma tables.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_gamma",
                since: 1,
                signature: &[
                    super::ArgumentType::Fd,
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
                Request::SetGamma { .. } => 0,
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
                Request::SetGamma { fd, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::Fd(fd),
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
                Request::SetGamma { fd, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].h = fd;
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
        /// size of gamma ramps
        ///
        /// Advertise the size of each gamma ramp.
        /// 
        /// This event is sent immediately when the gamma control object is created.
        GammaSize {size: u32, },
        /// object no longer valid
        ///
        /// This event indicates that the gamma control is no longer valid. This
        /// can happen for a number of reasons, including:
        /// - The output doesn't support gamma tables
        /// - Setting the gamma tables failed
        /// - Another client already has exclusive gamma control for this output
        /// - The compositor has transfered gamma control to another client
        /// 
        /// Upon receiving this event, the client should destroy this object.
        Failed,
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "gamma_size",
                since: 1,
                signature: &[
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
                Event::GammaSize { .. } => 0,
                Event::Failed => 1,
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
                    Ok(Event::GammaSize {
                        size: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
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
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::GammaSize {
                        size: _args[0].u,
                }) },
                1 => {
                    Ok(Event::Failed) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwlrGammaControlV1;

    impl Interface for ZwlrGammaControlV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_gamma_control_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_gamma_control_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// set the gamma table
        ///
        /// Set the gamma table. The file descriptor can be memory-mapped to provide
        /// the raw gamma table, which contains successive gamma ramps for the red,
        /// green and blue channels. Each gamma ramp is an array of 16-byte unsigned
        /// integers which has the same length as the gamma size.
        /// 
        /// The file descriptor data must have the same length as three times the
        /// gamma size.
        fn set_gamma(&self, fd: ::std::os::unix::io::RawFd) ->();
        /// destroy this control
        ///
        /// Destroys the gamma control object. If the object is still valid, this
        /// restores the original gamma tables.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwlrGammaControlV1> {
        fn set_gamma(&self, fd: ::std::os::unix::io::RawFd) ->()
        {
            let msg = Request::SetGamma {
                fd: fd,
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

