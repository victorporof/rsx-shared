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

use std::convert::TryInto;
use std::fmt::Debug;

use serde::{Deserialize as Des, Serialize as Ser};

use traits::{TDOMText, TResourceGroup, TStyleDeclarations};
use types::KnownElementName;

#[fundamental]
pub trait TClientRect: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {
    type Position: TClientPosition;
    type Size: TClientSize;

    fn position(&self) -> Self::Position;

    fn size(&self) -> Self::Size;

    fn offset_from_page(&self, (u32, u32)) -> (u32, u32);

    fn client_from_page(&self, (u32, u32)) -> (u32, u32);

    fn contains_point(&self, (u32, u32)) -> bool;
}

#[fundamental]
pub trait TClientPosition: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TClientSize: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TMeasuredImage: Debug + PartialEq + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TShapedText: Debug + PartialEq + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TLayoutNode: Debug + PartialEq {
    type Styles: TStyleDeclarations;
    type Resources: TResourceGroup;
    type TextMeasureMetadata;
    type ImageMeasureMetadata;
    type NormalMeasureMetadata;
    type ReflowDirection;
    type ClientPosition: TClientPosition + 'static;
    type BoundingClientRect: TClientRect + 'static;
    type MeasuredImage: TMeasuredImage + 'static;
    type ShapedText: TShapedText + 'static;

    fn make_initial_layout_node<T>(T) -> Self
    where
        T: TryInto<KnownElementName>;

    fn reset_custom_styles<T>(&mut self, T)
    where
        T: TryInto<KnownElementName>;

    fn is_tainted(&self) -> bool;

    fn insert_child(&mut self, &mut Self, usize);

    fn append_child(&mut self, &mut Self);

    fn remove_child(&mut self, &mut Self);

    fn apply_rules<'a, I>(&mut self, iter: I)
    where
        Self::Styles: 'a,
        I: Iterator<Item = &'a Self::Styles>
    {
        iter.for_each(|styles| self.apply_styles(styles));
    }

    fn apply_styles(&mut self, &Self::Styles);

    fn mark_dirty(&mut self);

    fn measure_self_as_image<T>(&mut self, &Self::Resources, &T, &Self::ImageMeasureMetadata)
    where
        T: TDOMText;

    fn measure_self_as_text<T>(&mut self, &Self::Resources, &T, &Self::TextMeasureMetadata)
    where
        T: TDOMText;

    fn measure_self_as_normal(&mut self, &Self::Resources, &Self::NormalMeasureMetadata);

    fn reflow_subtree(&mut self, u32, u32, Self::ReflowDirection);

    fn set_computed_client_position(&mut self, Self::ClientPosition);

    fn get_local_bounding_client_rect(&self) -> Self::BoundingClientRect;

    fn get_global_bounding_client_rect(&self) -> Self::BoundingClientRect;

    fn get_measured_image(&self) -> &Self::MeasuredImage;

    fn get_shaped_text(&self) -> &Self::ShapedText;
}
