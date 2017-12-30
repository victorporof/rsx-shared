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

use self_tokenize_macro::SelfTokenize;
use self_tokenize_trait::ToCustomTokens;

use types::{
    VIRTUAL_AUX_CLICK,
    VIRTUAL_CLICK,
    VIRTUAL_CONTEXT_MENU,
    VIRTUAL_DBL_CLICK,
    VIRTUAL_KEY_DOWN,
    VIRTUAL_KEY_PRESS,
    VIRTUAL_KEY_UP,
    VIRTUAL_MOUSE_DOWN,
    VIRTUAL_MOUSE_MOVE,
    VIRTUAL_MOUSE_UP,
    VIRTUAL_SELECT,
    VIRTUAL_WHEEL
};

pub type DOMNodeRawId = u64;
pub type EventTypeId = u8;
pub type KnownAttributeNameId = u8;
pub type KnownElementNameId = u16;

// See https://developer.mozilla.org/en-US/docs/Web/Events
// TODO: Handle more event types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Serialize, Deserialize, Primitive, SelfTokenize)]
#[repr(u8)]
pub enum EventType {
    // Keyboard events
    KeyDown = VIRTUAL_KEY_DOWN,
    KeyPress = VIRTUAL_KEY_PRESS,
    KeyUp = VIRTUAL_KEY_UP,

    // Mouse events
    MouseMove = VIRTUAL_MOUSE_MOVE,
    MouseDown = VIRTUAL_MOUSE_DOWN,
    MouseUp = VIRTUAL_MOUSE_UP,
    AuxClick = VIRTUAL_AUX_CLICK,
    Click = VIRTUAL_CLICK,
    DblClick = VIRTUAL_DBL_CLICK,
    ContextMenu = VIRTUAL_CONTEXT_MENU,
    Wheel = VIRTUAL_WHEEL,
    Select = VIRTUAL_SELECT,

    // Synthetic mouse events
    MouseEnter = 21,
    MouseLeave = 22,
    MouseOver = 23,
    MouseOut = 24
}

// See https://www.w3.org/TR/html51/dom.html#sec-global-attributes
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Primitive, SelfTokenize)]
#[repr(u8)]
pub enum KnownAttributeName {
    // HTML global attributes
    Accesskey = 1,
    Class = 2,
    CntEditable = 3,
    Contextmenu = 4,
    Dir = 5,
    Draggable = 6,
    Dropzone = 7,
    Hidden = 8,
    Id = 9,
    Lang = 10,
    Spellcheck = 11,
    Src = 12,
    Style = 13,
    Tabindex = 14,
    Title = 15,
    Translate = 16
}

// See https://developer.mozilla.org/en-US/docs/Web/HTML/Element
// See https://facebook.github.io/react-native/docs/components-and-apis.html#basic-components
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Primitive, SelfTokenize)]
#[repr(u16)]
pub enum KnownElementName {
    // RSX
    Root = 1,

    // HTML content sectioning
    Address = 10,
    Article = 11,
    Aside = 12,
    Footer = 13,
    Header = 14,
    Nav = 15,
    Section = 16,

    // HTML text sectioning
    Hgroup = 21,
    H1 = 22,
    H2 = 23,
    H3 = 24,
    H4 = 25,
    H5 = 26,
    H6 = 27,

    // HTML text content
    Main = 31,
    Div = 32,
    Span = 33,
    P = 34,
    Ol = 35,
    Ul = 36,
    Li = 37,
    Dl = 38,
    Dt = 39,
    Dd = 40,
    Figure = 41,
    Figcaption = 42,
    Hr = 43,
    Pre = 44,
    Blockquote = 45,

    // HTML inline text semantics
    A = 51,
    Bold = 52,
    Italic = 53,
    Underline = 54,
    Strikethrough = 55,
    Emphasis = 56,
    Mark = 57,
    Quotation = 58,
    Citation = 59,
    Code = 60,
    Data = 61,
    Time = 62,
    Sub = 63,
    Sup = 64,
    Br = 65,
    Wbr = 66,

    // HTML image and multimedia
    Image = 71,
    Area = 72,
    Map = 73,
    Audio = 74,
    Video = 75,
    Track = 76,

    // HTML forms
    Button = 81,
    Datalist = 82,
    Fieldset = 83,
    Form = 84,
    Input = 85,
    Label = 86,
    Legend = 87,
    Meter = 88,
    Optgroup = 89,
    Option = 90,
    Output = 91,
    Progress = 92,
    Select = 93,
    Textarea = 94,

    // React Fiber components
    Fragment = 1000,

    // React Native basic components
    View = 1001,
    Text = 1002,
    // Image = 71,
    TextInput = 1004,
    ScrollView = 1005,

    // React Native user interface
    // Button = 81,
    Picker = 1007,
    Slider = 1008,
    Switch = 1009,

    // React Native list views
    FlatList = 1010,
    SectionList = 1011
}
