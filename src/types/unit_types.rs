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

#[cfg(feature = "impl-external-yoga")]
use yoga;

use self_tokenize_macro::SelfTokenize;
use self_tokenize_trait::ToCustomTokens;

#[derive(Debug, PartialEq, Copy, Clone, SelfTokenize)]
pub enum SharedUnit {
    None,
    Point(u32),
    Percent(u32),
    Auto
}

impl SharedUnit {
    pub fn point(&self) -> u32 {
        if let &SharedUnit::Point(value) = self {
            value
        } else {
            panic!("Not a point");
        }
    }

    pub fn percent(&self) -> u32 {
        if let &SharedUnit::Percent(value) = self {
            value
        } else {
            panic!("Not a percentage");
        }
    }
}

impl AsRef<str> for SharedUnit {
    fn as_ref(&self) -> &str {
        match self {
            &SharedUnit::None => "",
            &SharedUnit::Point(_) => "pt",
            &SharedUnit::Percent(_) => "%",
            &SharedUnit::Auto => "auto"
        }
    }
}

#[cfg(feature = "impl-external-yoga")]
impl From<yoga::StyleUnit> for SharedUnit {
    fn from(value: yoga::StyleUnit) -> Self {
        match value {
            yoga::StyleUnit::UndefinedValue => SharedUnit::None,
            yoga::StyleUnit::Point(v) => SharedUnit::Point(v.into_inner() as u32),
            yoga::StyleUnit::Percent(v) => SharedUnit::Percent(v.into_inner() as u32),
            yoga::StyleUnit::Auto => SharedUnit::Auto
        }
    }
}
