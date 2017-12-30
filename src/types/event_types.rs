/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use std::cell::RefCell;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use self_tokenize_macro::SelfTokenize;
use self_tokenize_trait::ToCustomTokens;

use traits::TCallback;
use types::EventType;

pub type VirtualEventTypeId = u8;
pub type HeadlessClosureId = u64;

pub const VIRTUAL_KEY_DOWN: u8 = VirtualEventType::KeyDown as u8;
pub const VIRTUAL_KEY_PRESS: u8 = VirtualEventType::KeyPress as u8;
pub const VIRTUAL_KEY_UP: u8 = VirtualEventType::KeyUp as u8;
pub const VIRTUAL_MOUSE_MOVE: u8 = VirtualEventType::MouseMove as u8;
pub const VIRTUAL_MOUSE_DOWN: u8 = VirtualEventType::MouseDown as u8;
pub const VIRTUAL_MOUSE_UP: u8 = VirtualEventType::MouseUp as u8;
pub const VIRTUAL_AUX_CLICK: u8 = VirtualEventType::AuxClick as u8;
pub const VIRTUAL_CLICK: u8 = VirtualEventType::Click as u8;
pub const VIRTUAL_DBL_CLICK: u8 = VirtualEventType::DblClick as u8;
pub const VIRTUAL_CONTEXT_MENU: u8 = VirtualEventType::ContextMenu as u8;
pub const VIRTUAL_WHEEL: u8 = VirtualEventType::Wheel as u8;
pub const VIRTUAL_SELECT: u8 = VirtualEventType::Select as u8;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Primitive, SelfTokenize)]
#[repr(u8)]
pub enum VirtualEventType {
    // Keyboard events
    KeyDown = 1,
    KeyPress = 2,
    KeyUp = 3,

    // Mouse events
    MouseMove = 11,
    MouseDown = 12,
    MouseUp = 13,
    AuxClick = 14,
    Click = 15,
    DblClick = 16,
    ContextMenu = 17,
    Wheel = 18,
    Select = 19
}

impl PartialEq<EventType> for VirtualEventType {
    fn eq(&self, other: &EventType) -> bool {
        *self as isize == *other as isize
    }
}

impl PartialEq<VirtualEventType> for EventType {
    fn eq(&self, other: &VirtualEventType) -> bool {
        other == self
    }
}

pub enum Closure<A> {
    Headless { f: HeadlessClosureId },
    Fn { f: Rc<Fn(A)> },
    FnMut { f: Rc<RefCell<FnMut(A)>> }
}

impl<A> Closure<A> {
    pub fn headless(f: HeadlessClosureId) -> Self {
        Closure::Headless { f }
    }

    pub fn new<T: 'static>(closure: T) -> Self
    where
        T: Fn(A)
    {
        Closure::Fn {
            f: Rc::new(closure)
        }
    }

    pub fn new_mut<T: 'static>(closure: T) -> Self
    where
        T: FnMut(A)
    {
        Closure::FnMut {
            f: Rc::new(RefCell::new(closure))
        }
    }
}

impl<A> From<()> for Closure<A> {
    fn from(_: ()) -> Self {
        Closure::headless(0)
    }
}

impl<A> From<HeadlessClosureId> for Closure<A> {
    fn from(value: HeadlessClosureId) -> Self {
        Closure::headless(value)
    }
}

impl<A, T: 'static> From<T> for Closure<A>
where
    T: Fn(A)
{
    fn from(value: T) -> Self {
        Closure::new(value)
    }
}

default impl<A, T: 'static> From<T> for Closure<A>
where
    T: FnMut(A)
{
    fn from(value: T) -> Self {
        Closure::new_mut(value)
    }
}

impl<A> TCallback<A> for Closure<A> {
    type InvokeOk = ();
    type InvokeErr = A;

    fn call(&self, args: A) -> Result<Self::InvokeOk, Self::InvokeErr> {
        match self {
            &Closure::Headless { .. } => Err(args),
            &Closure::Fn { ref f } => Ok(f.deref()(args)),
            &Closure::FnMut { ref f } => Ok(f.borrow_mut().deref_mut()(args))
        }
    }

    fn call_mut(&mut self, args: A) -> Result<Self::InvokeOk, Self::InvokeErr> {
        self.call(args)
    }

    fn call_once(self, args: A) -> Result<Self::InvokeOk, Self::InvokeErr> {
        self.call(args)
    }
}

impl<A> fmt::Debug for Closure<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Closure::Headless { f: data } => write!(f, "Closure::Headless {{ f: {:?} }}", data),
            &Closure::Fn { .. } => write!(f, "Closure::Fn {{}}"),
            &Closure::FnMut { .. } => write!(f, "Closure::FnMut {{}}")
        }
    }
}

impl<A> PartialEq for Closure<A> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Closure::Headless { f: this }, &Closure::Headless { f: that }) => this == that,
            (&Closure::Fn { f: ref this }, &Closure::Fn { f: ref that }) => Rc::ptr_eq(this, that),
            (&Closure::FnMut { f: ref this }, &Closure::FnMut { f: ref that }) => Rc::ptr_eq(this, that),
            _ => false
        }
    }
}

impl<A> Clone for Closure<A> {
    fn clone(&self) -> Self {
        match self {
            &Closure::Headless { f } => Closure::Headless { f },
            &Closure::Fn { ref f } => Closure::Fn { f: Rc::clone(f) },
            &Closure::FnMut { ref f } => Closure::FnMut { f: Rc::clone(f) }
        }
    }
}
