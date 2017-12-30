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
use std::path::Path;
use std::rc::Rc;

use traits::{
    TClientPosition,
    TClientRect,
    TClientSize,
    TComputedStyles,
    TDOMNode,
    TDOMText,
    TDOMTree,
    TDimensionsInfo,
    TEncodedFont,
    TEncodedImage,
    TEvent,
    TEventManager,
    TFileCache,
    TFontCache,
    TFontInstanceKey,
    TFontKey,
    TFontKeysAPI,
    TGenericEvent,
    TGlyphInstance,
    TGlyphStore,
    TImageCache,
    TImageKeysAPI,
    TInheritedStyles,
    TKeyboardEvent,
    TLayoutNode,
    TMeasuredImage,
    TMediaKey,
    TMouseEvent,
    TResourceGroup,
    TShapedText,
    TStyleDeclarations,
    TUIEvent
};
use types::{
    Closure,
    DOMNodeRawId,
    DummyComputedBorderStyle,
    DummyComputedBoxShadow,
    DummyComputedColor,
    DummyComputedCursor,
    DummyComputedFontCaps,
    DummyComputedFontName,
    DummyComputedFontSize,
    DummyComputedFontStretch,
    DummyComputedFontStyle,
    DummyComputedFontWeight,
    DummyComputedTextShadow,
    DummyComputedVisibility,
    EventType,
    FontEncodedData,
    FontInstanceResourceData,
    FontResourceData,
    ImageEncodedData,
    ImageResourceData,
    KnownElementName,
    VirtualEventType
};

impl TGenericEvent for () {}

impl TEvent for () {
    fn target(&self) -> DOMNodeRawId {
        unimplemented!()
    }

    fn ty(&self) -> EventType {
        unimplemented!()
    }
}

impl TUIEvent for () {
    fn alt_key(&self) -> bool {
        unimplemented!()
    }

    fn ctrl_key(&self) -> bool {
        unimplemented!()
    }

    fn meta_key(&self) -> bool {
        unimplemented!()
    }

    fn shift_key(&self) -> bool {
        unimplemented!()
    }
}

impl TKeyboardEvent for () {
    type KeyCode = ();

    fn code(&self) -> Self::KeyCode {
        unimplemented!()
    }

    fn key(&self) -> &'static str {
        unimplemented!()
    }

    fn get_modifier_state(&self) -> bool {
        unimplemented!()
    }

    fn repeat(&self) -> bool {
        unimplemented!()
    }
}

impl TMouseEvent for () {
    type MouseButton = ();

    fn button(&self) -> Self::MouseButton {
        unimplemented!()
    }

    fn client_x(&self) -> u32 {
        unimplemented!()
    }

    fn client_y(&self) -> u32 {
        unimplemented!()
    }

    fn offset_x(&self) -> u32 {
        unimplemented!()
    }

    fn offset_y(&self) -> u32 {
        unimplemented!()
    }

    fn page_x(&self) -> u32 {
        unimplemented!()
    }

    fn page_y(&self) -> u32 {
        unimplemented!()
    }
}

impl TEventManager for () {
    type Target = ();
    type KeyCode = ();
    type MouseButton = ();
    type KeyEventData = ();
    type MouseEventData = ();

    fn add_event_listener<F>(&mut self, _: <Self::Target as TDOMNode>::Id, _: EventType, _: F)
    where
        F: Into<Closure<<Self::Target as TDOMNode>::Event>>
    {
        unimplemented!()
    }

    fn remove_event_listener<F>(&mut self, _: <Self::Target as TDOMNode>::Id, _: EventType, _: F)
    where
        F: Into<Closure<<Self::Target as TDOMNode>::Event>>
    {
        unimplemented!()
    }

    fn receive_key_event(&mut self, _: VirtualEventType, _: Self::KeyEventData) {
        unimplemented!()
    }

    fn receive_mouse_event(&mut self, _: VirtualEventType, _: Self::MouseEventData) {
        unimplemented!()
    }

    fn broadcast_events<A>(&mut self, _: &A)
    where
        A: TDOMTree<Node = Self::Target>
    {
        unimplemented!()
    }

