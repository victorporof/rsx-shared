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

#[cfg(feature = "impl-external-webrender")]
use webrender::api::{FontInstanceKey, FontKey, GlyphInstance, ImageKey, LayoutPoint};

#[cfg(feature = "impl-external-webrender")]
use traits::{TFontInstanceKey, TFontKey, TGlyphInstance, TMediaKey};

#[cfg(feature = "impl-external-webrender")]
impl TMediaKey for ImageKey {}

#[cfg(feature = "impl-external-webrender")]
impl TFontKey for FontKey {}

#[cfg(feature = "impl-external-webrender")]
impl TFontInstanceKey for FontInstanceKey {}

#[cfg(feature = "impl-external-webrender")]
impl TGlyphInstance for GlyphInstance {
    fn new(glyph_index: u32, x_64: i32, y_64: i32) -> Self {
        GlyphInstance {
            index: glyph_index,
            point: LayoutPoint::new(x_64 as f32 / 64.0, y_64 as f32 / 64.0)
        }
    }
}
