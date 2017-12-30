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
use std::hash::Hash;

use serde::{Deserialize as Des, Serialize as Ser};

use types::{FontEncodedData, FontInstanceResourceData, FontResourceData, ImageEncodedData, ImageResourceData};

#[fundamental]
pub trait TMediaKey: Debug + Ord + Hash + Copy + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TFontKey: Debug + Ord + Hash + Copy + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TFontInstanceKey: Debug + Ord + Hash + Copy + Ser + for<'a> Des<'a> {}

#[fundamental]
pub trait TImageKeysAPI {
    type RootRendererAPI;
    type ResourceUpdates;
    type ImageKey: TMediaKey;

    fn new(Self::RootRendererAPI) -> Self;

    fn add_image(&mut self, ImageEncodedData, ImageResourceData) -> Self::ImageKey;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}

#[fundamental]
pub trait TFontKeysAPI {
    type RootRendererAPI;
    type ResourceUpdates;
    type FontKey: TFontKey;
    type FontInstanceKey: TFontInstanceKey;
    type GlyphInstance: TGlyphInstance;

    fn new(Self::RootRendererAPI) -> Self;

    fn add_font(&mut self, FontEncodedData, FontResourceData) -> Self::FontKey;

    fn add_font_instance(&mut self, Self::FontKey, FontInstanceResourceData) -> Self::FontInstanceKey;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}

#[fundamental]
pub trait TDimensionsInfo: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {
    type ResourceKey: TMediaKey;

    fn resource_key(&self) -> Self::ResourceKey;

    fn width(&self) -> u32;

    fn height(&self) -> u32;
}

#[fundamental]
pub trait TGlyphInstance: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {
    fn new(u32, i32, i32) -> Self;
}

#[fundamental]
pub trait TGlyphStore: Debug + PartialEq + Clone + Ser + for<'a> Des<'a> {
    type FontKey: TFontKey;
    type FontInstanceKey: TFontInstanceKey;
    type Glyph: TGlyphInstance;

    fn font_key(&self) -> Self::FontKey;

    fn font_instance_key(&self) -> Self::FontInstanceKey;

    fn width_f(&self) -> f32;

    fn height_f(&self) -> f32;

    fn width_64(&self) -> i32;

    fn height_64(&self) -> i32;

    fn glyphs(&self) -> &[Self::Glyph];
}