    fn intercept_events<A, F>(&mut self, _: &A, _: F)
    where
        A: TDOMTree<Node = Self::Target>,
        F: FnMut(<Self::Target as TDOMNode>::Event)
    {
        unimplemented!()
    }
}

impl TResourceGroup for () {
    type Files = ();
    type Images = ();
    type Fonts = ();

    fn files(&self) -> Self::Files {
        unimplemented!()
    }

    fn images(&self) -> Self::Images {
        unimplemented!()
    }

    fn fonts(&self) -> Self::Fonts {
        unimplemented!()
    }
}

impl TFileCache for () {
    type File = ();
    type ResourceUpdates = ();

    fn add_file<P>(&mut self, _: P) -> Option<()>
    where
        P: AsRef<Path>
    {
        unimplemented!()
    }

    fn get_file<P>(&self, _: P) -> Option<Self::File>
    where
        P: AsRef<Path>
    {
        unimplemented!()
    }

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        unimplemented!()
    }
}

impl TImageCache for () {
    type Image = ();
    type ImageId = ();
    type ResourceUpdates = ();
    type Dimensions = ();

    fn add_raw<P, T>(&mut self, _: P, _: T) -> Option<()>
    where
        P: AsRef<str>,
        T: Into<Rc<Vec<u8>>>
    {
        unimplemented!()
    }

    fn add_image<P, E>(&mut self, _: P, _: &E) -> Option<()>
    where
        P: AsRef<str>,
        E: TEncodedImage
    {
        unimplemented!()
    }

    fn add_image_with_id<E>(&mut self, _: Self::ImageId, _: &E) -> Option<()>
    where
        E: TEncodedImage
    {
        unimplemented!()
    }

    fn get_image<P>(&self, _: P) -> Option<Self::Image>
    where
        P: AsRef<str>
    {
        unimplemented!()
    }

    fn measure_image<P>(&self, _: P) -> Option<Self::Dimensions>
    where
        P: AsRef<str>
    {
        unimplemented!()
    }

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        unimplemented!()
    }
}

impl TFontCache for () {
    type FontInstance = ();
    type FontId = ();
    type ResourceUpdates = ();
    type Glyphs = ();

    fn add_raw<P, T>(&mut self, _: P, _: T, _: usize) -> Option<()>
    where
        T: Into<Rc<Vec<u8>>>,
        P: AsRef<str>
    {
        unimplemented!()
    }

    fn add_font<P, E>(&mut self, _: P, _: &E, _: usize) -> Option<()>
    where
        P: AsRef<str>,
        E: TEncodedFont
    {
        unimplemented!()
    }

    fn add_font_with_id<E>(&mut self, _: Self::FontId, _: &E, _: usize) -> Option<()>
    where
        E: TEncodedFont
    {
        unimplemented!()
    }

    fn get_family_name<P>(&self, _: P) -> Option<String>
    where
        P: AsRef<str>
    {
        unimplemented!()
    }

    fn get_family_name_for_id(&self, _: Self::FontId) -> Option<String> {
        unimplemented!()
    }

    fn set_default_font<T>(&mut self, _: T, _: u32, _: u32)
    where
        T: AsRef<str>
    {
        unimplemented!()
    }

    fn get_default_font(&self) -> Option<Self::FontInstance> {
        unimplemented!()
    }

    fn get_default_font_with_size(&self, _: u32) -> Option<Self::FontInstance> {
        unimplemented!()
    }

    fn get_font<T>(&self, _: T) -> Option<Self::FontInstance>
    where
        T: AsRef<str>
    {
        unimplemented!()
    }

    fn get_font_with_size<T>(&self, _: T, _: u32) -> Option<Self::FontInstance>
    where
        T: AsRef<str>
    {
        unimplemented!()
    }

    fn get_default_font_with_size_and_dpi(&self, _: u32, _: u32) -> Option<Self::FontInstance> {
        unimplemented!()
    }

    fn get_font_with_size_and_dpi<T>(&self, _: T, _: u32, _: u32) -> Option<Self::FontInstance>
    where
        T: AsRef<str>
    {
        unimplemented!()
    }

