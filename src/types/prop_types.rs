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

use std::any::Any;
use std::ops::Deref;
use std::rc::Rc;

use traits::TProp;

#[derive(Debug, Clone)]
pub struct Prop(Rc<Any>);

impl TProp for Prop {}

impl Prop {
    pub fn new<T>(value: T) -> Self
    where
        T: Any
    {
        Prop(Rc::new(value))
    }
}

impl PartialEq for Prop {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for Prop {
    type Target = Any;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
