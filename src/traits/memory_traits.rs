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

#[fundamental]
pub trait TMemoryAPI: Sized {
    fn as_ptr(&self) -> *mut () {
        self as *const Self as *mut ()
    }

    fn into_raw(self) -> *mut () {
        Box::into_raw(box self) as *mut ()
    }

    unsafe fn from_raw(raw: *mut ()) -> Option<Self> {
        let ptr = raw as *mut Self;
        if !ptr.is_null() {
            Some(*Box::from_raw(ptr))
        } else {
            None
        }
    }

    unsafe fn get_ref<'a>(raw: *mut ()) -> Option<&'a Self> {
        let ptr = raw as *const Self;
        if !ptr.is_null() {
            ptr.as_ref()
        } else {
            None
        }
    }

    unsafe fn get_mut<'a>(raw: *mut ()) -> Option<&'a mut Self> {
        let ptr = raw as *mut Self;
        if !ptr.is_null() {
            ptr.as_mut()
        } else {
            None
        }
    }

    unsafe fn get_static_ref(raw: *mut ()) -> Option<&'static Self> {
        Self::get_ref(raw)
    }

    unsafe fn get_static_mut(raw: *mut ()) -> Option<&'static mut Self> {
        Self::get_mut(raw)
    }

    unsafe fn drop_raw(raw: *mut ()) {
        Self::from_raw(raw);
    }
}

#[fundamental]
pub trait TMemoryAddrAPI: TMemoryAPI {
    fn as_addr(&self) -> usize {
        self.as_ptr() as usize
    }

    fn into_addr(self) -> usize {
        self.into_raw() as usize
    }

    unsafe fn from_addr(addr: usize) -> Option<Self> {
        <Self as TMemoryAPI>::from_raw(addr as *mut ())
    }

    unsafe fn get_ref<'a>(addr: usize) -> Option<&'a Self> {
        <Self as TMemoryAPI>::get_ref(addr as *mut ())
    }

    unsafe fn get_mut<'a>(addr: usize) -> Option<&'a mut Self> {
        <Self as TMemoryAPI>::get_mut(addr as *mut ())
    }

    unsafe fn get_static_ref(addr: usize) -> Option<&'static Self> {
        <Self as TMemoryAPI>::get_static_ref(addr as *mut ())
    }

    unsafe fn get_static_mut(addr: usize) -> Option<&'static mut Self> {
        <Self as TMemoryAPI>::get_static_mut(addr as *mut ())
    }

    unsafe fn drop_addr(addr: usize) {
        <Self as TMemoryAPI>::drop_raw(addr as *mut ());
    }
}