    fn shape_text_h<T>(&self, _: &Self::FontInstance, _: T) -> Option<Self::Glyphs>
    where
        T: AsRef<str>
    {
        unimplemented!()
    }

    fn shape_text_v<T>(&self, _: &Self::FontInstance, _: T) -> Option<Self::Glyphs>
    where
        T: AsRef<str>
    {
        unimplemented!()
    }

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        unimplemented!()
    }
}

impl TMediaKey for () {}

impl TFontKey for () {}

impl TFontInstanceKey for () {}

impl TImageKeysAPI for () {
    type RootRendererAPI = ();
    type ResourceUpdates = ();
    type ImageKey = ();

    fn new(_: Self::RootRendererAPI) -> Self {
        unimplemented!()
    }

    fn add_image(&mut self, _: ImageEncodedData, _: ImageResourceData) -> Self::ImageKey {
        unimplemented!()
    }

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        unimplemented!()
    }
}

impl TFontKeysAPI for () {
    type RootRendererAPI = ();
    type ResourceUpdates = ();
    type FontKey = ();
    type FontInstanceKey = ();
    type GlyphInstance = ();

    fn new(_: Self::RootRendererAPI) -> Self {
        unimplemented!()
    }

    fn add_font(&mut self, _: FontEncodedData, _: FontResourceData) -> Self::FontKey {
        unimplemented!()
    }

    fn add_font_instance(&mut self, _: Self::FontKey, _: FontInstanceResourceData) -> Self::FontInstanceKey {
        unimplemented!()
    }

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates {
        unimplemented!()
    }
}

impl TDimensionsInfo for () {
    type ResourceKey = ();

    fn resource_key(&self) -> Self::ResourceKey {
        unimplemented!()
    }

    fn width(&self) -> u32 {
        unimplemented!()
    }

    fn height(&self) -> u32 {
        unimplemented!()
    }
}

impl TGlyphInstance for () {
    fn new(_: u32, _: i32, _: i32) -> Self {
        unimplemented!()
    }
}

impl TGlyphStore for () {
    type FontKey = ();
    type FontInstanceKey = ();
    type Glyph = ();

    fn font_key(&self) -> Self::FontKey {
        unimplemented!()
    }

    fn font_instance_key(&self) -> Self::FontInstanceKey {
        unimplemented!()
    }

    fn width_f(&self) -> f32 {
        unimplemented!()
    }

    fn height_f(&self) -> f32 {
        unimplemented!()
    }

    fn width_64(&self) -> i32 {
        unimplemented!()
    }

    fn height_64(&self) -> i32 {
        unimplemented!()
    }

    fn glyphs(&self) -> &[Self::Glyph] {
        unimplemented!()
    }
}

impl TStyleDeclarations for () {
    type LayoutStyle = ();
    type ThemeStyle = ();

    fn make_user_agent_styles<T>(_: T) -> Self
    where
        T: TryInto<KnownElementName>
    {
        ()
    }

    fn for_each_layout_style<F>(&self, _: F)
    where
        F: FnMut(&Self::LayoutStyle)
    {
        unimplemented!()
    }

    fn for_each_theme_style<F>(&self, _: F)
    where
        F: FnMut(&Self::ThemeStyle)
    {
        unimplemented!()
    }
}

impl TInheritedStyles for () {
    type Styles = ();
    type Cursor = DummyComputedCursor;
    type Color = DummyComputedColor;
    type TextShadow = DummyComputedTextShadow;
    type FontName = DummyComputedFontName;
    type FontStyle = DummyComputedFontStyle;
    type FontCaps = DummyComputedFontCaps;
    type FontWeight = DummyComputedFontWeight;
    type FontSize = DummyComputedFontSize;
    type FontStretch = DummyComputedFontStretch;
    type Visibility = DummyComputedVisibility;

    fn inherit_styles(&mut self, _: &Self) {
        unimplemented!()
    }

    fn cursor(&self) -> Self::Cursor {
        unimplemented!()
    }

    fn color(&self) -> Self::Color {
        unimplemented!()
    }

