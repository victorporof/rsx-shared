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

use std::fmt::Debug;

use traits::{TDOMNode, TDOMTree};
use types::{Closure, DOMNodeRawId, EventType, VirtualEventType};

use serde::{Deserialize as Des, Serialize as Ser};

#[fundamental]
pub trait TCallback<A>: Debug + PartialEq + Clone {
    type InvokeOk;
    type InvokeErr;

    fn call(&self, A) -> Result<Self::InvokeOk, Self::InvokeErr>;

    fn call_mut(&mut self, A) -> Result<Self::InvokeOk, Self::InvokeErr>;

    fn call_once(self, A) -> Result<Self::InvokeOk, Self::InvokeErr>;
}

#[fundamental]
// See https://developer.mozilla.org/en-US/docs/Web/API/Event
pub trait TEvent: Debug + PartialEq + Ser + for<'a> Des<'a> {
    fn target(&self) -> DOMNodeRawId;

    fn ty(&self) -> EventType;
}

#[fundamental]
pub trait TUIEvent: TEvent {
    fn alt_key(&self) -> bool;

    fn ctrl_key(&self) -> bool;

    fn meta_key(&self) -> bool;

    fn shift_key(&self) -> bool;
}

#[fundamental]
// See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent
pub trait TKeyboardEvent: TUIEvent {
    type KeyCode;

    fn code(&self) -> Self::KeyCode;

    fn key(&self) -> &'static str;

    fn get_modifier_state(&self) -> bool;

    fn repeat(&self) -> bool;
}

#[fundamental]
// See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent
pub trait TMouseEvent: TUIEvent {
    type MouseButton;

    fn button(&self) -> Self::MouseButton;

    fn client_x(&self) -> u32;

    fn client_y(&self) -> u32;

    fn offset_x(&self) -> u32;

    fn offset_y(&self) -> u32;

    fn page_x(&self) -> u32;

    fn page_y(&self) -> u32;
}

#[fundamental]
pub trait TGenericEvent: TKeyboardEvent + TMouseEvent {}

#[fundamental]
pub trait TEventManager: Debug + PartialEq + Default {
    type Target: TDOMNode;
    type KeyCode;
    type MouseButton;
    type KeyEventData;
    type MouseEventData;

    fn add_event_listener<F>(&mut self, <Self::Target as TDOMNode>::Id, EventType, F)
    where
        F: Into<Closure<<Self::Target as TDOMNode>::Event>>;

    fn remove_event_listener<F>(&mut self, <Self::Target as TDOMNode>::Id, EventType, F)
    where
        F: Into<Closure<<Self::Target as TDOMNode>::Event>>;

    fn receive_key_event(&mut self, VirtualEventType, Self::KeyEventData);

    fn receive_mouse_event(&mut self, VirtualEventType, Self::MouseEventData);

    fn broadcast_events<A>(&mut self, &A)
    where
        A: TDOMTree<Node = Self::Target>;

    fn intercept_events<A, F>(&mut self, &A, F)
    where
        A: TDOMTree<Node = Self::Target>,
        F: FnMut(<Self::Target as TDOMNode>::Event);
}
