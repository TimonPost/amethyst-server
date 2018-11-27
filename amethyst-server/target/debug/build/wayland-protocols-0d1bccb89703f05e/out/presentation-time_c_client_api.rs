
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2013-2014 Collabora, Ltd.

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

pub mod wp_presentation {
    //! timed presentation related wl_surface requests
    //!
    //! The main feature of this interface is accurate presentation
    //! timing feedback to ensure smooth video playback while maintaining
    //! audio/video synchronization. Some features use the concept of a
    //! presentation clock, which is defined in the
    //! presentation.clock_id event.
    //! 
    //! A content update for a wl_surface is submitted by a
    //! wl_surface.commit request. Request 'feedback' associates with
    //! the wl_surface.commit and provides feedback on the content
    //! update, particularly the final realized presentation time.
    //! 
    //! 
    //! 
    //! When the final realized presentation time is available, e.g.
    //! after a framebuffer flip completes, the requested
    //! presentation_feedback.presented events are sent. The final
    //! presentation time can differ from the compositor's predicted
    //! display update time and the update's target time, especially
    //! when the compositor misses its target vertical blanking period.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// fatal presentation errors
    ///
    /// These fatal protocol errors may be emitted in response to
    /// illegal presentation requests.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// invalid value in tv_nsec
        InvalidTimestamp = 0,
        /// invalid flag
        InvalidFlag = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidTimestamp),
                1 => Some(Error::InvalidFlag),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// unbind from the presentation interface
        ///
        /// Informs the server that the client will no longer be using
        /// this protocol object. Existing objects created by this object
        /// are not affected.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// request presentation feedback information
        ///
        /// Request presentation feedback for the current content submission
        /// on the given surface. This creates a new presentation_feedback
        /// object, which will deliver the feedback information once. If
        /// multiple presentation_feedback objects are created for the same
        /// submission, they will all deliver the same information.
        /// 
        /// For details on what information is returned, see the
        /// presentation_feedback interface.
        Feedback {surface: Proxy<super::wl_surface::WlSurface>, callback: Proxy<super::wp_presentation_feedback::WpPresentationFeedback>, },
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
                name: "feedback",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::NewId,
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
                Request::Feedback { .. } => 1,
            }
        }

        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<super::wp_presentation_feedback::WpPresentationFeedback>(version, meta.child())),
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
                Request::Feedback { surface, callback, } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::Object(surface.id()),
                        Argument::NewId(callback.id()),
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
                Request::Feedback { surface, callback, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.c_ptr() as *mut _;
                    _args_array[1].o = callback.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// clock ID for timestamps
        ///
        /// This event tells the client in which clock domain the
        /// compositor interprets the timestamps used by the presentation
        /// extension. This clock is called the presentation clock.
        /// 
        /// The compositor sends this event when the client binds to the
        /// presentation interface. The presentation clock does not change
        /// during the lifetime of the client connection.
        /// 
        /// The clock identifier is platform dependent. On Linux/glibc,
        /// the identifier value is one of the clockid_t values accepted
        /// by clock_gettime(). clock_gettime() is defined by
        /// POSIX.1-2001.
        /// 
        /// Timestamps in this clock domain are expressed as tv_sec_hi,
        /// tv_sec_lo, tv_nsec triples, each component being an unsigned
        /// 32-bit value. Whole seconds are in tv_sec which is a 64-bit
        /// value combined from tv_sec_hi and tv_sec_lo, and the
        /// additional fractional part in tv_nsec as nanoseconds. Hence,
        /// for valid timestamps tv_nsec must be in [0, 999999999].
        /// 
        /// Note that clock_id applies only to the presentation clock,
        /// and implies nothing about e.g. the timestamps used in the
        /// Wayland core protocol input events.
        /// 
        /// Compositors should prefer a clock which does not jump and is
        /// not slewed e.g. by NTP. The absolute value of the clock is
        /// irrelevant. Precision of one millisecond or better is
        /// recommended. Clients must be able to query the current clock
        /// value directly, not by asking the compositor.
        ClockId {clk_id: u32, },
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "clock_id",
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
                Event::ClockId { .. } => 0,
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
                    Ok(Event::ClockId {
                        clk_id: {
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
                    Ok(Event::ClockId {
                        clk_id: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WpPresentation;

    impl Interface for WpPresentation {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wp_presentation";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wp_presentation_interface }
        }

    }
    pub trait RequestsTrait {
        /// unbind from the presentation interface
        ///
        /// Informs the server that the client will no longer be using
        /// this protocol object. Existing objects created by this object
        /// are not affected.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// request presentation feedback information
        ///
        /// Request presentation feedback for the current content submission
        /// on the given surface. This creates a new presentation_feedback
        /// object, which will deliver the feedback information once. If
        /// multiple presentation_feedback objects are created for the same
        /// submission, they will all deliver the same information.
        /// 
        /// For details on what information is returned, see the
        /// presentation_feedback interface.
        fn feedback<F>(&self, surface: &Proxy<super::wl_surface::WlSurface>, implementor: F) ->Result<Proxy<super::wp_presentation_feedback::WpPresentationFeedback>, ()>
            where F: FnOnce(NewProxy<super::wp_presentation_feedback::WpPresentationFeedback>) -> Proxy<super::wp_presentation_feedback::WpPresentationFeedback>;
    }

    impl RequestsTrait for Proxy<WpPresentation> {
        fn destroy(&self) ->()
        {
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn feedback<F>(&self, surface: &Proxy<super::wl_surface::WlSurface>, implementor: F) ->Result<Proxy<super::wp_presentation_feedback::WpPresentationFeedback>, ()>
            where F: FnOnce(NewProxy<super::wp_presentation_feedback::WpPresentationFeedback>) -> Proxy<super::wp_presentation_feedback::WpPresentationFeedback>
        {
            let msg = Request::Feedback {
                surface: surface.clone(),
                callback: self.child_placeholder(),
            };
            self.send_constructor(msg, implementor, None)
        }

    }
}

pub mod wp_presentation_feedback {
    //! presentation time feedback event
    //!
    //! A presentation_feedback object returns an indication that a
    //! wl_surface content update has become visible to the user.
    //! One object corresponds to one content update submission
    //! (wl_surface.commit). There are two possible outcomes: the
    //! content update is presented to the user, and a presentation
    //! timestamp delivered; or, the user did not see the content
    //! update because it was superseded or its surface destroyed,
    //! and the content update is discarded.
    //! 
    //! Once a presentation_feedback object has delivered a 'presented'
    //! or 'discarded' event it is automatically destroyed.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup, MessageDesc, ArgumentType, Object, Message, Argument, ObjectMetadata};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// bitmask of flags in presented event
    ///
    /// These flags provide information about how the presentation of
    /// the related content update was done. The intent is to help
    /// clients assess the reliability of the feedback and the visual
    /// quality with respect to possible tearing and timings. The
    /// flags are:
    /// 
    /// VSYNC:
    /// The presentation was synchronized to the "vertical retrace" by
    /// the display hardware such that tearing does not happen.
    /// Relying on user space scheduling is not acceptable for this
    /// flag. If presentation is done by a copy to the active
    /// frontbuffer, then it must guarantee that tearing cannot
    /// happen.
    /// 
    /// HW_CLOCK:
    /// The display hardware provided measurements that the hardware
    /// driver converted into a presentation timestamp. Sampling a
    /// clock in user space is not acceptable for this flag.
    /// 
    /// HW_COMPLETION:
    /// The display hardware signalled that it started using the new
    /// image content. The opposite of this is e.g. a timer being used
    /// to guess when the display hardware has switched to the new
    /// image content.
    /// 
    /// ZERO_COPY:
    /// The presentation of this update was done zero-copy. This means
    /// the buffer from the client was given to display hardware as
    /// is, without copying it. Compositing with OpenGL counts as
    /// copying, even if textured directly from the client buffer.
    /// Possible zero-copy cases include direct scanout of a
    /// fullscreen surface and a surface on a hardware overlay.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Kind {
        /// presentation was vsync'd
        Vsync = 0x1,
        /// hardware provided the presentation timestamp
        HwClock = 0x2,
        /// hardware signalled the start of the presentation
        HwCompletion = 0x4,
        /// presentation was done zero-copy
        ZeroCopy = 0x8,
    }
    impl Kind {
        pub fn from_raw(n: u32) -> Option<Kind> {
            match n {
                0x1 => Some(Kind::Vsync),
                0x2 => Some(Kind::HwClock),
                0x4 => Some(Kind::HwCompletion),
                0x8 => Some(Kind::ZeroCopy),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
    }

    impl super::MessageGroup for Request {
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
            panic!("Request::from_raw can not be used Client-side.")
        }

        fn into_raw(self, sender_id: u32) -> Message {
            match self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
            }
        }
    }

    pub enum Event {
        /// presentation synchronized to this output
        ///
        /// As presentation can be synchronized to only one output at a
        /// time, this event tells which output it was. This event is only
        /// sent prior to the presented event.
        /// 
        /// As clients may bind to the same global wl_output multiple
        /// times, this event is sent for each bound instance that matches
        /// the synchronized output. If a client has not bound to the
        /// right wl_output global at all, this event is not sent.
        SyncOutput {output: Proxy<super::wl_output::WlOutput>, },
        /// the content update was displayed
        ///
        /// The associated content update was displayed to the user at the
        /// indicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of
        /// the timestamp, see presentation.clock_id event.
        /// 
        /// The timestamp corresponds to the time when the content update
        /// turned into light the first time on the surface's main output.
        /// Compositors may approximate this from the framebuffer flip
        /// completion events from the system, and the latency of the
        /// physical display path if known.
        /// 
        /// This event is preceded by all related sync_output events
        /// telling which output's refresh cycle the feedback corresponds
        /// to, i.e. the main output for the surface. Compositors are
        /// recommended to choose the output containing the largest part
        /// of the wl_surface, or keeping the output they previously
        /// chose. Having a stable presentation output association helps
        /// clients predict future output refreshes (vblank).
        /// 
        /// The 'refresh' argument gives the compositor's prediction of how
        /// many nanoseconds after tv_sec, tv_nsec the very next output
        /// refresh may occur. This is to further aid clients in
        /// predicting future refreshes, i.e., estimating the timestamps
        /// targeting the next few vblanks. If such prediction cannot
        /// usefully be done, the argument is zero.
        /// 
        /// If the output does not have a constant refresh rate, explicit
        /// video mode switches excluded, then the refresh argument must
        /// be zero.
        /// 
        /// The 64-bit value combined from seq_hi and seq_lo is the value
        /// of the output's vertical retrace counter when the content
        /// update was first scanned out to the display. This value must
        /// be compatible with the definition of MSC in
        /// GLX_OML_sync_control specification. Note, that if the display
        /// path has a non-zero latency, the time instant specified by
        /// this counter may differ from the timestamp's.
        /// 
        /// If the output does not have a concept of vertical retrace or a
        /// refresh cycle, or the output device is self-refreshing without
        /// a way to query the refresh count, then the arguments seq_hi
        /// and seq_lo must be zero.
        Presented {tv_sec_hi: u32, tv_sec_lo: u32, tv_nsec: u32, refresh: u32, seq_hi: u32, seq_lo: u32, flags: u32, },
        /// the content update was not displayed
        ///
        /// The content update was never displayed to the user.
        Discarded,
    }

    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "sync_output",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                ]
            },
            super::MessageDesc {
                name: "presented",
                since: 1,
                signature: &[
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
                name: "discarded",
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
                Event::SyncOutput { .. } => 0,
                Event::Presented { .. } => 1,
                Event::Discarded => 2,
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
                    Ok(Event::SyncOutput {
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Presented {
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
                        refresh: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        seq_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        seq_lo: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                        flags: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(())
                            }
                        },
                    })
                },
                2 => {
                    Ok(Event::Discarded)
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
                    Ok(Event::SyncOutput {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(_args[0].o as *mut _),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 7);
                    Ok(Event::Presented {
                        tv_sec_hi: _args[0].u,
                        tv_sec_lo: _args[1].u,
                        tv_nsec: _args[2].u,
                        refresh: _args[3].u,
                        seq_hi: _args[4].u,
                        seq_lo: _args[5].u,
                        flags: _args[6].u,
                }) },
                2 => {
                    Ok(Event::Discarded) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WpPresentationFeedback;

    impl Interface for WpPresentationFeedback {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wp_presentation_feedback";
        const VERSION: u32 = 1;


        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wp_presentation_feedback_interface }
        }

    }
    pub trait RequestsTrait {
    }

    impl RequestsTrait for Proxy<WpPresentationFeedback> {
    }
}