    fn text_shadows_copy(&self) -> Vec<Self::TextShadow> {
        unimplemented!()
    }

    fn font_names_copy(&self) -> Vec<Self::FontName> {
        unimplemented!()
    }

    fn font_style(&self) -> Self::FontStyle {
        unimplemented!()
    }

    fn font_caps(&self) -> Self::FontCaps {
        unimplemented!()
    }

    fn font_weight(&self) -> Self::FontWeight {
        unimplemented!()
    }

    fn font_size(&self) -> Self::FontSize {
        unimplemented!()
    }

    fn font_stretch(&self) -> Self::FontStretch {
        unimplemented!()
    }

    fn visibility(&self) -> Self::Visibility {
        unimplemented!()
    }

    fn find_font<F, O>(&self, _: F) -> Option<O>
    where
        F: FnMut(&Self::FontName) -> Option<O>
    {
        unimplemented!()
    }
}

impl TComputedStyles for () {
    type BackgroundColor = DummyComputedColor;
    type Opacity = u32;
    type BorderSize = u32;
    type BorderColor = DummyComputedColor;
    type BorderStyle = DummyComputedBorderStyle;
    type BoxShadow = DummyComputedBoxShadow;

    fn make_initial_computed_styles<T>(_: T) -> Self
    where
        T: TryInto<KnownElementName>
    {
        unimplemented!()
    }

    fn reset_custom_styles<T>(&mut self, _: T)
    where
        T: TryInto<KnownElementName>
    {
        unimplemented!()
    }

    fn apply_rules<'a, I>(&mut self, _: I)
    where
        Self::Styles: 'a,
        I: Iterator<Item = &'a Self::Styles>
    {
        unimplemented!()
    }

    fn apply_styles(&mut self, _: &Self::Styles) {
        unimplemented!()
    }

    fn background_color(&self) -> Self::BackgroundColor {
        unimplemented!()
    }

    fn opacity(&self) -> Self::Opacity {
        unimplemented!()
    }

    fn border_bottom_width(&self) -> Self::BorderSize {
        unimplemented!()
    }

    fn border_bottom_color(&self) -> Self::BorderColor {
        unimplemented!()
    }

    fn border_bottom_style(&self) -> Self::BorderStyle {
        unimplemented!()
    }

    fn border_left_width(&self) -> Self::BorderSize {
        unimplemented!()
    }

    fn border_left_color(&self) -> Self::BorderColor {
        unimplemented!()
    }

    fn border_left_style(&self) -> Self::BorderStyle {
        unimplemented!()
    }

    fn border_right_width(&self) -> Self::BorderSize {
        unimplemented!()
    }

    fn border_right_color(&self) -> Self::BorderColor {
        unimplemented!()
    }

    fn border_right_style(&self) -> Self::BorderStyle {
        unimplemented!()
    }

    fn border_top_width(&self) -> Self::BorderSize {
        unimplemented!()
    }

    fn border_top_color(&self) -> Self::BorderColor {
        unimplemented!()
    }

    fn border_top_style(&self) -> Self::BorderStyle {
        unimplemented!()
    }

    fn box_shadows_copy(&self) -> Vec<Self::BoxShadow> {
        unimplemented!()
    }
}

impl TClientRect for () {
    type Position = ();
    type Size = ();

    fn position(&self) -> Self::Position {
        unimplemented!()
    }

    fn size(&self) -> Self::Size {
        unimplemented!()
    }

    fn offset_from_page(&self, _: (u32, u32)) -> (u32, u32) {
        unimplemented!()
    }

    fn client_from_page(&self, _: (u32, u32)) -> (u32, u32) {
        unimplemented!()
    }

    fn contains_point(&self, _: (u32, u32)) -> bool {
        unimplemented!()
    }
}

impl TClientPosition for () {}

impl TClientSize for () {}

impl TMeasuredImage for () {}

impl TShapedText for () {}

impl TLayoutNode for () {
    type Styles = ();
    type Resources = ();
    type TextMeasureMetadata = ();
    type ImageMeasureMetadata = ();
    type NormalMeasureMetadata = ();
    type ReflowDirection = ();
    type ClientPosition = ();
    type BoundingClientRect = ();
    type MeasuredImage = ();
    type ShapedText = ();

