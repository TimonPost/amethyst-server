
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

pub mod zwp_xwayland_keyboard_grab_manager_v1 {
    //! context object for keyboard grab manager
    //!
    //! A global interface used for grabbing the keyboard.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the keyboard grab manager
        ///
        /// Destroy the keyboard grab manager.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// grab the keyboard to a surface
        ///
        /// The grab_keyboard request asks for a grab of the keyboard, forcing
        /// the keyboard focus for the given seat upon the given surface.
        /// 
        /// The protocol provides no guarantee that the grab is ever satisfied,
        /// and does not require the compositor to send an error if the grab
        /// cannot ever be satisfied. It is thus possible to request a keyboard
        /// grab that will never be effective.
        /// 
        /// The protocol:
        /// 
        /// * does not guarantee that the grab itself is applied for a surface,
        /// the grab request may be silently ignored by the compositor,
        /// * does not guarantee that any events are sent to this client even
        /// if the grab is applied to a surface,
        /// * does not guarantee that events sent to this client are exhaustive,
        /// a compositor may filter some events for its own consumption,
        /// * does not guarantee that events sent to this client are continuous,
        /// a compositor may change and reroute keyboard events while the grab
        /// is nominally active.
        GrabKeyboard {id: Proxy<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1>, surface: Proxy<super::wl_surface::WlSurface>, seat: Proxy<super::wl_seat::WlSeat>, },
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
                name: "grab_keyboard",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Object,
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
                Request::GrabKeyboard { .. } => 1,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1>(version, meta.child())),
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
                Request::GrabKeyboard { id, surface, seat, } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::NewId(id.id()),
                        Argument::Object(surface.id()),
                        Argument::Object(seat.id()),
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
                Request::GrabKeyboard { id, surface, seat, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    _args_array[2].o = seat.c_ptr() as *mut _;
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


    pub struct ZwpXwaylandKeyboardGrabManagerV1;

    impl Interface for ZwpXwaylandKeyboardGrabManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_xwayland_keyboard_grab_manager_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_xwayland_keyboard_grab_manager_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// destroy the keyboard grab manager
        ///
        /// Destroy the keyboard grab manager.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// grab the keyboard to a surface
        ///
        /// The grab_keyboard request asks for a grab of the keyboard, forcing
        /// the keyboard focus for the given seat upon the given surface.
        /// 
        /// The protocol provides no guarantee that the grab is ever satisfied,
        /// and does not require the compositor to send an error if the grab
        /// cannot ever be satisfied. It is thus possible to request a keyboard
        /// grab that will never be effective.
        /// 
        /// The protocol:
        /// 
        /// * does not guarantee that the grab itself is applied for a surface,
        /// the grab request may be silently ignored by the compositor,
        /// * does not guarantee that any events are sent to this client even
        /// if the grab is applied to a surface,
        /// * does not guarantee that events sent to this client are exhaustive,
        /// a compositor may filter some events for its own consumption,
        /// * does not guarantee that events sent to this client are continuous,
        /// a compositor may change and reroute keyboard events while the grab
        /// is nominally active.
        fn grab_keyboard<F>(&self, surface: &Proxy<super::wl_surface::WlSurface>, seat: &Proxy<super::wl_seat::WlSeat>, implementor: F) ->Result<Proxy<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1>, ()>
            where F: FnOnce(NewProxy<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1>) -> Proxy<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1>;
    }

    impl RequestsTrait for Proxy<ZwpXwaylandKeyboardGrabManagerV1> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn grab_keyboard<F>(&self, surface: &Proxy<super::wl_surface::WlSurface>, seat: &Proxy<super::wl_seat::WlSeat>, implementor: F) ->Result<Proxy<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1>, ()>
            where F: FnOnce(NewProxy<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1>) -> Proxy<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1>
        {
            let msg = Request::GrabKeyboard {
                id: self.child_placeholder(),
                surface: surface.clone(),
                seat: seat.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }

    }
}

pub mod zwp_xwayland_keyboard_grab_v1 {
    //! interface for grabbing the keyboard
    //!
    //! A global interface used for grabbing the keyboard.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the grabbed keyboard object
        ///
        /// Destroy the grabbed keyboard object. If applicable, the compositor
        /// will ungrab the keyboard.
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


    pub struct ZwpXwaylandKeyboardGrabV1;

    impl Interface for ZwpXwaylandKeyboardGrabV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_xwayland_keyboard_grab_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_xwayland_keyboard_grab_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// destroy the grabbed keyboard object
        ///
        /// Destroy the grabbed keyboard object. If applicable, the compositor
        /// will ungrab the keyboard.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpXwaylandKeyboardGrabV1> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

