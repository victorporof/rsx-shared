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

#[cfg(feature = "impl-external-image")]
use image;

use self_tokenize_macro::SelfTokenize;
use self_tokenize_trait::ToCustomTokens;

#[derive(Debug, PartialEq, Copy, Clone, SelfTokenize)]
pub enum ImagePixelFormat {
    Gray(u8),
    RGB(u8),
    Palette(u8),
    GrayA(u8),
    RGBA(u8),
    BGRA(u8)
}

impl AsRef<str> for ImagePixelFormat {
    fn as_ref(&self) -> &str {
        match self {
            &ImagePixelFormat::Gray(_) => "gray",
            &ImagePixelFormat::RGB(_) => "rgb",
            &ImagePixelFormat::Palette(_) => "palette",
            &ImagePixelFormat::GrayA(_) => "gray-alpha",
            &ImagePixelFormat::RGBA(_) => "rgba",
            &ImagePixelFormat::BGRA(_) => "bgra"
        }
    }
}

#[cfg(feature = "impl-external-image")]
impl From<image::ColorType> for ImagePixelFormat {
    fn from(value: image::ColorType) -> Self {
        match value {
            image::ColorType::Gray(v) => ImagePixelFormat::Gray(v),
            image::ColorType::RGB(v) => ImagePixelFormat::RGB(v),
            image::ColorType::Palette(v) => ImagePixelFormat::Palette(v),
            image::ColorType::GrayA(v) => ImagePixelFormat::GrayA(v),
            image::ColorType::RGBA(v) => ImagePixelFormat::RGBA(v)
        }
    }
}

#[cfg(feature = "impl-external-image")]
impl Into<image::ColorType> for ImagePixelFormat {
    fn into(self) -> image::ColorType {
        match self {
            ImagePixelFormat::Gray(v) => image::ColorType::Gray(v),
            ImagePixelFormat::RGB(v) => image::ColorType::RGB(v),
            ImagePixelFormat::Palette(v) => image::ColorType::Palette(v),
            ImagePixelFormat::GrayA(v) => image::ColorType::GrayA(v),
            ImagePixelFormat::RGBA(v) => image::ColorType::RGBA(v),
            ImagePixelFormat::BGRA(_) => panic!("Unsupported operation: Can't convert BGRA to RGBA")
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, SelfTokenize)]
pub enum ImageEncodingFormat {
    PNG,
    JPEG,
    GIF,
    WEBP,
    PNM,
    TIFF,
    TGA,
    BMP,
    ICO,
    HDR
}

impl AsRef<str> for ImageEncodingFormat {
    fn as_ref(&self) -> &str {
        match self {
            &ImageEncodingFormat::PNG => "png",
            &ImageEncodingFormat::JPEG => "jpeg",
            &ImageEncodingFormat::GIF => "gif",
            &ImageEncodingFormat::WEBP => "webp",
            &ImageEncodingFormat::PNM => "pnm",
            &ImageEncodingFormat::TIFF => "tiff",
            &ImageEncodingFormat::TGA => "tga",
            &ImageEncodingFormat::BMP => "bmp",
            &ImageEncodingFormat::ICO => "ico",
            &ImageEncodingFormat::HDR => "hdr"
        }
    }
}

#[cfg(feature = "impl-external-image")]
impl From<image::ImageFormat> for ImageEncodingFormat {
    fn from(value: image::ImageFormat) -> Self {
        match value {
            image::ImageFormat::PNG => ImageEncodingFormat::PNG,
            image::ImageFormat::JPEG => ImageEncodingFormat::JPEG,
            image::ImageFormat::GIF => ImageEncodingFormat::GIF,
            image::ImageFormat::WEBP => ImageEncodingFormat::WEBP,
            image::ImageFormat::PNM => ImageEncodingFormat::PNM,
            image::ImageFormat::TIFF => ImageEncodingFormat::TIFF,
            image::ImageFormat::TGA => ImageEncodingFormat::TGA,
            image::ImageFormat::BMP => ImageEncodingFormat::BMP,
            image::ImageFormat::ICO => ImageEncodingFormat::ICO,
            image::ImageFormat::HDR => ImageEncodingFormat::HDR,
            _ => unreachable!()
        }
    }
}

#[cfg(feature = "impl-external-image")]
impl Into<image::ImageFormat> for ImageEncodingFormat {
    fn into(self) -> image::ImageFormat {
        match self {
            ImageEncodingFormat::PNG => image::ImageFormat::PNG,
            ImageEncodingFormat::JPEG => image::ImageFormat::JPEG,
            ImageEncodingFormat::GIF => image::ImageFormat::GIF,
            ImageEncodingFormat::WEBP => image::ImageFormat::WEBP,
            ImageEncodingFormat::PNM => image::ImageFormat::PNM,
            ImageEncodingFormat::TIFF => image::ImageFormat::TIFF,
            ImageEncodingFormat::TGA => image::ImageFormat::TGA,
            ImageEncodingFormat::BMP => image::ImageFormat::BMP,
            ImageEncodingFormat::ICO => image::ImageFormat::ICO,
            ImageEncodingFormat::HDR => image::ImageFormat::HDR
        }
    }
}
