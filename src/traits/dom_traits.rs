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

use serde::{Deserialize as Des, Serialize as Ser};

use traits::{TComputedStyles, TGenericEvent, TLayoutNode, TMemoryAPI, TMemoryAddrAPI, TStyleDeclarations};
use types::KnownElementName;

#[fundamental]
pub trait TDOMText: Debug + Ord + Clone + AsRef<str> + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TDOMTree: Debug + PartialEq {
    type Node: TDOMNode;

    fn get_node(&self, <Self::Node as TDOMNode>::Id) -> &Self::Node;

    fn get_node_mut(&mut self, <Self::Node as TDOMNode>::Id) -> &mut Self::Node;

    fn get_node_mut_pair(&mut self, (<Self::Node as TDOMNode>::Id, <Self::Node as TDOMNode>::Id)) -> (&mut Self::Node, &mut Self::Node);
}

#[fundamental]
pub trait TDOMNode: Debug + PartialEq {
    type Id: Debug + PartialEq;
    type Data: Debug + PartialEq;
    type Event: TGenericEvent;
    type Styles: TStyleDeclarations;
    type ComputedStyles: TComputedStyles;
    type LayoutNode: TLayoutNode;

    fn data(&self) -> &Self::Data;

    fn is_void(&self) -> bool;

    fn is_shadow_host(&self) -> bool;

    fn is_text(&self) -> bool;

    fn is_normal(&self) -> bool;

    fn is_known(&self, KnownElementName) -> bool;

    fn computed_styles(&self) -> &Self::ComputedStyles;

    fn layout_node(&self) -> &Self::LayoutNode;

    fn reflow_subtree(&mut self, u32, u32, <Self::LayoutNode as TLayoutNode>::ReflowDirection);

    fn set_computed_client_position(&mut self, <Self::LayoutNode as TLayoutNode>::ClientPosition);

    fn get_local_bounding_client_rect(&self) -> <Self::LayoutNode as TLayoutNode>::BoundingClientRect;

    fn get_global_bounding_client_rect(&self) -> <Self::LayoutNode as TLayoutNode>::BoundingClientRect;

    fn get_measured_image(&self) -> &<Self::LayoutNode as TLayoutNode>::MeasuredImage;

    fn get_shaped_text(&self) -> &<Self::LayoutNode as TLayoutNode>::ShapedText;
}

impl<T> TMemoryAPI for T
where
    T: TDOMNode
{
}

impl<T> TMemoryAddrAPI for T
where
    T: TDOMNode
{
}
