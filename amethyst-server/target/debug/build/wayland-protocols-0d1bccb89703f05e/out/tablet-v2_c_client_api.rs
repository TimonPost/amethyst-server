
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright 2014 © Stephen "Lyude" Chandler Paul
    Copyright 2015-2016 © Red Hat, Inc.

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation files
    (the "Software"), to deal in the Software without restriction,
    including without limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of the Software,
    and to permit persons to whom the Software is furnished to do so,
    subject to the following conditions:

    The above copyright notice and this permission notice (including the
    next paragraph) shall be included in all copies or substantial
    portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
    BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
    ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
    CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

pub mod zwp_tablet_manager_v2 {
    //! controller object for graphic tablet devices
    //!
    //! An object that provides access to the graphics tablets available on this
    //! system. All tablets are associated with a seat, to get access to the
    //! actual tablets, use wp_tablet_manager.get_tablet_seat.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// get the tablet seat
        ///
        /// Get the wp_tablet_seat object for the given seat. This object
        /// provides access to all graphics tablets in this seat.
        GetTabletSeat {tablet_seat: Proxy<super::zwp_tablet_seat_v2::ZwpTabletSeatV2>, seat: Proxy<super::wl_seat::WlSeat>, },
        /// release the memory for the tablet manager object
        ///
        /// Destroy the wp_tablet_manager object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "get_tablet_seat",
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
                Request::GetTabletSeat { .. } => 0,
                Request::Destroy => 1,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::zwp_tablet_seat_v2::ZwpTabletSeatV2>(version, meta.child())),
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }

        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetTabletSeat { tablet_seat, seat, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::NewId(tablet_seat.id()),
                        Argument::Object(seat.id()),
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
                Request::GetTabletSeat { tablet_seat, seat, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = tablet_seat.c_ptr() as *mut _;
                    _args_array[1].o = seat.c_ptr() as *mut _;
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


    pub struct ZwpTabletManagerV2;

    impl Interface for ZwpTabletManagerV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_manager_v2";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_manager_v2_interface }
        }

    }
    pub trait RequestsTrait {
        /// get the tablet seat
        ///
        /// Get the wp_tablet_seat object for the given seat. This object
        /// provides access to all graphics tablets in this seat.
        fn get_tablet_seat<F>(&self, seat: &Proxy<super::wl_seat::WlSeat>, implementor: F) ->Result<Proxy<super::zwp_tablet_seat_v2::ZwpTabletSeatV2>, ()>
            where F: FnOnce(NewProxy<super::zwp_tablet_seat_v2::ZwpTabletSeatV2>) -> Proxy<super::zwp_tablet_seat_v2::ZwpTabletSeatV2>;
        /// release the memory for the tablet manager object
        ///
        /// Destroy the wp_tablet_manager object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletManagerV2> {
        fn get_tablet_seat<F>(&self, seat: &Proxy<super::wl_seat::WlSeat>, implementor: F) ->Result<Proxy<super::zwp_tablet_seat_v2::ZwpTabletSeatV2>, ()>
            where F: FnOnce(NewProxy<super::zwp_tablet_seat_v2::ZwpTabletSeatV2>) -> Proxy<super::zwp_tablet_seat_v2::ZwpTabletSeatV2>
        {
            let msg = Request::GetTabletSeat {
                tablet_seat: self.child_placeholder(),
                seat: seat.clone(),
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

pub mod zwp_tablet_seat_v2 {
    //! controller object for graphic tablet devices of a seat
    //!
    //! An object that provides access to the graphics tablets available on this
    //! seat. After binding to this interface, the compositor sends a set of
    //! wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// release the memory for the tablet seat object
        ///
        /// Destroy the wp_tablet_seat object. Objects created from this
        /// object are unaffected and should be destroyed separately.
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
        /// new device notification
        ///
        /// This event is sent whenever a new tablet becomes available on this
        /// seat. This event only provides the object id of the tablet, any
        /// static information about the tablet (device name, vid/pid, etc.) is
        /// sent through the wp_tablet interface.
        TabletAdded {id: NewProxy<super::zwp_tablet_v2::ZwpTabletV2>, },
        /// a new tool has been used with a tablet
        ///
        /// This event is sent whenever a tool that has not previously been used
        /// with a tablet comes into use. This event only provides the object id
        /// of the tool; any static information about the tool (capabilities,
        /// type, etc.) is sent through the wp_tablet_tool interface.
        ToolAdded {id: NewProxy<super::zwp_tablet_tool_v2::ZwpTabletToolV2>, },
        /// new pad notification
        ///
        /// This event is sent whenever a new pad is known to the system. Typically,
        /// pads are physically attached to tablets and a pad_added event is
        /// sent immediately after the wp_tablet_seat.tablet_added.
        /// However, some standalone pad devices logically attach to tablets at
        /// runtime, and the client must wait for wp_tablet_pad.enter to know
        /// the tablet a pad is attached to.
        /// 
        /// This event only provides the object id of the pad. All further
        /// features (buttons, strips, rings) are sent through the wp_tablet_pad
        /// interface.
        PadAdded {id: NewProxy<super::zwp_tablet_pad_v2::ZwpTabletPadV2>, },
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "tablet_added",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                ]
            },
            super::MessageDesc {
                name: "tool_added",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                ]
            },
            super::MessageDesc {
                name: "pad_added",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
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
                Event::TabletAdded { .. } => 0,
                Event::ToolAdded { .. } => 1,
                Event::PadAdded { .. } => 2,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::zwp_tablet_v2::ZwpTabletV2>(version, meta.child())),
                1 => Some(Object::from_interface::<super::zwp_tablet_tool_v2::ZwpTabletToolV2>(version, meta.child())),
                2 => Some(Object::from_interface::<super::zwp_tablet_pad_v2::ZwpTabletPadV2>(version, meta.child())),
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::TabletAdded {
                        id: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::ToolAdded {
                        id: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::PadAdded {
                        id: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
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
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::TabletAdded {
                        id: NewProxy::<super::zwp_tablet_v2::ZwpTabletV2>::from_c_ptr(_args[0].o as *mut _),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::ToolAdded {
                        id: NewProxy::<super::zwp_tablet_tool_v2::ZwpTabletToolV2>::from_c_ptr(_args[0].o as *mut _),
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::PadAdded {
                        id: NewProxy::<super::zwp_tablet_pad_v2::ZwpTabletPadV2>::from_c_ptr(_args[0].o as *mut _),
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletSeatV2;

    impl Interface for ZwpTabletSeatV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_seat_v2";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_seat_v2_interface }
        }

    }
    pub trait RequestsTrait {
        /// release the memory for the tablet seat object
        ///
        /// Destroy the wp_tablet_seat object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletSeatV2> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zwp_tablet_tool_v2 {
    //! a physical tablet tool
    //!
    //! An object that represents a physical tool that has been, or is
    //! currently in use with a tablet in this seat. Each wp_tablet_tool
    //! object stays valid until the client destroys it; the compositor
    //! reuses the wp_tablet_tool object to indicate that the object's
    //! respective physical tool has come into proximity of a tablet again.
    //! 
    //! A wp_tablet_tool object's relation to a physical tool depends on the
    //! tablet's ability to report serial numbers. If the tablet supports
    //! this capability, then the object represents a specific physical tool
    //! and can be identified even when used on multiple tablets.
    //! 
    //! A tablet tool has a number of static characteristics, e.g. tool type,
    //! hardware_serial and capabilities. These capabilities are sent in an
    //! event sequence after the wp_tablet_seat.tool_added event before any
    //! actual events from this tool. This initial event sequence is
    //! terminated by a wp_tablet_tool.done event.
    //! 
    //! Tablet tool events are grouped by wp_tablet_tool.frame events.
    //! Any events received before a wp_tablet_tool.frame event should be
    //! considered part of the same hardware state change.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// a physical tool type
    ///
    /// Describes the physical type of a tool. The physical type of a tool
    /// generally defines its base usage.
    /// 
    /// The mouse tool represents a mouse-shaped tool that is not a relative
    /// device but bound to the tablet's surface, providing absolute
    /// coordinates.
    /// 
    /// The lens tool is a mouse-shaped tool with an attached lens to
    /// provide precision focus.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Type {
        /// Pen
        Pen = 0x140,
        /// Eraser
        Eraser = 0x141,
        /// Brush
        Brush = 0x142,
        /// Pencil
        Pencil = 0x143,
        /// Airbrush
        Airbrush = 0x144,
        /// Finger
        Finger = 0x145,
        /// Mouse
        Mouse = 0x146,
        /// Lens
        Lens = 0x147,
    }
    impl Type {
        pub fn from_raw(n: u32) -> Option<Type> {
            match n {
                0x140 => Some(Type::Pen),
                0x141 => Some(Type::Eraser),
                0x142 => Some(Type::Brush),
                0x143 => Some(Type::Pencil),
                0x144 => Some(Type::Airbrush),
                0x145 => Some(Type::Finger),
                0x146 => Some(Type::Mouse),
                0x147 => Some(Type::Lens),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// capability flags for a tool
    ///
    /// Describes extra capabilities on a tablet.
    /// 
    /// Any tool must provide x and y values, extra axes are
    /// device-specific.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Capability {
        /// Tilt axes
        Tilt = 1,
        /// Pressure axis
        Pressure = 2,
        /// Distance axis
        Distance = 3,
        /// Z-rotation axis
        Rotation = 4,
        /// Slider axis
        Slider = 5,
        /// Wheel axis
        Wheel = 6,
    }
    impl Capability {
        pub fn from_raw(n: u32) -> Option<Capability> {
            match n {
                1 => Some(Capability::Tilt),
                2 => Some(Capability::Pressure),
                3 => Some(Capability::Distance),
                4 => Some(Capability::Rotation),
                5 => Some(Capability::Slider),
                6 => Some(Capability::Wheel),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// physical button state
    ///
    /// Describes the physical state of a button that produced the button event.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ButtonState {
        /// button is not pressed
        Released = 0,
        /// button is pressed
        Pressed = 1,
    }
    impl ButtonState {
        pub fn from_raw(n: u32) -> Option<ButtonState> {
            match n {
                0 => Some(ButtonState::Released),
                1 => Some(ButtonState::Pressed),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }


    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// given wl_surface has another role
        Role = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// set the tablet tool's surface
        ///
        /// Sets the surface of the cursor used for this tool on the given
        /// tablet. This request only takes effect if the tool is in proximity
        /// of one of the requesting client's surfaces or the surface parameter
        /// is the current pointer surface. If there was a previous surface set
        /// with this request it is replaced. If surface is NULL, the cursor
        /// image is hidden.
        /// 
        /// The parameters hotspot_x and hotspot_y define the position of the
        /// pointer surface relative to the pointer location. Its top-left corner
        /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
        /// coordinates of the pointer location, in surface-local coordinates.
        /// 
        /// On surface.attach requests to the pointer surface, hotspot_x and
        /// hotspot_y are decremented by the x and y parameters passed to the
        /// request. Attach must be confirmed by wl_surface.commit as usual.
        /// 
        /// The hotspot can also be updated by passing the currently set pointer
        /// surface to this request with new values for hotspot_x and hotspot_y.
        /// 
        /// The current and pending input regions of the wl_surface are cleared,
        /// and wl_surface.set_input_region is ignored until the wl_surface is no
        /// longer used as the cursor. When the use as a cursor ends, the current
        /// and pending input regions become undefined, and the wl_surface is
        /// unmapped.
        /// 
        /// This request gives the surface the role of a wp_tablet_tool cursor. A
        /// surface may only ever be used as the cursor surface for one
        /// wp_tablet_tool. If the surface already has another role or has
        /// previously been used as cursor surface for a different tool, a
        /// protocol error is raised.
        SetCursor {serial: u32, surface: Option<Proxy<super::wl_surface::WlSurface>>, hotspot_x: i32, hotspot_y: i32, },
        /// destroy the tool object
        ///
        /// This destroys the client's resource for this tool object.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_cursor",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
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
                Request::SetCursor { .. } => 0,
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
                Request::SetCursor { serial, surface, hotspot_x, hotspot_y, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::Uint(serial),
                        Argument::Object(surface.map(|o| o.id()).unwrap_or(0)),
                        Argument::Int(hotspot_x),
                        Argument::Int(hotspot_y),
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
                Request::SetCursor { serial, surface, hotspot_x, hotspot_y, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].o = surface.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[2].i = hotspot_x;
                    _args_array[3].i = hotspot_y;
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
        /// tool type
        ///
        /// The tool type is the high-level type of the tool and usually decides
        /// the interaction expected from this tool.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        Type {tool_type: Type, },
        /// unique hardware serial number of the tool
        ///
        /// If the physical tool can be identified by a unique 64-bit serial
        /// number, this event notifies the client of this serial number.
        /// 
        /// If multiple tablets are available in the same seat and the tool is
        /// uniquely identifiable by the serial number, that tool may move
        /// between tablets.
        /// 
        /// Otherwise, if the tool has no serial number and this event is
        /// missing, the tool is tied to the tablet it first comes into
        /// proximity with. Even if the physical tool is used on multiple
        /// tablets, separate wp_tablet_tool objects will be created, one per
        /// tablet.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        HardwareSerial {hardware_serial_hi: u32, hardware_serial_lo: u32, },
        /// hardware id notification in Wacom's format
        ///
        /// This event notifies the client of a hardware id available on this tool.
        /// 
        /// The hardware id is a device-specific 64-bit id that provides extra
        /// information about the tool in use, beyond the wl_tool.type
        /// enumeration. The format of the id is specific to tablets made by
        /// Wacom Inc. For example, the hardware id of a Wacom Grip
        /// Pen (a stylus) is 0x802.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        HardwareIdWacom {hardware_id_hi: u32, hardware_id_lo: u32, },
        /// tool capability notification
        ///
        /// This event notifies the client of any capabilities of this tool,
        /// beyond the main set of x/y axes and tip up/down detection.
        /// 
        /// One event is sent for each extra capability available on this tool.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        Capability {capability: Capability, },
        /// tool description events sequence complete
        ///
        /// This event signals the end of the initial burst of descriptive
        /// events. A client may consider the static description of the tool to
        /// be complete and finalize initialization of the tool.
        Done,
        /// tool removed
        ///
        /// This event is sent when the tool is removed from the system and will
        /// send no further events. Should the physical tool come back into
        /// proximity later, a new wp_tablet_tool object will be created.
        /// 
        /// It is compositor-dependent when a tool is removed. A compositor may
        /// remove a tool on proximity out, tablet removal or any other reason.
        /// A compositor may also keep a tool alive until shutdown.
        /// 
        /// If the tool is currently in proximity, a proximity_out event will be
        /// sent before the removed event. See wp_tablet_tool.proximity_out for
        /// the handling of any buttons logically down.
        /// 
        /// When this event is received, the client must wp_tablet_tool.destroy
        /// the object.
        Removed,
        /// proximity in event
        ///
        /// Notification that this tool is focused on a certain surface.
        /// 
        /// This event can be received when the tool has moved from one surface to
        /// another, or when the tool has come back into proximity above the
        /// surface.
        /// 
        /// If any button is logically down when the tool comes into proximity,
        /// the respective button event is sent after the proximity_in event but
        /// within the same frame as the proximity_in event.
        ProximityIn {serial: u32, tablet: Proxy<super::zwp_tablet_v2::ZwpTabletV2>, surface: Proxy<super::wl_surface::WlSurface>, },
        /// proximity out event
        ///
        /// Notification that this tool has either left proximity, or is no
        /// longer focused on a certain surface.
        /// 
        /// When the tablet tool leaves proximity of the tablet, button release
        /// events are sent for each button that was held down at the time of
        /// leaving proximity. These events are sent before the proximity_out
        /// event but within the same wp_tablet.frame.
        /// 
        /// If the tool stays within proximity of the tablet, but the focus
        /// changes from one surface to another, a button release event may not
        /// be sent until the button is actually released or the tool leaves the
        /// proximity of the tablet.
        ProximityOut,
        /// tablet tool is making contact
        ///
        /// Sent whenever the tablet tool comes in contact with the surface of the
        /// tablet.
        /// 
        /// If the tool is already in contact with the tablet when entering the
        /// input region, the client owning said region will receive a
        /// wp_tablet.proximity_in event, followed by a wp_tablet.down
        /// event and a wp_tablet.frame event.
        /// 
        /// Note that this event describes logical contact, not physical
        /// contact. On some devices, a compositor may not consider a tool in
        /// logical contact until a minimum physical pressure threshold is
        /// exceeded.
        Down {serial: u32, },
        /// tablet tool is no longer making contact
        ///
        /// Sent whenever the tablet tool stops making contact with the surface of
        /// the tablet, or when the tablet tool moves out of the input region
        /// and the compositor grab (if any) is dismissed.
        /// 
        /// If the tablet tool moves out of the input region while in contact
        /// with the surface of the tablet and the compositor does not have an
        /// ongoing grab on the surface, the client owning said region will
        /// receive a wp_tablet.up event, followed by a wp_tablet.proximity_out
        /// event and a wp_tablet.frame event. If the compositor has an ongoing
        /// grab on this device, this event sequence is sent whenever the grab
        /// is dismissed in the future.
        /// 
        /// Note that this event describes logical contact, not physical
        /// contact. On some devices, a compositor may not consider a tool out
        /// of logical contact until physical pressure falls below a specific
        /// threshold.
        Up,
        /// motion event
        ///
        /// Sent whenever a tablet tool moves.
        Motion {x: f64, y: f64, },
        /// pressure change event
        ///
        /// Sent whenever the pressure axis on a tool changes. The value of this
        /// event is normalized to a value between 0 and 65535.
        /// 
        /// Note that pressure may be nonzero even when a tool is not in logical
        /// contact. See the down and up events for more details.
        Pressure {pressure: u32, },
        /// distance change event
        ///
        /// Sent whenever the distance axis on a tool changes. The value of this
        /// event is normalized to a value between 0 and 65535.
        /// 
        /// Note that distance may be nonzero even when a tool is not in logical
        /// contact. See the down and up events for more details.
        Distance {distance: u32, },
        /// tilt change event
        ///
        /// Sent whenever one or both of the tilt axes on a tool change. Each tilt
        /// value is in degrees, relative to the z-axis of the tablet.
        /// The angle is positive when the top of a tool tilts along the
        /// positive x or y axis.
        Tilt {tilt_x: f64, tilt_y: f64, },
        /// z-rotation change event
        ///
        /// Sent whenever the z-rotation axis on the tool changes. The
        /// rotation value is in degrees clockwise from the tool's
        /// logical neutral position.
        Rotation {degrees: f64, },
        /// Slider position change event
        ///
        /// Sent whenever the slider position on the tool changes. The
        /// value is normalized between -65535 and 65535, with 0 as the logical
        /// neutral position of the slider.
        /// 
        /// The slider is available on e.g. the Wacom Airbrush tool.
        Slider {position: i32, },
        /// Wheel delta event
        ///
        /// Sent whenever the wheel on the tool emits an event. This event
        /// contains two values for the same axis change. The degrees value is
        /// in the same orientation as the wl_pointer.vertical_scroll axis. The
        /// clicks value is in discrete logical clicks of the mouse wheel. This
        /// value may be zero if the movement of the wheel was less
        /// than one logical click.
        /// 
        /// Clients should choose either value and avoid mixing degrees and
        /// clicks. The compositor may accumulate values smaller than a logical
        /// click and emulate click events when a certain threshold is met.
        /// Thus, wl_tablet_tool.wheel events with non-zero clicks values may
        /// have different degrees values.
        Wheel {degrees: f64, clicks: i32, },
        /// button event
        ///
        /// Sent whenever a button on the tool is pressed or released.
        /// 
        /// If a button is held down when the tool moves in or out of proximity,
        /// button events are generated by the compositor. See
        /// wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for
        /// details.
        Button {serial: u32, button: u32, state: ButtonState, },
        /// frame event
        ///
        /// Marks the end of a series of axis and/or button updates from the
        /// tablet. The Wayland protocol requires axis updates to be sent
        /// sequentially, however all events within a frame should be considered
        /// one hardware event.
        Frame {time: u32, },
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "type",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "hardware_serial",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "hardware_id_wacom",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "capability",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "done",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "removed",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "proximity_in",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                ]
            },
            super::MessageDesc {
                name: "proximity_out",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "down",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "up",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "motion",
                since: 1,
                signature: &[
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ]
            },
            super::MessageDesc {
                name: "pressure",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "distance",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "tilt",
                since: 1,
                signature: &[
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Fixed,
                ]
            },
            super::MessageDesc {
                name: "rotation",
                since: 1,
                signature: &[
                    super::ArgumentType::Fixed,
                ]
            },
            super::MessageDesc {
                name: "slider",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                ]
            },
            super::MessageDesc {
                name: "wheel",
                since: 1,
                signature: &[
                    super::ArgumentType::Fixed,
                    super::ArgumentType::Int,
                ]
            },
            super::MessageDesc {
                name: "button",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "frame",
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
                Event::Type { .. } => 0,
                Event::HardwareSerial { .. } => 1,
                Event::HardwareIdWacom { .. } => 2,
                Event::Capability { .. } => 3,
                Event::Done => 4,
                Event::Removed => 5,
                Event::ProximityIn { .. } => 6,
                Event::ProximityOut => 7,
                Event::Down { .. } => 8,
                Event::Up => 9,
                Event::Motion { .. } => 10,
                Event::Pressure { .. } => 11,
                Event::Distance { .. } => 12,
                Event::Tilt { .. } => 13,
                Event::Rotation { .. } => 14,
                Event::Slider { .. } => 15,
                Event::Wheel { .. } => 16,
                Event::Button { .. } => 17,
                Event::Frame { .. } => 18,
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
                    Ok(Event::Type {
                        tool_type: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Type::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::HardwareSerial {
                        hardware_serial_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        hardware_serial_lo: {
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
                    Ok(Event::HardwareIdWacom {
                        hardware_id_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        hardware_id_lo: {
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
                    Ok(Event::Capability {
                        capability: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Capability::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                4 => {
                    Ok(Event::Done)
                },
                5 => {
                    Ok(Event::Removed)
                },
                6 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::ProximityIn {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        tablet: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                7 => {
                    Ok(Event::ProximityOut)
                },
                8 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Down {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                9 => {
                    Ok(Event::Up)
                },
                10 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Motion {
                        x: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(())
                            }
                        },
                        y: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                11 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Pressure {
                        pressure: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                12 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Distance {
                        distance: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                13 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Tilt {
                        tilt_x: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(())
                            }
                        },
                        tilt_y: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                14 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Rotation {
                        degrees: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                15 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Slider {
                        position: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                16 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Wheel {
                        degrees: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(())
                            }
                        },
                        clicks: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                17 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Button {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        button: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        state: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                ButtonState::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                18 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Frame {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
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
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Type {
                        tool_type: Type::from_raw(_args[0].u).ok_or(())?,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::HardwareSerial {
                        hardware_serial_hi: _args[0].u,
                        hardware_serial_lo: _args[1].u,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::HardwareIdWacom {
                        hardware_id_hi: _args[0].u,
                        hardware_id_lo: _args[1].u,
                }) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Capability {
                        capability: Capability::from_raw(_args[0].u).ok_or(())?,
                }) },
                4 => {
                    Ok(Event::Done) },
                5 => {
                    Ok(Event::Removed) },
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::ProximityIn {
                        serial: _args[0].u,
                        tablet: Proxy::<super::zwp_tablet_v2::ZwpTabletV2>::from_c_ptr(_args[1].o as *mut _),
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[2].o as *mut _),
                }) },
                7 => {
                    Ok(Event::ProximityOut) },
                8 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Down {
                        serial: _args[0].u,
                }) },
                9 => {
                    Ok(Event::Up) },
                10 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Motion {
                        x: (_args[0].f as f64)/256.,
                        y: (_args[1].f as f64)/256.,
                }) },
                11 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Pressure {
                        pressure: _args[0].u,
                }) },
                12 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Distance {
                        distance: _args[0].u,
                }) },
                13 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Tilt {
                        tilt_x: (_args[0].f as f64)/256.,
                        tilt_y: (_args[1].f as f64)/256.,
                }) },
                14 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Rotation {
                        degrees: (_args[0].f as f64)/256.,
                }) },
                15 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Slider {
                        position: _args[0].i,
                }) },
                16 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Wheel {
                        degrees: (_args[0].f as f64)/256.,
                        clicks: _args[1].i,
                }) },
                17 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Button {
                        serial: _args[0].u,
                        button: _args[1].u,
                        state: ButtonState::from_raw(_args[2].u).ok_or(())?,
                }) },
                18 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Frame {
                        time: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletToolV2;

    impl Interface for ZwpTabletToolV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_tool_v2";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_tool_v2_interface }
        }

    }
    pub trait RequestsTrait {
        /// set the tablet tool's surface
        ///
        /// Sets the surface of the cursor used for this tool on the given
        /// tablet. This request only takes effect if the tool is in proximity
        /// of one of the requesting client's surfaces or the surface parameter
        /// is the current pointer surface. If there was a previous surface set
        /// with this request it is replaced. If surface is NULL, the cursor
        /// image is hidden.
        /// 
        /// The parameters hotspot_x and hotspot_y define the position of the
        /// pointer surface relative to the pointer location. Its top-left corner
        /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
        /// coordinates of the pointer location, in surface-local coordinates.
        /// 
        /// On surface.attach requests to the pointer surface, hotspot_x and
        /// hotspot_y are decremented by the x and y parameters passed to the
        /// request. Attach must be confirmed by wl_surface.commit as usual.
        /// 
        /// The hotspot can also be updated by passing the currently set pointer
        /// surface to this request with new values for hotspot_x and hotspot_y.
        /// 
        /// The current and pending input regions of the wl_surface are cleared,
        /// and wl_surface.set_input_region is ignored until the wl_surface is no
        /// longer used as the cursor. When the use as a cursor ends, the current
        /// and pending input regions become undefined, and the wl_surface is
        /// unmapped.
        /// 
        /// This request gives the surface the role of a wp_tablet_tool cursor. A
        /// surface may only ever be used as the cursor surface for one
        /// wp_tablet_tool. If the surface already has another role or has
        /// previously been used as cursor surface for a different tool, a
        /// protocol error is raised.
        fn set_cursor(&self, serial: u32, surface: Option<&Proxy<super::wl_surface::WlSurface>>, hotspot_x: i32, hotspot_y: i32) ->();
        /// destroy the tool object
        ///
        /// This destroys the client's resource for this tool object.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletToolV2> {
        fn set_cursor(&self, serial: u32, surface: Option<&Proxy<super::wl_surface::WlSurface>>, hotspot_x: i32, hotspot_y: i32) ->()
        {
            let msg = Request::SetCursor {
                serial: serial,
                surface : surface.map(|o| o.clone()),
                hotspot_x: hotspot_x,
                hotspot_y: hotspot_y,
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

pub mod zwp_tablet_v2 {
    //! graphics tablet device
    //!
    //! The wp_tablet interface represents one graphics tablet device. The
    //! tablet interface itself does not generate events; all events are
    //! generated by wp_tablet_tool objects when in proximity above a tablet.
    //! 
    //! A tablet has a number of static characteristics, e.g. device name and
    //! pid/vid. These capabilities are sent in an event sequence after the
    //! wp_tablet_seat.tablet_added event. This initial event sequence is
    //! terminated by a wp_tablet.done event.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the tablet object
        ///
        /// This destroys the client's resource for this tablet object.
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
        /// tablet device name
        ///
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        Name {name: String, },
        /// tablet device USB vendor/product id
        ///
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        Id {vid: u32, pid: u32, },
        /// path to the device
        ///
        /// A system-specific device path that indicates which device is behind
        /// this wp_tablet. This information may be used to gather additional
        /// information about the device, e.g. through libwacom.
        /// 
        /// A device may have more than one device path. If so, multiple
        /// wp_tablet.path events are sent. A device may be emulated and not
        /// have a device path, and in that case this event will not be sent.
        /// 
        /// The format of the path is unspecified, it may be a device node, a
        /// sysfs path, or some other identifier. It is up to the client to
        /// identify the string provided.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        Path {path: String, },
        /// tablet description events sequence complete
        ///
        /// This event is sent immediately to signal the end of the initial
        /// burst of descriptive events. A client may consider the static
        /// description of the tablet to be complete and finalize initialization
        /// of the tablet.
        Done,
        /// tablet removed event
        ///
        /// Sent when the tablet has been removed from the system. When a tablet
        /// is removed, some tools may be removed.
        /// 
        /// When this event is received, the client must wp_tablet.destroy
        /// the object.
        Removed,
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "name",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                ]
            },
            super::MessageDesc {
                name: "id",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "path",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                ]
            },
            super::MessageDesc {
                name: "done",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "removed",
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
                Event::Name { .. } => 0,
                Event::Id { .. } => 1,
                Event::Path { .. } => 2,
                Event::Done => 3,
                Event::Removed => 4,
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
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Id {
                        vid: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        pid: {
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
                    Ok(Event::Path {
                        path: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into());
                                s
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                3 => {
                    Ok(Event::Done)
                },
                4 => {
                    Ok(Event::Removed)
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
                    Ok(Event::Name {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Id {
                        vid: _args[0].u,
                        pid: _args[1].u,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Path {
                        path: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                3 => {
                    Ok(Event::Done) },
                4 => {
                    Ok(Event::Removed) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletV2;

    impl Interface for ZwpTabletV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_v2";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_v2_interface }
        }

    }
    pub trait RequestsTrait {
        /// destroy the tablet object
        ///
        /// This destroys the client's resource for this tablet object.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletV2> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zwp_tablet_pad_ring_v2 {
    //! pad ring
    //!
    //! A circular interaction area, such as the touch ring on the Wacom Intuos
    //! Pro series tablets.
    //! 
    //! Events on a ring are logically grouped by the wl_tablet_pad_ring.frame
    //! event.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// ring axis source
    ///
    /// Describes the source types for ring events. This indicates to the
    /// client how a ring event was physically generated; a client may
    /// adjust the user interface accordingly. For example, events
    /// from a "finger" source may trigger kinetic scrolling.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Source {
        /// finger
        Finger = 1,
    }
    impl Source {
        pub fn from_raw(n: u32) -> Option<Source> {
            match n {
                1 => Some(Source::Finger),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// set compositor feedback
        ///
        /// Request that the compositor use the provided feedback string
        /// associated with this ring. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever the ring is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with the ring; compositors may use this
        /// information to offer visual feedback about the button layout
        /// (eg. on-screen displays).
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// ring. Requests providing other serials than the most recent one will be
        /// ignored.
        SetFeedback {description: String, serial: u32, },
        /// destroy the ring object
        ///
        /// This destroys the client's resource for this ring object.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_feedback",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                    super::ArgumentType::Uint,
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
                Request::SetFeedback { .. } => 0,
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
                Request::SetFeedback { description, serial, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::Str(unsafe { ::std::ffi::CString::from_vec_unchecked(description.into()) }),
                        Argument::Uint(serial),
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
                Request::SetFeedback { description, serial, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(description).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].u = serial;
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
        /// ring event source
        ///
        /// Source information for ring events.
        /// 
        /// This event does not occur on its own. It is sent before a
        /// wp_tablet_pad_ring.frame event and carries the source information
        /// for all events within that frame.
        /// 
        /// The source specifies how this event was generated. If the source is
        /// wp_tablet_pad_ring.source.finger, a wp_tablet_pad_ring.stop event
        /// will be sent when the user lifts the finger off the device.
        /// 
        /// This event is optional. If the source is unknown for an interaction,
        /// no event is sent.
        Source {source: Source, },
        /// angle changed
        ///
        /// Sent whenever the angle on a ring changes.
        /// 
        /// The angle is provided in degrees clockwise from the logical
        /// north of the ring in the pad's current rotation.
        Angle {degrees: f64, },
        /// interaction stopped
        ///
        /// Stop notification for ring events.
        /// 
        /// For some wp_tablet_pad_ring.source types, a wp_tablet_pad_ring.stop
        /// event is sent to notify a client that the interaction with the ring
        /// has terminated. This enables the client to implement kinetic scrolling.
        /// See the wp_tablet_pad_ring.source documentation for information on
        /// when this event may be generated.
        /// 
        /// Any wp_tablet_pad_ring.angle events with the same source after this
        /// event should be considered as the start of a new interaction.
        Stop,
        /// end of a ring event sequence
        ///
        /// Indicates the end of a set of ring events that logically belong
        /// together. A client is expected to accumulate the data in all events
        /// within the frame before proceeding.
        /// 
        /// All wp_tablet_pad_ring events before a wp_tablet_pad_ring.frame event belong
        /// logically together. For example, on termination of a finger interaction
        /// on a ring the compositor will send a wp_tablet_pad_ring.source event,
        /// a wp_tablet_pad_ring.stop event and a wp_tablet_pad_ring.frame event.
        /// 
        /// A wp_tablet_pad_ring.frame event is sent for every logical event
        /// group, even if the group only contains a single wp_tablet_pad_ring
        /// event. Specifically, a client may get a sequence: angle, frame,
        /// angle, frame, etc.
        Frame {time: u32, },
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "source",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "angle",
                since: 1,
                signature: &[
                    super::ArgumentType::Fixed,
                ]
            },
            super::MessageDesc {
                name: "stop",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "frame",
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
                Event::Source { .. } => 0,
                Event::Angle { .. } => 1,
                Event::Stop => 2,
                Event::Frame { .. } => 3,
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
                    Ok(Event::Source {
                        source: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Source::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Angle {
                        degrees: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                2 => {
                    Ok(Event::Stop)
                },
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Frame {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
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
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Source {
                        source: Source::from_raw(_args[0].u).ok_or(())?,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Angle {
                        degrees: (_args[0].f as f64)/256.,
                }) },
                2 => {
                    Ok(Event::Stop) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Frame {
                        time: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletPadRingV2;

    impl Interface for ZwpTabletPadRingV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_pad_ring_v2";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_pad_ring_v2_interface }
        }

    }
    pub trait RequestsTrait {
        /// set compositor feedback
        ///
        /// Request that the compositor use the provided feedback string
        /// associated with this ring. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever the ring is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with the ring; compositors may use this
        /// information to offer visual feedback about the button layout
        /// (eg. on-screen displays).
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// ring. Requests providing other serials than the most recent one will be
        /// ignored.
        fn set_feedback(&self, description: String, serial: u32) ->();
        /// destroy the ring object
        ///
        /// This destroys the client's resource for this ring object.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletPadRingV2> {
        fn set_feedback(&self, description: String, serial: u32) ->()
        {
            let msg = Request::SetFeedback {
                description: description,
                serial: serial,
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

pub mod zwp_tablet_pad_strip_v2 {
    //! pad strip
    //!
    //! A linear interaction area, such as the strips found in Wacom Cintiq
    //! models.
    //! 
    //! Events on a strip are logically grouped by the wl_tablet_pad_strip.frame
    //! event.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// strip axis source
    ///
    /// Describes the source types for strip events. This indicates to the
    /// client how a strip event was physically generated; a client may
    /// adjust the user interface accordingly. For example, events
    /// from a "finger" source may trigger kinetic scrolling.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Source {
        /// finger
        Finger = 1,
    }
    impl Source {
        pub fn from_raw(n: u32) -> Option<Source> {
            match n {
                1 => Some(Source::Finger),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// set compositor feedback
        ///
        /// Requests the compositor to use the provided feedback string
        /// associated with this strip. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever the strip is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with the strip, and compositors may use this
        /// information to offer visual feedback about the button layout
        /// (eg. on-screen displays).
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// strip. Requests providing other serials than the most recent one will be
        /// ignored.
        SetFeedback {description: String, serial: u32, },
        /// destroy the strip object
        ///
        /// This destroys the client's resource for this strip object.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_feedback",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                    super::ArgumentType::Uint,
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
                Request::SetFeedback { .. } => 0,
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
                Request::SetFeedback { description, serial, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::Str(unsafe { ::std::ffi::CString::from_vec_unchecked(description.into()) }),
                        Argument::Uint(serial),
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
                Request::SetFeedback { description, serial, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(description).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].u = serial;
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
        /// strip event source
        ///
        /// Source information for strip events.
        /// 
        /// This event does not occur on its own. It is sent before a
        /// wp_tablet_pad_strip.frame event and carries the source information
        /// for all events within that frame.
        /// 
        /// The source specifies how this event was generated. If the source is
        /// wp_tablet_pad_strip.source.finger, a wp_tablet_pad_strip.stop event
        /// will be sent when the user lifts their finger off the device.
        /// 
        /// This event is optional. If the source is unknown for an interaction,
        /// no event is sent.
        Source {source: Source, },
        /// position changed
        ///
        /// Sent whenever the position on a strip changes.
        /// 
        /// The position is normalized to a range of [0, 65535], the 0-value
        /// represents the top-most and/or left-most position of the strip in
        /// the pad's current rotation.
        Position {position: u32, },
        /// interaction stopped
        ///
        /// Stop notification for strip events.
        /// 
        /// For some wp_tablet_pad_strip.source types, a wp_tablet_pad_strip.stop
        /// event is sent to notify a client that the interaction with the strip
        /// has terminated. This enables the client to implement kinetic
        /// scrolling. See the wp_tablet_pad_strip.source documentation for
        /// information on when this event may be generated.
        /// 
        /// Any wp_tablet_pad_strip.position events with the same source after this
        /// event should be considered as the start of a new interaction.
        Stop,
        /// end of a strip event sequence
        ///
        /// Indicates the end of a set of events that represent one logical
        /// hardware strip event. A client is expected to accumulate the data
        /// in all events within the frame before proceeding.
        /// 
        /// All wp_tablet_pad_strip events before a wp_tablet_pad_strip.frame event belong
        /// logically together. For example, on termination of a finger interaction
        /// on a strip the compositor will send a wp_tablet_pad_strip.source event,
        /// a wp_tablet_pad_strip.stop event and a wp_tablet_pad_strip.frame
        /// event.
        /// 
        /// A wp_tablet_pad_strip.frame event is sent for every logical event
        /// group, even if the group only contains a single wp_tablet_pad_strip
        /// event. Specifically, a client may get a sequence: position, frame,
        /// position, frame, etc.
        Frame {time: u32, },
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "source",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "position",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "stop",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "frame",
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
                Event::Source { .. } => 0,
                Event::Position { .. } => 1,
                Event::Stop => 2,
                Event::Frame { .. } => 3,
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
                    Ok(Event::Source {
                        source: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Source::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Position {
                        position: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                2 => {
                    Ok(Event::Stop)
                },
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Frame {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
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
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Source {
                        source: Source::from_raw(_args[0].u).ok_or(())?,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Position {
                        position: _args[0].u,
                }) },
                2 => {
                    Ok(Event::Stop) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Frame {
                        time: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletPadStripV2;

    impl Interface for ZwpTabletPadStripV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_pad_strip_v2";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_pad_strip_v2_interface }
        }

    }
    pub trait RequestsTrait {
        /// set compositor feedback
        ///
        /// Requests the compositor to use the provided feedback string
        /// associated with this strip. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever the strip is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with the strip, and compositors may use this
        /// information to offer visual feedback about the button layout
        /// (eg. on-screen displays).
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// strip. Requests providing other serials than the most recent one will be
        /// ignored.
        fn set_feedback(&self, description: String, serial: u32) ->();
        /// destroy the strip object
        ///
        /// This destroys the client's resource for this strip object.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletPadStripV2> {
        fn set_feedback(&self, description: String, serial: u32) ->()
        {
            let msg = Request::SetFeedback {
                description: description,
                serial: serial,
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

pub mod zwp_tablet_pad_group_v2 {
    //! a set of buttons, rings and strips
    //!
    //! A pad group describes a distinct (sub)set of buttons, rings and strips
    //! present in the tablet. The criteria of this grouping is usually positional,
    //! eg. if a tablet has buttons on the left and right side, 2 groups will be
    //! presented. The physical arrangement of groups is undisclosed and may
    //! change on the fly.
    //! 
    //! Pad groups will announce their features during pad initialization. Between
    //! the corresponding wp_tablet_pad.group event and wp_tablet_pad_group.done, the
    //! pad group will announce the buttons, rings and strips contained in it,
    //! plus the number of supported modes.
    //! 
    //! Modes are a mechanism to allow multiple groups of actions for every element
    //! in the pad group. The number of groups and available modes in each is
    //! persistent across device plugs. The current mode is user-switchable, it
    //! will be announced through the wp_tablet_pad_group.mode_switch event both
    //! whenever it is switched, and after wp_tablet_pad.enter.
    //! 
    //! The current mode logically applies to all elements in the pad group,
    //! although it is at clients' discretion whether to actually perform different
    //! actions, and/or issue the respective .set_feedback requests to notify the
    //! compositor. See the wp_tablet_pad_group.mode_switch event for more details.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the pad object
        ///
        /// Destroy the wp_tablet_pad_group object. Objects created from this object
        /// are unaffected and should be destroyed separately.
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
        /// buttons announced
        ///
        /// Sent on wp_tablet_pad_group initialization to announce the available
        /// buttons in the group. Button indices start at 0, a button may only be
        /// in one group at a time.
        /// 
        /// This event is first sent in the initial burst of events before the
        /// wp_tablet_pad_group.done event.
        /// 
        /// Some buttons are reserved by the compositor. These buttons may not be
        /// assigned to any wp_tablet_pad_group. Compositors may broadcast this
        /// event in the case of changes to the mapping of these reserved buttons.
        /// If the compositor happens to reserve all buttons in a group, this event
        /// will be sent with an empty array.
        Buttons {buttons: Vec<u8>, },
        /// ring announced
        ///
        /// Sent on wp_tablet_pad_group initialization to announce available rings.
        /// One event is sent for each ring available on this pad group.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad_group.done event.
        Ring {ring: NewProxy<super::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2>, },
        /// strip announced
        ///
        /// Sent on wp_tablet_pad initialization to announce available strips.
        /// One event is sent for each strip available on this pad group.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad_group.done event.
        Strip {strip: NewProxy<super::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2>, },
        /// mode-switch ability announced
        ///
        /// Sent on wp_tablet_pad_group initialization to announce that the pad
        /// group may switch between modes. A client may use a mode to store a
        /// specific configuration for buttons, rings and strips and use the
        /// wl_tablet_pad_group.mode_switch event to toggle between these
        /// configurations. Mode indices start at 0.
        /// 
        /// Switching modes is compositor-dependent. See the
        /// wp_tablet_pad_group.mode_switch event for more details.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad_group.done event. This event is only sent when more than
        /// more than one mode is available.
        Modes {modes: u32, },
        /// tablet group description events sequence complete
        ///
        /// This event is sent immediately to signal the end of the initial
        /// burst of descriptive events. A client may consider the static
        /// description of the tablet to be complete and finalize initialization
        /// of the tablet group.
        Done,
        /// mode switch event
        ///
        /// Notification that the mode was switched.
        /// 
        /// A mode applies to all buttons, rings and strips in a group
        /// simultaneously, but a client is not required to assign different actions
        /// for each mode. For example, a client may have mode-specific button
        /// mappings but map the ring to vertical scrolling in all modes. Mode
        /// indices start at 0.
        /// 
        /// Switching modes is compositor-dependent. The compositor may provide
        /// visual cues to the client about the mode, e.g. by toggling LEDs on
        /// the tablet device. Mode-switching may be software-controlled or
        /// controlled by one or more physical buttons. For example, on a Wacom
        /// Intuos Pro, the button inside the ring may be assigned to switch
        /// between modes.
        /// 
        /// The compositor will also send this event after wp_tablet_pad.enter on
        /// each group in order to notify of the current mode. Groups that only
        /// feature one mode will use mode=0 when emitting this event.
        /// 
        /// If a button action in the new mode differs from the action in the
        /// previous mode, the client should immediately issue a
        /// wp_tablet_pad.set_feedback request for each changed button.
        /// 
        /// If a ring or strip action in the new mode differs from the action
        /// in the previous mode, the client should immediately issue a
        /// wp_tablet_ring.set_feedback or wp_tablet_strip.set_feedback request
        /// for each changed ring or strip.
        ModeSwitch {time: u32, serial: u32, mode: u32, },
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "buttons",
                since: 1,
                signature: &[
                    super::ArgumentType::Array,
                ]
            },
            super::MessageDesc {
                name: "ring",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                ]
            },
            super::MessageDesc {
                name: "strip",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                ]
            },
            super::MessageDesc {
                name: "modes",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "done",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "mode_switch",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
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
                Event::Buttons { .. } => 0,
                Event::Ring { .. } => 1,
                Event::Strip { .. } => 2,
                Event::Modes { .. } => 3,
                Event::Done => 4,
                Event::ModeSwitch { .. } => 5,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<super::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2>(version, meta.child())),
                2 => Some(Object::from_interface::<super::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2>(version, meta.child())),
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Buttons {
                        buttons: {
                            if let Some(Argument::Array(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Ring {
                        ring: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Strip {
                        strip: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Modes {
                        modes: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                4 => {
                    Ok(Event::Done)
                },
                5 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::ModeSwitch {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        mode: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
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
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Buttons {
                        buttons: { let array = &*_args[0].a; ::std::slice::from_raw_parts(array.data as *const u8, array.size).to_owned() },
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Ring {
                        ring: NewProxy::<super::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2>::from_c_ptr(_args[0].o as *mut _),
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Strip {
                        strip: NewProxy::<super::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2>::from_c_ptr(_args[0].o as *mut _),
                }) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Modes {
                        modes: _args[0].u,
                }) },
                4 => {
                    Ok(Event::Done) },
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::ModeSwitch {
                        time: _args[0].u,
                        serial: _args[1].u,
                        mode: _args[2].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletPadGroupV2;

    impl Interface for ZwpTabletPadGroupV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_pad_group_v2";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_pad_group_v2_interface }
        }

    }
    pub trait RequestsTrait {
        /// destroy the pad object
        ///
        /// Destroy the wp_tablet_pad_group object. Objects created from this object
        /// are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletPadGroupV2> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zwp_tablet_pad_v2 {
    //! a set of buttons, rings and strips
    //!
    //! A pad device is a set of buttons, rings and strips
    //! usually physically present on the tablet device itself. Some
    //! exceptions exist where the pad device is physically detached, e.g. the
    //! Wacom ExpressKey Remote.
    //! 
    //! Pad devices have no axes that control the cursor and are generally
    //! auxiliary devices to the tool devices used on the tablet surface.
    //! 
    //! A pad device has a number of static characteristics, e.g. the number
    //! of rings. These capabilities are sent in an event sequence after the
    //! wp_tablet_seat.pad_added event before any actual events from this pad.
    //! This initial event sequence is terminated by a wp_tablet_pad.done
    //! event.
    //! 
    //! All pad features (buttons, rings and strips) are logically divided into
    //! groups and all pads have at least one group. The available groups are
    //! notified through the wp_tablet_pad.group event; the compositor will
    //! emit one event per group before emitting wp_tablet_pad.done.
    //! 
    //! Groups may have multiple modes. Modes allow clients to map multiple
    //! actions to a single pad feature. Only one mode can be active per group,
    //! although different groups may have different active modes.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// physical button state
    ///
    /// Describes the physical state of a button that caused the button
    /// event.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ButtonState {
        /// the button is not pressed
        Released = 0,
        /// the button is pressed
        Pressed = 1,
    }
    impl ButtonState {
        pub fn from_raw(n: u32) -> Option<ButtonState> {
            match n {
                0 => Some(ButtonState::Released),
                1 => Some(ButtonState::Pressed),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// set compositor feedback
        ///
        /// Requests the compositor to use the provided feedback string
        /// associated with this button. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever a button is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with each button, and compositors may use
        /// this information to offer visual feedback on the button layout
        /// (e.g. on-screen displays).
        /// 
        /// Button indices start at 0. Setting the feedback string on a button
        /// that is reserved by the compositor (i.e. not belonging to any
        /// wp_tablet_pad_group) does not generate an error but the compositor
        /// is free to ignore the request.
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// button. Requests providing other serials than the most recent one will
        /// be ignored.
        SetFeedback {button: u32, description: String, serial: u32, },
        /// destroy the pad object
        ///
        /// Destroy the wp_tablet_pad object. Objects created from this object
        /// are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_feedback",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Str,
                    super::ArgumentType::Uint,
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
                Request::SetFeedback { .. } => 0,
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
                Request::SetFeedback { button, description, serial, } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::Uint(button),
                        Argument::Str(unsafe { ::std::ffi::CString::from_vec_unchecked(description.into()) }),
                        Argument::Uint(serial),
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
                Request::SetFeedback { button, description, serial, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = button;
                    let _arg_1 = ::std::ffi::CString::new(description).unwrap();
                    _args_array[1].s = _arg_1.as_ptr();
                    _args_array[2].u = serial;
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
        /// group announced
        ///
        /// Sent on wp_tablet_pad initialization to announce available groups.
        /// One event is sent for each pad group available.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad.done event. At least one group will be announced.
        Group {pad_group: NewProxy<super::zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2>, },
        /// path to the device
        ///
        /// A system-specific device path that indicates which device is behind
        /// this wp_tablet_pad. This information may be used to gather additional
        /// information about the device, e.g. through libwacom.
        /// 
        /// The format of the path is unspecified, it may be a device node, a
        /// sysfs path, or some other identifier. It is up to the client to
        /// identify the string provided.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad.done event.
        Path {path: String, },
        /// buttons announced
        ///
        /// Sent on wp_tablet_pad initialization to announce the available
        /// buttons.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad.done event. This event is only sent when at least one
        /// button is available.
        Buttons {buttons: u32, },
        /// pad description event sequence complete
        ///
        /// This event signals the end of the initial burst of descriptive
        /// events. A client may consider the static description of the pad to
        /// be complete and finalize initialization of the pad.
        Done,
        /// physical button state
        ///
        /// Sent whenever the physical state of a button changes.
        Button {time: u32, button: u32, state: ButtonState, },
        /// enter event
        ///
        /// Notification that this pad is focused on the specified surface.
        Enter {serial: u32, tablet: Proxy<super::zwp_tablet_v2::ZwpTabletV2>, surface: Proxy<super::wl_surface::WlSurface>, },
        /// enter event
        ///
        /// Notification that this pad is no longer focused on the specified
        /// surface.
        Leave {serial: u32, surface: Proxy<super::wl_surface::WlSurface>, },
        /// pad removed event
        ///
        /// Sent when the pad has been removed from the system. When a tablet
        /// is removed its pad(s) will be removed too.
        /// 
        /// When this event is received, the client must destroy all rings, strips
        /// and groups that were offered by this pad, and issue wp_tablet_pad.destroy
        /// the pad itself.
        Removed,
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "group",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                ]
            },
            super::MessageDesc {
                name: "path",
                since: 1,
                signature: &[
                    super::ArgumentType::Str,
                ]
            },
            super::MessageDesc {
                name: "buttons",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "done",
                since: 1,
                signature: &[
                ]
            },
            super::MessageDesc {
                name: "button",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ]
            },
            super::MessageDesc {
                name: "enter",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                ]
            },
            super::MessageDesc {
                name: "leave",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Object,
                ]
            },
            super::MessageDesc {
                name: "removed",
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
                Event::Group { .. } => 0,
                Event::Path { .. } => 1,
                Event::Buttons { .. } => 2,
                Event::Done => 3,
                Event::Button { .. } => 4,
                Event::Enter { .. } => 5,
                Event::Leave { .. } => 6,
                Event::Removed => 7,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<super::zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2>(version, meta.child())),
                _ => None
            }
        }

        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Group {
                        pad_group: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Path {
                        path: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into());
                                s
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Buttons {
                        buttons: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                3 => {
                    Ok(Event::Done)
                },
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Button {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        button: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        state: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                ButtonState::from_raw(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                5 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Enter {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        tablet: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                6 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Leave {
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        surface: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                7 => {
                    Ok(Event::Removed)
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
                    Ok(Event::Group {
                        pad_group: NewProxy::<super::zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2>::from_c_ptr(_args[0].o as *mut _),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Path {
                        path: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Buttons {
                        buttons: _args[0].u,
                }) },
                3 => {
                    Ok(Event::Done) },
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Button {
                        time: _args[0].u,
                        button: _args[1].u,
                        state: ButtonState::from_raw(_args[2].u).ok_or(())?,
                }) },
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Enter {
                        serial: _args[0].u,
                        tablet: Proxy::<super::zwp_tablet_v2::ZwpTabletV2>::from_c_ptr(_args[1].o as *mut _),
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[2].o as *mut _),
                }) },
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Leave {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[1].o as *mut _),
                }) },
                7 => {
                    Ok(Event::Removed) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletPadV2;

    impl Interface for ZwpTabletPadV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_pad_v2";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_pad_v2_interface }
        }

    }
    pub trait RequestsTrait {
        /// set compositor feedback
        ///
        /// Requests the compositor to use the provided feedback string
        /// associated with this button. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever a button is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with each button, and compositors may use
        /// this information to offer visual feedback on the button layout
        /// (e.g. on-screen displays).
        /// 
        /// Button indices start at 0. Setting the feedback string on a button
        /// that is reserved by the compositor (i.e. not belonging to any
        /// wp_tablet_pad_group) does not generate an error but the compositor
        /// is free to ignore the request.
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// button. Requests providing other serials than the most recent one will
        /// be ignored.
        fn set_feedback(&self, button: u32, description: String, serial: u32) ->();
        /// destroy the pad object
        ///
        /// Destroy the wp_tablet_pad object. Objects created from this object
        /// are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletPadV2> {
        fn set_feedback(&self, button: u32, description: String, serial: u32) ->()
        {
            let msg = Request::SetFeedback {
                button: button,
                description: description,
                serial: serial,
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

