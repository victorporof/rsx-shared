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

use types::unit_types::SharedUnit;

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedCursor;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedCursor {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedColor;

#[cfg(feature = "impl-dummy")]
impl Into<[u8; 4]> for DummyComputedColor {
    fn into(self) -> [u8; 4] {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedTextShadow;

#[cfg(feature = "impl-dummy")]
impl Into<[u8; 4]> for DummyComputedTextShadow {
    fn into(self) -> [u8; 4] {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontName;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedFontName {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontStyle;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedFontStyle {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontCaps;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedFontCaps {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontWeight;

#[cfg(feature = "impl-dummy")]
impl TryInto<u32> for DummyComputedFontWeight {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontSize;

#[cfg(feature = "impl-dummy")]
impl TryInto<SharedUnit> for DummyComputedFontSize {
    type Error = ();

    fn try_into(self) -> Result<SharedUnit, Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontStretch;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedFontStretch {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedVisibility;

#[cfg(feature = "impl-dummy")]
impl Into<bool> for DummyComputedVisibility {
    fn into(self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedBorderStyle;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedBorderStyle {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedBoxShadow;

#[cfg(feature = "impl-dummy")]
impl Into<[u8; 4]> for DummyComputedBoxShadow {
    fn into(self) -> [u8; 4] {
        unimplemented!()
    }
}
