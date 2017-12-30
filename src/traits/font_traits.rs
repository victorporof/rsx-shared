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
use std::rc::Rc;

use traits::TGlyphStore;
use types::FontEncodedData;

#[fundamental]
pub trait TFontCache: Clone + 'static {
    type FontInstance;
    type FontId;
    type ResourceUpdates;
    type Glyphs: TGlyphStore;

    fn add_raw<P, T>(&mut self, P, T, usize) -> Option<()>
    where
        T: Into<Rc<Vec<u8>>>,
        P: AsRef<str>;

    fn add_font<P, E>(&mut self, P, &E, usize) -> Option<()>
    where
        P: AsRef<str>,
        E: TEncodedFont;

    fn add_font_with_id<E>(&mut self, Self::FontId, &E, usize) -> Option<()>
    where
        E: TEncodedFont;

    fn get_family_name<P>(&self, P) -> Option<String>
    where
        P: AsRef<str>;

    fn get_family_name_for_id(&self, Self::FontId) -> Option<String>;

    fn set_default_font<T>(&mut self, T, u32, u32)
    where
        T: AsRef<str>;

    fn get_default_font(&self) -> Option<Self::FontInstance>;

    fn get_default_font_with_size(&self, u32) -> Option<Self::FontInstance>;

    fn get_default_font_with_size_and_dpi(&self, u32, u32) -> Option<Self::FontInstance>;

    fn get_font<T>(&self, T) -> Option<Self::FontInstance>
    where
        T: AsRef<str>;

    fn get_font_with_size<T>(&self, T, u32) -> Option<Self::FontInstance>
    where
        T: AsRef<str>;

    fn get_font_with_size_and_dpi<T>(&self, T, u32, u32) -> Option<Self::FontInstance>
    where
        T: AsRef<str>;

    fn shape_text_h<T>(&self, &Self::FontInstance, T) -> Option<Self::Glyphs>
    where
        T: AsRef<str>;

    fn shape_text_v<T>(&self, &Self::FontInstance, T) -> Option<Self::Glyphs>
    where
        T: AsRef<str>;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}

#[fundamental]
pub trait TEncodedFont: Debug + PartialEq {
    type Error;

    fn from_bytes<T>(T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Into<Rc<Vec<u8>>>;

    fn from_data_uri<T>(T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Into<Rc<String>>;

    fn bytes(&self) -> Option<&Rc<Vec<u8>>>;

    fn data_uri(&self) -> Option<&Rc<String>>;

    fn info(&self) -> FontEncodedData;
}
