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

use traits::{TDOMTree, TResourceGroup};

#[fundamental]
pub trait TRuntime {
    type RootRendererAPI;
    type DOMResources: TResourceGroup;
    type DOMTree: TDOMTree;
    type VirtualEventMetadata;
    type ReflowMetadata;
    type BuiltDisplayList;
    type ResourceUpdates;

    fn new<S, R>(&Self::RootRendererAPI, S, R) -> Self
    where
        S: Fn(&mut Self::DOMResources),
        R: Fn() -> Self::DOMTree;

    fn should_set_window_position(&mut self) -> Option<(i32, i32)>;

    fn should_set_window_size(&mut self) -> Option<(u32, u32)>;

    fn should_redraw(&mut self) -> bool;

    fn handle_event(&mut self, Self::VirtualEventMetadata) -> bool;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;

    fn generate_display_list(&mut self, Self::ReflowMetadata) -> Self::BuiltDisplayList;
}

#[fundamental]
pub trait TRunner {
    type Runtime: TRuntime;

    fn run<F>(F)
    where
        F: FnMut(&<Self::Runtime as TRuntime>::RootRendererAPI) -> Self::Runtime;
}
