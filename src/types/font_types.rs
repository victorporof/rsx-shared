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

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum FontEncodedData<'a> {
    Bytes { bytes: &'a Rc<Vec<u8>> },
    DataUri { data_uri: &'a Rc<String> }
}

impl<'a> FontEncodedData<'a> {
    pub fn new(bytes: &'a Rc<Vec<u8>>) -> Self {
        FontEncodedData::Bytes { bytes }
    }
}

#[derive(Debug, PartialEq)]
pub struct FontResourceData<'a> {
    pub bytes: &'a Rc<Vec<u8>>,
    pub face_index: usize
}

impl<'a> FontResourceData<'a> {
    pub fn new(bytes: &'a Rc<Vec<u8>>, face_index: usize) -> Self {
        FontResourceData { bytes, face_index }
    }
}

#[derive(Debug, PartialEq)]
pub struct FontInstanceResourceData {
    pub size: u32,
    pub dpi: u32
}

impl FontInstanceResourceData {
    pub fn new(size: u32, dpi: u32) -> Self {
        FontInstanceResourceData { size, dpi }
    }
}
