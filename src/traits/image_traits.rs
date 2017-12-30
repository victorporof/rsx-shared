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

use traits::TDimensionsInfo;
use types::{ImageEncodedData, ImageEncodingFormat};

#[fundamental]
pub trait TImageCache: Clone + 'static {
    type Image;
    type ImageId;
    type ResourceUpdates;
    type Dimensions: TDimensionsInfo;

    fn add_raw<P, T>(&mut self, P, T) -> Option<()>
    where
        T: Into<Rc<Vec<u8>>>,
        P: AsRef<str>;

    fn add_image<P, E>(&mut self, P, &E) -> Option<()>
    where
        P: AsRef<str>,
        E: TEncodedImage;

    fn add_image_with_id<E>(&mut self, Self::ImageId, &E) -> Option<()>
    where
        E: TEncodedImage;

    fn get_image<P>(&self, P) -> Option<Self::Image>
    where
        P: AsRef<str>;

    fn measure_image<P>(&self, P) -> Option<Self::Dimensions>
    where
        P: AsRef<str>;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}

#[fundamental]
pub trait TEncodedImage: Debug + PartialEq {
    type Error;

    fn from_bytes<T>(T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Into<Rc<Vec<u8>>>;

    fn from_data_uri<T>(T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Into<Rc<String>>;

    fn format(&self) -> Option<ImageEncodingFormat>;

    fn bytes(&self) -> Option<&Rc<Vec<u8>>>;

    fn data_uri(&self) -> Option<&Rc<String>>;

    fn size_info(&self) -> Option<(u32, u32)>;

    fn info(&self) -> ImageEncodedData;
}
