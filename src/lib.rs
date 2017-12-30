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

#![cfg_attr(feature = "cargo-clippy", allow(match_ref_pats, unit_arg))]
#![feature(box_syntax)]
#![feature(fundamental)]
#![feature(proc_macro)]
#![feature(specialization)]
#![feature(try_from)]

#[macro_use]
extern crate enum_primitive_derive;
#[cfg(feature = "impl-external-image")]
extern crate image;
extern crate num_traits;
extern crate self_tokenize_macro;
extern crate self_tokenize_trait;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "impl-external-webrender")]
extern crate webrender;
#[cfg(feature = "impl-external-yoga")]
extern crate yoga;

#[cfg(feature = "impl-blanket")]
mod impl_blanket;
#[cfg(feature = "impl-dummy")]
mod impl_dummy;
mod impl_external;

pub mod consts;
pub mod traits;
pub mod types;