    fn make_initial_layout_node<T>(_: T) -> Self
    where
        T: TryInto<KnownElementName>
    {
        ()
    }

    fn reset_custom_styles<T>(&mut self, _: T)
    where
        T: TryInto<KnownElementName>
    {
        unimplemented!()
    }

    fn is_tainted(&self) -> bool {
        unimplemented!()
    }

    fn insert_child(&mut self, _: &mut Self, _: usize) {
        unimplemented!()
    }

    fn append_child(&mut self, _: &mut Self) {
        unimplemented!()
    }

    fn remove_child(&mut self, _: &mut Self) {
        unimplemented!()
    }

    fn apply_rules<'a, I>(&mut self, _: I)
    where
        I: Iterator<Item = &'a Self::Styles>
    {
        unimplemented!()
    }

    fn apply_styles(&mut self, _: &Self::Styles) {
        unimplemented!()
    }

    fn mark_dirty(&mut self) {
        unimplemented!()
    }

    fn measure_self_as_image<T>(&mut self, _: &Self::Resources, _: &T, _: &Self::ImageMeasureMetadata)
    where
        T: TDOMText
    {
        unimplemented!()
    }

    fn measure_self_as_text<T>(&mut self, _: &Self::Resources, _: &T, _: &Self::TextMeasureMetadata)
    where
        T: TDOMText
    {
        unimplemented!()
    }

    fn measure_self_as_normal(&mut self, _: &Self::Resources, _: &Self::NormalMeasureMetadata) {
        unimplemented!()
    }

    fn reflow_subtree(&mut self, _: u32, _: u32, _: Self::ReflowDirection) {
        unimplemented!()
    }

    fn set_computed_client_position(&mut self, _: Self::ClientPosition) {
        unimplemented!()
    }

    fn get_local_bounding_client_rect(&self) -> Self::BoundingClientRect {
        unimplemented!()
    }

    fn get_global_bounding_client_rect(&self) -> Self::BoundingClientRect {
        unimplemented!()
    }

    fn get_measured_image(&self) -> &Self::MeasuredImage {
        unimplemented!()
    }

    fn get_shaped_text(&self) -> &Self::ShapedText {
        unimplemented!()
    }
}

impl TDOMNode for () {
    type Data = ();
    type Id = ();
    type Event = ();
    type Styles = ();
    type ComputedStyles = ();
    type LayoutNode = ();

    fn data(&self) -> &Self::Data {
        unimplemented!()
    }

    fn is_void(&self) -> bool {
        unimplemented!()
    }

    fn is_shadow_host(&self) -> bool {
        unimplemented!()
    }

    fn is_text(&self) -> bool {
        unimplemented!()
    }

    fn is_normal(&self) -> bool {
        unimplemented!()
    }

    fn is_known(&self, _: KnownElementName) -> bool {
        unimplemented!()
    }

    fn computed_styles(&self) -> &Self::ComputedStyles {
        unimplemented!()
    }

    fn layout_node(&self) -> &Self::LayoutNode {
        unimplemented!()
    }

    fn reflow_subtree(&mut self, _: u32, _: u32, _: <Self::LayoutNode as TLayoutNode>::ReflowDirection) {
        unimplemented!()
    }

    fn set_computed_client_position(&mut self, _: <Self::LayoutNode as TLayoutNode>::ClientPosition) {
        unimplemented!()
    }

    fn get_local_bounding_client_rect(&self) -> <Self::LayoutNode as TLayoutNode>::BoundingClientRect {
        unimplemented!()
    }

    fn get_global_bounding_client_rect(&self) -> <Self::LayoutNode as TLayoutNode>::BoundingClientRect {
        unimplemented!()
    }

    fn get_measured_image(&self) -> &<Self::LayoutNode as TLayoutNode>::MeasuredImage {
        unimplemented!()
    }

    fn get_shaped_text(&self) -> &<Self::LayoutNode as TLayoutNode>::ShapedText {
        unimplemented!()
    }
}
