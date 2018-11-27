
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

pub mod zwp_keyboard_shortcuts_inhibit_manager_v1 {
    //! context object for keyboard grab_manager
    //!
    //! A global interface used for inhibiting the compositor keyboard shortcuts.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// the shortcuts are already inhibited for this surface
        AlreadyInhibited = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyInhibited),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// destroy the keyboard shortcuts inhibitor object
        ///
        /// Destroy the keyboard shortcuts inhibitor manager.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// create a new keyboard shortcuts inhibitor object
        ///
        /// Create a new keyboard shortcuts inhibitor object associated with
        /// the given surface for the given seat.
        /// 
        /// If shortcuts are already inhibited for the specified seat and surface,
        /// a protocol error "already_inhibited" is raised by the compositor.
        InhibitShortcuts {id: Proxy<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1>, surface: Proxy<super::wl_surface::WlSurface>, seat: Proxy<super::wl_seat::WlSeat>, },
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
                name: "inhibit_shortcuts",
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
                Request::InhibitShortcuts { .. } => 1,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1>(version, meta.child())),
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
                Request::InhibitShortcuts { id, surface, seat, } => Message {
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
                Request::InhibitShortcuts { id, surface, seat, } => {
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


    pub struct ZwpKeyboardShortcutsInhibitManagerV1;

    impl Interface for ZwpKeyboardShortcutsInhibitManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_keyboard_shortcuts_inhibit_manager_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_keyboard_shortcuts_inhibit_manager_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// destroy the keyboard shortcuts inhibitor object
        ///
        /// Destroy the keyboard shortcuts inhibitor manager.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// create a new keyboard shortcuts inhibitor object
        ///
        /// Create a new keyboard shortcuts inhibitor object associated with
        /// the given surface for the given seat.
        /// 
        /// If shortcuts are already inhibited for the specified seat and surface,
        /// a protocol error "already_inhibited" is raised by the compositor.
        fn inhibit_shortcuts<F>(&self, surface: &Proxy<super::wl_surface::WlSurface>, seat: &Proxy<super::wl_seat::WlSeat>, implementor: F) ->Result<Proxy<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1>, ()>
            where F: FnOnce(NewProxy<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1>) -> Proxy<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1>;
    }

    impl RequestsTrait for Proxy<ZwpKeyboardShortcutsInhibitManagerV1> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn inhibit_shortcuts<F>(&self, surface: &Proxy<super::wl_surface::WlSurface>, seat: &Proxy<super::wl_seat::WlSeat>, implementor: F) ->Result<Proxy<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1>, ()>
            where F: FnOnce(NewProxy<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1>) -> Proxy<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1>
        {
            let msg = Request::InhibitShortcuts {
                id: self.child_placeholder(),
                surface: surface.clone(),
                seat: seat.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }

    }
}

pub mod zwp_keyboard_shortcuts_inhibitor_v1 {
    //! context object for keyboard shortcuts inhibitor
    //!
    //! A keyboard shortcuts inhibitor instructs the compositor to ignore
    //! its own keyboard shortcuts when the associated surface has keyboard
    //! focus. As a result, when the surface has keyboard focus on the given
    //! seat, it will receive all key events originating from the specified
    //! seat, even those which would normally be caught by the compositor for
    //! its own shortcuts.
    //! 
    //! The Wayland compositor is however under no obligation to disable
    //! all of its shortcuts, and may keep some special key combo for its own
    //! use, including but not limited to one allowing the user to forcibly
    //! restore normal keyboard events routing in the case of an unwilling
    //! client. The compositor may also use the same key combo to reactivate
    //! an existing shortcut inhibitor that was previously deactivated on
    //! user request.
    //! 
    //! When the compositor restores its own keyboard shortcuts, an
    //! "inactive" event is emitted to notify the client that the keyboard
    //! shortcuts inhibitor is not effectively active for the surface and
    //! seat any more, and the client should not expect to receive all
    //! keyboard events.
    //! 
    //! When the keyboard shortcuts inhibitor is inactive, the client has
    //! no way to forcibly reactivate the keyboard shortcuts inhibitor.
    //! 
    //! The user can chose to re-enable a previously deactivated keyboard
    //! shortcuts inhibitor using any mechanism the compositor may offer,
    //! in which case the compositor will send an "active" event to notify
    //! the client.
    //! 
    //! If the surface is destroyed, unmapped, or loses the seat's keyboard
    //! focus, the keyboard shortcuts inhibitor becomes irrelevant and the
    //! compositor will restore its own keyboard shortcuts but no "inactive"
    //! event is emitted in this case.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the keyboard shortcuts inhibitor object
        ///
        /// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
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
        /// shortcuts are inhibited
        ///
        /// This event indicates that the shortcut inhibitor is active.
        /// 
        /// The compositor sends this event every time compositor shortcuts
        /// are inhibited on behalf of the surface. When active, the client
        /// may receive input events normally reserved by the compositor
        /// (see zwp_keyboard_shortcuts_inhibitor_v1).
        /// 
        /// This occurs typically when the initial request "inhibit_shortcuts"
        /// first becomes active or when the user instructs the compositor to
        /// re-enable and existing shortcuts inhibitor using any mechanism
        /// offered by the compositor.
        Active,
        /// shortcuts are restored
        ///
        /// This event indicates that the shortcuts inhibitor is inactive,
        /// normal shortcuts processing is restored by the compositor.
        Inactive,
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "active",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "inactive",
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
                Event::Active => 0,
                Event::Inactive => 1,
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
                    Ok(Event::Active)
                },
                1 => {
                    Ok(Event::Inactive)
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
                    Ok(Event::Active) },
                1 => {
                    Ok(Event::Inactive) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpKeyboardShortcutsInhibitorV1;

    impl Interface for ZwpKeyboardShortcutsInhibitorV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_keyboard_shortcuts_inhibitor_v1";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_keyboard_shortcuts_inhibitor_v1_interface }
        }

    }
    pub trait RequestsTrait {
        /// destroy the keyboard shortcuts inhibitor object
        ///
        /// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpKeyboardShortcutsInhibitorV1> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

