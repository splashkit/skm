#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::{
    ffi::{CStr, CString, c_void},
    mem::{forget, zeroed, MaybeUninit},
    os::raw::{c_char, c_int},
    ptr::{null_mut},
    cell::{RefCell},
};

type __sklib_ptr = *mut c_void;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyCode {
    UnknownKey = 0,    BackspaceKey = 8,    TabKey = 9,    ClearKey = 12,    ReturnKey = 13,    PauseKey = 19,    EscapeKey = 27,    SpaceKey = 32,    ExclaimKey = 33,    DoubleQuoteKey = 34,    HashKey = 35,    DollarKey = 36,    AmpersandKey = 38,    QuoteKey = 39,    LeftParenKey = 40,    RightParenKey = 41,    AsteriskKey = 42,    PlusKey = 43,    CommaKey = 44,    MinusKey = 45,    PeriodKey = 46,    SlashKey = 47,    Num0Key = 48,    Num1Key = 49,    Num2Key = 50,    Num3Key = 51,    Num4Key = 52,    Num5Key = 53,    Num6Key = 54,    Num7Key = 55,    Num8Key = 56,    Num9Key = 57,    ColonKey = 58,    SemiColonKey = 59,    LessKey = 60,    EqualsKey = 61,    GreaterKey = 62,    QuestionKey = 63,    AtKey = 64,    LeftBracketKey = 91,    BackslashKey = 92,    RightBracketKey = 93,    CaretKey = 94,    UnderscoreKey = 95,    BackquoteKey = 96,    AKey = 97,    BKey = 98,    CKey = 99,    DKey = 100,    EKey = 101,    FKey = 102,    GKey = 103,    HKey = 104,    IKey = 105,    JKey = 106,    KKey = 107,    LKey = 108,    MKey = 109,    NKey = 110,    OKey = 111,    PKey = 112,    QKey = 113,    RKey = 114,    SKey = 115,    TKey = 116,    UKey = 117,    VKey = 118,    WKey = 119,    XKey = 120,    YKey = 121,    ZKey = 122,    DeleteKey = 127,    Keypad0 = 256,    Keypad1 = 257,    Keypad2 = 258,    Keypad3 = 259,    Keypad4 = 260,    Keypad5 = 261,    Keypad6 = 262,    Keypad7 = 263,    Keypad8 = 264,    Keypad9 = 265,    KeypadPeriod = 266,    KeypadDivide = 267,    KeypadMultiply = 268,    KeypadMinus = 269,    KeypadPlus = 270,    KeypadEnter = 271,    KeypadEquals = 272,    UpKey = 273,    DownKey = 274,    RightKey = 275,    LeftKey = 276,    InsertKey = 277,    HomeKey = 278,    EndKey = 279,    PageUpKey = 280,    PageDownKey = 281,    F1Key = 282,    F2Key = 283,    F3Key = 284,    F4Key = 285,    F5Key = 286,    F6Key = 287,    F7Key = 288,    F8Key = 289,    F9Key = 290,    F10Key = 291,    F11Key = 292,    F12Key = 293,    F13Key = 294,    F14Key = 295,    F15Key = 296,    NumLockKey = 300,    CapsLockKey = 301,    ScrollLockKey = 302,    RightShiftKey = 303,    LeftShiftKey = 304,    RightCtrlKey = 305,    LeftCtrlKey = 306,    RightAltKey = 307,    LeftAltKey = 308,    LeftSuperKey = 311,    RightSuperKey = 312,    ModeKey = 313,    HelpKey = 315,    SysReqKey = 317,    MenuKey = 319,    PowerKey = 320,}

impl From<i32> for KeyCode {
    fn from(v: i32) -> Self {
        __skadapter__to_key_code(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogLevel {
    None,    Info,    Debug,    Warning,    Error,    Fatal,}

impl From<i32> for LogLevel {
    fn from(v: i32) -> Self {
        __skadapter__to_log_level(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogMode {
    LogNone,    LogConsole,    LogFileOnly,    LogConsoleAndFile,}

impl From<i32> for LogMode {
    fn from(v: i32) -> Self {
        __skadapter__to_log_mode(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    NoButton,    LeftButton,    MiddleButton,    RightButton,    MouseX1Button,    MouseX2Button,}

impl From<i32> for MouseButton {
    fn from(v: i32) -> Self {
        __skadapter__to_mouse_button(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConnectionType {
    TCP,    UDP,    Unknown,}

impl From<i32> for ConnectionType {
    fn from(v: i32) -> Self {
        __skadapter__to_connection_type(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResourceKind {
    AnimationResource,    BundleResource,    FontResource,    ImageResource,    JsonResource,    MusicResource,    ServerResource,    SoundResource,    TimerResource,    OtherResource,}

impl From<i32> for ResourceKind {
    fn from(v: i32) -> Self {
        __skadapter__to_resource_kind(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CollisionTestKind {
    PixelCollisions,    AabbCollisions,}

impl From<i32> for CollisionTestKind {
    fn from(v: i32) -> Self {
        __skadapter__to_collision_test_kind(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpriteEventKind {
    SpriteArrivedEvent,    SpriteAnimationEndedEvent,    SpriteTouchedEvent,    SpriteClickedEvent,}

impl From<i32> for SpriteEventKind {
    fn from(v: i32) -> Self {
        __skadapter__to_sprite_event_kind(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DrawingDest {
    DrawToScreen,    DrawToWorld,    DrawDefault,}

impl From<i32> for DrawingDest {
    fn from(v: i32) -> Self {
        __skadapter__to_drawing_dest(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FontStyle {
    NormalFont = 0,    BoldFont = 1,    ItalicFont = 2,    UnderlineFont = 4,}

impl From<i32> for FontStyle {
    fn from(v: i32) -> Self {
        __skadapter__to_font_style(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HttpStatusCode {
    HttpStatusOk = 200,    HttpStatusCreated = 201,    HttpStatusNoContent = 204,    HttpStatusMovedPermanently = 301,    HttpStatusFound = 302,    HttpStatusSeeOther = 303,    HttpStatusBadRequest = 400,    HttpStatusUnauthorized = 401,    HttpStatusForbidden = 403,    HttpStatusNotFound = 404,    HttpStatusMethodNotAllowed = 405,    HttpStatusRequestTimeout = 408,    HttpStatusConflict = 409,    HttpStatusInternalServerError = 500,    HttpStatusNotImplemented = 501,    HttpStatusServiceUnavailable = 503,}

impl From<i32> for HttpStatusCode {
    fn from(v: i32) -> Self {
        __skadapter__to_http_status_code(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterfaceStyle {
    FlatDarkStyle = 0,    ShadedDarkStyle = 1,    FlatLightStyle = 2,    ShadedLightStyle = 3,    Bubble = 4,    BubbleMulticolored = 5,}

impl From<i32> for InterfaceStyle {
    fn from(v: i32) -> Self {
        __skadapter__to_interface_style(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PinModes {
    GpioInput = 0,    GpioOutput = 1,    GpioAlt0 = 4,    GpioAlt1 = 5,    GpioAlt2 = 6,    GpioAlt3 = 7,    GpioAlt4 = 3,    GpioAlt5 = 2,}

impl From<i32> for PinModes {
    fn from(v: i32) -> Self {
        __skadapter__to_pin_modes(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PinValues {
    GpioLow = 0,    GpioHigh = 1,}

impl From<i32> for PinValues {
    fn from(v: i32) -> Self {
        __skadapter__to_pin_values(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pins {
    Pin1 = 1,    Pin2 = 2,    Pin3 = 3,    Pin4 = 4,    Pin5 = 5,    Pin6 = 6,    Pin7 = 7,    Pin8 = 8,    Pin9 = 9,    Pin10 = 10,    Pin11 = 11,    Pin12 = 12,    Pin13 = 13,    Pin14 = 14,    Pin15 = 15,    Pin16 = 16,    Pin17 = 17,    Pin18 = 18,    Pin19 = 19,    Pin20 = 20,    Pin21 = 21,    Pin22 = 22,    Pin23 = 23,    Pin24 = 24,    Pin25 = 25,    Pin26 = 26,    Pin27 = 27,    Pin28 = 28,    Pin29 = 29,    Pin30 = 30,    Pin31 = 31,    Pin32 = 32,    Pin33 = 33,    Pin34 = 34,    Pin35 = 35,    Pin36 = 36,    Pin37 = 37,    Pin38 = 38,    Pin39 = 39,    Pin40 = 40,}

impl From<i32> for Pins {
    fn from(v: i32) -> Self {
        __skadapter__to_pins(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PullUpDown {
    PudOff = 0,    PudDown = 1,    PudUp = 2,}

impl From<i32> for PullUpDown {
    fn from(v: i32) -> Self {
        __skadapter__to_pull_up_down(v)
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    HttpGetMethod,    HttpPostMethod,    HttpPutMethod,    HttpDeleteMethod,    HttpOptionsMethod,    HttpTraceMethod,    UnknownHttpMethod,}

impl From<i32> for HttpMethod {
    fn from(v: i32) -> Self {
        __skadapter__to_http_method(v)
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_matrix_2d {
    pub elements: [f64; 9],
}
impl __sklib_matrix_2d {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2D {
    pub elements: [[f64; 3]; 3],
}
impl Default for Matrix2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Matrix2D {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_point_2d {
    pub x: f64,
    pub y: f64,
}
impl __sklib_point_2d {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
impl Default for Point2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Point2D {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_circle {
    pub center: __sklib_point_2d,
    pub radius: f64,
}
impl __sklib_circle {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point2D,
    pub radius: f64,
}
impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}
impl Circle {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl __sklib_color {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}
impl Color {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl __sklib_rectangle {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}
impl Rectangle {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_drawing_options {
    pub dest: __sklib_ptr,
    pub scale_x: f32,
    pub scale_y: f32,
    pub angle: f32,
    pub anchor_offset_x: f32,
    pub anchor_offset_y: f32,
    pub flip_x: i32,
    pub flip_y: i32,
    pub is_part: i32,
    pub part: __sklib_rectangle,
    pub draw_cell: i32,
    pub camera: i32,
    pub line_width: i32,
    pub anim: __sklib_ptr,
}
impl __sklib_drawing_options {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawingOptions {
    pub dest: __sklib_ptr,
    pub scale_x: f32,
    pub scale_y: f32,
    pub angle: f32,
    pub anchor_offset_x: f32,
    pub anchor_offset_y: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub is_part: bool,
    pub part: Rectangle,
    pub draw_cell: i32,
    pub camera: DrawingDest,
    pub line_width: i32,
    pub anim: Animation,
}
impl Default for DrawingOptions {
    fn default() -> Self {
        Self::new()
    }
}
impl DrawingOptions {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_line {
    pub start_point: __sklib_point_2d,
    pub end_point: __sklib_point_2d,
}
impl __sklib_line {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub start_point: Point2D,
    pub end_point: Point2D,
}
impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}
impl Line {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_quad {
    pub points: [__sklib_point_2d; 4],
}
impl __sklib_quad {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    pub points: [Point2D; 4],
}
impl Default for Quad {
    fn default() -> Self {
        Self::new()
    }
}
impl Quad {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_triangle {
    pub points: [__sklib_point_2d; 3],
}
impl __sklib_triangle {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub points: [Point2D; 3],
}
impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}
impl Triangle {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __sklib_vector_2d {
    pub x: f64,
    pub y: f64,
}
impl __sklib_vector_2d {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}
impl Default for Vector2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Vector2D {
    pub fn new() -> Self {
        let uninit = MaybeUninit::zeroed();
        unsafe { uninit.assume_init() }
    }
    pub fn is_null(&self) -> bool {
        *self == Self::default()
    }
}
pub type KeyCallback = Box<dyn Fn(i32) -> () + Send + Sync>;
pub type __sklib_key_callback = extern "C" fn(i32) -> ();
pub type FreeNotifier = Box<dyn Fn(__sklib_ptr) -> () + Send + Sync>;
pub type __sklib_free_notifier = extern "C" fn(__sklib_ptr) -> ();
pub type SpriteEventHandler = Box<dyn Fn(__sklib_ptr, i32) -> () + Send + Sync>;
pub type __sklib_sprite_event_handler = extern "C" fn(__sklib_ptr, i32) -> ();
pub type SpriteFloatFunction = Box<dyn Fn(__sklib_ptr, f32) -> () + Send + Sync>;
pub type __sklib_sprite_float_function = extern "C" fn(__sklib_ptr, f32) -> ();
pub type SpriteFunction = Box<dyn Fn(__sklib_ptr) -> () + Send + Sync>;
pub type __sklib_sprite_function = extern "C" fn(__sklib_ptr) -> ();
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Json {
    ptr: __sklib_ptr,
}
impl Json {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Music {
    ptr: __sklib_ptr,
}
impl Music {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Connection {
    ptr: __sklib_ptr,
}
impl Connection {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message {
    ptr: __sklib_ptr,
}
impl Message {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerSocket {
    ptr: __sklib_ptr,
}
impl ServerSocket {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoundEffect {
    ptr: __sklib_ptr,
}
impl SoundEffect {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sprite {
    ptr: __sklib_ptr,
}
impl Sprite {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    ptr: __sklib_ptr,
}
impl Timer {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Animation {
    ptr: __sklib_ptr,
}
impl Animation {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationScript {
    ptr: __sklib_ptr,
}
impl AnimationScript {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitmap {
    ptr: __sklib_ptr,
}
impl Bitmap {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Display {
    ptr: __sklib_ptr,
}
impl Display {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Font {
    ptr: __sklib_ptr,
}
impl Font {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HttpResponse {
    ptr: __sklib_ptr,
}
impl HttpResponse {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HttpRequest {
    ptr: __sklib_ptr,
}
impl HttpRequest {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WebServer {
    ptr: __sklib_ptr,
}
impl WebServer {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    ptr: __sklib_ptr,
}
impl Window {
    fn new(ptr: __sklib_ptr) -> Self {
        Self { ptr }
    }

    fn as_ptr(&self) -> __sklib_ptr {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
extern "C" {
    #[link_name = "__sklib__animation_count__animation_script"]
    fn __sklib__animation_count__animation_script(script: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__animation_current_cell__animation"]
    fn __sklib__animation_current_cell__animation(anim: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__animation_current_vector__animation"]
    fn __sklib__animation_current_vector__animation(anim: __sklib_ptr) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__animation_ended__animation"]
    fn __sklib__animation_ended__animation(anim: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__animation_entered_frame__animation"]
    fn __sklib__animation_entered_frame__animation(anim: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__animation_frame_time__animation"]
    fn __sklib__animation_frame_time__animation(anim: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__animation_index__animation_script__string_ref"]
    fn __sklib__animation_index__animation_script__string_ref(script: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__animation_name__animation"]
    fn __sklib__animation_name__animation(temp: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__animation_script_name__animation_script"]
    fn __sklib__animation_script_name__animation_script(script: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__animation_script_named__string_ref"]
    fn __sklib__animation_script_named__string_ref(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__animation_script__string_ref"]
    fn __sklib__assign_animation__animation__animation_script__string_ref(anim: __sklib_ptr, script: __sklib_ptr, name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__animation_script__string_ref__bool"]
    fn __sklib__assign_animation__animation__animation_script__string_ref__bool(anim: __sklib_ptr, script: __sklib_ptr, name: __sklib_string, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__animation_script__int"]
    fn __sklib__assign_animation__animation__animation_script__int(anim: __sklib_ptr, script: __sklib_ptr, idx: i32);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__animation_script__int__bool"]
    fn __sklib__assign_animation__animation__animation_script__int__bool(anim: __sklib_ptr, script: __sklib_ptr, idx: i32, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__string_ref__string_ref"]
    fn __sklib__assign_animation__animation__string_ref__string_ref(anim: __sklib_ptr, script_name: __sklib_string, name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__string_ref__string_ref__bool"]
    fn __sklib__assign_animation__animation__string_ref__string_ref__bool(anim: __sklib_ptr, script_name: __sklib_string, name: __sklib_string, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__int"]
    fn __sklib__assign_animation__animation__int(anim: __sklib_ptr, idx: i32);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__int__bool"]
    fn __sklib__assign_animation__animation__int__bool(anim: __sklib_ptr, idx: i32, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__string"]
    fn __sklib__assign_animation__animation__string(anim: __sklib_ptr, name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__assign_animation__animation__string__bool"]
    fn __sklib__assign_animation__animation__string__bool(anim: __sklib_ptr, name: __sklib_string, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__create_animation__animation_script__int__bool"]
    fn __sklib__create_animation__animation_script__int__bool(script: __sklib_ptr, idx: i32, with_sound: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_animation__animation_script__string_ref"]
    fn __sklib__create_animation__animation_script__string_ref(script: __sklib_ptr, name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_animation__animation_script__string_ref__bool"]
    fn __sklib__create_animation__animation_script__string_ref__bool(script: __sklib_ptr, name: __sklib_string, with_sound: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_animation__string_ref__string_ref"]
    fn __sklib__create_animation__string_ref__string_ref(script_name: __sklib_string, name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_animation__string_ref__string_ref__bool"]
    fn __sklib__create_animation__string_ref__string_ref__bool(script_name: __sklib_string, name: __sklib_string, with_sound: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__free_all_animation_scripts"]
    fn __sklib__free_all_animation_scripts();
}
extern "C" {
    #[link_name = "__sklib__free_animation__animation"]
    fn __sklib__free_animation__animation(ani: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__free_animation_script__animation_script"]
    fn __sklib__free_animation_script__animation_script(script_to_free: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__free_animation_script__string_ref"]
    fn __sklib__free_animation_script__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__has_animation_named__animation_script__string_ref"]
    fn __sklib__has_animation_named__animation_script__string_ref(script: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_animation_script__string_ref"]
    fn __sklib__has_animation_script__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__load_animation_script__string_ref__string_ref"]
    fn __sklib__load_animation_script__string_ref__string_ref(name: __sklib_string, filename: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__restart_animation__animation"]
    fn __sklib__restart_animation__animation(anim: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__restart_animation__animation__bool"]
    fn __sklib__restart_animation__animation__bool(anim: __sklib_ptr, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__update_animation__animation__float__bool"]
    fn __sklib__update_animation__animation__float__bool(anim: __sklib_ptr, pct: f32, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__update_animation__animation"]
    fn __sklib__update_animation__animation(anim: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__update_animation__animation__float"]
    fn __sklib__update_animation__animation__float(anim: __sklib_ptr, pct: f32);
}
extern "C" {
    #[link_name = "__sklib__audio_ready"]
    fn __sklib__audio_ready() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__close_audio"]
    fn __sklib__close_audio();
}
extern "C" {
    #[link_name = "__sklib__open_audio"]
    fn __sklib__open_audio();
}
extern "C" {
    #[link_name = "__sklib__contains__string_ref__string_ref"]
    fn __sklib__contains__string_ref__string_ref(text: __sklib_string, subtext: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__convert_to_double__string_ref"]
    fn __sklib__convert_to_double__string_ref(text: __sklib_string) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__convert_to_integer__string_ref"]
    fn __sklib__convert_to_integer__string_ref(text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__index_of__string_ref__string_ref"]
    fn __sklib__index_of__string_ref__string_ref(text: __sklib_string, subtext: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_double__string_ref"]
    fn __sklib__is_double__string_ref(text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_integer__string_ref"]
    fn __sklib__is_integer__string_ref(text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_number__string_ref"]
    fn __sklib__is_number__string_ref(text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__length_of__string_ref"]
    fn __sklib__length_of__string_ref(text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__replace_all__string_ref__string_ref__string_ref"]
    fn __sklib__replace_all__string_ref__string_ref__string_ref(text: __sklib_string, substr: __sklib_string, newtext: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__split__string_ref__char"]
    fn __sklib__split__string_ref__char(text: __sklib_string, delimiter: c_char) 
        -> __sklib_vector_string;
}
extern "C" {
    #[link_name = "__sklib__to_lowercase__string_ref"]
    fn __sklib__to_lowercase__string_ref(text: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__to_uppercase__string_ref"]
    fn __sklib__to_uppercase__string_ref(text: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__trim__string_ref"]
    fn __sklib__trim__string_ref(text: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__free_resource_bundle__string"]
    fn __sklib__free_resource_bundle__string(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__has_resource_bundle__string_ref"]
    fn __sklib__has_resource_bundle__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__load_resource_bundle__string_ref__string_ref"]
    fn __sklib__load_resource_bundle__string_ref__string_ref(name: __sklib_string, filename: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__camera_position"]
    fn __sklib__camera_position() 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__camera_x"]
    fn __sklib__camera_x() 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__camera_y"]
    fn __sklib__camera_y() 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__center_camera_on__sprite__vector_2d_ref"]
    fn __sklib__center_camera_on__sprite__vector_2d_ref(s: __sklib_ptr, offset: __sklib_vector_2d);
}
extern "C" {
    #[link_name = "__sklib__center_camera_on__sprite__double__double"]
    fn __sklib__center_camera_on__sprite__double__double(s: __sklib_ptr, offset_x: f64, offset_y: f64);
}
extern "C" {
    #[link_name = "__sklib__move_camera_by__vector_2d_ref"]
    fn __sklib__move_camera_by__vector_2d_ref(offset: __sklib_vector_2d);
}
extern "C" {
    #[link_name = "__sklib__move_camera_by__double__double"]
    fn __sklib__move_camera_by__double__double(dx: f64, dy: f64);
}
extern "C" {
    #[link_name = "__sklib__move_camera_to__point_2d_ref"]
    fn __sklib__move_camera_to__point_2d_ref(pt: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__move_camera_to__double__double"]
    fn __sklib__move_camera_to__double__double(x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__point_in_window__window__point_2d_ref"]
    fn __sklib__point_in_window__window__point_2d_ref(wind: __sklib_ptr, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_on_screen__point_2d_ref"]
    fn __sklib__point_on_screen__point_2d_ref(pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__rect_in_window__window__rectangle_ref"]
    fn __sklib__rect_in_window__window__rectangle_ref(wind: __sklib_ptr, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__rect_on_screen__rectangle_ref"]
    fn __sklib__rect_on_screen__rectangle_ref(rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__screen_center"]
    fn __sklib__screen_center() 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__screen_rectangle"]
    fn __sklib__screen_rectangle() 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__set_camera_position__point_2d"]
    fn __sklib__set_camera_position__point_2d(pos: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__set_camera_x__double"]
    fn __sklib__set_camera_x__double(x: f64);
}
extern "C" {
    #[link_name = "__sklib__set_camera_y__double"]
    fn __sklib__set_camera_y__double(y: f64);
}
extern "C" {
    #[link_name = "__sklib__to_screen__point_2d_ref"]
    fn __sklib__to_screen__point_2d_ref(pt: __sklib_point_2d) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__to_screen__rectangle_ref"]
    fn __sklib__to_screen__rectangle_ref(rect: __sklib_rectangle) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__to_screen_x__double"]
    fn __sklib__to_screen_x__double(world_x: f64) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__to_screen_y__double"]
    fn __sklib__to_screen_y__double(world_y: f64) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__to_world__point_2d_ref"]
    fn __sklib__to_world__point_2d_ref(pt: __sklib_point_2d) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__to_world_x__double"]
    fn __sklib__to_world_x__double(screen_x: f64) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__to_world_y__double"]
    fn __sklib__to_world_y__double(screen_y: f64) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__vector_world_to_screen"]
    fn __sklib__vector_world_to_screen() 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__window_area__window"]
    fn __sklib__window_area__window(wind: __sklib_ptr) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__draw_circle__color__circle_ref"]
    fn __sklib__draw_circle__color__circle_ref(clr: __sklib_color, c: __sklib_circle);
}
extern "C" {
    #[link_name = "__sklib__draw_circle__color__circle_ref__drawing_options"]
    fn __sklib__draw_circle__color__circle_ref__drawing_options(clr: __sklib_color, c: __sklib_circle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_circle__color__double__double__double"]
    fn __sklib__draw_circle__color__double__double__double(clr: __sklib_color, x: f64, y: f64, radius: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_circle__color__double__double__double__drawing_options"]
    fn __sklib__draw_circle__color__double__double__double__drawing_options(clr: __sklib_color, x: f64, y: f64, radius: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_circle_on_bitmap__bitmap__color__double__double__double"]
    fn __sklib__draw_circle_on_bitmap__bitmap__color__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, radius: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_circle_on_bitmap__bitmap__color__double__double__double__drawing_options"]
    fn __sklib__draw_circle_on_bitmap__bitmap__color__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, radius: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_circle_on_window__window__color__double__double__double"]
    fn __sklib__draw_circle_on_window__window__color__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, radius: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_circle_on_window__window__color__double__double__double__drawing_options"]
    fn __sklib__draw_circle_on_window__window__color__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, radius: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_circle__color__circle_ref"]
    fn __sklib__fill_circle__color__circle_ref(clr: __sklib_color, c: __sklib_circle);
}
extern "C" {
    #[link_name = "__sklib__fill_circle__color__circle_ref__drawing_options"]
    fn __sklib__fill_circle__color__circle_ref__drawing_options(clr: __sklib_color, c: __sklib_circle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_circle__color__double__double__double"]
    fn __sklib__fill_circle__color__double__double__double(clr: __sklib_color, x: f64, y: f64, radius: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_circle__color__double__double__double__drawing_options"]
    fn __sklib__fill_circle__color__double__double__double__drawing_options(clr: __sklib_color, x: f64, y: f64, radius: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_circle_on_bitmap__bitmap__color__double__double__double"]
    fn __sklib__fill_circle_on_bitmap__bitmap__color__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, radius: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_circle_on_bitmap__bitmap__color__double__double__double__drawing_options"]
    fn __sklib__fill_circle_on_bitmap__bitmap__color__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, radius: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_circle_on_window__window__color__double__double__double"]
    fn __sklib__fill_circle_on_window__window__color__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, radius: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_circle_on_window__window__color__double__double__double__drawing_options"]
    fn __sklib__fill_circle_on_window__window__color__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, radius: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__center_point__circle_ref"]
    fn __sklib__center_point__circle_ref(c: __sklib_circle) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__circle_at__point_2d_ref__double"]
    fn __sklib__circle_at__point_2d_ref__double(pt: __sklib_point_2d, radius: f64) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__circle_at__double__double__double"]
    fn __sklib__circle_at__double__double__double(x: f64, y: f64, radius: f64) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__circle_radius__circle"]
    fn __sklib__circle_radius__circle(c: __sklib_circle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__circle_triangle_intersect__circle_ref__triangle_ref"]
    fn __sklib__circle_triangle_intersect__circle_ref__triangle_ref(c: __sklib_circle, tri: __sklib_triangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__circle_triangle_intersect__circle_ref__triangle_ref__point_2d_ref"]
    fn __sklib__circle_triangle_intersect__circle_ref__triangle_ref__point_2d_ref(c: __sklib_circle, tri: __sklib_triangle, p: &mut __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__circle_x__circle_ref"]
    fn __sklib__circle_x__circle_ref(c: __sklib_circle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__circle_y__circle_ref"]
    fn __sklib__circle_y__circle_ref(c: __sklib_circle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__circles_intersect__circle__circle"]
    fn __sklib__circles_intersect__circle__circle(c1: __sklib_circle, c2: __sklib_circle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__circles_intersect__double__double__double__double__double__double"]
    fn __sklib__circles_intersect__double__double__double__double__double__double(c1_x: f64, c1_y: f64, c1_radius: f64, c2_x: f64, c2_y: f64, c2_radius: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__closest_point_on_circle__point_2d_ref__circle_ref"]
    fn __sklib__closest_point_on_circle__point_2d_ref__circle_ref(from_pt: __sklib_point_2d, c: __sklib_circle) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__closest_point_on_line_from_circle__circle_ref__line_ref"]
    fn __sklib__closest_point_on_line_from_circle__circle_ref__line_ref(c: __sklib_circle, l: __sklib_line) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__closest_point_on_rect_from_circle__circle_ref__rectangle_ref"]
    fn __sklib__closest_point_on_rect_from_circle__circle_ref__rectangle_ref(c: __sklib_circle, rect: __sklib_rectangle) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__closest_point_on_triangle_from_circle__circle_ref__triangle_ref"]
    fn __sklib__closest_point_on_triangle_from_circle__circle_ref__triangle_ref(c: __sklib_circle, tri: __sklib_triangle) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__distant_point_on_circle__point_2d_ref__circle_ref"]
    fn __sklib__distant_point_on_circle__point_2d_ref__circle_ref(pt: __sklib_point_2d, c: __sklib_circle) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__distant_point_on_circle_heading__point_2d_ref__circle_ref__vector_2d_ref__point_2d_ref"]
    fn __sklib__distant_point_on_circle_heading__point_2d_ref__circle_ref__vector_2d_ref__point_2d_ref(pt: __sklib_point_2d, c: __sklib_circle, heading: __sklib_vector_2d, opposite_pt: &mut __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__ray_circle_intersect_distance__point_2d_ref__vector_2d_ref__circle_ref"]
    fn __sklib__ray_circle_intersect_distance__point_2d_ref__vector_2d_ref__circle_ref(ray_origin: __sklib_point_2d, ray_heading: __sklib_vector_2d, c: __sklib_circle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__tangent_points__point_2d_ref__circle_ref__point_2d_ref__point_2d_ref"]
    fn __sklib__tangent_points__point_2d_ref__circle_ref__point_2d_ref__point_2d_ref(from_pt: __sklib_point_2d, c: __sklib_circle, p1: &mut __sklib_point_2d, p2: &mut __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__widest_points__circle_ref__vector_2d_ref__point_2d_ref__point_2d_ref"]
    fn __sklib__widest_points__circle_ref__vector_2d_ref__point_2d_ref__point_2d_ref(c: __sklib_circle, along: __sklib_vector_2d, pt1: &mut __sklib_point_2d, pt2: &mut __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__current_clip"]
    fn __sklib__current_clip() 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__current_clip__bitmap"]
    fn __sklib__current_clip__bitmap(bmp: __sklib_ptr) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__current_clip__window"]
    fn __sklib__current_clip__window(wnd: __sklib_ptr) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__pop_clip__window"]
    fn __sklib__pop_clip__window(wnd: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__pop_clip"]
    fn __sklib__pop_clip();
}
extern "C" {
    #[link_name = "__sklib__pop_clip__bitmap"]
    fn __sklib__pop_clip__bitmap(bmp: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__push_clip__window__rectangle_ref"]
    fn __sklib__push_clip__window__rectangle_ref(wnd: __sklib_ptr, r: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__push_clip__bitmap__rectangle_ref"]
    fn __sklib__push_clip__bitmap__rectangle_ref(bmp: __sklib_ptr, r: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__push_clip__rectangle_ref"]
    fn __sklib__push_clip__rectangle_ref(r: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__reset_clip__bitmap"]
    fn __sklib__reset_clip__bitmap(bmp: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__reset_clip"]
    fn __sklib__reset_clip();
}
extern "C" {
    #[link_name = "__sklib__reset_clip__window"]
    fn __sklib__reset_clip__window(wnd: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__set_clip__rectangle_ref"]
    fn __sklib__set_clip__rectangle_ref(r: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__set_clip__bitmap__rectangle_ref"]
    fn __sklib__set_clip__bitmap__rectangle_ref(bmp: __sklib_ptr, r: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__set_clip__window__rectangle_ref"]
    fn __sklib__set_clip__window__rectangle_ref(wnd: __sklib_ptr, r: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__bitmap_circle_collision__bitmap__point_2d_ref__circle_ref"]
    fn __sklib__bitmap_circle_collision__bitmap__point_2d_ref__circle_ref(bmp: __sklib_ptr, pt: __sklib_point_2d, circ: __sklib_circle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_circle_collision__bitmap__double__double__circle_ref"]
    fn __sklib__bitmap_circle_collision__bitmap__double__double__circle_ref(bmp: __sklib_ptr, x: f64, y: f64, circ: __sklib_circle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_circle_collision__bitmap__int__matrix_2d_ref__circle_ref"]
    fn __sklib__bitmap_circle_collision__bitmap__int__matrix_2d_ref__circle_ref(bmp: __sklib_ptr, cell: i32, translation: __sklib_matrix_2d, circ: __sklib_circle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_circle_collision__bitmap__int__point_2d_ref__circle_ref"]
    fn __sklib__bitmap_circle_collision__bitmap__int__point_2d_ref__circle_ref(bmp: __sklib_ptr, cell: i32, pt: __sklib_point_2d, circ: __sklib_circle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_circle_collision__bitmap__int__double__double__circle_ref"]
    fn __sklib__bitmap_circle_collision__bitmap__int__double__double__circle_ref(bmp: __sklib_ptr, cell: i32, x: f64, y: f64, circ: __sklib_circle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_collision__bitmap__double__double__bitmap__double__double"]
    fn __sklib__bitmap_collision__bitmap__double__double__bitmap__double__double(bmp1: __sklib_ptr, x1: f64, y1: f64, bmp2: __sklib_ptr, x2: f64, y2: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_collision__bitmap__point_2d_ref__bitmap__point_2d_ref"]
    fn __sklib__bitmap_collision__bitmap__point_2d_ref__bitmap__point_2d_ref(bmp1: __sklib_ptr, pt1: __sklib_point_2d, bmp2: __sklib_ptr, pt2: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_collision__bitmap__int__matrix_2d_ref__bitmap__int__matrix_2d_ref"]
    fn __sklib__bitmap_collision__bitmap__int__matrix_2d_ref__bitmap__int__matrix_2d_ref(bmp1: __sklib_ptr, cell1: i32, matrix1: __sklib_matrix_2d, bmp2: __sklib_ptr, cell2: i32, matrix2: __sklib_matrix_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_collision__bitmap__int__point_2d_ref__bitmap__int__point_2d_ref"]
    fn __sklib__bitmap_collision__bitmap__int__point_2d_ref__bitmap__int__point_2d_ref(bmp1: __sklib_ptr, cell1: i32, pt1: __sklib_point_2d, bmp2: __sklib_ptr, cell2: i32, pt2: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_collision__bitmap__int__double__double__bitmap__int__double__double"]
    fn __sklib__bitmap_collision__bitmap__int__double__double__bitmap__int__double__double(bmp1: __sklib_ptr, cell1: i32, x1: f64, y1: f64, bmp2: __sklib_ptr, cell2: i32, x2: f64, y2: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_point_collision__bitmap__matrix_2d_ref__point_2d_ref"]
    fn __sklib__bitmap_point_collision__bitmap__matrix_2d_ref__point_2d_ref(bmp: __sklib_ptr, translation: __sklib_matrix_2d, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_point_collision__bitmap__point_2d_ref__point_2d_ref"]
    fn __sklib__bitmap_point_collision__bitmap__point_2d_ref__point_2d_ref(bmp: __sklib_ptr, bmp_pt: __sklib_point_2d, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_point_collision__bitmap__double__double__double__double"]
    fn __sklib__bitmap_point_collision__bitmap__double__double__double__double(bmp: __sklib_ptr, bmp_x: f64, bmp_y: f64, x: f64, y: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_point_collision__bitmap__int__matrix_2d_ref__point_2d_ref"]
    fn __sklib__bitmap_point_collision__bitmap__int__matrix_2d_ref__point_2d_ref(bmp: __sklib_ptr, cell: i32, translation: __sklib_matrix_2d, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_point_collision__bitmap__int__point_2d_ref__point_2d_ref"]
    fn __sklib__bitmap_point_collision__bitmap__int__point_2d_ref__point_2d_ref(bmp: __sklib_ptr, cell: i32, bmp_pt: __sklib_point_2d, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_point_collision__bitmap__int__double__double__double__double"]
    fn __sklib__bitmap_point_collision__bitmap__int__double__double__double__double(bmp: __sklib_ptr, cell: i32, bmp_x: f64, bmp_y: f64, x: f64, y: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_rectangle_collision__bitmap__point_2d_ref__rectangle_ref"]
    fn __sklib__bitmap_rectangle_collision__bitmap__point_2d_ref__rectangle_ref(bmp: __sklib_ptr, pt: __sklib_point_2d, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_rectangle_collision__bitmap__double__double__rectangle_ref"]
    fn __sklib__bitmap_rectangle_collision__bitmap__double__double__rectangle_ref(bmp: __sklib_ptr, x: f64, y: f64, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_rectangle_collision__bitmap__int__matrix_2d_ref__rectangle_ref"]
    fn __sklib__bitmap_rectangle_collision__bitmap__int__matrix_2d_ref__rectangle_ref(bmp: __sklib_ptr, cell: i32, translation: __sklib_matrix_2d, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_rectangle_collision__bitmap__int__point_2d_ref__rectangle_ref"]
    fn __sklib__bitmap_rectangle_collision__bitmap__int__point_2d_ref__rectangle_ref(bmp: __sklib_ptr, cell: i32, pt: __sklib_point_2d, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_rectangle_collision__bitmap__int__double__double__rectangle_ref"]
    fn __sklib__bitmap_rectangle_collision__bitmap__int__double__double__rectangle_ref(bmp: __sklib_ptr, cell: i32, x: f64, y: f64, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_bitmap_collision__sprite__bitmap__double__double"]
    fn __sklib__sprite_bitmap_collision__sprite__bitmap__double__double(s: __sklib_ptr, bmp: __sklib_ptr, x: f64, y: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_bitmap_collision__sprite__bitmap__int__point_2d_ref"]
    fn __sklib__sprite_bitmap_collision__sprite__bitmap__int__point_2d_ref(s: __sklib_ptr, bmp: __sklib_ptr, cell: i32, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_bitmap_collision__sprite__bitmap__int__double__double"]
    fn __sklib__sprite_bitmap_collision__sprite__bitmap__int__double__double(s: __sklib_ptr, bmp: __sklib_ptr, cell: i32, x: f64, y: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_collision__sprite__sprite"]
    fn __sklib__sprite_collision__sprite__sprite(s1: __sklib_ptr, s2: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_point_collision__sprite__point_2d_ref"]
    fn __sklib__sprite_point_collision__sprite__point_2d_ref(s: __sklib_ptr, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_rectangle_collision__sprite__rectangle_ref"]
    fn __sklib__sprite_rectangle_collision__sprite__rectangle_ref(s: __sklib_ptr, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__alpha_of__color"]
    fn __sklib__alpha_of__color(c: __sklib_color) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__blue_of__color"]
    fn __sklib__blue_of__color(c: __sklib_color) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__brightness_of__color"]
    fn __sklib__brightness_of__color(c: __sklib_color) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__color_alice_blue"]
    fn __sklib__color_alice_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_antique_white"]
    fn __sklib__color_antique_white() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_aqua"]
    fn __sklib__color_aqua() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_aquamarine"]
    fn __sklib__color_aquamarine() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_azure"]
    fn __sklib__color_azure() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_beige"]
    fn __sklib__color_beige() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_bisque"]
    fn __sklib__color_bisque() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_black"]
    fn __sklib__color_black() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_blanched_almond"]
    fn __sklib__color_blanched_almond() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_blue"]
    fn __sklib__color_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_blue_violet"]
    fn __sklib__color_blue_violet() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_bright_green"]
    fn __sklib__color_bright_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_brown"]
    fn __sklib__color_brown() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_burly_wood"]
    fn __sklib__color_burly_wood() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_cadet_blue"]
    fn __sklib__color_cadet_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_chartreuse"]
    fn __sklib__color_chartreuse() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_chocolate"]
    fn __sklib__color_chocolate() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_coral"]
    fn __sklib__color_coral() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_cornflower_blue"]
    fn __sklib__color_cornflower_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_cornsilk"]
    fn __sklib__color_cornsilk() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_crimson"]
    fn __sklib__color_crimson() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_cyan"]
    fn __sklib__color_cyan() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_blue"]
    fn __sklib__color_dark_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_cyan"]
    fn __sklib__color_dark_cyan() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_goldenrod"]
    fn __sklib__color_dark_goldenrod() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_gray"]
    fn __sklib__color_dark_gray() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_green"]
    fn __sklib__color_dark_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_khaki"]
    fn __sklib__color_dark_khaki() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_magenta"]
    fn __sklib__color_dark_magenta() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_olive_green"]
    fn __sklib__color_dark_olive_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_orange"]
    fn __sklib__color_dark_orange() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_orchid"]
    fn __sklib__color_dark_orchid() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_red"]
    fn __sklib__color_dark_red() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_salmon"]
    fn __sklib__color_dark_salmon() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_sea_green"]
    fn __sklib__color_dark_sea_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_slate_blue"]
    fn __sklib__color_dark_slate_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_slate_gray"]
    fn __sklib__color_dark_slate_gray() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_turquoise"]
    fn __sklib__color_dark_turquoise() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dark_violet"]
    fn __sklib__color_dark_violet() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_deep_pink"]
    fn __sklib__color_deep_pink() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_deep_sky_blue"]
    fn __sklib__color_deep_sky_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dim_gray"]
    fn __sklib__color_dim_gray() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_dodger_blue"]
    fn __sklib__color_dodger_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_firebrick"]
    fn __sklib__color_firebrick() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_floral_white"]
    fn __sklib__color_floral_white() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_forest_green"]
    fn __sklib__color_forest_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_fuchsia"]
    fn __sklib__color_fuchsia() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_gainsboro"]
    fn __sklib__color_gainsboro() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_ghost_white"]
    fn __sklib__color_ghost_white() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_gold"]
    fn __sklib__color_gold() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_goldenrod"]
    fn __sklib__color_goldenrod() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_gray"]
    fn __sklib__color_gray() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_green"]
    fn __sklib__color_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_green_yellow"]
    fn __sklib__color_green_yellow() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_honeydew"]
    fn __sklib__color_honeydew() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_hot_pink"]
    fn __sklib__color_hot_pink() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_indian_red"]
    fn __sklib__color_indian_red() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_indigo"]
    fn __sklib__color_indigo() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_ivory"]
    fn __sklib__color_ivory() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_khaki"]
    fn __sklib__color_khaki() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_lavender"]
    fn __sklib__color_lavender() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_lavender_blush"]
    fn __sklib__color_lavender_blush() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_lawn_green"]
    fn __sklib__color_lawn_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_lemon_chiffon"]
    fn __sklib__color_lemon_chiffon() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_blue"]
    fn __sklib__color_light_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_coral"]
    fn __sklib__color_light_coral() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_cyan"]
    fn __sklib__color_light_cyan() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_goldenrod_yellow"]
    fn __sklib__color_light_goldenrod_yellow() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_gray"]
    fn __sklib__color_light_gray() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_green"]
    fn __sklib__color_light_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_pink"]
    fn __sklib__color_light_pink() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_salmon"]
    fn __sklib__color_light_salmon() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_sea_green"]
    fn __sklib__color_light_sea_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_sky_blue"]
    fn __sklib__color_light_sky_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_slate_gray"]
    fn __sklib__color_light_slate_gray() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_steel_blue"]
    fn __sklib__color_light_steel_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_light_yellow"]
    fn __sklib__color_light_yellow() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_lime"]
    fn __sklib__color_lime() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_lime_green"]
    fn __sklib__color_lime_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_linen"]
    fn __sklib__color_linen() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_magenta"]
    fn __sklib__color_magenta() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_maroon"]
    fn __sklib__color_maroon() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_aquamarine"]
    fn __sklib__color_medium_aquamarine() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_blue"]
    fn __sklib__color_medium_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_orchid"]
    fn __sklib__color_medium_orchid() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_purple"]
    fn __sklib__color_medium_purple() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_sea_green"]
    fn __sklib__color_medium_sea_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_slate_blue"]
    fn __sklib__color_medium_slate_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_spring_green"]
    fn __sklib__color_medium_spring_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_turquoise"]
    fn __sklib__color_medium_turquoise() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_medium_violet_red"]
    fn __sklib__color_medium_violet_red() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_midnight_blue"]
    fn __sklib__color_midnight_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_mint_cream"]
    fn __sklib__color_mint_cream() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_misty_rose"]
    fn __sklib__color_misty_rose() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_moccasin"]
    fn __sklib__color_moccasin() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_navajo_white"]
    fn __sklib__color_navajo_white() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_navy"]
    fn __sklib__color_navy() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_old_lace"]
    fn __sklib__color_old_lace() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_olive"]
    fn __sklib__color_olive() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_olive_drab"]
    fn __sklib__color_olive_drab() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_orange"]
    fn __sklib__color_orange() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_orange_red"]
    fn __sklib__color_orange_red() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_orchid"]
    fn __sklib__color_orchid() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_pale_goldenrod"]
    fn __sklib__color_pale_goldenrod() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_pale_green"]
    fn __sklib__color_pale_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_pale_turquoise"]
    fn __sklib__color_pale_turquoise() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_pale_violet_red"]
    fn __sklib__color_pale_violet_red() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_papaya_whip"]
    fn __sklib__color_papaya_whip() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_peach_puff"]
    fn __sklib__color_peach_puff() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_peru"]
    fn __sklib__color_peru() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_pink"]
    fn __sklib__color_pink() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_plum"]
    fn __sklib__color_plum() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_powder_blue"]
    fn __sklib__color_powder_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_purple"]
    fn __sklib__color_purple() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_red"]
    fn __sklib__color_red() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_rosy_brown"]
    fn __sklib__color_rosy_brown() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_royal_blue"]
    fn __sklib__color_royal_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_saddle_brown"]
    fn __sklib__color_saddle_brown() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_salmon"]
    fn __sklib__color_salmon() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_sandy_brown"]
    fn __sklib__color_sandy_brown() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_sea_green"]
    fn __sklib__color_sea_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_sea_shell"]
    fn __sklib__color_sea_shell() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_sienna"]
    fn __sklib__color_sienna() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_silver"]
    fn __sklib__color_silver() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_sky_blue"]
    fn __sklib__color_sky_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_slate_blue"]
    fn __sklib__color_slate_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_slate_gray"]
    fn __sklib__color_slate_gray() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_snow"]
    fn __sklib__color_snow() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_spring_green"]
    fn __sklib__color_spring_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_steel_blue"]
    fn __sklib__color_steel_blue() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_swinburne_red"]
    fn __sklib__color_swinburne_red() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_tan"]
    fn __sklib__color_tan() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_teal"]
    fn __sklib__color_teal() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_thistle"]
    fn __sklib__color_thistle() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_to_string__color"]
    fn __sklib__color_to_string__color(c: __sklib_color) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__color_tomato"]
    fn __sklib__color_tomato() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_transparent"]
    fn __sklib__color_transparent() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_turquoise"]
    fn __sklib__color_turquoise() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_violet"]
    fn __sklib__color_violet() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_wheat"]
    fn __sklib__color_wheat() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_white"]
    fn __sklib__color_white() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_white_smoke"]
    fn __sklib__color_white_smoke() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_yellow"]
    fn __sklib__color_yellow() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_yellow_green"]
    fn __sklib__color_yellow_green() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__green_of__color"]
    fn __sklib__green_of__color(c: __sklib_color) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__hsb_color__double__double__double"]
    fn __sklib__hsb_color__double__double__double(hue: f64, saturation: f64, brightness: f64) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__hue_of__color"]
    fn __sklib__hue_of__color(c: __sklib_color) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__random_color"]
    fn __sklib__random_color() 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__random_rgb_color__int"]
    fn __sklib__random_rgb_color__int(alpha: i32) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__red_of__color"]
    fn __sklib__red_of__color(c: __sklib_color) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__rgb_color__double__double__double"]
    fn __sklib__rgb_color__double__double__double(red: f64, green: f64, blue: f64) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__rgb_color__int__int__int"]
    fn __sklib__rgb_color__int__int__int(red: i32, green: i32, blue: i32) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__rgba_color__double__double__double__double"]
    fn __sklib__rgba_color__double__double__double__double(red: f64, green: f64, blue: f64, alpha: f64) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__rgba_color__int__int__int__int"]
    fn __sklib__rgba_color__int__int__int__int(red: i32, green: i32, blue: i32, alpha: i32) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__saturation_of__color"]
    fn __sklib__saturation_of__color(c: __sklib_color) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__string_to_color__string"]
    fn __sklib__string_to_color__string(str: __sklib_string) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__option_defaults"]
    fn __sklib__option_defaults() 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_draw_to__bitmap"]
    fn __sklib__option_draw_to__bitmap(dest: __sklib_ptr) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_draw_to__bitmap__drawing_options"]
    fn __sklib__option_draw_to__bitmap__drawing_options(dest: __sklib_ptr, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_draw_to__window"]
    fn __sklib__option_draw_to__window(dest: __sklib_ptr) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_draw_to__window__drawing_options"]
    fn __sklib__option_draw_to__window__drawing_options(dest: __sklib_ptr, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_flip_x"]
    fn __sklib__option_flip_x() 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_flip_x__drawing_options"]
    fn __sklib__option_flip_x__drawing_options(opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_flip_xy"]
    fn __sklib__option_flip_xy() 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_flip_xy__drawing_options"]
    fn __sklib__option_flip_xy__drawing_options(opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_flip_y"]
    fn __sklib__option_flip_y() 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_flip_y__drawing_options"]
    fn __sklib__option_flip_y__drawing_options(opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_line_width__int"]
    fn __sklib__option_line_width__int(width: i32) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_line_width__int__drawing_options"]
    fn __sklib__option_line_width__int__drawing_options(width: i32, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_part_bmp__double__double__double__double"]
    fn __sklib__option_part_bmp__double__double__double__double(x: f64, y: f64, w: f64, h: f64) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_part_bmp__double__double__double__double__drawing_options"]
    fn __sklib__option_part_bmp__double__double__double__double__drawing_options(x: f64, y: f64, w: f64, h: f64, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_part_bmp__rectangle"]
    fn __sklib__option_part_bmp__rectangle(part: __sklib_rectangle) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_part_bmp__rectangle__drawing_options"]
    fn __sklib__option_part_bmp__rectangle__drawing_options(part: __sklib_rectangle, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_rotate_bmp__double"]
    fn __sklib__option_rotate_bmp__double(angle: f64) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_rotate_bmp__double__double__double"]
    fn __sklib__option_rotate_bmp__double__double__double(angle: f64, anchor_x: f64, anchor_y: f64) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_rotate_bmp__double__double__double__drawing_options"]
    fn __sklib__option_rotate_bmp__double__double__double__drawing_options(angle: f64, anchor_x: f64, anchor_y: f64, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_rotate_bmp__double__drawing_options"]
    fn __sklib__option_rotate_bmp__double__drawing_options(angle: f64, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_scale_bmp__double__double"]
    fn __sklib__option_scale_bmp__double__double(scale_x: f64, scale_y: f64) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_scale_bmp__double__double__drawing_options"]
    fn __sklib__option_scale_bmp__double__double__drawing_options(scale_x: f64, scale_y: f64, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_to_screen"]
    fn __sklib__option_to_screen() 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_to_screen__drawing_options"]
    fn __sklib__option_to_screen__drawing_options(opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_to_world"]
    fn __sklib__option_to_world() 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_to_world__drawing_options"]
    fn __sklib__option_to_world__drawing_options(opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_with_animation__animation"]
    fn __sklib__option_with_animation__animation(anim: __sklib_ptr) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_with_animation__animation__drawing_options"]
    fn __sklib__option_with_animation__animation__drawing_options(anim: __sklib_ptr, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_with_bitmap_cell__int"]
    fn __sklib__option_with_bitmap_cell__int(cell: i32) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__option_with_bitmap_cell__int__drawing_options"]
    fn __sklib__option_with_bitmap_cell__int__drawing_options(cell: i32, opts: __sklib_drawing_options) 
        -> __sklib_drawing_options;
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse__color__rectangle"]
    fn __sklib__draw_ellipse__color__rectangle(clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse__color__rectangle__drawing_options"]
    fn __sklib__draw_ellipse__color__rectangle__drawing_options(clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse__color__double__double__double__double"]
    fn __sklib__draw_ellipse__color__double__double__double__double(clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse__color__double__double__double__double__drawing_options"]
    fn __sklib__draw_ellipse__color__double__double__double__double__drawing_options(clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse_on_bitmap__bitmap__color__rectangle"]
    fn __sklib__draw_ellipse_on_bitmap__bitmap__color__rectangle(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse_on_bitmap__bitmap__color__rectangle__drawing_options"]
    fn __sklib__draw_ellipse_on_bitmap__bitmap__color__rectangle__drawing_options(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse_on_bitmap__bitmap__color__double__double__double__double"]
    fn __sklib__draw_ellipse_on_bitmap__bitmap__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse_on_bitmap__bitmap__color__double__double__double__double__drawing_options"]
    fn __sklib__draw_ellipse_on_bitmap__bitmap__color__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse_on_window__window__color__rectangle"]
    fn __sklib__draw_ellipse_on_window__window__color__rectangle(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse_on_window__window__color__rectangle__drawing_options"]
    fn __sklib__draw_ellipse_on_window__window__color__rectangle__drawing_options(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse_on_window__window__color__double__double__double__double"]
    fn __sklib__draw_ellipse_on_window__window__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_ellipse_on_window__window__color__double__double__double__double__drawing_options"]
    fn __sklib__draw_ellipse_on_window__window__color__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse__color__rectangle"]
    fn __sklib__fill_ellipse__color__rectangle(clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse__color__rectangle__drawing_options"]
    fn __sklib__fill_ellipse__color__rectangle__drawing_options(clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse__color__double__double__double__double"]
    fn __sklib__fill_ellipse__color__double__double__double__double(clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse__color__double__double__double__double__drawing_options"]
    fn __sklib__fill_ellipse__color__double__double__double__double__drawing_options(clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse_on_bitmap__bitmap__color__rectangle"]
    fn __sklib__fill_ellipse_on_bitmap__bitmap__color__rectangle(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse_on_bitmap__bitmap__color__rectangle__drawing_options"]
    fn __sklib__fill_ellipse_on_bitmap__bitmap__color__rectangle__drawing_options(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse_on_bitmap__bitmap__color__double__double__double__double"]
    fn __sklib__fill_ellipse_on_bitmap__bitmap__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse_on_bitmap__bitmap__color__double__double__double__double__drawing_options"]
    fn __sklib__fill_ellipse_on_bitmap__bitmap__color__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse_on_window__window__color__rectangle"]
    fn __sklib__fill_ellipse_on_window__window__color__rectangle(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse_on_window__window__color__rectangle__drawing_options"]
    fn __sklib__fill_ellipse_on_window__window__color__rectangle__drawing_options(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse_on_window__window__color__double__double__double__double"]
    fn __sklib__fill_ellipse_on_window__window__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_ellipse_on_window__window__color__double__double__double__double__drawing_options"]
    fn __sklib__fill_ellipse_on_window__window__color__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__cosine__float"]
    fn __sklib__cosine__float(degrees: f32) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sine__float"]
    fn __sklib__sine__float(degrees: f32) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__tangent__float"]
    fn __sklib__tangent__float(degrees: f32) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__clear_screen"]
    fn __sklib__clear_screen();
}
extern "C" {
    #[link_name = "__sklib__clear_screen__color"]
    fn __sklib__clear_screen__color(clr: __sklib_color);
}
extern "C" {
    #[link_name = "__sklib__display_details__unsigned_int"]
    fn __sklib__display_details__unsigned_int(index: u32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__display_height__display"]
    fn __sklib__display_height__display(disp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__display_name__display"]
    fn __sklib__display_name__display(disp: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__display_width__display"]
    fn __sklib__display_width__display(disp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__display_x__display"]
    fn __sklib__display_x__display(disp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__display_y__display"]
    fn __sklib__display_y__display(disp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__number_of_displays"]
    fn __sklib__number_of_displays() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__refresh_screen"]
    fn __sklib__refresh_screen();
}
extern "C" {
    #[link_name = "__sklib__refresh_screen__unsigned_int"]
    fn __sklib__refresh_screen__unsigned_int(target_fps: u32);
}
extern "C" {
    #[link_name = "__sklib__save_bitmap__bitmap__string_ref"]
    fn __sklib__save_bitmap__bitmap__string_ref(bmp: __sklib_ptr, basename: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__screen_height"]
    fn __sklib__screen_height() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__screen_width"]
    fn __sklib__screen_width() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__take_screenshot__string_ref"]
    fn __sklib__take_screenshot__string_ref(basename: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__take_screenshot__window__string_ref"]
    fn __sklib__take_screenshot__window__string_ref(wind: __sklib_ptr, basename: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__bitmap_bounding_circle__bitmap__point_2d_ref"]
    fn __sklib__bitmap_bounding_circle__bitmap__point_2d_ref(bmp: __sklib_ptr, pt: __sklib_point_2d) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_bounding_rectangle__bitmap"]
    fn __sklib__bitmap_bounding_rectangle__bitmap(bmp: __sklib_ptr) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_bounding_rectangle__bitmap__double__double"]
    fn __sklib__bitmap_bounding_rectangle__bitmap__double__double(bmp: __sklib_ptr, x: f64, y: f64) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_center__bitmap"]
    fn __sklib__bitmap_cell_center__bitmap(bmp: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_circle__bitmap__double__double"]
    fn __sklib__bitmap_cell_circle__bitmap__double__double(bmp: __sklib_ptr, x: f64, y: f64) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_circle__bitmap__point_2d"]
    fn __sklib__bitmap_cell_circle__bitmap__point_2d(bmp: __sklib_ptr, pt: __sklib_point_2d) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_circle__bitmap__point_2d__double"]
    fn __sklib__bitmap_cell_circle__bitmap__point_2d__double(bmp: __sklib_ptr, pt: __sklib_point_2d, scale: f64) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_columns__bitmap"]
    fn __sklib__bitmap_cell_columns__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_count__bitmap"]
    fn __sklib__bitmap_cell_count__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_height__bitmap"]
    fn __sklib__bitmap_cell_height__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_offset__bitmap__int"]
    fn __sklib__bitmap_cell_offset__bitmap__int(src: __sklib_ptr, cell: i32) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_rectangle__bitmap"]
    fn __sklib__bitmap_cell_rectangle__bitmap(src: __sklib_ptr) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_rectangle__bitmap__point_2d_ref"]
    fn __sklib__bitmap_cell_rectangle__bitmap__point_2d_ref(src: __sklib_ptr, pt: __sklib_point_2d) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_rows__bitmap"]
    fn __sklib__bitmap_cell_rows__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_cell_width__bitmap"]
    fn __sklib__bitmap_cell_width__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_center__bitmap"]
    fn __sklib__bitmap_center__bitmap(bmp: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__bitmap_filename__bitmap"]
    fn __sklib__bitmap_filename__bitmap(bmp: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__bitmap_height__bitmap"]
    fn __sklib__bitmap_height__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_height__string"]
    fn __sklib__bitmap_height__string(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_name__bitmap"]
    fn __sklib__bitmap_name__bitmap(bmp: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__bitmap_named__string"]
    fn __sklib__bitmap_named__string(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__bitmap_rectangle_of_cell__bitmap__int"]
    fn __sklib__bitmap_rectangle_of_cell__bitmap__int(src: __sklib_ptr, cell: i32) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__bitmap_set_cell_details__bitmap__int__int__int__int__int"]
    fn __sklib__bitmap_set_cell_details__bitmap__int__int__int__int__int(bmp: __sklib_ptr, width: i32, height: i32, columns: i32, rows: i32, count: i32);
}
extern "C" {
    #[link_name = "__sklib__bitmap_valid__bitmap"]
    fn __sklib__bitmap_valid__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_width__bitmap"]
    fn __sklib__bitmap_width__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_width__string"]
    fn __sklib__bitmap_width__string(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__clear_bitmap__bitmap__color"]
    fn __sklib__clear_bitmap__bitmap__color(bmp: __sklib_ptr, clr: __sklib_color);
}
extern "C" {
    #[link_name = "__sklib__clear_bitmap__string__color"]
    fn __sklib__clear_bitmap__string__color(name: __sklib_string, clr: __sklib_color);
}
extern "C" {
    #[link_name = "__sklib__create_bitmap__string__int__int"]
    fn __sklib__create_bitmap__string__int__int(name: __sklib_string, width: i32, height: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__draw_bitmap__bitmap__double__double"]
    fn __sklib__draw_bitmap__bitmap__double__double(bmp: __sklib_ptr, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_bitmap__bitmap__double__double__drawing_options"]
    fn __sklib__draw_bitmap__bitmap__double__double__drawing_options(bmp: __sklib_ptr, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_bitmap__string__double__double"]
    fn __sklib__draw_bitmap__string__double__double(name: __sklib_string, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_bitmap__string__double__double__drawing_options"]
    fn __sklib__draw_bitmap__string__double__double__drawing_options(name: __sklib_string, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_bitmap_on_bitmap__bitmap__bitmap__double__double"]
    fn __sklib__draw_bitmap_on_bitmap__bitmap__bitmap__double__double(destination: __sklib_ptr, bmp: __sklib_ptr, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_bitmap_on_bitmap__bitmap__bitmap__double__double__drawing_options"]
    fn __sklib__draw_bitmap_on_bitmap__bitmap__bitmap__double__double__drawing_options(destination: __sklib_ptr, bmp: __sklib_ptr, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_bitmap_on_window__window__bitmap__double__double"]
    fn __sklib__draw_bitmap_on_window__window__bitmap__double__double(destination: __sklib_ptr, bmp: __sklib_ptr, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_bitmap_on_window__window__bitmap__double__double__drawing_options"]
    fn __sklib__draw_bitmap_on_window__window__bitmap__double__double__drawing_options(destination: __sklib_ptr, bmp: __sklib_ptr, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__free_all_bitmaps"]
    fn __sklib__free_all_bitmaps();
}
extern "C" {
    #[link_name = "__sklib__free_bitmap__bitmap"]
    fn __sklib__free_bitmap__bitmap(to_delete: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__has_bitmap__string"]
    fn __sklib__has_bitmap__string(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__load_bitmap__string__string"]
    fn __sklib__load_bitmap__string__string(name: __sklib_string, filename: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__pixel_drawn_at_point__bitmap__point_2d_ref"]
    fn __sklib__pixel_drawn_at_point__bitmap__point_2d_ref(bmp: __sklib_ptr, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__pixel_drawn_at_point__bitmap__double__double"]
    fn __sklib__pixel_drawn_at_point__bitmap__double__double(bmp: __sklib_ptr, x: f64, y: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__pixel_drawn_at_point__bitmap__int__point_2d_ref"]
    fn __sklib__pixel_drawn_at_point__bitmap__int__point_2d_ref(bmp: __sklib_ptr, cell: i32, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__pixel_drawn_at_point__bitmap__int__double__double"]
    fn __sklib__pixel_drawn_at_point__bitmap__int__double__double(bmp: __sklib_ptr, cell: i32, x: f64, y: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__setup_collision_mask__bitmap"]
    fn __sklib__setup_collision_mask__bitmap(bmp: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__process_events"]
    fn __sklib__process_events();
}
extern "C" {
    #[link_name = "__sklib__quit_requested"]
    fn __sklib__quit_requested() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__reset_quit"]
    fn __sklib__reset_quit();
}
extern "C" {
    #[link_name = "__sklib__add_column__int"]
    fn __sklib__add_column__int(width: i32);
}
extern "C" {
    #[link_name = "__sklib__add_column_relative__double"]
    fn __sklib__add_column_relative__double(width: f64);
}
extern "C" {
    #[link_name = "__sklib__bitmap_button__bitmap"]
    fn __sklib__bitmap_button__bitmap(bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_button__bitmap__rectangle_ref"]
    fn __sklib__bitmap_button__bitmap__rectangle_ref(bmp: __sklib_ptr, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_button__bitmap__rectangle_ref__drawing_options"]
    fn __sklib__bitmap_button__bitmap__rectangle_ref__drawing_options(bmp: __sklib_ptr, rect: __sklib_rectangle, opts: __sklib_drawing_options) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_button__bitmap__drawing_options"]
    fn __sklib__bitmap_button__bitmap__drawing_options(bmp: __sklib_ptr, opts: __sklib_drawing_options) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_button__string_ref__bitmap"]
    fn __sklib__bitmap_button__string_ref__bitmap(label_text: __sklib_string, bmp: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__bitmap_button__string_ref__bitmap__drawing_options"]
    fn __sklib__bitmap_button__string_ref__bitmap__drawing_options(label_text: __sklib_string, bmp: __sklib_ptr, opts: __sklib_drawing_options) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__button__string_ref__rectangle_ref"]
    fn __sklib__button__string_ref__rectangle_ref(text: __sklib_string, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__button__string_ref"]
    fn __sklib__button__string_ref(text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__button__string_ref__string_ref"]
    fn __sklib__button__string_ref__string_ref(label_text: __sklib_string, text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__checkbox__string_ref__bool_ref__rectangle_ref"]
    fn __sklib__checkbox__string_ref__bool_ref__rectangle_ref(text: __sklib_string, value: i32, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__checkbox__string_ref__bool_ref"]
    fn __sklib__checkbox__string_ref__bool_ref(text: __sklib_string, value: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__checkbox__string_ref__string_ref__bool_ref"]
    fn __sklib__checkbox__string_ref__string_ref__bool_ref(label_text: __sklib_string, text: __sklib_string, value: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__color_slider__color_ref__rectangle_ref"]
    fn __sklib__color_slider__color_ref__rectangle_ref(clr: __sklib_color, rect: __sklib_rectangle) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_slider__color_ref"]
    fn __sklib__color_slider__color_ref(clr: __sklib_color) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__color_slider__string_ref__color_ref"]
    fn __sklib__color_slider__string_ref__color_ref(label_text: __sklib_string, clr: __sklib_color) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__disable_interface"]
    fn __sklib__disable_interface();
}
extern "C" {
    #[link_name = "__sklib__draw_interface"]
    fn __sklib__draw_interface();
}
extern "C" {
    #[link_name = "__sklib__enable_interface"]
    fn __sklib__enable_interface();
}
extern "C" {
    #[link_name = "__sklib__end_inset__string_ref"]
    fn __sklib__end_inset__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__end_panel__string_ref"]
    fn __sklib__end_panel__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__end_popup__string_ref"]
    fn __sklib__end_popup__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__end_treenode__string_ref"]
    fn __sklib__end_treenode__string_ref(label_text: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__enter_column"]
    fn __sklib__enter_column();
}
extern "C" {
    #[link_name = "__sklib__get_interface_label_width"]
    fn __sklib__get_interface_label_width() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__header__string_ref"]
    fn __sklib__header__string_ref(label_text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__hsb_color_slider__color_ref__rectangle_ref"]
    fn __sklib__hsb_color_slider__color_ref__rectangle_ref(clr: __sklib_color, rect: __sklib_rectangle) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__hsb_color_slider__color_ref"]
    fn __sklib__hsb_color_slider__color_ref(clr: __sklib_color) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__hsb_color_slider__string_ref__color_ref"]
    fn __sklib__hsb_color_slider__string_ref__color_ref(label_text: __sklib_string, clr: __sklib_color) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__interface_enabled"]
    fn __sklib__interface_enabled() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__interface_style_panel__rectangle_ref"]
    fn __sklib__interface_style_panel__rectangle_ref(initial_rectangle: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__label_element__string_ref"]
    fn __sklib__label_element__string_ref(text: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__label_element__string_ref__rectangle_ref"]
    fn __sklib__label_element__string_ref__rectangle_ref(text: __sklib_string, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__last_element_changed"]
    fn __sklib__last_element_changed() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__last_element_confirmed"]
    fn __sklib__last_element_confirmed() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__leave_column"]
    fn __sklib__leave_column();
}
extern "C" {
    #[link_name = "__sklib__number_box__float_ref__float__rectangle_ref"]
    fn __sklib__number_box__float_ref__float__rectangle_ref(value: f32, step: f32, rect: __sklib_rectangle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__number_box__float_ref__float"]
    fn __sklib__number_box__float_ref__float(value: f32, step: f32) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__number_box__string_ref__float_ref__float"]
    fn __sklib__number_box__string_ref__float_ref__float(label_text: __sklib_string, value: f32, step: f32) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__open_popup__string_ref"]
    fn __sklib__open_popup__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__paragraph__string_ref"]
    fn __sklib__paragraph__string_ref(text: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__paragraph__string_ref__rectangle_ref"]
    fn __sklib__paragraph__string_ref__rectangle_ref(text: __sklib_string, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__reset_layout"]
    fn __sklib__reset_layout();
}
extern "C" {
    #[link_name = "__sklib__set_interface_accent_color__color__float"]
    fn __sklib__set_interface_accent_color__color__float(clr: __sklib_color, contrast: f32);
}
extern "C" {
    #[link_name = "__sklib__set_interface_border_color__color"]
    fn __sklib__set_interface_border_color__color(clr: __sklib_color);
}
extern "C" {
    #[link_name = "__sklib__set_interface_colors_auto__color__color__float__float__float"]
    fn __sklib__set_interface_colors_auto__color__color__float__float__float(main_clr: __sklib_color, accent_clr: __sklib_color, contrast: f32, accent_contrast: f32, border_contrast: f32);
}
extern "C" {
    #[link_name = "__sklib__set_interface_element_color__color__float"]
    fn __sklib__set_interface_element_color__color__float(clr: __sklib_color, contrast: f32);
}
extern "C" {
    #[link_name = "__sklib__set_interface_element_shadows__int__color__point_2d"]
    fn __sklib__set_interface_element_shadows__int__color__point_2d(radius: i32, clr: __sklib_color, offset: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__set_interface_font__string_ref"]
    fn __sklib__set_interface_font__string_ref(fnt: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__set_interface_font__font"]
    fn __sklib__set_interface_font__font(fnt: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__set_interface_font_size__int"]
    fn __sklib__set_interface_font_size__int(size: i32);
}
extern "C" {
    #[link_name = "__sklib__set_interface_label_width__int"]
    fn __sklib__set_interface_label_width__int(width: i32);
}
extern "C" {
    #[link_name = "__sklib__set_interface_panel_shadows__int__color__point_2d"]
    fn __sklib__set_interface_panel_shadows__int__color__point_2d(radius: i32, clr: __sklib_color, offset: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__set_interface_root_text_color__color"]
    fn __sklib__set_interface_root_text_color__color(clr: __sklib_color);
}
extern "C" {
    #[link_name = "__sklib__set_interface_shadows__int__color__point_2d"]
    fn __sklib__set_interface_shadows__int__color__point_2d(radius: i32, clr: __sklib_color, offset: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__set_interface_spacing__int__int"]
    fn __sklib__set_interface_spacing__int__int(spacing: i32, padding: i32);
}
extern "C" {
    #[link_name = "__sklib__set_interface_style__interface_style"]
    fn __sklib__set_interface_style__interface_style(style: i32);
}
extern "C" {
    #[link_name = "__sklib__set_interface_style__interface_style__color"]
    fn __sklib__set_interface_style__interface_style__color(style: i32, clr: __sklib_color);
}
extern "C" {
    #[link_name = "__sklib__set_interface_text_color__color"]
    fn __sklib__set_interface_text_color__color(clr: __sklib_color);
}
extern "C" {
    #[link_name = "__sklib__set_layout_height__int"]
    fn __sklib__set_layout_height__int(height: i32);
}
extern "C" {
    #[link_name = "__sklib__single_line_layout"]
    fn __sklib__single_line_layout();
}
extern "C" {
    #[link_name = "__sklib__slider__float_ref__float__float__rectangle_ref"]
    fn __sklib__slider__float_ref__float__float__rectangle_ref(value: f32, min_value: f32, max_value: f32, rect: __sklib_rectangle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__slider__float_ref__float__float"]
    fn __sklib__slider__float_ref__float__float(value: f32, min_value: f32, max_value: f32) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__slider__string_ref__float_ref__float__float"]
    fn __sklib__slider__string_ref__float_ref__float__float(label_text: __sklib_string, value: f32, min_value: f32, max_value: f32) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__split_into_columns__int"]
    fn __sklib__split_into_columns__int(count: i32);
}
extern "C" {
    #[link_name = "__sklib__split_into_columns__int__int"]
    fn __sklib__split_into_columns__int__int(count: i32, last_width: i32);
}
extern "C" {
    #[link_name = "__sklib__split_into_columns_relative__int__double"]
    fn __sklib__split_into_columns_relative__int__double(count: i32, last_width: f64);
}
extern "C" {
    #[link_name = "__sklib__start_custom_layout"]
    fn __sklib__start_custom_layout();
}
extern "C" {
    #[link_name = "__sklib__start_inset__string_ref__rectangle_ref"]
    fn __sklib__start_inset__string_ref__rectangle_ref(name: __sklib_string, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__start_inset__string_ref__int"]
    fn __sklib__start_inset__string_ref__int(name: __sklib_string, height: i32);
}
extern "C" {
    #[link_name = "__sklib__start_panel__string_ref__rectangle"]
    fn __sklib__start_panel__string_ref__rectangle(name: __sklib_string, initial_rectangle: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__start_popup__string_ref"]
    fn __sklib__start_popup__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__start_treenode__string_ref"]
    fn __sklib__start_treenode__string_ref(label_text: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__text_box__string_ref"]
    fn __sklib__text_box__string_ref(value: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__text_box__string_ref__rectangle_ref"]
    fn __sklib__text_box__string_ref__rectangle_ref(value: __sklib_string, rect: __sklib_rectangle) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__text_box__string_ref__string_ref"]
    fn __sklib__text_box__string_ref__string_ref(label_text: __sklib_string, value: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__create_json"]
    fn __sklib__create_json() 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_json__string"]
    fn __sklib__create_json__string(json_string: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__free_all_json"]
    fn __sklib__free_all_json();
}
extern "C" {
    #[link_name = "__sklib__free_json__json"]
    fn __sklib__free_json__json(j: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__json_count_keys__json"]
    fn __sklib__json_count_keys__json(j: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__json_from_color__color"]
    fn __sklib__json_from_color__color(clr: __sklib_color) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__json_from_file__string_ref"]
    fn __sklib__json_from_file__string_ref(filename: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__json_from_string__string_ref"]
    fn __sklib__json_from_string__string_ref(j_string: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__json_has_key__json__string"]
    fn __sklib__json_has_key__json__string(j: __sklib_ptr, key: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__json_read_array__json__string__vector_double_ref"]
    fn __sklib__json_read_array__json__string__vector_double_ref(j: __sklib_ptr, key: __sklib_string, out_result: &mut __sklib_vector_double);
}
extern "C" {
    #[link_name = "__sklib__json_read_array__json__string__vector_json_ref"]
    fn __sklib__json_read_array__json__string__vector_json_ref(j: __sklib_ptr, key: __sklib_string, out_result: &mut __sklib_vector_json);
}
extern "C" {
    #[link_name = "__sklib__json_read_array__json__string__vector_string_ref"]
    fn __sklib__json_read_array__json__string__vector_string_ref(j: __sklib_ptr, key: __sklib_string, out_result: &mut __sklib_vector_string);
}
extern "C" {
    #[link_name = "__sklib__json_read_array__json__string__vector_bool_ref"]
    fn __sklib__json_read_array__json__string__vector_bool_ref(j: __sklib_ptr, key: __sklib_string, out_result: &mut __sklib_vector_bool);
}
extern "C" {
    #[link_name = "__sklib__json_read_bool__json__string"]
    fn __sklib__json_read_bool__json__string(j: __sklib_ptr, key: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__json_read_number__json__string"]
    fn __sklib__json_read_number__json__string(j: __sklib_ptr, key: __sklib_string) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__json_read_number_as_double__json__string"]
    fn __sklib__json_read_number_as_double__json__string(j: __sklib_ptr, key: __sklib_string) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__json_read_number_as_int__json__string"]
    fn __sklib__json_read_number_as_int__json__string(j: __sklib_ptr, key: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__json_read_object__json__string"]
    fn __sklib__json_read_object__json__string(j: __sklib_ptr, key: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__json_read_string__json__string"]
    fn __sklib__json_read_string__json__string(j: __sklib_ptr, key: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__json_set_array__json__string__vector_string"]
    fn __sklib__json_set_array__json__string__vector_string(j: __sklib_ptr, key: __sklib_string, value: __sklib_vector_string);
}
extern "C" {
    #[link_name = "__sklib__json_set_array__json__string__vector_double"]
    fn __sklib__json_set_array__json__string__vector_double(j: __sklib_ptr, key: __sklib_string, value: __sklib_vector_double);
}
extern "C" {
    #[link_name = "__sklib__json_set_array__json__string__vector_bool"]
    fn __sklib__json_set_array__json__string__vector_bool(j: __sklib_ptr, key: __sklib_string, value: __sklib_vector_bool);
}
extern "C" {
    #[link_name = "__sklib__json_set_array__json__string__vector_json"]
    fn __sklib__json_set_array__json__string__vector_json(j: __sklib_ptr, key: __sklib_string, value: __sklib_vector_json);
}
extern "C" {
    #[link_name = "__sklib__json_set_bool__json__string__bool"]
    fn __sklib__json_set_bool__json__string__bool(j: __sklib_ptr, key: __sklib_string, value: i32);
}
extern "C" {
    #[link_name = "__sklib__json_set_number__json__string__int"]
    fn __sklib__json_set_number__json__string__int(j: __sklib_ptr, key: __sklib_string, value: i32);
}
extern "C" {
    #[link_name = "__sklib__json_set_number__json__string__double"]
    fn __sklib__json_set_number__json__string__double(j: __sklib_ptr, key: __sklib_string, value: f64);
}
extern "C" {
    #[link_name = "__sklib__json_set_number__json__string__float"]
    fn __sklib__json_set_number__json__string__float(j: __sklib_ptr, key: __sklib_string, value: f32);
}
extern "C" {
    #[link_name = "__sklib__json_set_object__json__string__json"]
    fn __sklib__json_set_object__json__string__json(j: __sklib_ptr, key: __sklib_string, obj: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__json_set_string__json__string__string"]
    fn __sklib__json_set_string__json__string__string(j: __sklib_ptr, key: __sklib_string, value: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__json_to_color__json"]
    fn __sklib__json_to_color__json(j: __sklib_ptr) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__json_to_file__json__string_ref"]
    fn __sklib__json_to_file__json__string_ref(j: __sklib_ptr, filename: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__json_to_string__json"]
    fn __sklib__json_to_string__json(j: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__any_key_pressed"]
    fn __sklib__any_key_pressed() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__deregister_callback_on_key_down__key_callback_ptr"]
    fn __sklib__deregister_callback_on_key_down__key_callback_ptr(callback: __sklib_key_callback);
}
extern "C" {
    #[link_name = "__sklib__deregister_callback_on_key_typed__key_callback_ptr"]
    fn __sklib__deregister_callback_on_key_typed__key_callback_ptr(callback: __sklib_key_callback);
}
extern "C" {
    #[link_name = "__sklib__deregister_callback_on_key_up__key_callback_ptr"]
    fn __sklib__deregister_callback_on_key_up__key_callback_ptr(callback: __sklib_key_callback);
}
extern "C" {
    #[link_name = "__sklib__key_down__key_code"]
    fn __sklib__key_down__key_code(key: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__key_name__key_code"]
    fn __sklib__key_name__key_code(key: i32) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__key_released__key_code"]
    fn __sklib__key_released__key_code(key: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__key_typed__key_code"]
    fn __sklib__key_typed__key_code(key: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__key_up__key_code"]
    fn __sklib__key_up__key_code(key: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__register_callback_on_key_down__key_callback_ptr"]
    fn __sklib__register_callback_on_key_down__key_callback_ptr(callback: __sklib_key_callback);
}
extern "C" {
    #[link_name = "__sklib__register_callback_on_key_typed__key_callback_ptr"]
    fn __sklib__register_callback_on_key_typed__key_callback_ptr(callback: __sklib_key_callback);
}
extern "C" {
    #[link_name = "__sklib__register_callback_on_key_up__key_callback_ptr"]
    fn __sklib__register_callback_on_key_up__key_callback_ptr(callback: __sklib_key_callback);
}
extern "C" {
    #[link_name = "__sklib__draw_line__color__line_ref"]
    fn __sklib__draw_line__color__line_ref(clr: __sklib_color, l: __sklib_line);
}
extern "C" {
    #[link_name = "__sklib__draw_line__color__line_ref__drawing_options"]
    fn __sklib__draw_line__color__line_ref__drawing_options(clr: __sklib_color, l: __sklib_line, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_line__color__point_2d_ref__point_2d_ref"]
    fn __sklib__draw_line__color__point_2d_ref__point_2d_ref(clr: __sklib_color, from_pt: __sklib_point_2d, to_pt: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__draw_line__color__point_2d_ref__point_2d_ref__drawing_options_ref"]
    fn __sklib__draw_line__color__point_2d_ref__point_2d_ref__drawing_options_ref(clr: __sklib_color, from_pt: __sklib_point_2d, to_pt: __sklib_point_2d, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_line__color__double__double__double__double"]
    fn __sklib__draw_line__color__double__double__double__double(clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_line__color__double__double__double__double__drawing_options_ref"]
    fn __sklib__draw_line__color__double__double__double__double__drawing_options_ref(clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_bitmap__bitmap__color__line_ref"]
    fn __sklib__draw_line_on_bitmap__bitmap__color__line_ref(destination: __sklib_ptr, clr: __sklib_color, l: __sklib_line);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_bitmap__bitmap__color__line_ref__drawing_options"]
    fn __sklib__draw_line_on_bitmap__bitmap__color__line_ref__drawing_options(destination: __sklib_ptr, clr: __sklib_color, l: __sklib_line, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_bitmap__bitmap__color__point_2d_ref__point_2d_ref"]
    fn __sklib__draw_line_on_bitmap__bitmap__color__point_2d_ref__point_2d_ref(destination: __sklib_ptr, clr: __sklib_color, from_pt: __sklib_point_2d, to_pt: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_bitmap__bitmap__color__point_2d_ref__point_2d_ref__drawing_options_ref"]
    fn __sklib__draw_line_on_bitmap__bitmap__color__point_2d_ref__point_2d_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, from_pt: __sklib_point_2d, to_pt: __sklib_point_2d, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_bitmap__bitmap__color__double__double__double__double"]
    fn __sklib__draw_line_on_bitmap__bitmap__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_bitmap__bitmap__color__double__double__double__double__drawing_options_ref"]
    fn __sklib__draw_line_on_bitmap__bitmap__color__double__double__double__double__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_window__window__color__line_ref"]
    fn __sklib__draw_line_on_window__window__color__line_ref(destination: __sklib_ptr, clr: __sklib_color, l: __sklib_line);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_window__window__color__line_ref__drawing_options"]
    fn __sklib__draw_line_on_window__window__color__line_ref__drawing_options(destination: __sklib_ptr, clr: __sklib_color, l: __sklib_line, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_window__window__color__point_2d_ref__point_2d_ref"]
    fn __sklib__draw_line_on_window__window__color__point_2d_ref__point_2d_ref(destination: __sklib_ptr, clr: __sklib_color, from_pt: __sklib_point_2d, to_pt: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_window__window__color__point_2d_ref__point_2d_ref__drawing_options_ref"]
    fn __sklib__draw_line_on_window__window__color__point_2d_ref__point_2d_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, from_pt: __sklib_point_2d, to_pt: __sklib_point_2d, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_window__window__color__double__double__double__double"]
    fn __sklib__draw_line_on_window__window__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_line_on_window__window__color__double__double__double__double__drawing_options_ref"]
    fn __sklib__draw_line_on_window__window__color__double__double__double__double__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__closest_point_on_line__point_2d__line_ref"]
    fn __sklib__closest_point_on_line__point_2d__line_ref(from_pt: __sklib_point_2d, l: __sklib_line) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__closest_point_on_lines__point_2d__vector_line_ref__int_ref"]
    fn __sklib__closest_point_on_lines__point_2d__vector_line_ref__int_ref(from_pt: __sklib_point_2d, lines: __sklib_vector_line, line_idx: &mut i32) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__line_from__point_2d_ref__point_2d_ref"]
    fn __sklib__line_from__point_2d_ref__point_2d_ref(start: __sklib_point_2d, end_pt: __sklib_point_2d) 
        -> __sklib_line;
}
extern "C" {
    #[link_name = "__sklib__line_from__point_2d_ref__vector_2d_ref"]
    fn __sklib__line_from__point_2d_ref__vector_2d_ref(start: __sklib_point_2d, offset: __sklib_vector_2d) 
        -> __sklib_line;
}
extern "C" {
    #[link_name = "__sklib__line_from__vector_2d_ref"]
    fn __sklib__line_from__vector_2d_ref(v: __sklib_vector_2d) 
        -> __sklib_line;
}
extern "C" {
    #[link_name = "__sklib__line_from__double__double__double__double"]
    fn __sklib__line_from__double__double__double__double(x1: f64, y1: f64, x2: f64, y2: f64) 
        -> __sklib_line;
}
extern "C" {
    #[link_name = "__sklib__line_intersection_point__line_ref__line_ref__point_2d_ref"]
    fn __sklib__line_intersection_point__line_ref__line_ref__point_2d_ref(line1: __sklib_line, line2: __sklib_line, pt: &mut __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__line_intersects_circle__line_ref__circle_ref"]
    fn __sklib__line_intersects_circle__line_ref__circle_ref(l: __sklib_line, c: __sklib_circle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__line_intersects_lines__line_ref__vector_line_ref"]
    fn __sklib__line_intersects_lines__line_ref__vector_line_ref(l: __sklib_line, lines: __sklib_vector_line) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__line_intersects_rect__line_ref__rectangle_ref"]
    fn __sklib__line_intersects_rect__line_ref__rectangle_ref(l: __sklib_line, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__line_length__line_ref"]
    fn __sklib__line_length__line_ref(l: __sklib_line) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__line_length_squared__line_ref"]
    fn __sklib__line_length_squared__line_ref(l: __sklib_line) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__line_mid_point__line_ref"]
    fn __sklib__line_mid_point__line_ref(l: __sklib_line) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__line_normal__line_ref"]
    fn __sklib__line_normal__line_ref(l: __sklib_line) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__line_to_string__line_ref"]
    fn __sklib__line_to_string__line_ref(ln: __sklib_line) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__lines_from__rectangle_ref"]
    fn __sklib__lines_from__rectangle_ref(rect: __sklib_rectangle) 
        -> __sklib_vector_line;
}
extern "C" {
    #[link_name = "__sklib__lines_from__triangle_ref"]
    fn __sklib__lines_from__triangle_ref(t: __sklib_triangle) 
        -> __sklib_vector_line;
}
extern "C" {
    #[link_name = "__sklib__lines_intersect__line_ref__line_ref"]
    fn __sklib__lines_intersect__line_ref__line_ref(l1: __sklib_line, l2: __sklib_line) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__close_log_process"]
    fn __sklib__close_log_process();
}
extern "C" {
    #[link_name = "__sklib__init_custom_logger__log_mode"]
    fn __sklib__init_custom_logger__log_mode(mode: i32);
}
extern "C" {
    #[link_name = "__sklib__init_custom_logger__string__bool__log_mode"]
    fn __sklib__init_custom_logger__string__bool__log_mode(app_name: __sklib_string, override_prev_log: i32, mode: i32);
}
extern "C" {
    #[link_name = "__sklib__log__log_level__string"]
    fn __sklib__log__log_level__string(level: i32, message: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__apply_matrix__matrix_2d_ref__quad_ref"]
    fn __sklib__apply_matrix__matrix_2d_ref__quad_ref(matrix: __sklib_matrix_2d, q: &mut __sklib_quad);
}
extern "C" {
    #[link_name = "__sklib__apply_matrix__matrix_2d_ref__triangle_ref"]
    fn __sklib__apply_matrix__matrix_2d_ref__triangle_ref(m: __sklib_matrix_2d, tri: &mut __sklib_triangle);
}
extern "C" {
    #[link_name = "__sklib__identity_matrix"]
    fn __sklib__identity_matrix() 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__matrix_inverse__matrix_2d_ref"]
    fn __sklib__matrix_inverse__matrix_2d_ref(m: __sklib_matrix_2d) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__matrix_multiply__matrix_2d_ref__point_2d_ref"]
    fn __sklib__matrix_multiply__matrix_2d_ref__point_2d_ref(m: __sklib_matrix_2d, pt: __sklib_point_2d) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__matrix_multiply__matrix_2d_ref__matrix_2d_ref"]
    fn __sklib__matrix_multiply__matrix_2d_ref__matrix_2d_ref(m1: __sklib_matrix_2d, m2: __sklib_matrix_2d) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__matrix_multiply__matrix_2d_ref__vector_2d_ref"]
    fn __sklib__matrix_multiply__matrix_2d_ref__vector_2d_ref(m: __sklib_matrix_2d, v: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__matrix_to_string__matrix_2d_ref"]
    fn __sklib__matrix_to_string__matrix_2d_ref(matrix: __sklib_matrix_2d) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__rotation_matrix__double"]
    fn __sklib__rotation_matrix__double(deg: f64) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__scale_matrix__point_2d_ref"]
    fn __sklib__scale_matrix__point_2d_ref(scale: __sklib_point_2d) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__scale_matrix__vector_2d_ref"]
    fn __sklib__scale_matrix__vector_2d_ref(scale: __sklib_vector_2d) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__scale_matrix__double"]
    fn __sklib__scale_matrix__double(scale: f64) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__scale_rotate_translate_matrix__point_2d_ref__double__point_2d_ref"]
    fn __sklib__scale_rotate_translate_matrix__point_2d_ref__double__point_2d_ref(scale: __sklib_point_2d, deg: f64, translate: __sklib_point_2d) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__translation_matrix__point_2d_ref"]
    fn __sklib__translation_matrix__point_2d_ref(pt: __sklib_point_2d) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__translation_matrix__vector_2d_ref"]
    fn __sklib__translation_matrix__vector_2d_ref(pt: __sklib_vector_2d) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__translation_matrix__double__double"]
    fn __sklib__translation_matrix__double__double(dx: f64, dy: f64) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__hide_mouse"]
    fn __sklib__hide_mouse();
}
extern "C" {
    #[link_name = "__sklib__mouse_clicked__mouse_button"]
    fn __sklib__mouse_clicked__mouse_button(button: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__mouse_down__mouse_button"]
    fn __sklib__mouse_down__mouse_button(button: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__mouse_movement"]
    fn __sklib__mouse_movement() 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__mouse_position"]
    fn __sklib__mouse_position() 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__mouse_position_vector"]
    fn __sklib__mouse_position_vector() 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__mouse_shown"]
    fn __sklib__mouse_shown() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__mouse_up__mouse_button"]
    fn __sklib__mouse_up__mouse_button(button: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__mouse_wheel_scroll"]
    fn __sklib__mouse_wheel_scroll() 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__mouse_x"]
    fn __sklib__mouse_x() 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__mouse_y"]
    fn __sklib__mouse_y() 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__move_mouse__double__double"]
    fn __sklib__move_mouse__double__double(x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__move_mouse__point_2d"]
    fn __sklib__move_mouse__point_2d(point: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__show_mouse"]
    fn __sklib__show_mouse();
}
extern "C" {
    #[link_name = "__sklib__show_mouse__bool"]
    fn __sklib__show_mouse__bool(show: i32);
}
extern "C" {
    #[link_name = "__sklib__fade_music_in__string_ref__int"]
    fn __sklib__fade_music_in__string_ref__int(name: __sklib_string, ms: i32);
}
extern "C" {
    #[link_name = "__sklib__fade_music_in__string_ref__int__int"]
    fn __sklib__fade_music_in__string_ref__int__int(name: __sklib_string, times: i32, ms: i32);
}
extern "C" {
    #[link_name = "__sklib__fade_music_in__music__int"]
    fn __sklib__fade_music_in__music__int(data: __sklib_ptr, ms: i32);
}
extern "C" {
    #[link_name = "__sklib__fade_music_in__music__int__int"]
    fn __sklib__fade_music_in__music__int__int(data: __sklib_ptr, times: i32, ms: i32);
}
extern "C" {
    #[link_name = "__sklib__fade_music_out__int"]
    fn __sklib__fade_music_out__int(ms: i32);
}
extern "C" {
    #[link_name = "__sklib__free_all_music"]
    fn __sklib__free_all_music();
}
extern "C" {
    #[link_name = "__sklib__free_music__music"]
    fn __sklib__free_music__music(effect: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__has_music__string_ref"]
    fn __sklib__has_music__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__load_music__string_ref__string_ref"]
    fn __sklib__load_music__string_ref__string_ref(name: __sklib_string, filename: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__music_filename__music"]
    fn __sklib__music_filename__music(data: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__music_name__music"]
    fn __sklib__music_name__music(data: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__music_named__string_ref"]
    fn __sklib__music_named__string_ref(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__music_playing"]
    fn __sklib__music_playing() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__music_valid__music"]
    fn __sklib__music_valid__music(m: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__music_volume"]
    fn __sklib__music_volume() 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__pause_music"]
    fn __sklib__pause_music();
}
extern "C" {
    #[link_name = "__sklib__play_music__string_ref"]
    fn __sklib__play_music__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__play_music__string_ref__int"]
    fn __sklib__play_music__string_ref__int(name: __sklib_string, times: i32);
}
extern "C" {
    #[link_name = "__sklib__play_music__music"]
    fn __sklib__play_music__music(data: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__play_music__music__int"]
    fn __sklib__play_music__music__int(data: __sklib_ptr, times: i32);
}
extern "C" {
    #[link_name = "__sklib__play_music__music__int__double"]
    fn __sklib__play_music__music__int__double(data: __sklib_ptr, times: i32, volume: f64);
}
extern "C" {
    #[link_name = "__sklib__resume_music"]
    fn __sklib__resume_music();
}
extern "C" {
    #[link_name = "__sklib__set_music_volume__double"]
    fn __sklib__set_music_volume__double(volume: f64);
}
extern "C" {
    #[link_name = "__sklib__stop_music"]
    fn __sklib__stop_music();
}
extern "C" {
    #[link_name = "__sklib__accept_all_new_connections"]
    fn __sklib__accept_all_new_connections() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__accept_new_connection__server_socket"]
    fn __sklib__accept_new_connection__server_socket(server: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__broadcast_message__string_ref__server_socket"]
    fn __sklib__broadcast_message__string_ref__server_socket(a_msg: __sklib_string, svr: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__broadcast_message__string_ref"]
    fn __sklib__broadcast_message__string_ref(a_msg: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__broadcast_message__string_ref__string_ref"]
    fn __sklib__broadcast_message__string_ref__string_ref(a_msg: __sklib_string, name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__check_network_activity"]
    fn __sklib__check_network_activity();
}
extern "C" {
    #[link_name = "__sklib__clear_messages__string_ref"]
    fn __sklib__clear_messages__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__clear_messages__connection"]
    fn __sklib__clear_messages__connection(a_connection: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__clear_messages__server_socket"]
    fn __sklib__clear_messages__server_socket(svr: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__close_all_connections"]
    fn __sklib__close_all_connections();
}
extern "C" {
    #[link_name = "__sklib__close_all_servers"]
    fn __sklib__close_all_servers();
}
extern "C" {
    #[link_name = "__sklib__close_connection__connection"]
    fn __sklib__close_connection__connection(a_connection: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__close_connection__string_ref"]
    fn __sklib__close_connection__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__close_message__message"]
    fn __sklib__close_message__message(msg: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__close_server__string_ref"]
    fn __sklib__close_server__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__close_server__server_socket"]
    fn __sklib__close_server__server_socket(svr: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__connection_count__string_ref"]
    fn __sklib__connection_count__string_ref(name: __sklib_string) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__connection_count__server_socket"]
    fn __sklib__connection_count__server_socket(server: __sklib_ptr) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__connection_ip__connection"]
    fn __sklib__connection_ip__connection(a_connection: __sklib_ptr) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__connection_ip__string_ref"]
    fn __sklib__connection_ip__string_ref(name: __sklib_string) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__connection_named__string_ref"]
    fn __sklib__connection_named__string_ref(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__connection_port__connection"]
    fn __sklib__connection_port__connection(a_connection: __sklib_ptr) 
        -> u16;
}
extern "C" {
    #[link_name = "__sklib__connection_port__string_ref"]
    fn __sklib__connection_port__string_ref(name: __sklib_string) 
        -> u16;
}
extern "C" {
    #[link_name = "__sklib__create_server__string_ref__unsigned_short"]
    fn __sklib__create_server__string_ref__unsigned_short(name: __sklib_string, port: u16) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_server__string_ref__unsigned_short__connection_type"]
    fn __sklib__create_server__string_ref__unsigned_short__connection_type(name: __sklib_string, port: u16, protocol: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__dec_to_hex__unsigned_int"]
    fn __sklib__dec_to_hex__unsigned_int(a_dec: u32) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__fetch_new_connection__server_socket"]
    fn __sklib__fetch_new_connection__server_socket(server: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__has_connection__string_ref"]
    fn __sklib__has_connection__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_messages"]
    fn __sklib__has_messages() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_messages__connection"]
    fn __sklib__has_messages__connection(con: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_messages__string_ref"]
    fn __sklib__has_messages__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_messages__server_socket"]
    fn __sklib__has_messages__server_socket(svr: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_new_connections"]
    fn __sklib__has_new_connections() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_server__string_ref"]
    fn __sklib__has_server__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__hex_str_to_ipv4__string_ref"]
    fn __sklib__hex_str_to_ipv4__string_ref(a_hex: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__hex_to_dec_string__string_ref"]
    fn __sklib__hex_to_dec_string__string_ref(a_hex: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__ipv4_to_dec__string_ref"]
    fn __sklib__ipv4_to_dec__string_ref(a_ip: __sklib_string) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__ipv4_to_hex__string_ref"]
    fn __sklib__ipv4_to_hex__string_ref(a_ip: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__ipv4_to_str__unsigned_int"]
    fn __sklib__ipv4_to_str__unsigned_int(ip: u32) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__is_connection_open__connection"]
    fn __sklib__is_connection_open__connection(con: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_connection_open__string_ref"]
    fn __sklib__is_connection_open__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__last_connection__string_ref"]
    fn __sklib__last_connection__string_ref(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__last_connection__server_socket"]
    fn __sklib__last_connection__server_socket(server: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__message_connection__message"]
    fn __sklib__message_connection__message(msg: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__message_count__server_socket"]
    fn __sklib__message_count__server_socket(svr: __sklib_ptr) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__message_count__connection"]
    fn __sklib__message_count__connection(a_connection: __sklib_ptr) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__message_count__string_ref"]
    fn __sklib__message_count__string_ref(name: __sklib_string) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__message_data__message"]
    fn __sklib__message_data__message(msg: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__message_data_bytes__message"]
    fn __sklib__message_data_bytes__message(msg: __sklib_ptr) 
        -> __sklib_vector_int8_t;
}
extern "C" {
    #[link_name = "__sklib__message_host__message"]
    fn __sklib__message_host__message(msg: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__message_port__message"]
    fn __sklib__message_port__message(msg: __sklib_ptr) 
        -> u16;
}
extern "C" {
    #[link_name = "__sklib__message_protocol__message"]
    fn __sklib__message_protocol__message(msg: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__my_ip"]
    fn __sklib__my_ip() 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__name_for_connection__string__unsigned_int"]
    fn __sklib__name_for_connection__string__unsigned_int(host: __sklib_string, port: u32) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__new_connection_count__server_socket"]
    fn __sklib__new_connection_count__server_socket(server: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__open_connection__string_ref__string_ref__unsigned_short"]
    fn __sklib__open_connection__string_ref__string_ref__unsigned_short(name: __sklib_string, host: __sklib_string, port: u16) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__open_connection__string_ref__string_ref__unsigned_short__connection_type"]
    fn __sklib__open_connection__string_ref__string_ref__unsigned_short__connection_type(name: __sklib_string, host: __sklib_string, port: u16, protocol: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__read_message"]
    fn __sklib__read_message() 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__read_message__connection"]
    fn __sklib__read_message__connection(a_connection: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__read_message__string_ref"]
    fn __sklib__read_message__string_ref(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__read_message__server_socket"]
    fn __sklib__read_message__server_socket(svr: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__read_message_data__string_ref"]
    fn __sklib__read_message_data__string_ref(name: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__read_message_data__connection"]
    fn __sklib__read_message_data__connection(a_connection: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__read_message_data__server_socket"]
    fn __sklib__read_message_data__server_socket(svr: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__reconnect__connection"]
    fn __sklib__reconnect__connection(a_connection: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__reconnect__string_ref"]
    fn __sklib__reconnect__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__release_all_connections"]
    fn __sklib__release_all_connections();
}
extern "C" {
    #[link_name = "__sklib__reset_new_connection_count__server_socket"]
    fn __sklib__reset_new_connection_count__server_socket(server: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__retrieve_connection__string_ref__int"]
    fn __sklib__retrieve_connection__string_ref__int(name: __sklib_string, idx: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__retrieve_connection__server_socket__int"]
    fn __sklib__retrieve_connection__server_socket__int(server: __sklib_ptr, idx: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__send_message_to__string_ref__connection"]
    fn __sklib__send_message_to__string_ref__connection(a_msg: __sklib_string, a_connection: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__send_message_to__string_ref__string_ref"]
    fn __sklib__send_message_to__string_ref__string_ref(a_msg: __sklib_string, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__server_has_new_connection__string_ref"]
    fn __sklib__server_has_new_connection__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__server_has_new_connection__server_socket"]
    fn __sklib__server_has_new_connection__server_socket(server: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__server_named__string_ref"]
    fn __sklib__server_named__string_ref(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__set_udp_packet_size__unsigned_int"]
    fn __sklib__set_udp_packet_size__unsigned_int(udp_packet_size: u32);
}
extern "C" {
    #[link_name = "__sklib__udp_packet_size"]
    fn __sklib__udp_packet_size() 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__draw_pixel__color__point_2d_ref"]
    fn __sklib__draw_pixel__color__point_2d_ref(clr: __sklib_color, pt: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel__color__point_2d_ref__drawing_options"]
    fn __sklib__draw_pixel__color__point_2d_ref__drawing_options(clr: __sklib_color, pt: __sklib_point_2d, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel__color__double__double"]
    fn __sklib__draw_pixel__color__double__double(clr: __sklib_color, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel__color__double__double__drawing_options"]
    fn __sklib__draw_pixel__color__double__double__drawing_options(clr: __sklib_color, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel_on_bitmap__bitmap__color__point_2d_ref"]
    fn __sklib__draw_pixel_on_bitmap__bitmap__color__point_2d_ref(destination: __sklib_ptr, clr: __sklib_color, pt: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel_on_bitmap__bitmap__color__point_2d_ref__drawing_options"]
    fn __sklib__draw_pixel_on_bitmap__bitmap__color__point_2d_ref__drawing_options(destination: __sklib_ptr, clr: __sklib_color, pt: __sklib_point_2d, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel_on_bitmap__bitmap__color__double__double"]
    fn __sklib__draw_pixel_on_bitmap__bitmap__color__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel_on_bitmap__bitmap__color__double__double__drawing_options"]
    fn __sklib__draw_pixel_on_bitmap__bitmap__color__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel_on_window__window__color__point_2d_ref"]
    fn __sklib__draw_pixel_on_window__window__color__point_2d_ref(destination: __sklib_ptr, clr: __sklib_color, pt: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel_on_window__window__color__point_2d_ref__drawing_options"]
    fn __sklib__draw_pixel_on_window__window__color__point_2d_ref__drawing_options(destination: __sklib_ptr, clr: __sklib_color, pt: __sklib_point_2d, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel_on_window__window__color__double__double"]
    fn __sklib__draw_pixel_on_window__window__color__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_pixel_on_window__window__color__double__double__drawing_options"]
    fn __sklib__draw_pixel_on_window__window__color__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__get_pixel__bitmap__point_2d_ref"]
    fn __sklib__get_pixel__bitmap__point_2d_ref(bmp: __sklib_ptr, pt: __sklib_point_2d) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__get_pixel__bitmap__double__double"]
    fn __sklib__get_pixel__bitmap__double__double(bmp: __sklib_ptr, x: f64, y: f64) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__get_pixel__point_2d_ref"]
    fn __sklib__get_pixel__point_2d_ref(pt: __sklib_point_2d) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__get_pixel__double__double"]
    fn __sklib__get_pixel__double__double(x: f64, y: f64) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__get_pixel__window__point_2d_ref"]
    fn __sklib__get_pixel__window__point_2d_ref(wnd: __sklib_ptr, pt: __sklib_point_2d) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__get_pixel__window__double__double"]
    fn __sklib__get_pixel__window__double__double(wnd: __sklib_ptr, x: f64, y: f64) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__get_pixel_from_window__window__point_2d_ref"]
    fn __sklib__get_pixel_from_window__window__point_2d_ref(destination: __sklib_ptr, pt: __sklib_point_2d) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__get_pixel_from_window__window__double__double"]
    fn __sklib__get_pixel_from_window__window__double__double(destination: __sklib_ptr, x: f64, y: f64) 
        -> __sklib_color;
}
extern "C" {
    #[link_name = "__sklib__point_at__double__double"]
    fn __sklib__point_at__double__double(x: f64, y: f64) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__point_at_origin"]
    fn __sklib__point_at_origin() 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__point_in_circle__point_2d_ref__circle_ref"]
    fn __sklib__point_in_circle__point_2d_ref__circle_ref(pt: __sklib_point_2d, c: __sklib_circle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_in_circle__double__double__double__double__double"]
    fn __sklib__point_in_circle__double__double__double__double__double(ptx: f64, pty: f64, cx: f64, cy: f64, radius: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_in_quad__point_2d_ref__quad_ref"]
    fn __sklib__point_in_quad__point_2d_ref__quad_ref(pt: __sklib_point_2d, q: __sklib_quad) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_in_rectangle__point_2d_ref__rectangle_ref"]
    fn __sklib__point_in_rectangle__point_2d_ref__rectangle_ref(pt: __sklib_point_2d, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_in_rectangle__double__double__double__double__double__double"]
    fn __sklib__point_in_rectangle__double__double__double__double__double__double(ptx: f64, pty: f64, rect_x: f64, rect_y: f64, rect_width: f64, rect_height: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_in_triangle__point_2d_ref__triangle_ref"]
    fn __sklib__point_in_triangle__point_2d_ref__triangle_ref(pt: __sklib_point_2d, tri: __sklib_triangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_line_distance__point_2d_ref__line_ref"]
    fn __sklib__point_line_distance__point_2d_ref__line_ref(pt: __sklib_point_2d, l: __sklib_line) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__point_offset_by__point_2d_ref__vector_2d_ref"]
    fn __sklib__point_offset_by__point_2d_ref__vector_2d_ref(start_point: __sklib_point_2d, offset: __sklib_vector_2d) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__point_offset_from_origin__vector_2d_ref"]
    fn __sklib__point_offset_from_origin__vector_2d_ref(offset: __sklib_vector_2d) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__point_on_line__point_2d_ref__line_ref"]
    fn __sklib__point_on_line__point_2d_ref__line_ref(pt: __sklib_point_2d, l: __sklib_line) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_on_line__point_2d_ref__line_ref__float"]
    fn __sklib__point_on_line__point_2d_ref__line_ref__float(pt: __sklib_point_2d, l: __sklib_line, proximity: f32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__point_point_angle__point_2d_ref__point_2d_ref"]
    fn __sklib__point_point_angle__point_2d_ref__point_2d_ref(pt1: __sklib_point_2d, pt2: __sklib_point_2d) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__point_point_distance__point_2d_ref__point_2d_ref"]
    fn __sklib__point_point_distance__point_2d_ref__point_2d_ref(pt1: __sklib_point_2d, pt2: __sklib_point_2d) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__point_to_string__point_2d_ref"]
    fn __sklib__point_to_string__point_2d_ref(pt: __sklib_point_2d) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__random_bitmap_point__bitmap"]
    fn __sklib__random_bitmap_point__bitmap(bmp: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__random_screen_point"]
    fn __sklib__random_screen_point() 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__random_window_point__window"]
    fn __sklib__random_window_point__window(wind: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__same_point__point_2d_ref__point_2d_ref"]
    fn __sklib__same_point__point_2d_ref__point_2d_ref(pt1: __sklib_point_2d, pt2: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__quad_from__point_2d_ref__point_2d_ref__point_2d_ref__point_2d_ref"]
    fn __sklib__quad_from__point_2d_ref__point_2d_ref__point_2d_ref__point_2d_ref(p1: __sklib_point_2d, p2: __sklib_point_2d, p3: __sklib_point_2d, p4: __sklib_point_2d) 
        -> __sklib_quad;
}
extern "C" {
    #[link_name = "__sklib__quad_from__rectangle_ref"]
    fn __sklib__quad_from__rectangle_ref(rect: __sklib_rectangle) 
        -> __sklib_quad;
}
extern "C" {
    #[link_name = "__sklib__quad_from__rectangle_ref__matrix_2d_ref"]
    fn __sklib__quad_from__rectangle_ref__matrix_2d_ref(rect: __sklib_rectangle, transform: __sklib_matrix_2d) 
        -> __sklib_quad;
}
extern "C" {
    #[link_name = "__sklib__quad_from__double__double__double__double__double__double__double__double"]
    fn __sklib__quad_from__double__double__double__double__double__double__double__double(x_top_left: f64, y_top_left: f64, x_top_right: f64, y_top_right: f64, x_bottom_left: f64, y_bottom_left: f64, x_bottom_right: f64, y_bottom_right: f64) 
        -> __sklib_quad;
}
extern "C" {
    #[link_name = "__sklib__quads_intersect__quad_ref__quad_ref"]
    fn __sklib__quads_intersect__quad_ref__quad_ref(q1: __sklib_quad, q2: __sklib_quad) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__set_quad_point__quad_ref__int__point_2d_ref"]
    fn __sklib__set_quad_point__quad_ref__int__point_2d_ref(q: &mut __sklib_quad, idx: i32, value: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__triangles_from__quad_ref"]
    fn __sklib__triangles_from__quad_ref(q: __sklib_quad) 
        -> __sklib_vector_triangle;
}
extern "C" {
    #[link_name = "__sklib__rnd__int__int"]
    fn __sklib__rnd__int__int(min: i32, max: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__rnd"]
    fn __sklib__rnd() 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__rnd__int"]
    fn __sklib__rnd__int(ubound: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_gpio"]
    fn __sklib__has_gpio() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__raspi_cleanup"]
    fn __sklib__raspi_cleanup();
}
extern "C" {
    #[link_name = "__sklib__raspi_get_mode__pins"]
    fn __sklib__raspi_get_mode__pins(pin: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__raspi_init"]
    fn __sklib__raspi_init();
}
extern "C" {
    #[link_name = "__sklib__raspi_read__pins"]
    fn __sklib__raspi_read__pins(pin: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__raspi_set_mode__pins__pin_modes"]
    fn __sklib__raspi_set_mode__pins__pin_modes(pin: i32, mode: i32);
}
extern "C" {
    #[link_name = "__sklib__raspi_set_pull_up_down__pins__pull_up_down"]
    fn __sklib__raspi_set_pull_up_down__pins__pull_up_down(pin: i32, pud: i32);
}
extern "C" {
    #[link_name = "__sklib__raspi_set_pwm_dutycycle__pins__int"]
    fn __sklib__raspi_set_pwm_dutycycle__pins__int(pin: i32, dutycycle: i32);
}
extern "C" {
    #[link_name = "__sklib__raspi_set_pwm_frequency__pins__int"]
    fn __sklib__raspi_set_pwm_frequency__pins__int(pin: i32, frequency: i32);
}
extern "C" {
    #[link_name = "__sklib__raspi_set_pwm_range__pins__int"]
    fn __sklib__raspi_set_pwm_range__pins__int(pin: i32, range: i32);
}
extern "C" {
    #[link_name = "__sklib__raspi_write__pins__pin_values"]
    fn __sklib__raspi_write__pins__pin_values(pin: i32, value: i32);
}
extern "C" {
    #[link_name = "__sklib__draw_quad__color__quad_ref"]
    fn __sklib__draw_quad__color__quad_ref(clr: __sklib_color, q: __sklib_quad);
}
extern "C" {
    #[link_name = "__sklib__draw_quad__color__quad_ref__drawing_options_ref"]
    fn __sklib__draw_quad__color__quad_ref__drawing_options_ref(clr: __sklib_color, q: __sklib_quad, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_quad_on_bitmap__bitmap__color__quad_ref"]
    fn __sklib__draw_quad_on_bitmap__bitmap__color__quad_ref(destination: __sklib_ptr, clr: __sklib_color, q: __sklib_quad);
}
extern "C" {
    #[link_name = "__sklib__draw_quad_on_bitmap__bitmap__color__quad_ref__drawing_options_ref"]
    fn __sklib__draw_quad_on_bitmap__bitmap__color__quad_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, q: __sklib_quad, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_quad_on_window__window__color__quad_ref"]
    fn __sklib__draw_quad_on_window__window__color__quad_ref(destination: __sklib_ptr, clr: __sklib_color, q: __sklib_quad);
}
extern "C" {
    #[link_name = "__sklib__draw_quad_on_window__window__color__quad_ref__drawing_options_ref"]
    fn __sklib__draw_quad_on_window__window__color__quad_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, q: __sklib_quad, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle__color__rectangle_ref"]
    fn __sklib__draw_rectangle__color__rectangle_ref(clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle__color__rectangle_ref__drawing_options_ref"]
    fn __sklib__draw_rectangle__color__rectangle_ref__drawing_options_ref(clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle__color__double__double__double__double"]
    fn __sklib__draw_rectangle__color__double__double__double__double(clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle__color__double__double__double__double__drawing_options_ref"]
    fn __sklib__draw_rectangle__color__double__double__double__double__drawing_options_ref(clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle_on_bitmap__bitmap__color__rectangle_ref"]
    fn __sklib__draw_rectangle_on_bitmap__bitmap__color__rectangle_ref(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle_on_bitmap__bitmap__color__rectangle_ref__drawing_options_ref"]
    fn __sklib__draw_rectangle_on_bitmap__bitmap__color__rectangle_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle_on_bitmap__bitmap__color__double__double__double__double"]
    fn __sklib__draw_rectangle_on_bitmap__bitmap__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle_on_bitmap__bitmap__color__double__double__double__double__drawing_options"]
    fn __sklib__draw_rectangle_on_bitmap__bitmap__color__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle_on_window__window__color__rectangle_ref"]
    fn __sklib__draw_rectangle_on_window__window__color__rectangle_ref(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle_on_window__window__color__rectangle_ref__drawing_options_ref"]
    fn __sklib__draw_rectangle_on_window__window__color__rectangle_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle_on_window__window__color__double__double__double__double"]
    fn __sklib__draw_rectangle_on_window__window__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_rectangle_on_window__window__color__double__double__double__double__drawing_options"]
    fn __sklib__draw_rectangle_on_window__window__color__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_quad__color__quad_ref"]
    fn __sklib__fill_quad__color__quad_ref(clr: __sklib_color, q: __sklib_quad);
}
extern "C" {
    #[link_name = "__sklib__fill_quad__color__quad_ref__drawing_options_ref"]
    fn __sklib__fill_quad__color__quad_ref__drawing_options_ref(clr: __sklib_color, q: __sklib_quad, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_quad_on_bitmap__bitmap__color__quad_ref"]
    fn __sklib__fill_quad_on_bitmap__bitmap__color__quad_ref(destination: __sklib_ptr, clr: __sklib_color, q: __sklib_quad);
}
extern "C" {
    #[link_name = "__sklib__fill_quad_on_bitmap__bitmap__color__quad_ref__drawing_options_ref"]
    fn __sklib__fill_quad_on_bitmap__bitmap__color__quad_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, q: __sklib_quad, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_quad_on_window__window__color__quad_ref"]
    fn __sklib__fill_quad_on_window__window__color__quad_ref(destination: __sklib_ptr, clr: __sklib_color, q: __sklib_quad);
}
extern "C" {
    #[link_name = "__sklib__fill_quad_on_window__window__color__quad_ref__drawing_options_ref"]
    fn __sklib__fill_quad_on_window__window__color__quad_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, q: __sklib_quad, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle__color__rectangle_ref"]
    fn __sklib__fill_rectangle__color__rectangle_ref(clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle__color__rectangle_ref__drawing_options_ref"]
    fn __sklib__fill_rectangle__color__rectangle_ref__drawing_options_ref(clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle__color__double__double__double__double"]
    fn __sklib__fill_rectangle__color__double__double__double__double(clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle__color__double__double__double__double__drawing_options_ref"]
    fn __sklib__fill_rectangle__color__double__double__double__double__drawing_options_ref(clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle_on_bitmap__bitmap__color__rectangle_ref"]
    fn __sklib__fill_rectangle_on_bitmap__bitmap__color__rectangle_ref(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle_on_bitmap__bitmap__color__rectangle_ref__drawing_options_ref"]
    fn __sklib__fill_rectangle_on_bitmap__bitmap__color__rectangle_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle_on_bitmap__bitmap__color__double__double__double__double"]
    fn __sklib__fill_rectangle_on_bitmap__bitmap__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle_on_bitmap__bitmap__color__double__double__double__double__drawing_options_ref"]
    fn __sklib__fill_rectangle_on_bitmap__bitmap__color__double__double__double__double__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle_on_window__window__color__rectangle_ref"]
    fn __sklib__fill_rectangle_on_window__window__color__rectangle_ref(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle_on_window__window__color__rectangle_ref__drawing_options_ref"]
    fn __sklib__fill_rectangle_on_window__window__color__rectangle_ref__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, rect: __sklib_rectangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle_on_window__window__color__double__double__double__double"]
    fn __sklib__fill_rectangle_on_window__window__color__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_rectangle_on_window__window__color__double__double__double__double__drawing_options_ref"]
    fn __sklib__fill_rectangle_on_window__window__color__double__double__double__double__drawing_options_ref(destination: __sklib_ptr, clr: __sklib_color, x: f64, y: f64, width: f64, height: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__inset_rectangle__rectangle_ref__float"]
    fn __sklib__inset_rectangle__rectangle_ref__float(rect: __sklib_rectangle, inset_amount: f32) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__intersection__rectangle_ref__rectangle_ref"]
    fn __sklib__intersection__rectangle_ref__rectangle_ref(rect1: __sklib_rectangle, rect2: __sklib_rectangle) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_around__circle_ref"]
    fn __sklib__rectangle_around__circle_ref(c: __sklib_circle) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_around__line_ref"]
    fn __sklib__rectangle_around__line_ref(l: __sklib_line) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_around__quad_ref"]
    fn __sklib__rectangle_around__quad_ref(q: __sklib_quad) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_around__triangle_ref"]
    fn __sklib__rectangle_around__triangle_ref(t: __sklib_triangle) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_bottom__rectangle_ref"]
    fn __sklib__rectangle_bottom__rectangle_ref(rect: __sklib_rectangle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__rectangle_center__rectangle_ref"]
    fn __sklib__rectangle_center__rectangle_ref(rect: __sklib_rectangle) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__rectangle_from__point_2d__double__double"]
    fn __sklib__rectangle_from__point_2d__double__double(pt: __sklib_point_2d, width: f64, height: f64) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_from__point_2d__point_2d"]
    fn __sklib__rectangle_from__point_2d__point_2d(pt1: __sklib_point_2d, pt2: __sklib_point_2d) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_from__double__double__double__double"]
    fn __sklib__rectangle_from__double__double__double__double(x: f64, y: f64, width: f64, height: f64) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_left__rectangle_ref"]
    fn __sklib__rectangle_left__rectangle_ref(rect: __sklib_rectangle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__rectangle_offset_by__rectangle_ref__vector_2d_ref"]
    fn __sklib__rectangle_offset_by__rectangle_ref__vector_2d_ref(rect: __sklib_rectangle, offset: __sklib_vector_2d) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__rectangle_right__rectangle_ref"]
    fn __sklib__rectangle_right__rectangle_ref(rect: __sklib_rectangle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__rectangle_to_string__rectangle_ref"]
    fn __sklib__rectangle_to_string__rectangle_ref(rect: __sklib_rectangle) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__rectangle_top__rectangle_ref"]
    fn __sklib__rectangle_top__rectangle_ref(rect: __sklib_rectangle) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__rectangles_intersect__rectangle_ref__rectangle_ref"]
    fn __sklib__rectangles_intersect__rectangle_ref__rectangle_ref(rect1: __sklib_rectangle, rect2: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__deregister_free_notifier__free_notifier_ptr"]
    fn __sklib__deregister_free_notifier__free_notifier_ptr(handler: __sklib_free_notifier);
}
extern "C" {
    #[link_name = "__sklib__path_to_resource__string_ref__resource_kind"]
    fn __sklib__path_to_resource__string_ref__resource_kind(filename: __sklib_string, kind: i32) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__path_to_resources"]
    fn __sklib__path_to_resources() 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__path_to_resources__resource_kind"]
    fn __sklib__path_to_resources__resource_kind(kind: i32) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__register_free_notifier__free_notifier_ptr"]
    fn __sklib__register_free_notifier__free_notifier_ptr(r#fn: __sklib_free_notifier);
}
extern "C" {
    #[link_name = "__sklib__set_resources_path__string_ref"]
    fn __sklib__set_resources_path__string_ref(path: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__fade_all_sound_effects_out__int"]
    fn __sklib__fade_all_sound_effects_out__int(ms: i32);
}
extern "C" {
    #[link_name = "__sklib__fade_sound_effect_out__sound_effect__int"]
    fn __sklib__fade_sound_effect_out__sound_effect__int(effect: __sklib_ptr, ms: i32);
}
extern "C" {
    #[link_name = "__sklib__free_all_sound_effects"]
    fn __sklib__free_all_sound_effects();
}
extern "C" {
    #[link_name = "__sklib__free_sound_effect__sound_effect"]
    fn __sklib__free_sound_effect__sound_effect(effect: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__has_sound_effect__string_ref"]
    fn __sklib__has_sound_effect__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__load_sound_effect__string_ref__string_ref"]
    fn __sklib__load_sound_effect__string_ref__string_ref(name: __sklib_string, filename: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__play_sound_effect__string_ref"]
    fn __sklib__play_sound_effect__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__play_sound_effect__string_ref__double"]
    fn __sklib__play_sound_effect__string_ref__double(name: __sklib_string, volume: f64);
}
extern "C" {
    #[link_name = "__sklib__play_sound_effect__string_ref__int"]
    fn __sklib__play_sound_effect__string_ref__int(name: __sklib_string, times: i32);
}
extern "C" {
    #[link_name = "__sklib__play_sound_effect__string_ref__int__double"]
    fn __sklib__play_sound_effect__string_ref__int__double(name: __sklib_string, times: i32, volume: f64);
}
extern "C" {
    #[link_name = "__sklib__play_sound_effect__sound_effect"]
    fn __sklib__play_sound_effect__sound_effect(effect: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__play_sound_effect__sound_effect__double"]
    fn __sklib__play_sound_effect__sound_effect__double(effect: __sklib_ptr, volume: f64);
}
extern "C" {
    #[link_name = "__sklib__play_sound_effect__sound_effect__int"]
    fn __sklib__play_sound_effect__sound_effect__int(effect: __sklib_ptr, times: i32);
}
extern "C" {
    #[link_name = "__sklib__play_sound_effect__sound_effect__int__double"]
    fn __sklib__play_sound_effect__sound_effect__int__double(effect: __sklib_ptr, times: i32, volume: f64);
}
extern "C" {
    #[link_name = "__sklib__sound_effect_filename__sound_effect"]
    fn __sklib__sound_effect_filename__sound_effect(effect: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__sound_effect_name__sound_effect"]
    fn __sklib__sound_effect_name__sound_effect(effect: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__sound_effect_named__string_ref"]
    fn __sklib__sound_effect_named__string_ref(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__sound_effect_playing__string_ref"]
    fn __sklib__sound_effect_playing__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sound_effect_playing__sound_effect"]
    fn __sklib__sound_effect_playing__sound_effect(effect: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sound_effect_valid__sound_effect"]
    fn __sklib__sound_effect_valid__sound_effect(effect: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__stop_sound_effect__string_ref"]
    fn __sklib__stop_sound_effect__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__stop_sound_effect__sound_effect"]
    fn __sklib__stop_sound_effect__sound_effect(effect: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__call_for_all_sprites__sprite_float_function_ptr__float"]
    fn __sklib__call_for_all_sprites__sprite_float_function_ptr__float(r#fn: __sklib_sprite_float_function, val: f32);
}
extern "C" {
    #[link_name = "__sklib__call_for_all_sprites__sprite_function_ptr"]
    fn __sklib__call_for_all_sprites__sprite_function_ptr(r#fn: __sklib_sprite_function);
}
extern "C" {
    #[link_name = "__sklib__call_on_sprite_event__sprite_event_handler_ptr"]
    fn __sklib__call_on_sprite_event__sprite_event_handler_ptr(handler: __sklib_sprite_event_handler);
}
extern "C" {
    #[link_name = "__sklib__center_point__sprite"]
    fn __sklib__center_point__sprite(s: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__create_sprite__bitmap"]
    fn __sklib__create_sprite__bitmap(layer: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_sprite__bitmap__animation_script"]
    fn __sklib__create_sprite__bitmap__animation_script(layer: __sklib_ptr, ani: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_sprite__string_ref"]
    fn __sklib__create_sprite__string_ref(bitmap_name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_sprite__string_ref__bitmap"]
    fn __sklib__create_sprite__string_ref__bitmap(name: __sklib_string, layer: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_sprite__string_ref__bitmap__animation_script"]
    fn __sklib__create_sprite__string_ref__bitmap__animation_script(name: __sklib_string, layer: __sklib_ptr, ani: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_sprite__string_ref__string_ref"]
    fn __sklib__create_sprite__string_ref__string_ref(bitmap_name: __sklib_string, animation_name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__create_sprite_pack__string_ref"]
    fn __sklib__create_sprite_pack__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__current_sprite_pack"]
    fn __sklib__current_sprite_pack() 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__draw_all_sprites"]
    fn __sklib__draw_all_sprites();
}
extern "C" {
    #[link_name = "__sklib__draw_sprite__sprite__vector_2d_ref"]
    fn __sklib__draw_sprite__sprite__vector_2d_ref(s: __sklib_ptr, offset: __sklib_vector_2d);
}
extern "C" {
    #[link_name = "__sklib__draw_sprite__sprite"]
    fn __sklib__draw_sprite__sprite(s: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__draw_sprite__sprite__double__double"]
    fn __sklib__draw_sprite__sprite__double__double(s: __sklib_ptr, x_offset: f64, y_offset: f64);
}
extern "C" {
    #[link_name = "__sklib__free_all_sprites"]
    fn __sklib__free_all_sprites();
}
extern "C" {
    #[link_name = "__sklib__free_sprite__sprite"]
    fn __sklib__free_sprite__sprite(s: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__free_sprite_pack__string_ref"]
    fn __sklib__free_sprite_pack__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__has_sprite__string_ref"]
    fn __sklib__has_sprite__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_sprite_pack__string_ref"]
    fn __sklib__has_sprite_pack__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__move_sprite__sprite"]
    fn __sklib__move_sprite__sprite(s: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__move_sprite__sprite__vector_2d_ref"]
    fn __sklib__move_sprite__sprite__vector_2d_ref(s: __sklib_ptr, distance: __sklib_vector_2d);
}
extern "C" {
    #[link_name = "__sklib__move_sprite__sprite__vector_2d_ref__float"]
    fn __sklib__move_sprite__sprite__vector_2d_ref__float(s: __sklib_ptr, distance: __sklib_vector_2d, pct: f32);
}
extern "C" {
    #[link_name = "__sklib__move_sprite__sprite__float"]
    fn __sklib__move_sprite__sprite__float(s: __sklib_ptr, pct: f32);
}
extern "C" {
    #[link_name = "__sklib__move_sprite_to__sprite__double__double"]
    fn __sklib__move_sprite_to__sprite__double__double(s: __sklib_ptr, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__select_sprite_pack__string_ref"]
    fn __sklib__select_sprite_pack__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__sprite_add_layer__sprite__bitmap__string_ref"]
    fn __sklib__sprite_add_layer__sprite__bitmap__string_ref(s: __sklib_ptr, new_layer: __sklib_ptr, layer_name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_add_to_velocity__sprite__vector_2d_ref"]
    fn __sklib__sprite_add_to_velocity__sprite__vector_2d_ref(s: __sklib_ptr, value: __sklib_vector_2d);
}
extern "C" {
    #[link_name = "__sklib__sprite_add_value__sprite__string_ref"]
    fn __sklib__sprite_add_value__sprite__string_ref(s: __sklib_ptr, name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__sprite_add_value__sprite__string_ref__float"]
    fn __sklib__sprite_add_value__sprite__string_ref__float(s: __sklib_ptr, name: __sklib_string, init_val: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_anchor_point__sprite"]
    fn __sklib__sprite_anchor_point__sprite(s: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__sprite_anchor_position__sprite"]
    fn __sklib__sprite_anchor_position__sprite(s: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__sprite_animation_has_ended__sprite"]
    fn __sklib__sprite_animation_has_ended__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_animation_name__sprite"]
    fn __sklib__sprite_animation_name__sprite(s: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__sprite_at__sprite__point_2d_ref"]
    fn __sklib__sprite_at__sprite__point_2d_ref(s: __sklib_ptr, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_bring_layer_forward__sprite__int"]
    fn __sklib__sprite_bring_layer_forward__sprite__int(s: __sklib_ptr, visible_layer: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_bring_layer_to_front__sprite__int"]
    fn __sklib__sprite_bring_layer_to_front__sprite__int(s: __sklib_ptr, visible_layer: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_call_on_event__sprite__sprite_event_handler_ptr"]
    fn __sklib__sprite_call_on_event__sprite__sprite_event_handler_ptr(s: __sklib_ptr, handler: __sklib_sprite_event_handler);
}
extern "C" {
    #[link_name = "__sklib__sprite_circle__sprite"]
    fn __sklib__sprite_circle__sprite(s: __sklib_ptr) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__sprite_collision_bitmap__sprite"]
    fn __sklib__sprite_collision_bitmap__sprite(s: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__sprite_collision_circle__sprite"]
    fn __sklib__sprite_collision_circle__sprite(s: __sklib_ptr) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__sprite_collision_kind__sprite"]
    fn __sklib__sprite_collision_kind__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_collision_rectangle__sprite"]
    fn __sklib__sprite_collision_rectangle__sprite(s: __sklib_ptr) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__sprite_current_cell__sprite"]
    fn __sklib__sprite_current_cell__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_current_cell_rectangle__sprite"]
    fn __sklib__sprite_current_cell_rectangle__sprite(s: __sklib_ptr) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__sprite_dx__sprite"]
    fn __sklib__sprite_dx__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_dy__sprite"]
    fn __sklib__sprite_dy__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_has_value__sprite__string"]
    fn __sklib__sprite_has_value__sprite__string(s: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_heading__sprite"]
    fn __sklib__sprite_heading__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_height__sprite"]
    fn __sklib__sprite_height__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_hide_layer__sprite__string_ref"]
    fn __sklib__sprite_hide_layer__sprite__string_ref(s: __sklib_ptr, name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__sprite_hide_layer__sprite__int"]
    fn __sklib__sprite_hide_layer__sprite__int(s: __sklib_ptr, id: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_layer__sprite__string_ref"]
    fn __sklib__sprite_layer__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer__sprite__int"]
    fn __sklib__sprite_layer__sprite__int(s: __sklib_ptr, idx: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_circle__sprite__string_ref"]
    fn __sklib__sprite_layer_circle__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_circle__sprite__int"]
    fn __sklib__sprite_layer_circle__sprite__int(s: __sklib_ptr, idx: i32) 
        -> __sklib_circle;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_count__sprite"]
    fn __sklib__sprite_layer_count__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_height__sprite__string_ref"]
    fn __sklib__sprite_layer_height__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_height__sprite__int"]
    fn __sklib__sprite_layer_height__sprite__int(s: __sklib_ptr, idx: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_index__sprite__string_ref"]
    fn __sklib__sprite_layer_index__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_name__sprite__int"]
    fn __sklib__sprite_layer_name__sprite__int(s: __sklib_ptr, idx: i32) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_offset__sprite__string_ref"]
    fn __sklib__sprite_layer_offset__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_offset__sprite__int"]
    fn __sklib__sprite_layer_offset__sprite__int(s: __sklib_ptr, idx: i32) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_rectangle__sprite__string_ref"]
    fn __sklib__sprite_layer_rectangle__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_rectangle__sprite__int"]
    fn __sklib__sprite_layer_rectangle__sprite__int(s: __sklib_ptr, idx: i32) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_width__sprite__string_ref"]
    fn __sklib__sprite_layer_width__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_layer_width__sprite__int"]
    fn __sklib__sprite_layer_width__sprite__int(s: __sklib_ptr, idx: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_location_matrix__sprite"]
    fn __sklib__sprite_location_matrix__sprite(s: __sklib_ptr) 
        -> __sklib_matrix_2d;
}
extern "C" {
    #[link_name = "__sklib__sprite_mass__sprite"]
    fn __sklib__sprite_mass__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_move_from_anchor_point__sprite"]
    fn __sklib__sprite_move_from_anchor_point__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_move_to__sprite__point_2d_ref__float"]
    fn __sklib__sprite_move_to__sprite__point_2d_ref__float(s: __sklib_ptr, pt: __sklib_point_2d, taking_seconds: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_name__sprite"]
    fn __sklib__sprite_name__sprite(s: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__sprite_named__string_ref"]
    fn __sklib__sprite_named__string_ref(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__sprite_offscreen__sprite"]
    fn __sklib__sprite_offscreen__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_on_screen_at__sprite__point_2d_ref"]
    fn __sklib__sprite_on_screen_at__sprite__point_2d_ref(s: __sklib_ptr, pt: __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_on_screen_at__sprite__double__double"]
    fn __sklib__sprite_on_screen_at__sprite__double__double(s: __sklib_ptr, x: f64, y: f64) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_position__sprite"]
    fn __sklib__sprite_position__sprite(s: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__sprite_replay_animation__sprite"]
    fn __sklib__sprite_replay_animation__sprite(s: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__sprite_replay_animation__sprite__bool"]
    fn __sklib__sprite_replay_animation__sprite__bool(s: __sklib_ptr, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_rotation__sprite"]
    fn __sklib__sprite_rotation__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_scale__sprite"]
    fn __sklib__sprite_scale__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_screen_rectangle__sprite"]
    fn __sklib__sprite_screen_rectangle__sprite(s: __sklib_ptr) 
        -> __sklib_rectangle;
}
extern "C" {
    #[link_name = "__sklib__sprite_send_layer_backward__sprite__int"]
    fn __sklib__sprite_send_layer_backward__sprite__int(s: __sklib_ptr, visible_layer: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_send_layer_to_back__sprite__int"]
    fn __sklib__sprite_send_layer_to_back__sprite__int(s: __sklib_ptr, visible_layer: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_anchor_point__sprite__point_2d_ref"]
    fn __sklib__sprite_set_anchor_point__sprite__point_2d_ref(s: __sklib_ptr, pt: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_collision_bitmap__sprite__bitmap"]
    fn __sklib__sprite_set_collision_bitmap__sprite__bitmap(s: __sklib_ptr, bmp: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_collision_kind__sprite__collision_test_kind"]
    fn __sklib__sprite_set_collision_kind__sprite__collision_test_kind(s: __sklib_ptr, value: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_dx__sprite__float"]
    fn __sklib__sprite_set_dx__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_dy__sprite__float"]
    fn __sklib__sprite_set_dy__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_heading__sprite__float"]
    fn __sklib__sprite_set_heading__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_layer_offset__sprite__string_ref__vector_2d_ref"]
    fn __sklib__sprite_set_layer_offset__sprite__string_ref__vector_2d_ref(s: __sklib_ptr, name: __sklib_string, value: __sklib_vector_2d);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_layer_offset__sprite__int__vector_2d_ref"]
    fn __sklib__sprite_set_layer_offset__sprite__int__vector_2d_ref(s: __sklib_ptr, idx: i32, value: __sklib_vector_2d);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_mass__sprite__float"]
    fn __sklib__sprite_set_mass__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_move_from_anchor_point__sprite__bool"]
    fn __sklib__sprite_set_move_from_anchor_point__sprite__bool(s: __sklib_ptr, value: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_position__sprite__point_2d_ref"]
    fn __sklib__sprite_set_position__sprite__point_2d_ref(s: __sklib_ptr, value: __sklib_point_2d);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_rotation__sprite__float"]
    fn __sklib__sprite_set_rotation__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_scale__sprite__float"]
    fn __sklib__sprite_set_scale__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_speed__sprite__float"]
    fn __sklib__sprite_set_speed__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_value__sprite__string_ref__float"]
    fn __sklib__sprite_set_value__sprite__string_ref__float(s: __sklib_ptr, name: __sklib_string, val: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_velocity__sprite__vector_2d_ref"]
    fn __sklib__sprite_set_velocity__sprite__vector_2d_ref(s: __sklib_ptr, value: __sklib_vector_2d);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_x__sprite__float"]
    fn __sklib__sprite_set_x__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_set_y__sprite__float"]
    fn __sklib__sprite_set_y__sprite__float(s: __sklib_ptr, value: f32);
}
extern "C" {
    #[link_name = "__sklib__sprite_show_layer__sprite__string_ref"]
    fn __sklib__sprite_show_layer__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_show_layer__sprite__int"]
    fn __sklib__sprite_show_layer__sprite__int(s: __sklib_ptr, id: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_speed__sprite"]
    fn __sklib__sprite_speed__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_start_animation__sprite__string_ref"]
    fn __sklib__sprite_start_animation__sprite__string_ref(s: __sklib_ptr, named: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__sprite_start_animation__sprite__string_ref__bool"]
    fn __sklib__sprite_start_animation__sprite__string_ref__bool(s: __sklib_ptr, named: __sklib_string, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_start_animation__sprite__int"]
    fn __sklib__sprite_start_animation__sprite__int(s: __sklib_ptr, idx: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_start_animation__sprite__int__bool"]
    fn __sklib__sprite_start_animation__sprite__int__bool(s: __sklib_ptr, idx: i32, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_stop_calling_on_event__sprite__sprite_event_handler_ptr"]
    fn __sklib__sprite_stop_calling_on_event__sprite__sprite_event_handler_ptr(s: __sklib_ptr, handler: __sklib_sprite_event_handler);
}
extern "C" {
    #[link_name = "__sklib__sprite_toggle_layer_visible__sprite__string_ref"]
    fn __sklib__sprite_toggle_layer_visible__sprite__string_ref(s: __sklib_ptr, name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__sprite_toggle_layer_visible__sprite__int"]
    fn __sklib__sprite_toggle_layer_visible__sprite__int(s: __sklib_ptr, id: i32);
}
extern "C" {
    #[link_name = "__sklib__sprite_value__sprite__string_ref"]
    fn __sklib__sprite_value__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_value_count__sprite"]
    fn __sklib__sprite_value_count__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_velocity__sprite"]
    fn __sklib__sprite_velocity__sprite(s: __sklib_ptr) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__sprite_visible_index_of_layer__sprite__string_ref"]
    fn __sklib__sprite_visible_index_of_layer__sprite__string_ref(s: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_visible_index_of_layer__sprite__int"]
    fn __sklib__sprite_visible_index_of_layer__sprite__int(s: __sklib_ptr, id: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_visible_layer__sprite__int"]
    fn __sklib__sprite_visible_layer__sprite__int(s: __sklib_ptr, idx: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_visible_layer_count__sprite"]
    fn __sklib__sprite_visible_layer_count__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_visible_layer_id__sprite__int"]
    fn __sklib__sprite_visible_layer_id__sprite__int(s: __sklib_ptr, idx: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_width__sprite"]
    fn __sklib__sprite_width__sprite(s: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__sprite_x__sprite"]
    fn __sklib__sprite_x__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__sprite_y__sprite"]
    fn __sklib__sprite_y__sprite(s: __sklib_ptr) 
        -> f32;
}
extern "C" {
    #[link_name = "__sklib__stop_calling_on_sprite_event__sprite_event_handler_ptr"]
    fn __sklib__stop_calling_on_sprite_event__sprite_event_handler_ptr(handler: __sklib_sprite_event_handler);
}
extern "C" {
    #[link_name = "__sklib__update_all_sprites"]
    fn __sklib__update_all_sprites();
}
extern "C" {
    #[link_name = "__sklib__update_all_sprites__float"]
    fn __sklib__update_all_sprites__float(pct: f32);
}
extern "C" {
    #[link_name = "__sklib__update_sprite__sprite"]
    fn __sklib__update_sprite__sprite(s: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__update_sprite__sprite__bool"]
    fn __sklib__update_sprite__sprite__bool(s: __sklib_ptr, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__update_sprite__sprite__float"]
    fn __sklib__update_sprite__sprite__float(s: __sklib_ptr, pct: f32);
}
extern "C" {
    #[link_name = "__sklib__update_sprite__sprite__float__bool"]
    fn __sklib__update_sprite__sprite__float__bool(s: __sklib_ptr, pct: f32, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__update_sprite_animation__sprite"]
    fn __sklib__update_sprite_animation__sprite(s: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__update_sprite_animation__sprite__bool"]
    fn __sklib__update_sprite_animation__sprite__bool(s: __sklib_ptr, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__update_sprite_animation__sprite__float"]
    fn __sklib__update_sprite_animation__sprite__float(s: __sklib_ptr, pct: f32);
}
extern "C" {
    #[link_name = "__sklib__update_sprite_animation__sprite__float__bool"]
    fn __sklib__update_sprite_animation__sprite__float__bool(s: __sklib_ptr, pct: f32, with_sound: i32);
}
extern "C" {
    #[link_name = "__sklib__vector_from_center_sprite_to_point__sprite__point_2d_ref"]
    fn __sklib__vector_from_center_sprite_to_point__sprite__point_2d_ref(s: __sklib_ptr, pt: __sklib_point_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_from_to__sprite__sprite"]
    fn __sklib__vector_from_to__sprite__sprite(s1: __sklib_ptr, s2: __sklib_ptr) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__read_char"]
    fn __sklib__read_char() 
        -> c_char;
}
extern "C" {
    #[link_name = "__sklib__read_line"]
    fn __sklib__read_line() 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__terminal_has_input"]
    fn __sklib__terminal_has_input() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__write__char"]
    fn __sklib__write__char(data: c_char);
}
extern "C" {
    #[link_name = "__sklib__write__double"]
    fn __sklib__write__double(data: f64);
}
extern "C" {
    #[link_name = "__sklib__write__int"]
    fn __sklib__write__int(data: i32);
}
extern "C" {
    #[link_name = "__sklib__write__string"]
    fn __sklib__write__string(text: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__write_line__char"]
    fn __sklib__write_line__char(data: c_char);
}
extern "C" {
    #[link_name = "__sklib__write_line"]
    fn __sklib__write_line();
}
extern "C" {
    #[link_name = "__sklib__write_line__double"]
    fn __sklib__write_line__double(data: f64);
}
extern "C" {
    #[link_name = "__sklib__write_line__int"]
    fn __sklib__write_line__int(data: i32);
}
extern "C" {
    #[link_name = "__sklib__write_line__string"]
    fn __sklib__write_line__string(line: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__draw_text__string_ref__color_ref__string_ref__int__double__double"]
    fn __sklib__draw_text__string_ref__color_ref__string_ref__int__double__double(text: __sklib_string, clr: __sklib_color, fnt: __sklib_string, font_size: i32, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref"]
    fn __sklib__draw_text__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref(text: __sklib_string, clr: __sklib_color, fnt: __sklib_string, font_size: i32, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_text__string_ref__color_ref__double__double"]
    fn __sklib__draw_text__string_ref__color_ref__double__double(text: __sklib_string, clr: __sklib_color, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text__string_ref__color_ref__double__double__drawing_options_ref"]
    fn __sklib__draw_text__string_ref__color_ref__double__double__drawing_options_ref(text: __sklib_string, clr: __sklib_color, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_text__string_ref__color_ref__font__int__double__double"]
    fn __sklib__draw_text__string_ref__color_ref__font__int__double__double(text: __sklib_string, clr: __sklib_color, fnt: __sklib_ptr, font_size: i32, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text__string_ref__color_ref__font__int__double__double__drawing_options_ref"]
    fn __sklib__draw_text__string_ref__color_ref__font__int__double__double__drawing_options_ref(text: __sklib_string, clr: __sklib_color, fnt: __sklib_ptr, font_size: i32, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__string_ref__int__double__double"]
    fn __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__string_ref__int__double__double(bmp: __sklib_ptr, text: __sklib_string, clr: __sklib_color, fnt: __sklib_string, font_size: i32, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref"]
    fn __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref(bmp: __sklib_ptr, text: __sklib_string, clr: __sklib_color, fnt: __sklib_string, font_size: i32, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__double__double"]
    fn __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__double__double(bmp: __sklib_ptr, text: __sklib_string, clr: __sklib_color, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__double__double__drawing_options_ref"]
    fn __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__double__double__drawing_options_ref(bmp: __sklib_ptr, text: __sklib_string, clr: __sklib_color, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__font__int__double__double"]
    fn __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__font__int__double__double(bmp: __sklib_ptr, text: __sklib_string, clr: __sklib_color, fnt: __sklib_ptr, font_size: i32, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__font__int__double__double__drawing_options_ref"]
    fn __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__font__int__double__double__drawing_options_ref(bmp: __sklib_ptr, text: __sklib_string, clr: __sklib_color, fnt: __sklib_ptr, font_size: i32, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_window__window__string_ref__color_ref__string_ref__int__double__double"]
    fn __sklib__draw_text_on_window__window__string_ref__color_ref__string_ref__int__double__double(wnd: __sklib_ptr, text: __sklib_string, clr: __sklib_color, fnt: __sklib_string, font_size: i32, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_window__window__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref"]
    fn __sklib__draw_text_on_window__window__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref(wnd: __sklib_ptr, text: __sklib_string, clr: __sklib_color, fnt: __sklib_string, font_size: i32, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_window__window__string_ref__color_ref__double__double"]
    fn __sklib__draw_text_on_window__window__string_ref__color_ref__double__double(wnd: __sklib_ptr, text: __sklib_string, clr: __sklib_color, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_window__window__string_ref__color_ref__double__double__drawing_options_ref"]
    fn __sklib__draw_text_on_window__window__string_ref__color_ref__double__double__drawing_options_ref(wnd: __sklib_ptr, text: __sklib_string, clr: __sklib_color, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_window__window__string_ref__color_ref__font__int__double__double"]
    fn __sklib__draw_text_on_window__window__string_ref__color_ref__font__int__double__double(wnd: __sklib_ptr, text: __sklib_string, clr: __sklib_color, fnt: __sklib_ptr, font_size: i32, x: f64, y: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_text_on_window__window__string_ref__color_ref__font__int__double__double__drawing_options_ref"]
    fn __sklib__draw_text_on_window__window__string_ref__color_ref__font__int__double__double__drawing_options_ref(wnd: __sklib_ptr, text: __sklib_string, clr: __sklib_color, fnt: __sklib_ptr, font_size: i32, x: f64, y: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__font_has_size__string_ref__int"]
    fn __sklib__font_has_size__string_ref__int(name: __sklib_string, font_size: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__font_has_size__font__int"]
    fn __sklib__font_has_size__font__int(fnt: __sklib_ptr, font_size: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__font_load_size__string_ref__int"]
    fn __sklib__font_load_size__string_ref__int(name: __sklib_string, font_size: i32);
}
extern "C" {
    #[link_name = "__sklib__font_load_size__font__int"]
    fn __sklib__font_load_size__font__int(fnt: __sklib_ptr, font_size: i32);
}
extern "C" {
    #[link_name = "__sklib__font_named__string"]
    fn __sklib__font_named__string(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__free_all_fonts"]
    fn __sklib__free_all_fonts();
}
extern "C" {
    #[link_name = "__sklib__free_font__font"]
    fn __sklib__free_font__font(fnt: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__get_font_style__string_ref"]
    fn __sklib__get_font_style__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__get_font_style__font"]
    fn __sklib__get_font_style__font(fnt: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__get_system_font"]
    fn __sklib__get_system_font() 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__has_font__font"]
    fn __sklib__has_font__font(fnt: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_font__string"]
    fn __sklib__has_font__string(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__load_font__string_ref__string_ref"]
    fn __sklib__load_font__string_ref__string_ref(name: __sklib_string, filename: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__set_font_style__string_ref__font_style"]
    fn __sklib__set_font_style__string_ref__font_style(name: __sklib_string, style: i32);
}
extern "C" {
    #[link_name = "__sklib__set_font_style__font__font_style"]
    fn __sklib__set_font_style__font__font_style(fnt: __sklib_ptr, style: i32);
}
extern "C" {
    #[link_name = "__sklib__text_height__string_ref__string_ref__int"]
    fn __sklib__text_height__string_ref__string_ref__int(text: __sklib_string, fnt: __sklib_string, font_size: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__text_height__string_ref__font__int"]
    fn __sklib__text_height__string_ref__font__int(text: __sklib_string, fnt: __sklib_ptr, font_size: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__text_width__string_ref__string_ref__int"]
    fn __sklib__text_width__string_ref__string_ref__int(text: __sklib_string, fnt: __sklib_string, font_size: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__text_width__string_ref__font__int"]
    fn __sklib__text_width__string_ref__font__int(text: __sklib_string, fnt: __sklib_ptr, font_size: i32) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__draw_collected_text__color__font__int__drawing_options_ref"]
    fn __sklib__draw_collected_text__color__font__int__drawing_options_ref(clr: __sklib_color, fnt: __sklib_ptr, font_size: i32, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__end_reading_text"]
    fn __sklib__end_reading_text();
}
extern "C" {
    #[link_name = "__sklib__end_reading_text__window"]
    fn __sklib__end_reading_text__window(wind: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__reading_text"]
    fn __sklib__reading_text() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__reading_text__window"]
    fn __sklib__reading_text__window(wind: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__start_reading_text__rectangle"]
    fn __sklib__start_reading_text__rectangle(rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__start_reading_text__rectangle__string"]
    fn __sklib__start_reading_text__rectangle__string(rect: __sklib_rectangle, initial_text: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__start_reading_text__window__rectangle"]
    fn __sklib__start_reading_text__window__rectangle(wind: __sklib_ptr, rect: __sklib_rectangle);
}
extern "C" {
    #[link_name = "__sklib__start_reading_text__window__rectangle__string"]
    fn __sklib__start_reading_text__window__rectangle__string(wind: __sklib_ptr, rect: __sklib_rectangle, initial_text: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__text_entry_cancelled"]
    fn __sklib__text_entry_cancelled() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__text_entry_cancelled__window"]
    fn __sklib__text_entry_cancelled__window(wind: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__text_input"]
    fn __sklib__text_input() 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__text_input__window"]
    fn __sklib__text_input__window(wind: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__create_timer__string"]
    fn __sklib__create_timer__string(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__free_all_timers"]
    fn __sklib__free_all_timers();
}
extern "C" {
    #[link_name = "__sklib__free_timer__timer"]
    fn __sklib__free_timer__timer(to_free: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__has_timer__string"]
    fn __sklib__has_timer__string(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__pause_timer__string"]
    fn __sklib__pause_timer__string(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__pause_timer__timer"]
    fn __sklib__pause_timer__timer(to_pause: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__reset_timer__string"]
    fn __sklib__reset_timer__string(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__reset_timer__timer"]
    fn __sklib__reset_timer__timer(tmr: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__resume_timer__string"]
    fn __sklib__resume_timer__string(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__resume_timer__timer"]
    fn __sklib__resume_timer__timer(to_resume: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__start_timer__string"]
    fn __sklib__start_timer__string(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__start_timer__timer"]
    fn __sklib__start_timer__timer(to_start: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__stop_timer__string"]
    fn __sklib__stop_timer__string(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__stop_timer__timer"]
    fn __sklib__stop_timer__timer(to_stop: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__timer_named__string"]
    fn __sklib__timer_named__string(name: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__timer_paused__string"]
    fn __sklib__timer_paused__string(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__timer_paused__timer"]
    fn __sklib__timer_paused__timer(to_get: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__timer_started__string"]
    fn __sklib__timer_started__string(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__timer_started__timer"]
    fn __sklib__timer_started__timer(to_get: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__timer_ticks__string"]
    fn __sklib__timer_ticks__string(name: __sklib_string) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__timer_ticks__timer"]
    fn __sklib__timer_ticks__timer(to_get: __sklib_ptr) 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__draw_triangle__color__triangle_ref"]
    fn __sklib__draw_triangle__color__triangle_ref(clr: __sklib_color, tri: __sklib_triangle);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle__color__triangle_ref__drawing_options"]
    fn __sklib__draw_triangle__color__triangle_ref__drawing_options(clr: __sklib_color, tri: __sklib_triangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle__color__double__double__double__double__double__double"]
    fn __sklib__draw_triangle__color__double__double__double__double__double__double(clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle__color__double__double__double__double__double__double__drawing_options"]
    fn __sklib__draw_triangle__color__double__double__double__double__double__double__drawing_options(clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle_on_bitmap__bitmap__color__triangle_ref"]
    fn __sklib__draw_triangle_on_bitmap__bitmap__color__triangle_ref(destination: __sklib_ptr, clr: __sklib_color, tri: __sklib_triangle);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle_on_bitmap__bitmap__color__triangle_ref__drawing_options"]
    fn __sklib__draw_triangle_on_bitmap__bitmap__color__triangle_ref__drawing_options(destination: __sklib_ptr, clr: __sklib_color, tri: __sklib_triangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double"]
    fn __sklib__draw_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double__drawing_options"]
    fn __sklib__draw_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle_on_window__window__color__triangle_ref"]
    fn __sklib__draw_triangle_on_window__window__color__triangle_ref(destination: __sklib_ptr, clr: __sklib_color, tri: __sklib_triangle);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle_on_window__window__color__triangle_ref__drawing_options"]
    fn __sklib__draw_triangle_on_window__window__color__triangle_ref__drawing_options(destination: __sklib_ptr, clr: __sklib_color, tri: __sklib_triangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle_on_window__window__color__double__double__double__double__double__double"]
    fn __sklib__draw_triangle_on_window__window__color__double__double__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
}
extern "C" {
    #[link_name = "__sklib__draw_triangle_on_window__window__color__double__double__double__double__double__double__drawing_options"]
    fn __sklib__draw_triangle_on_window__window__color__double__double__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle__color__triangle_ref"]
    fn __sklib__fill_triangle__color__triangle_ref(clr: __sklib_color, tri: __sklib_triangle);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle__color__triangle_ref__drawing_options"]
    fn __sklib__fill_triangle__color__triangle_ref__drawing_options(clr: __sklib_color, tri: __sklib_triangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle__color__double__double__double__double__double__double"]
    fn __sklib__fill_triangle__color__double__double__double__double__double__double(clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle__color__double__double__double__double__double__double__drawing_options"]
    fn __sklib__fill_triangle__color__double__double__double__double__double__double__drawing_options(clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle_on_bitmap__bitmap__color__triangle_ref"]
    fn __sklib__fill_triangle_on_bitmap__bitmap__color__triangle_ref(destination: __sklib_ptr, clr: __sklib_color, tri: __sklib_triangle);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle_on_bitmap__bitmap__color__triangle_ref__drawing_options"]
    fn __sklib__fill_triangle_on_bitmap__bitmap__color__triangle_ref__drawing_options(destination: __sklib_ptr, clr: __sklib_color, tri: __sklib_triangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double"]
    fn __sklib__fill_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double__drawing_options"]
    fn __sklib__fill_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle_on_window__window__color__triangle_ref"]
    fn __sklib__fill_triangle_on_window__window__color__triangle_ref(destination: __sklib_ptr, clr: __sklib_color, tri: __sklib_triangle);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle_on_window__window__color__triangle_ref__drawing_options"]
    fn __sklib__fill_triangle_on_window__window__color__triangle_ref__drawing_options(destination: __sklib_ptr, clr: __sklib_color, tri: __sklib_triangle, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle_on_window__window__color__double__double__double__double__double__double"]
    fn __sklib__fill_triangle_on_window__window__color__double__double__double__double__double__double(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
}
extern "C" {
    #[link_name = "__sklib__fill_triangle_on_window__window__color__double__double__double__double__double__double__drawing_options"]
    fn __sklib__fill_triangle_on_window__window__color__double__double__double__double__double__double__drawing_options(destination: __sklib_ptr, clr: __sklib_color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: __sklib_drawing_options);
}
extern "C" {
    #[link_name = "__sklib__triangle_barycenter__triangle_ref"]
    fn __sklib__triangle_barycenter__triangle_ref(tri: __sklib_triangle) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__triangle_from__point_2d_ref__point_2d_ref__point_2d_ref"]
    fn __sklib__triangle_from__point_2d_ref__point_2d_ref__point_2d_ref(p1: __sklib_point_2d, p2: __sklib_point_2d, p3: __sklib_point_2d) 
        -> __sklib_triangle;
}
extern "C" {
    #[link_name = "__sklib__triangle_from__double__double__double__double__double__double"]
    fn __sklib__triangle_from__double__double__double__double__double__double(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) 
        -> __sklib_triangle;
}
extern "C" {
    #[link_name = "__sklib__triangle_rectangle_intersect__triangle_ref__rectangle_ref"]
    fn __sklib__triangle_rectangle_intersect__triangle_ref__rectangle_ref(tri: __sklib_triangle, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__triangle_to_string__triangle_ref"]
    fn __sklib__triangle_to_string__triangle_ref(tri: __sklib_triangle) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__triangles_intersect__triangle_ref__triangle_ref"]
    fn __sklib__triangles_intersect__triangle_ref__triangle_ref(t1: __sklib_triangle, t2: __sklib_triangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__current_ticks"]
    fn __sklib__current_ticks() 
        -> u32;
}
extern "C" {
    #[link_name = "__sklib__delay__int"]
    fn __sklib__delay__int(milliseconds: i32);
}
extern "C" {
    #[link_name = "__sklib__display_dialog__string_ref__string_ref__font__int"]
    fn __sklib__display_dialog__string_ref__string_ref__font__int(title: __sklib_string, msg: __sklib_string, output_font: __sklib_ptr, font_size: i32);
}
extern "C" {
    #[link_name = "__sklib__file_as_string__string__resource_kind"]
    fn __sklib__file_as_string__string__resource_kind(filename: __sklib_string, kind: i32) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__angle_between__vector_2d_ref__vector_2d_ref"]
    fn __sklib__angle_between__vector_2d_ref__vector_2d_ref(v1: __sklib_vector_2d, v2: __sklib_vector_2d) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__dot_product__vector_2d_ref__vector_2d_ref"]
    fn __sklib__dot_product__vector_2d_ref__vector_2d_ref(v1: __sklib_vector_2d, v2: __sklib_vector_2d) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__is_zero_vector__vector_2d_ref"]
    fn __sklib__is_zero_vector__vector_2d_ref(v: __sklib_vector_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__ray_intersection_point__point_2d_ref__vector_2d_ref__line_ref__point_2d_ref"]
    fn __sklib__ray_intersection_point__point_2d_ref__vector_2d_ref__line_ref__point_2d_ref(from_pt: __sklib_point_2d, heading: __sklib_vector_2d, l: __sklib_line, pt: &mut __sklib_point_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__unit_vector__vector_2d_ref"]
    fn __sklib__unit_vector__vector_2d_ref(v: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_add__vector_2d_ref__vector_2d_ref"]
    fn __sklib__vector_add__vector_2d_ref__vector_2d_ref(v1: __sklib_vector_2d, v2: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_angle__vector_2d"]
    fn __sklib__vector_angle__vector_2d(v: __sklib_vector_2d) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__vector_from_angle__double__double"]
    fn __sklib__vector_from_angle__double__double(angle: f64, magnitude: f64) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_from_line__line_ref"]
    fn __sklib__vector_from_line__line_ref(l: __sklib_line) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_from_point_to_rect__point_2d_ref__rectangle_ref"]
    fn __sklib__vector_from_point_to_rect__point_2d_ref__rectangle_ref(pt: __sklib_point_2d, rect: __sklib_rectangle) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_in_rect__vector_2d_ref__rectangle_ref"]
    fn __sklib__vector_in_rect__vector_2d_ref__rectangle_ref(v: __sklib_vector_2d, rect: __sklib_rectangle) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__vector_invert__vector_2d_ref"]
    fn __sklib__vector_invert__vector_2d_ref(v: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_limit__vector_2d_ref__double"]
    fn __sklib__vector_limit__vector_2d_ref__double(v: __sklib_vector_2d, limit: f64) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_magnitude__vector_2d_ref"]
    fn __sklib__vector_magnitude__vector_2d_ref(v: __sklib_vector_2d) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__vector_magnitude_squared__vector_2d_ref"]
    fn __sklib__vector_magnitude_squared__vector_2d_ref(v: __sklib_vector_2d) 
        -> f64;
}
extern "C" {
    #[link_name = "__sklib__vector_multiply__vector_2d_ref__double"]
    fn __sklib__vector_multiply__vector_2d_ref__double(v1: __sklib_vector_2d, s: f64) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_normal__vector_2d_ref"]
    fn __sklib__vector_normal__vector_2d_ref(v: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_out_of_circle_from_circle__circle_ref__circle_ref__vector_2d_ref"]
    fn __sklib__vector_out_of_circle_from_circle__circle_ref__circle_ref__vector_2d_ref(src: __sklib_circle, bounds: __sklib_circle, velocity: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_out_of_circle_from_point__point_2d_ref__circle_ref__vector_2d_ref"]
    fn __sklib__vector_out_of_circle_from_point__point_2d_ref__circle_ref__vector_2d_ref(pt: __sklib_point_2d, c: __sklib_circle, velocity: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_out_of_rect_from_circle__circle_ref__rectangle_ref__vector_2d_ref"]
    fn __sklib__vector_out_of_rect_from_circle__circle_ref__rectangle_ref__vector_2d_ref(c: __sklib_circle, rect: __sklib_rectangle, velocity: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_out_of_rect_from_point__point_2d_ref__rectangle_ref__vector_2d_ref"]
    fn __sklib__vector_out_of_rect_from_point__point_2d_ref__rectangle_ref__vector_2d_ref(pt: __sklib_point_2d, rect: __sklib_rectangle, velocity: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_out_of_rect_from_rect__rectangle_ref__rectangle_ref__vector_2d_ref"]
    fn __sklib__vector_out_of_rect_from_rect__rectangle_ref__rectangle_ref__vector_2d_ref(src: __sklib_rectangle, bounds: __sklib_rectangle, velocity: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_point_to_point__point_2d_ref__point_2d_ref"]
    fn __sklib__vector_point_to_point__point_2d_ref__point_2d_ref(start: __sklib_point_2d, end_pt: __sklib_point_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_subtract__vector_2d_ref__vector_2d_ref"]
    fn __sklib__vector_subtract__vector_2d_ref__vector_2d_ref(v1: __sklib_vector_2d, v2: __sklib_vector_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_to__point_2d_ref"]
    fn __sklib__vector_to__point_2d_ref(p1: __sklib_point_2d) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_to__double__double"]
    fn __sklib__vector_to__double__double(x: f64, y: f64) 
        -> __sklib_vector_2d;
}
extern "C" {
    #[link_name = "__sklib__vector_to_string__vector_2d_ref"]
    fn __sklib__vector_to_string__vector_2d_ref(v: __sklib_vector_2d) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__vectors_equal__vector_2d_ref__vector_2d"]
    fn __sklib__vectors_equal__vector_2d_ref__vector_2d(v1: __sklib_vector_2d, v2: __sklib_vector_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__vectors_not_equal__vector_2d_ref__vector_2d"]
    fn __sklib__vectors_not_equal__vector_2d_ref__vector_2d(v1: __sklib_vector_2d, v2: __sklib_vector_2d) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__download_bitmap__string_ref__string_ref__unsigned_short"]
    fn __sklib__download_bitmap__string_ref__string_ref__unsigned_short(name: __sklib_string, url: __sklib_string, port: u16) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__download_font__string_ref__string_ref__unsigned_short"]
    fn __sklib__download_font__string_ref__string_ref__unsigned_short(name: __sklib_string, url: __sklib_string, port: u16) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__download_music__string_ref__string_ref__unsigned_short"]
    fn __sklib__download_music__string_ref__string_ref__unsigned_short(name: __sklib_string, url: __sklib_string, port: u16) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__download_sound_effect__string_ref__string_ref__unsigned_short"]
    fn __sklib__download_sound_effect__string_ref__string_ref__unsigned_short(name: __sklib_string, url: __sklib_string, port: u16) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__free_response__http_response"]
    fn __sklib__free_response__http_response(response: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__http_get__string_ref__unsigned_short"]
    fn __sklib__http_get__string_ref__unsigned_short(url: __sklib_string, port: u16) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__http_post__string_ref__unsigned_short__string_ref__vector_string_ref"]
    fn __sklib__http_post__string_ref__unsigned_short__string_ref__vector_string_ref(url: __sklib_string, port: u16, body: __sklib_string, headers: __sklib_vector_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__http_post__string_ref__unsigned_short__string"]
    fn __sklib__http_post__string_ref__unsigned_short__string(url: __sklib_string, port: u16, body: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__http_response_to_string__http_response"]
    fn __sklib__http_response_to_string__http_response(response: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__save_response_to_file__http_response__string"]
    fn __sklib__save_response_to_file__http_response__string(response: __sklib_ptr, path: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__has_incoming_requests__web_server"]
    fn __sklib__has_incoming_requests__web_server(server: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_delete_request_for__http_request__string_ref"]
    fn __sklib__is_delete_request_for__http_request__string_ref(request: __sklib_ptr, path: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_get_request_for__http_request__string_ref"]
    fn __sklib__is_get_request_for__http_request__string_ref(request: __sklib_ptr, path: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_options_request_for__http_request__string_ref"]
    fn __sklib__is_options_request_for__http_request__string_ref(request: __sklib_ptr, path: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_post_request_for__http_request__string_ref"]
    fn __sklib__is_post_request_for__http_request__string_ref(request: __sklib_ptr, path: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_put_request_for__http_request__string_ref"]
    fn __sklib__is_put_request_for__http_request__string_ref(request: __sklib_ptr, path: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_request_for__http_request__http_method__string_ref"]
    fn __sklib__is_request_for__http_request__http_method__string_ref(request: __sklib_ptr, method: i32, path: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_trace_request_for__http_request__string_ref"]
    fn __sklib__is_trace_request_for__http_request__string_ref(request: __sklib_ptr, path: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__next_web_request__web_server"]
    fn __sklib__next_web_request__web_server(server: __sklib_ptr) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__request_body__http_request"]
    fn __sklib__request_body__http_request(r: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__request_has_query_parameter__http_request__string_ref"]
    fn __sklib__request_has_query_parameter__http_request__string_ref(r: __sklib_ptr, name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__request_headers__http_request"]
    fn __sklib__request_headers__http_request(r: __sklib_ptr) 
        -> __sklib_vector_string;
}
extern "C" {
    #[link_name = "__sklib__request_method__http_request"]
    fn __sklib__request_method__http_request(r: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__request_query_parameter__http_request__string_ref__string_ref"]
    fn __sklib__request_query_parameter__http_request__string_ref__string_ref(r: __sklib_ptr, name: __sklib_string, default_value: __sklib_string) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__request_query_string__http_request"]
    fn __sklib__request_query_string__http_request(r: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__request_uri__http_request"]
    fn __sklib__request_uri__http_request(r: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__request_uri_stubs__http_request"]
    fn __sklib__request_uri_stubs__http_request(r: __sklib_ptr) 
        -> __sklib_vector_string;
}
extern "C" {
    #[link_name = "__sklib__send_css_file_response__http_request__string_ref"]
    fn __sklib__send_css_file_response__http_request__string_ref(r: __sklib_ptr, filename: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__send_file_response__http_request__string_ref__string_ref"]
    fn __sklib__send_file_response__http_request__string_ref__string_ref(r: __sklib_ptr, filename: __sklib_string, content_type: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__send_html_file_response__http_request__string_ref"]
    fn __sklib__send_html_file_response__http_request__string_ref(r: __sklib_ptr, filename: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__send_javascript_file_response__http_request__string_ref"]
    fn __sklib__send_javascript_file_response__http_request__string_ref(r: __sklib_ptr, filename: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__send_response__http_request"]
    fn __sklib__send_response__http_request(r: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__send_response__http_request__string_ref"]
    fn __sklib__send_response__http_request__string_ref(r: __sklib_ptr, message: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__send_response__http_request__http_status_code"]
    fn __sklib__send_response__http_request__http_status_code(r: __sklib_ptr, code: i32);
}
extern "C" {
    #[link_name = "__sklib__send_response__http_request__http_status_code__string_ref"]
    fn __sklib__send_response__http_request__http_status_code__string_ref(r: __sklib_ptr, code: i32, message: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__send_response__http_request__http_status_code__string_ref__string_ref"]
    fn __sklib__send_response__http_request__http_status_code__string_ref__string_ref(r: __sklib_ptr, code: i32, message: __sklib_string, content_type: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__send_response__http_request__http_status_code__string_ref__string_ref__vector_string_ref"]
    fn __sklib__send_response__http_request__http_status_code__string_ref__string_ref__vector_string_ref(r: __sklib_ptr, code: i32, message: __sklib_string, content_type: __sklib_string, headers: __sklib_vector_string);
}
extern "C" {
    #[link_name = "__sklib__send_response__http_request__json"]
    fn __sklib__send_response__http_request__json(r: __sklib_ptr, j: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__split_uri_stubs__string_ref"]
    fn __sklib__split_uri_stubs__string_ref(uri: __sklib_string) 
        -> __sklib_vector_string;
}
extern "C" {
    #[link_name = "__sklib__start_web_server"]
    fn __sklib__start_web_server() 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__start_web_server__unsigned_short"]
    fn __sklib__start_web_server__unsigned_short(port: u16) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__stop_web_server__web_server"]
    fn __sklib__stop_web_server__web_server(server: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__clear_window__window__color"]
    fn __sklib__clear_window__window__color(wind: __sklib_ptr, clr: __sklib_color);
}
extern "C" {
    #[link_name = "__sklib__close_all_windows"]
    fn __sklib__close_all_windows();
}
extern "C" {
    #[link_name = "__sklib__close_current_window"]
    fn __sklib__close_current_window();
}
extern "C" {
    #[link_name = "__sklib__close_window__string_ref"]
    fn __sklib__close_window__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__close_window__window"]
    fn __sklib__close_window__window(wind: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__current_window"]
    fn __sklib__current_window() 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__current_window_has_border"]
    fn __sklib__current_window_has_border() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__current_window_height"]
    fn __sklib__current_window_height() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__current_window_is_fullscreen"]
    fn __sklib__current_window_is_fullscreen() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__current_window_position"]
    fn __sklib__current_window_position() 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__current_window_toggle_border"]
    fn __sklib__current_window_toggle_border();
}
extern "C" {
    #[link_name = "__sklib__current_window_toggle_fullscreen"]
    fn __sklib__current_window_toggle_fullscreen();
}
extern "C" {
    #[link_name = "__sklib__current_window_width"]
    fn __sklib__current_window_width() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__current_window_x"]
    fn __sklib__current_window_x() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__current_window_y"]
    fn __sklib__current_window_y() 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__has_window__string"]
    fn __sklib__has_window__string(caption: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__is_current_window__window"]
    fn __sklib__is_current_window__window(wind: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__move_current_window_to__int__int"]
    fn __sklib__move_current_window_to__int__int(x: i32, y: i32);
}
extern "C" {
    #[link_name = "__sklib__move_window_to__string_ref__int__int"]
    fn __sklib__move_window_to__string_ref__int__int(name: __sklib_string, x: i32, y: i32);
}
extern "C" {
    #[link_name = "__sklib__move_window_to__window__int__int"]
    fn __sklib__move_window_to__window__int__int(wind: __sklib_ptr, x: i32, y: i32);
}
extern "C" {
    #[link_name = "__sklib__open_window__string__int__int"]
    fn __sklib__open_window__string__int__int(caption: __sklib_string, width: i32, height: i32) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__refresh_window__window"]
    fn __sklib__refresh_window__window(wind: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__refresh_window__window__unsigned_int"]
    fn __sklib__refresh_window__window__unsigned_int(wind: __sklib_ptr, target_fps: u32);
}
extern "C" {
    #[link_name = "__sklib__resize_current_window__int__int"]
    fn __sklib__resize_current_window__int__int(width: i32, height: i32);
}
extern "C" {
    #[link_name = "__sklib__resize_window__window__int__int"]
    fn __sklib__resize_window__window__int__int(wnd: __sklib_ptr, width: i32, height: i32);
}
extern "C" {
    #[link_name = "__sklib__set_current_window__string_ref"]
    fn __sklib__set_current_window__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__set_current_window__window"]
    fn __sklib__set_current_window__window(wind: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__window_caption__window"]
    fn __sklib__window_caption__window(wind: __sklib_ptr) 
        -> __sklib_string;
}
extern "C" {
    #[link_name = "__sklib__window_close_requested__string_ref"]
    fn __sklib__window_close_requested__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_close_requested__window"]
    fn __sklib__window_close_requested__window(wind: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_has_border__string_ref"]
    fn __sklib__window_has_border__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_has_border__window"]
    fn __sklib__window_has_border__window(wnd: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_has_focus__window"]
    fn __sklib__window_has_focus__window(wind: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_height__string_ref"]
    fn __sklib__window_height__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_height__window"]
    fn __sklib__window_height__window(wind: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_is_fullscreen__string_ref"]
    fn __sklib__window_is_fullscreen__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_is_fullscreen__window"]
    fn __sklib__window_is_fullscreen__window(wnd: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_named__string"]
    fn __sklib__window_named__string(caption: __sklib_string) 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__window_position__string_ref"]
    fn __sklib__window_position__string_ref(name: __sklib_string) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__window_position__window"]
    fn __sklib__window_position__window(wnd: __sklib_ptr) 
        -> __sklib_point_2d;
}
extern "C" {
    #[link_name = "__sklib__window_set_icon__window__bitmap"]
    fn __sklib__window_set_icon__window__bitmap(wind: __sklib_ptr, bmp: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__window_toggle_border__string_ref"]
    fn __sklib__window_toggle_border__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__window_toggle_border__window"]
    fn __sklib__window_toggle_border__window(wnd: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__window_toggle_fullscreen__string_ref"]
    fn __sklib__window_toggle_fullscreen__string_ref(name: __sklib_string);
}
extern "C" {
    #[link_name = "__sklib__window_toggle_fullscreen__window"]
    fn __sklib__window_toggle_fullscreen__window(wnd: __sklib_ptr);
}
extern "C" {
    #[link_name = "__sklib__window_width__string_ref"]
    fn __sklib__window_width__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_width__window"]
    fn __sklib__window_width__window(wind: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_with_focus"]
    fn __sklib__window_with_focus() 
        -> __sklib_ptr;
}
extern "C" {
    #[link_name = "__sklib__window_x__string_ref"]
    fn __sklib__window_x__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_x__window"]
    fn __sklib__window_x__window(wnd: __sklib_ptr) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_y__string_ref"]
    fn __sklib__window_y__string_ref(name: __sklib_string) 
        -> i32;
}
extern "C" {
    #[link_name = "__sklib__window_y__window"]
    fn __sklib__window_y__window(wnd: __sklib_ptr) 
        -> i32;
}
#[inline]
pub(crate) fn __skadapter__to_sklib_bool(v: bool) -> i32 {
    if v { 1 } else { 0 }
}
#[inline]
pub(crate) fn __skadapter__to_bool(v: i32) -> bool {
    v != 0
}
#[inline]
pub(crate) fn __skadapter__to_sklib_char(v: char) -> i8 {
  return v as i8;
}
#[inline]
pub(crate) fn __skadapter__to_char(v: i8) -> char 
{
  return v as u8 as char;
}
#[inline]
pub(crate) fn __skadapter__to_sklib_int8_t(v: i8) -> i8 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_int8_t(v: i8) -> i8 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_int(v: i32) -> i32 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_int(v: i32) -> i32 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_short(v: i16) -> i16 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_short(v: i16) -> i16 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_int64_t(v: i64) -> i64 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_int64_t(v: i64) -> i64 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_float(v: f32) -> f32 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_float(v: f32) -> f32 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_double(v: f64) -> f64 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_double(v: f64) -> f64 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_byte(v: u8) -> u8 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_byte(v: u8) -> u8 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_unsigned_char(v: u8) -> u8 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_unsigned_char(v: u8) -> u8 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_unsigned_int(v: u32) -> u32 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_unsigned_int(v: u32) -> u32 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_sklib_unsigned_short(v: u16) -> u16 {
    v
}
#[inline]
pub(crate) fn __skadapter__to_unsigned_short(v: u16) -> u16 {
    v
}
pub(crate) fn __skadapter__to_key_code(v: i32) -> KeyCode {
    match v {
        0 => KeyCode::UnknownKey,
        8 => KeyCode::BackspaceKey,
        9 => KeyCode::TabKey,
        12 => KeyCode::ClearKey,
        13 => KeyCode::ReturnKey,
        19 => KeyCode::PauseKey,
        27 => KeyCode::EscapeKey,
        32 => KeyCode::SpaceKey,
        33 => KeyCode::ExclaimKey,
        34 => KeyCode::DoubleQuoteKey,
        35 => KeyCode::HashKey,
        36 => KeyCode::DollarKey,
        38 => KeyCode::AmpersandKey,
        39 => KeyCode::QuoteKey,
        40 => KeyCode::LeftParenKey,
        41 => KeyCode::RightParenKey,
        42 => KeyCode::AsteriskKey,
        43 => KeyCode::PlusKey,
        44 => KeyCode::CommaKey,
        45 => KeyCode::MinusKey,
        46 => KeyCode::PeriodKey,
        47 => KeyCode::SlashKey,
        48 => KeyCode::Num0Key,
        49 => KeyCode::Num1Key,
        50 => KeyCode::Num2Key,
        51 => KeyCode::Num3Key,
        52 => KeyCode::Num4Key,
        53 => KeyCode::Num5Key,
        54 => KeyCode::Num6Key,
        55 => KeyCode::Num7Key,
        56 => KeyCode::Num8Key,
        57 => KeyCode::Num9Key,
        58 => KeyCode::ColonKey,
        59 => KeyCode::SemiColonKey,
        60 => KeyCode::LessKey,
        61 => KeyCode::EqualsKey,
        62 => KeyCode::GreaterKey,
        63 => KeyCode::QuestionKey,
        64 => KeyCode::AtKey,
        91 => KeyCode::LeftBracketKey,
        92 => KeyCode::BackslashKey,
        93 => KeyCode::RightBracketKey,
        94 => KeyCode::CaretKey,
        95 => KeyCode::UnderscoreKey,
        96 => KeyCode::BackquoteKey,
        97 => KeyCode::AKey,
        98 => KeyCode::BKey,
        99 => KeyCode::CKey,
        100 => KeyCode::DKey,
        101 => KeyCode::EKey,
        102 => KeyCode::FKey,
        103 => KeyCode::GKey,
        104 => KeyCode::HKey,
        105 => KeyCode::IKey,
        106 => KeyCode::JKey,
        107 => KeyCode::KKey,
        108 => KeyCode::LKey,
        109 => KeyCode::MKey,
        110 => KeyCode::NKey,
        111 => KeyCode::OKey,
        112 => KeyCode::PKey,
        113 => KeyCode::QKey,
        114 => KeyCode::RKey,
        115 => KeyCode::SKey,
        116 => KeyCode::TKey,
        117 => KeyCode::UKey,
        118 => KeyCode::VKey,
        119 => KeyCode::WKey,
        120 => KeyCode::XKey,
        121 => KeyCode::YKey,
        122 => KeyCode::ZKey,
        127 => KeyCode::DeleteKey,
        256 => KeyCode::Keypad0,
        257 => KeyCode::Keypad1,
        258 => KeyCode::Keypad2,
        259 => KeyCode::Keypad3,
        260 => KeyCode::Keypad4,
        261 => KeyCode::Keypad5,
        262 => KeyCode::Keypad6,
        263 => KeyCode::Keypad7,
        264 => KeyCode::Keypad8,
        265 => KeyCode::Keypad9,
        266 => KeyCode::KeypadPeriod,
        267 => KeyCode::KeypadDivide,
        268 => KeyCode::KeypadMultiply,
        269 => KeyCode::KeypadMinus,
        270 => KeyCode::KeypadPlus,
        271 => KeyCode::KeypadEnter,
        272 => KeyCode::KeypadEquals,
        273 => KeyCode::UpKey,
        274 => KeyCode::DownKey,
        275 => KeyCode::RightKey,
        276 => KeyCode::LeftKey,
        277 => KeyCode::InsertKey,
        278 => KeyCode::HomeKey,
        279 => KeyCode::EndKey,
        280 => KeyCode::PageUpKey,
        281 => KeyCode::PageDownKey,
        282 => KeyCode::F1Key,
        283 => KeyCode::F2Key,
        284 => KeyCode::F3Key,
        285 => KeyCode::F4Key,
        286 => KeyCode::F5Key,
        287 => KeyCode::F6Key,
        288 => KeyCode::F7Key,
        289 => KeyCode::F8Key,
        290 => KeyCode::F9Key,
        291 => KeyCode::F10Key,
        292 => KeyCode::F11Key,
        293 => KeyCode::F12Key,
        294 => KeyCode::F13Key,
        295 => KeyCode::F14Key,
        296 => KeyCode::F15Key,
        300 => KeyCode::NumLockKey,
        301 => KeyCode::CapsLockKey,
        302 => KeyCode::ScrollLockKey,
        303 => KeyCode::RightShiftKey,
        304 => KeyCode::LeftShiftKey,
        305 => KeyCode::RightCtrlKey,
        306 => KeyCode::LeftCtrlKey,
        307 => KeyCode::RightAltKey,
        308 => KeyCode::LeftAltKey,
        311 => KeyCode::LeftSuperKey,
        312 => KeyCode::RightSuperKey,
        313 => KeyCode::ModeKey,
        315 => KeyCode::HelpKey,
        317 => KeyCode::SysReqKey,
        319 => KeyCode::MenuKey,
        320 => KeyCode::PowerKey,
        _ => panic!("Invalid KeyCode value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_key_code(v: KeyCode) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_log_level(v: i32) -> LogLevel {
    match v {
        0 => LogLevel::None,
        1 => LogLevel::Info,
        2 => LogLevel::Debug,
        3 => LogLevel::Warning,
        4 => LogLevel::Error,
        5 => LogLevel::Fatal,
        _ => panic!("Invalid LogLevel value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_log_level(v: LogLevel) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_log_mode(v: i32) -> LogMode {
    match v {
        0 => LogMode::LogNone,
        1 => LogMode::LogConsole,
        2 => LogMode::LogFileOnly,
        3 => LogMode::LogConsoleAndFile,
        _ => panic!("Invalid LogMode value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_log_mode(v: LogMode) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_mouse_button(v: i32) -> MouseButton {
    match v {
        0 => MouseButton::NoButton,
        1 => MouseButton::LeftButton,
        2 => MouseButton::MiddleButton,
        3 => MouseButton::RightButton,
        4 => MouseButton::MouseX1Button,
        5 => MouseButton::MouseX2Button,
        _ => panic!("Invalid MouseButton value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_mouse_button(v: MouseButton) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_connection_type(v: i32) -> ConnectionType {
    match v {
        0 => ConnectionType::TCP,
        1 => ConnectionType::UDP,
        2 => ConnectionType::Unknown,
        _ => panic!("Invalid ConnectionType value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_connection_type(v: ConnectionType) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_resource_kind(v: i32) -> ResourceKind {
    match v {
        0 => ResourceKind::AnimationResource,
        1 => ResourceKind::BundleResource,
        2 => ResourceKind::FontResource,
        3 => ResourceKind::ImageResource,
        4 => ResourceKind::JsonResource,
        5 => ResourceKind::MusicResource,
        6 => ResourceKind::ServerResource,
        7 => ResourceKind::SoundResource,
        8 => ResourceKind::TimerResource,
        9 => ResourceKind::OtherResource,
        _ => panic!("Invalid ResourceKind value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_resource_kind(v: ResourceKind) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_collision_test_kind(v: i32) -> CollisionTestKind {
    match v {
        0 => CollisionTestKind::PixelCollisions,
        1 => CollisionTestKind::AabbCollisions,
        _ => panic!("Invalid CollisionTestKind value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_collision_test_kind(v: CollisionTestKind) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_sprite_event_kind(v: i32) -> SpriteEventKind {
    match v {
        0 => SpriteEventKind::SpriteArrivedEvent,
        1 => SpriteEventKind::SpriteAnimationEndedEvent,
        2 => SpriteEventKind::SpriteTouchedEvent,
        3 => SpriteEventKind::SpriteClickedEvent,
        _ => panic!("Invalid SpriteEventKind value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_sprite_event_kind(v: SpriteEventKind) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_drawing_dest(v: i32) -> DrawingDest {
    match v {
        0 => DrawingDest::DrawToScreen,
        1 => DrawingDest::DrawToWorld,
        2 => DrawingDest::DrawDefault,
        _ => panic!("Invalid DrawingDest value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_drawing_dest(v: DrawingDest) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_font_style(v: i32) -> FontStyle {
    match v {
        0 => FontStyle::NormalFont,
        1 => FontStyle::BoldFont,
        2 => FontStyle::ItalicFont,
        4 => FontStyle::UnderlineFont,
        _ => panic!("Invalid FontStyle value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_font_style(v: FontStyle) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_http_status_code(v: i32) -> HttpStatusCode {
    match v {
        200 => HttpStatusCode::HttpStatusOk,
        201 => HttpStatusCode::HttpStatusCreated,
        204 => HttpStatusCode::HttpStatusNoContent,
        301 => HttpStatusCode::HttpStatusMovedPermanently,
        302 => HttpStatusCode::HttpStatusFound,
        303 => HttpStatusCode::HttpStatusSeeOther,
        400 => HttpStatusCode::HttpStatusBadRequest,
        401 => HttpStatusCode::HttpStatusUnauthorized,
        403 => HttpStatusCode::HttpStatusForbidden,
        404 => HttpStatusCode::HttpStatusNotFound,
        405 => HttpStatusCode::HttpStatusMethodNotAllowed,
        408 => HttpStatusCode::HttpStatusRequestTimeout,
        409 => HttpStatusCode::HttpStatusConflict,
        500 => HttpStatusCode::HttpStatusInternalServerError,
        501 => HttpStatusCode::HttpStatusNotImplemented,
        503 => HttpStatusCode::HttpStatusServiceUnavailable,
        _ => panic!("Invalid HttpStatusCode value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_http_status_code(v: HttpStatusCode) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_interface_style(v: i32) -> InterfaceStyle {
    match v {
        0 => InterfaceStyle::FlatDarkStyle,
        1 => InterfaceStyle::ShadedDarkStyle,
        2 => InterfaceStyle::FlatLightStyle,
        3 => InterfaceStyle::ShadedLightStyle,
        4 => InterfaceStyle::Bubble,
        5 => InterfaceStyle::BubbleMulticolored,
        _ => panic!("Invalid InterfaceStyle value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_interface_style(v: InterfaceStyle) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_pin_modes(v: i32) -> PinModes {
    match v {
        0 => PinModes::GpioInput,
        1 => PinModes::GpioOutput,
        4 => PinModes::GpioAlt0,
        5 => PinModes::GpioAlt1,
        6 => PinModes::GpioAlt2,
        7 => PinModes::GpioAlt3,
        3 => PinModes::GpioAlt4,
        2 => PinModes::GpioAlt5,
        _ => panic!("Invalid PinModes value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_pin_modes(v: PinModes) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_pin_values(v: i32) -> PinValues {
    match v {
        0 => PinValues::GpioLow,
        1 => PinValues::GpioHigh,
        _ => panic!("Invalid PinValues value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_pin_values(v: PinValues) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_pins(v: i32) -> Pins {
    match v {
        1 => Pins::Pin1,
        2 => Pins::Pin2,
        3 => Pins::Pin3,
        4 => Pins::Pin4,
        5 => Pins::Pin5,
        6 => Pins::Pin6,
        7 => Pins::Pin7,
        8 => Pins::Pin8,
        9 => Pins::Pin9,
        10 => Pins::Pin10,
        11 => Pins::Pin11,
        12 => Pins::Pin12,
        13 => Pins::Pin13,
        14 => Pins::Pin14,
        15 => Pins::Pin15,
        16 => Pins::Pin16,
        17 => Pins::Pin17,
        18 => Pins::Pin18,
        19 => Pins::Pin19,
        20 => Pins::Pin20,
        21 => Pins::Pin21,
        22 => Pins::Pin22,
        23 => Pins::Pin23,
        24 => Pins::Pin24,
        25 => Pins::Pin25,
        26 => Pins::Pin26,
        27 => Pins::Pin27,
        28 => Pins::Pin28,
        29 => Pins::Pin29,
        30 => Pins::Pin30,
        31 => Pins::Pin31,
        32 => Pins::Pin32,
        33 => Pins::Pin33,
        34 => Pins::Pin34,
        35 => Pins::Pin35,
        36 => Pins::Pin36,
        37 => Pins::Pin37,
        38 => Pins::Pin38,
        39 => Pins::Pin39,
        40 => Pins::Pin40,
        _ => panic!("Invalid Pins value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_pins(v: Pins) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_pull_up_down(v: i32) -> PullUpDown {
    match v {
        0 => PullUpDown::PudOff,
        1 => PullUpDown::PudDown,
        2 => PullUpDown::PudUp,
        _ => panic!("Invalid PullUpDown value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_pull_up_down(v: PullUpDown) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_http_method(v: i32) -> HttpMethod {
    match v {
        0 => HttpMethod::HttpGetMethod,
        1 => HttpMethod::HttpPostMethod,
        2 => HttpMethod::HttpPutMethod,
        3 => HttpMethod::HttpDeleteMethod,
        4 => HttpMethod::HttpOptionsMethod,
        5 => HttpMethod::HttpTraceMethod,
        6 => HttpMethod::UnknownHttpMethod,
        _ => panic!("Invalid HttpMethod value: {}", v)
    }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_http_method(v: HttpMethod) -> i32 {
    v as i32
}

pub(crate) fn __skadapter__to_sklib_matrix_2d(v: Matrix2D) -> __sklib_matrix_2d {
    let mut result = __sklib_matrix_2d::new();
    result.elements[0] = __skadapter__to_sklib_double(v.elements[0][0]);
    result.elements[1] = __skadapter__to_sklib_double(v.elements[0][1]);
    result.elements[2] = __skadapter__to_sklib_double(v.elements[0][2]);
    result.elements[3] = __skadapter__to_sklib_double(v.elements[1][0]);
    result.elements[4] = __skadapter__to_sklib_double(v.elements[1][1]);
    result.elements[5] = __skadapter__to_sklib_double(v.elements[1][2]);
    result.elements[6] = __skadapter__to_sklib_double(v.elements[2][0]);
    result.elements[7] = __skadapter__to_sklib_double(v.elements[2][1]);
    result.elements[8] = __skadapter__to_sklib_double(v.elements[2][2]);
    result
}
pub(crate) fn __skadapter__to_matrix_2d(v: __sklib_matrix_2d) -> Matrix2D {
    let mut result = Matrix2D::new();
    result.elements[0][0] = __skadapter__to_double(v.elements[0]);
    result.elements[0][1] = __skadapter__to_double(v.elements[1]);
    result.elements[0][2] = __skadapter__to_double(v.elements[2]);
    result.elements[1][0] = __skadapter__to_double(v.elements[3]);
    result.elements[1][1] = __skadapter__to_double(v.elements[4]);
    result.elements[1][2] = __skadapter__to_double(v.elements[5]);
    result.elements[2][0] = __skadapter__to_double(v.elements[6]);
    result.elements[2][1] = __skadapter__to_double(v.elements[7]);
    result.elements[2][2] = __skadapter__to_double(v.elements[8]);
    result
}
pub(crate) fn __skadapter__to_sklib_point_2d(v: Point2D) -> __sklib_point_2d {
    let mut result = __sklib_point_2d::new();
    result.x = __skadapter__to_sklib_double(v.x);
    result.y = __skadapter__to_sklib_double(v.y);
    result
}
pub(crate) fn __skadapter__to_point_2d(v: __sklib_point_2d) -> Point2D {
    let mut result = Point2D::new();
    result.x = __skadapter__to_double(v.x);
    result.y = __skadapter__to_double(v.y);
    result
}
pub(crate) fn __skadapter__to_sklib_circle(v: Circle) -> __sklib_circle {
    let mut result = __sklib_circle::new();
    result.center = __skadapter__to_sklib_point_2d(v.center);
    result.radius = __skadapter__to_sklib_double(v.radius);
    result
}
pub(crate) fn __skadapter__to_circle(v: __sklib_circle) -> Circle {
    let mut result = Circle::new();
    result.center = __skadapter__to_point_2d(v.center);
    result.radius = __skadapter__to_double(v.radius);
    result
}
pub(crate) fn __skadapter__to_sklib_color(v: Color) -> __sklib_color {
    let mut result = __sklib_color::new();
    result.r = __skadapter__to_sklib_float(v.r);
    result.g = __skadapter__to_sklib_float(v.g);
    result.b = __skadapter__to_sklib_float(v.b);
    result.a = __skadapter__to_sklib_float(v.a);
    result
}
pub(crate) fn __skadapter__to_color(v: __sklib_color) -> Color {
    let mut result = Color::new();
    result.r = __skadapter__to_float(v.r);
    result.g = __skadapter__to_float(v.g);
    result.b = __skadapter__to_float(v.b);
    result.a = __skadapter__to_float(v.a);
    result
}
pub(crate) fn __skadapter__to_sklib_rectangle(v: Rectangle) -> __sklib_rectangle {
    let mut result = __sklib_rectangle::new();
    result.x = __skadapter__to_sklib_double(v.x);
    result.y = __skadapter__to_sklib_double(v.y);
    result.width = __skadapter__to_sklib_double(v.width);
    result.height = __skadapter__to_sklib_double(v.height);
    result
}
pub(crate) fn __skadapter__to_rectangle(v: __sklib_rectangle) -> Rectangle {
    let mut result = Rectangle::new();
    result.x = __skadapter__to_double(v.x);
    result.y = __skadapter__to_double(v.y);
    result.width = __skadapter__to_double(v.width);
    result.height = __skadapter__to_double(v.height);
    result
}
pub(crate) fn __skadapter__to_sklib_drawing_options(v: DrawingOptions) -> __sklib_drawing_options {
    let mut result = __sklib_drawing_options::new();
    result.dest = __skadapter__to_sklib_ptr(v.dest);
    result.scale_x = __skadapter__to_sklib_float(v.scale_x);
    result.scale_y = __skadapter__to_sklib_float(v.scale_y);
    result.angle = __skadapter__to_sklib_float(v.angle);
    result.anchor_offset_x = __skadapter__to_sklib_float(v.anchor_offset_x);
    result.anchor_offset_y = __skadapter__to_sklib_float(v.anchor_offset_y);
    result.flip_x = __skadapter__to_sklib_bool(v.flip_x);
    result.flip_y = __skadapter__to_sklib_bool(v.flip_y);
    result.is_part = __skadapter__to_sklib_bool(v.is_part);
    result.part = __skadapter__to_sklib_rectangle(v.part);
    result.draw_cell = __skadapter__to_sklib_int(v.draw_cell);
    result.camera = __skadapter__to_sklib_drawing_dest(v.camera);
    result.line_width = __skadapter__to_sklib_int(v.line_width);
    result.anim = __skadapter__to_sklib_animation(v.anim);
    result
}
pub(crate) fn __skadapter__to_drawing_options(v: __sklib_drawing_options) -> DrawingOptions {
    let mut result = DrawingOptions::new();
    result.dest = __skadapter__to_ptr(v.dest);
    result.scale_x = __skadapter__to_float(v.scale_x);
    result.scale_y = __skadapter__to_float(v.scale_y);
    result.angle = __skadapter__to_float(v.angle);
    result.anchor_offset_x = __skadapter__to_float(v.anchor_offset_x);
    result.anchor_offset_y = __skadapter__to_float(v.anchor_offset_y);
    result.flip_x = __skadapter__to_bool(v.flip_x);
    result.flip_y = __skadapter__to_bool(v.flip_y);
    result.is_part = __skadapter__to_bool(v.is_part);
    result.part = __skadapter__to_rectangle(v.part);
    result.draw_cell = __skadapter__to_int(v.draw_cell);
    result.camera = __skadapter__to_drawing_dest(v.camera);
    result.line_width = __skadapter__to_int(v.line_width);
    result.anim = __skadapter__to_animation(v.anim);
    result
}
pub(crate) fn __skadapter__to_sklib_line(v: Line) -> __sklib_line {
    let mut result = __sklib_line::new();
    result.start_point = __skadapter__to_sklib_point_2d(v.start_point);
    result.end_point = __skadapter__to_sklib_point_2d(v.end_point);
    result
}
pub(crate) fn __skadapter__to_line(v: __sklib_line) -> Line {
    let mut result = Line::new();
    result.start_point = __skadapter__to_point_2d(v.start_point);
    result.end_point = __skadapter__to_point_2d(v.end_point);
    result
}
pub(crate) fn __skadapter__to_sklib_quad(v: Quad) -> __sklib_quad {
    let mut result = __sklib_quad::new();
    result.points[0] = __skadapter__to_sklib_point_2d(v.points[0]);
    result.points[1] = __skadapter__to_sklib_point_2d(v.points[1]);
    result.points[2] = __skadapter__to_sklib_point_2d(v.points[2]);
    result.points[3] = __skadapter__to_sklib_point_2d(v.points[3]);
    result
}
pub(crate) fn __skadapter__to_quad(v: __sklib_quad) -> Quad {
    let mut result = Quad::new();
    result.points[0] = __skadapter__to_point_2d(v.points[0]);
    result.points[1] = __skadapter__to_point_2d(v.points[1]);
    result.points[2] = __skadapter__to_point_2d(v.points[2]);
    result.points[3] = __skadapter__to_point_2d(v.points[3]);
    result
}
pub(crate) fn __skadapter__to_sklib_triangle(v: Triangle) -> __sklib_triangle {
    let mut result = __sklib_triangle::new();
    result.points[0] = __skadapter__to_sklib_point_2d(v.points[0]);
    result.points[1] = __skadapter__to_sklib_point_2d(v.points[1]);
    result.points[2] = __skadapter__to_sklib_point_2d(v.points[2]);
    result
}
pub(crate) fn __skadapter__to_triangle(v: __sklib_triangle) -> Triangle {
    let mut result = Triangle::new();
    result.points[0] = __skadapter__to_point_2d(v.points[0]);
    result.points[1] = __skadapter__to_point_2d(v.points[1]);
    result.points[2] = __skadapter__to_point_2d(v.points[2]);
    result
}
pub(crate) fn __skadapter__to_sklib_vector_2d(v: Vector2D) -> __sklib_vector_2d {
    let mut result = __sklib_vector_2d::new();
    result.x = __skadapter__to_sklib_double(v.x);
    result.y = __skadapter__to_sklib_double(v.y);
    result
}
pub(crate) fn __skadapter__to_vector_2d(v: __sklib_vector_2d) -> Vector2D {
    let mut result = Vector2D::new();
    result.x = __skadapter__to_double(v.x);
    result.y = __skadapter__to_double(v.y);
    result
}
#[repr(C)]
pub(crate) struct __sklib_vector_string {
    data_from_app: *mut __sklib_string,
    size_from_app: u32,
    data_from_lib: *mut __sklib_string,
    size_from_lib: u32,
}

impl __sklib_vector_string {
    pub fn new(size: u32) -> Self {
        let mut vec = Vec::with_capacity(size as usize);
        vec.resize(size as usize, unsafe { zeroed() });
        let ptr = vec.as_mut_ptr();
        forget(vec);
        Self {
            data_from_app: ptr,
            size_from_app: size,
            data_from_lib: null_mut(),
            size_from_lib: 0,
        }
    }
}

extern "C" {
    fn __sklib__free__sklib_vector_string(v: __sklib_vector_string);
}

pub(crate) fn __skadapter__free__sklib_vector_string(v: &mut __sklib_vector_string) {
    if !v.data_from_app.is_null() {
        unsafe {
            for i in 0..v.size_from_app {
                __skadapter__free__sklib_string(*v.data_from_app.add(i as usize));
            }
            Vec::from_raw_parts(v.data_from_app, v.size_from_app as usize, v.size_from_app as usize);
        }
        v.data_from_app = null_mut();
    }
}

pub(crate) fn __skadapter__to_sklib_vector_string(v: Vec<String>) -> __sklib_vector_string {
    let result = __sklib_vector_string::new(v.len() as u32);
    for (i, item) in v.iter().enumerate() {
        unsafe {
            *result.data_from_app.add(i) = __skadapter__to_sklib_string(item.clone());
        }
    }
    result
}

pub(crate) fn __skadapter__to_vector_string(v: __sklib_vector_string) -> Vec<String> {
    let mut result = Vec::with_capacity(v.size_from_lib as usize);
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_string(item));
        }
        __sklib__free__sklib_vector_string(v);
    }
    result
}

pub(crate) fn __skadapter__update_from_vector_string(v: __sklib_vector_string, result: &mut Vec<String>) {
    result.clear();
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_string(item));
        }
        __sklib__free__sklib_vector_string(v);
    }
}
#[repr(C)]
pub(crate) struct __sklib_vector_line {
    data_from_app: *mut __sklib_line,
    size_from_app: u32,
    data_from_lib: *mut __sklib_line,
    size_from_lib: u32,
}

impl __sklib_vector_line {
    pub fn new(size: u32) -> Self {
        let mut vec = Vec::with_capacity(size as usize);
        vec.resize(size as usize, unsafe { zeroed() });
        let ptr = vec.as_mut_ptr();
        forget(vec);
        Self {
            data_from_app: ptr,
            size_from_app: size,
            data_from_lib: null_mut(),
            size_from_lib: 0,
        }
    }
}

extern "C" {
    fn __sklib__free__sklib_vector_line(v: __sklib_vector_line);
}

pub(crate) fn __skadapter__free__sklib_vector_line(v: &mut __sklib_vector_line) {
    if !v.data_from_app.is_null() {
        unsafe {
            Vec::from_raw_parts(v.data_from_app, v.size_from_app as usize, v.size_from_app as usize);
        }
        v.data_from_app = null_mut();
    }
}

pub(crate) fn __skadapter__to_sklib_vector_line(v: Vec<Line>) -> __sklib_vector_line {
    let result = __sklib_vector_line::new(v.len() as u32);
    for (i, item) in v.iter().enumerate() {
        unsafe {
            *result.data_from_app.add(i) = __skadapter__to_sklib_line(item.clone());
        }
    }
    result
}

pub(crate) fn __skadapter__to_vector_line(v: __sklib_vector_line) -> Vec<Line> {
    let mut result = Vec::with_capacity(v.size_from_lib as usize);
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_line(item));
        }
        __sklib__free__sklib_vector_line(v);
    }
    result
}

pub(crate) fn __skadapter__update_from_vector_line(v: __sklib_vector_line, result: &mut Vec<Line>) {
    result.clear();
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_line(item));
        }
        __sklib__free__sklib_vector_line(v);
    }
}
#[repr(C)]
pub(crate) struct __sklib_vector_int8_t {
    data_from_app: *mut i8,
    size_from_app: u32,
    data_from_lib: *mut i8,
    size_from_lib: u32,
}

impl __sklib_vector_int8_t {
    pub fn new(size: u32) -> Self {
        let mut vec = Vec::with_capacity(size as usize);
        vec.resize(size as usize, unsafe { zeroed() });
        let ptr = vec.as_mut_ptr();
        forget(vec);
        Self {
            data_from_app: ptr,
            size_from_app: size,
            data_from_lib: null_mut(),
            size_from_lib: 0,
        }
    }
}

extern "C" {
    fn __sklib__free__sklib_vector_int8_t(v: __sklib_vector_int8_t);
}

pub(crate) fn __skadapter__free__sklib_vector_int8_t(v: &mut __sklib_vector_int8_t) {
    if !v.data_from_app.is_null() {
        unsafe {
            Vec::from_raw_parts(v.data_from_app, v.size_from_app as usize, v.size_from_app as usize);
        }
        v.data_from_app = null_mut();
    }
}

pub(crate) fn __skadapter__to_sklib_vector_int8_t(v: Vec<i8>) -> __sklib_vector_int8_t {
    let result = __sklib_vector_int8_t::new(v.len() as u32);
    for (i, item) in v.iter().enumerate() {
        unsafe {
            *result.data_from_app.add(i) = __skadapter__to_sklib_int8_t(item.clone());
        }
    }
    result
}

pub(crate) fn __skadapter__to_vector_int8_t(v: __sklib_vector_int8_t) -> Vec<i8> {
    let mut result = Vec::with_capacity(v.size_from_lib as usize);
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_int8_t(item));
        }
        __sklib__free__sklib_vector_int8_t(v);
    }
    result
}

pub(crate) fn __skadapter__update_from_vector_int8_t(v: __sklib_vector_int8_t, result: &mut Vec<i8>) {
    result.clear();
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_int8_t(item));
        }
        __sklib__free__sklib_vector_int8_t(v);
    }
}
#[repr(C)]
pub(crate) struct __sklib_vector_triangle {
    data_from_app: *mut __sklib_triangle,
    size_from_app: u32,
    data_from_lib: *mut __sklib_triangle,
    size_from_lib: u32,
}

impl __sklib_vector_triangle {
    pub fn new(size: u32) -> Self {
        let mut vec = Vec::with_capacity(size as usize);
        vec.resize(size as usize, unsafe { zeroed() });
        let ptr = vec.as_mut_ptr();
        forget(vec);
        Self {
            data_from_app: ptr,
            size_from_app: size,
            data_from_lib: null_mut(),
            size_from_lib: 0,
        }
    }
}

extern "C" {
    fn __sklib__free__sklib_vector_triangle(v: __sklib_vector_triangle);
}

pub(crate) fn __skadapter__free__sklib_vector_triangle(v: &mut __sklib_vector_triangle) {
    if !v.data_from_app.is_null() {
        unsafe {
            Vec::from_raw_parts(v.data_from_app, v.size_from_app as usize, v.size_from_app as usize);
        }
        v.data_from_app = null_mut();
    }
}

pub(crate) fn __skadapter__to_sklib_vector_triangle(v: Vec<Triangle>) -> __sklib_vector_triangle {
    let result = __sklib_vector_triangle::new(v.len() as u32);
    for (i, item) in v.iter().enumerate() {
        unsafe {
            *result.data_from_app.add(i) = __skadapter__to_sklib_triangle(item.clone());
        }
    }
    result
}

pub(crate) fn __skadapter__to_vector_triangle(v: __sklib_vector_triangle) -> Vec<Triangle> {
    let mut result = Vec::with_capacity(v.size_from_lib as usize);
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_triangle(item));
        }
        __sklib__free__sklib_vector_triangle(v);
    }
    result
}

pub(crate) fn __skadapter__update_from_vector_triangle(v: __sklib_vector_triangle, result: &mut Vec<Triangle>) {
    result.clear();
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_triangle(item));
        }
        __sklib__free__sklib_vector_triangle(v);
    }
}
#[repr(C)]
pub(crate) struct __sklib_vector_double {
    data_from_app: *mut f64,
    size_from_app: u32,
    data_from_lib: *mut f64,
    size_from_lib: u32,
}

impl __sklib_vector_double {
    pub fn new(size: u32) -> Self {
        let mut vec = Vec::with_capacity(size as usize);
        vec.resize(size as usize, unsafe { zeroed() });
        let ptr = vec.as_mut_ptr();
        forget(vec);
        Self {
            data_from_app: ptr,
            size_from_app: size,
            data_from_lib: null_mut(),
            size_from_lib: 0,
        }
    }
}

extern "C" {
    fn __sklib__free__sklib_vector_double(v: __sklib_vector_double);
}

pub(crate) fn __skadapter__free__sklib_vector_double(v: &mut __sklib_vector_double) {
    if !v.data_from_app.is_null() {
        unsafe {
            Vec::from_raw_parts(v.data_from_app, v.size_from_app as usize, v.size_from_app as usize);
        }
        v.data_from_app = null_mut();
    }
}

pub(crate) fn __skadapter__to_sklib_vector_double(v: Vec<f64>) -> __sklib_vector_double {
    let result = __sklib_vector_double::new(v.len() as u32);
    for (i, item) in v.iter().enumerate() {
        unsafe {
            *result.data_from_app.add(i) = __skadapter__to_sklib_double(item.clone());
        }
    }
    result
}

pub(crate) fn __skadapter__to_vector_double(v: __sklib_vector_double) -> Vec<f64> {
    let mut result = Vec::with_capacity(v.size_from_lib as usize);
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_double(item));
        }
        __sklib__free__sklib_vector_double(v);
    }
    result
}

pub(crate) fn __skadapter__update_from_vector_double(v: __sklib_vector_double, result: &mut Vec<f64>) {
    result.clear();
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_double(item));
        }
        __sklib__free__sklib_vector_double(v);
    }
}
#[repr(C)]
pub(crate) struct __sklib_vector_json {
    data_from_app: *mut __sklib_ptr,
    size_from_app: u32,
    data_from_lib: *mut __sklib_ptr,
    size_from_lib: u32,
}

impl __sklib_vector_json {
    pub fn new(size: u32) -> Self {
        let mut vec = Vec::with_capacity(size as usize);
        vec.resize(size as usize, unsafe { zeroed() });
        let ptr = vec.as_mut_ptr();
        forget(vec);
        Self {
            data_from_app: ptr,
            size_from_app: size,
            data_from_lib: null_mut(),
            size_from_lib: 0,
        }
    }
}

extern "C" {
    fn __sklib__free__sklib_vector_json(v: __sklib_vector_json);
}

pub(crate) fn __skadapter__free__sklib_vector_json(v: &mut __sklib_vector_json) {
    if !v.data_from_app.is_null() {
        unsafe {
            Vec::from_raw_parts(v.data_from_app, v.size_from_app as usize, v.size_from_app as usize);
        }
        v.data_from_app = null_mut();
    }
}

pub(crate) fn __skadapter__to_sklib_vector_json(v: Vec<Json>) -> __sklib_vector_json {
    let result = __sklib_vector_json::new(v.len() as u32);
    for (i, item) in v.iter().enumerate() {
        unsafe {
            *result.data_from_app.add(i) = __skadapter__to_sklib_json(item.clone());
        }
    }
    result
}

pub(crate) fn __skadapter__to_vector_json(v: __sklib_vector_json) -> Vec<Json> {
    let mut result = Vec::with_capacity(v.size_from_lib as usize);
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_json(item));
        }
        __sklib__free__sklib_vector_json(v);
    }
    result
}

pub(crate) fn __skadapter__update_from_vector_json(v: __sklib_vector_json, result: &mut Vec<Json>) {
    result.clear();
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_json(item));
        }
        __sklib__free__sklib_vector_json(v);
    }
}
#[repr(C)]
pub(crate) struct __sklib_vector_bool {
    data_from_app: *mut i32,
    size_from_app: u32,
    data_from_lib: *mut i32,
    size_from_lib: u32,
}

impl __sklib_vector_bool {
    pub fn new(size: u32) -> Self {
        let mut vec = Vec::with_capacity(size as usize);
        vec.resize(size as usize, unsafe { zeroed() });
        let ptr = vec.as_mut_ptr();
        forget(vec);
        Self {
            data_from_app: ptr,
            size_from_app: size,
            data_from_lib: null_mut(),
            size_from_lib: 0,
        }
    }
}

extern "C" {
    fn __sklib__free__sklib_vector_bool(v: __sklib_vector_bool);
}

pub(crate) fn __skadapter__free__sklib_vector_bool(v: &mut __sklib_vector_bool) {
    if !v.data_from_app.is_null() {
        unsafe {
            Vec::from_raw_parts(v.data_from_app, v.size_from_app as usize, v.size_from_app as usize);
        }
        v.data_from_app = null_mut();
    }
}

pub(crate) fn __skadapter__to_sklib_vector_bool(v: Vec<bool>) -> __sklib_vector_bool {
    let result = __sklib_vector_bool::new(v.len() as u32);
    for (i, item) in v.iter().enumerate() {
        unsafe {
            *result.data_from_app.add(i) = __skadapter__to_sklib_bool(item.clone());
        }
    }
    result
}

pub(crate) fn __skadapter__to_vector_bool(v: __sklib_vector_bool) -> Vec<bool> {
    let mut result = Vec::with_capacity(v.size_from_lib as usize);
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_bool(item));
        }
        __sklib__free__sklib_vector_bool(v);
    }
    result
}

pub(crate) fn __skadapter__update_from_vector_bool(v: __sklib_vector_bool, result: &mut Vec<bool>) {
    result.clear();
    unsafe {
        for i in 0..v.size_from_lib {
            let item = (*v.data_from_lib.add(i as usize)).clone();
            result.push(__skadapter__to_bool(item));
        }
        __sklib__free__sklib_vector_bool(v);
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sklib_string {
    pub str: *mut c_char,
    pub size: c_int,
    pub ptr: __sklib_ptr,
}
impl __sklib_string {
    pub fn new() -> Self {
        Self {
            str: null_mut(),
            size: 0,
            ptr: null_mut(),
        }
    }
}
pub(crate) fn __skadapter__to_sklib_string<S: AsRef<str>>(s: S) -> __sklib_string {
    let s = s.as_ref();
    let c_string = CString::new(s).expect("Failed to create CString");
    let ptr = c_string.into_raw();
    let size = s.len() as c_int;
    
    __sklib_string {
        str: ptr,
        size,
        ptr: null_mut(),
    }
}

pub(crate) fn __skadapter__to_string(v: __sklib_string) -> String {
    unsafe {
        let result = if v.str.is_null() {
            String::new()
        } else {
            CStr::from_ptr(v.str)
                .to_string_lossy()
                .into_owned()
        };
        
        if !v.str.is_null() {
            let _ = CString::from_raw(v.str);
        }
        
        result
    }
}

pub(crate) fn __skadapter__free__sklib_string(_s: __sklib_string) {
    // The actual freeing is handled by the C library
}
#[inline]
pub(crate) fn __skadapter__to_sklib_key_callback(callback: KeyCallback) -> __sklib_key_callback {
    thread_local! {
        static CALLBACK: RefCell<Option<KeyCallback>> = RefCell::new(None);
    }
    
    CALLBACK.with(|cell| {
        *cell.borrow_mut() = Some(callback);
    });

    extern "C" fn wrapper(
        code: i32,
    ) -> () {
        CALLBACK.with(|cell| {
            if let Some(cb) = &*cell.borrow() {
                cb(
                    __skadapter__to_int(code)                )
            }
        })
    }
    wrapper
}
#[inline]
pub(crate) fn __skadapter__to_sklib_free_notifier(callback: FreeNotifier) -> __sklib_free_notifier {
    thread_local! {
        static CALLBACK: RefCell<Option<FreeNotifier>> = RefCell::new(None);
    }
    
    CALLBACK.with(|cell| {
        *cell.borrow_mut() = Some(callback);
    });

    extern "C" fn wrapper(
        pointer: __sklib_ptr,
    ) -> () {
        CALLBACK.with(|cell| {
            if let Some(cb) = &*cell.borrow() {
                cb(
                    pointer                )
            }
        })
    }
    wrapper
}
#[inline]
pub(crate) fn __skadapter__to_sklib_sprite_event_handler(callback: SpriteEventHandler) -> __sklib_sprite_event_handler {
    thread_local! {
        static CALLBACK: RefCell<Option<SpriteEventHandler>> = RefCell::new(None);
    }
    
    CALLBACK.with(|cell| {
        *cell.borrow_mut() = Some(callback);
    });

    extern "C" fn wrapper(
        s: __sklib_ptr,
        evt: i32,
    ) -> () {
        CALLBACK.with(|cell| {
            if let Some(cb) = &*cell.borrow() {
                cb(
                    s,                    __skadapter__to_int(evt)                )
            }
        })
    }
    wrapper
}
#[inline]
pub(crate) fn __skadapter__to_sklib_sprite_float_function(callback: SpriteFloatFunction) -> __sklib_sprite_float_function {
    thread_local! {
        static CALLBACK: RefCell<Option<SpriteFloatFunction>> = RefCell::new(None);
    }
    
    CALLBACK.with(|cell| {
        *cell.borrow_mut() = Some(callback);
    });

    extern "C" fn wrapper(
        s: __sklib_ptr,
        f: f32,
    ) -> () {
        CALLBACK.with(|cell| {
            if let Some(cb) = &*cell.borrow() {
                cb(
                    s,                    __skadapter__to_float(f)                )
            }
        })
    }
    wrapper
}
#[inline]
pub(crate) fn __skadapter__to_sklib_sprite_function(callback: SpriteFunction) -> __sklib_sprite_function {
    thread_local! {
        static CALLBACK: RefCell<Option<SpriteFunction>> = RefCell::new(None);
    }
    
    CALLBACK.with(|cell| {
        *cell.borrow_mut() = Some(callback);
    });

    extern "C" fn wrapper(
        s: __sklib_ptr,
    ) -> () {
        CALLBACK.with(|cell| {
            if let Some(cb) = &*cell.borrow() {
                cb(
                    s                )
            }
        })
    }
    wrapper
}
#[inline]
pub(crate) fn __skadapter__to_sklib_ptr(v: __sklib_ptr) -> __sklib_ptr {
    v
}

#[inline]
pub(crate) fn __skadapter__to_ptr(v: __sklib_ptr) -> __sklib_ptr {
    v
}
#[inline]
pub(crate) fn __skadapter__to_json(v: __sklib_ptr) -> Json {
    Json { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_json(v: Json) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_music(v: __sklib_ptr) -> Music {
    Music { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_music(v: Music) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_connection(v: __sklib_ptr) -> Connection {
    Connection { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_connection(v: Connection) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_message(v: __sklib_ptr) -> Message {
    Message { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_message(v: Message) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_server_socket(v: __sklib_ptr) -> ServerSocket {
    ServerSocket { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_server_socket(v: ServerSocket) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_sound_effect(v: __sklib_ptr) -> SoundEffect {
    SoundEffect { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_sound_effect(v: SoundEffect) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_sprite(v: __sklib_ptr) -> Sprite {
    Sprite { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_sprite(v: Sprite) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_timer(v: __sklib_ptr) -> Timer {
    Timer { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_timer(v: Timer) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_animation(v: __sklib_ptr) -> Animation {
    Animation { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_animation(v: Animation) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_animation_script(v: __sklib_ptr) -> AnimationScript {
    AnimationScript { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_animation_script(v: AnimationScript) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_bitmap(v: __sklib_ptr) -> Bitmap {
    Bitmap { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_bitmap(v: Bitmap) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_display(v: __sklib_ptr) -> Display {
    Display { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_display(v: Display) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_font(v: __sklib_ptr) -> Font {
    Font { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_font(v: Font) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_http_response(v: __sklib_ptr) -> HttpResponse {
    HttpResponse { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_http_response(v: HttpResponse) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_http_request(v: __sklib_ptr) -> HttpRequest {
    HttpRequest { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_http_request(v: HttpRequest) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_web_server(v: __sklib_ptr) -> WebServer {
    WebServer { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_web_server(v: WebServer) -> __sklib_ptr {
    v.ptr
}
#[inline]
pub(crate) fn __skadapter__to_window(v: __sklib_ptr) -> Window {
    Window { ptr: v }
}
#[inline]
pub(crate) fn __skadapter__to_sklib_window(v: Window) -> __sklib_ptr {
    v.ptr
}
pub fn animation_count(script: AnimationScript) -> i32 {
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    unsafe {
        let __skreturn = __sklib__animation_count__animation_script(__skparam__script);
        __skadapter__to_int(__skreturn)
    }
}
pub fn animation_current_cell(anim: Animation) -> i32 {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    unsafe {
        let __skreturn = __sklib__animation_current_cell__animation(__skparam__anim);
        __skadapter__to_int(__skreturn)
    }
}
pub fn animation_current_vector(anim: Animation) -> Vector2D {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    unsafe {
        let __skreturn = __sklib__animation_current_vector__animation(__skparam__anim);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn animation_ended(anim: Animation) -> bool {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    unsafe {
        let __skreturn = __sklib__animation_ended__animation(__skparam__anim);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn animation_entered_frame(anim: Animation) -> bool {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    unsafe {
        let __skreturn = __sklib__animation_entered_frame__animation(__skparam__anim);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn animation_frame_time(anim: Animation) -> f32 {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    unsafe {
        let __skreturn = __sklib__animation_frame_time__animation(__skparam__anim);
        __skadapter__to_float(__skreturn)
    }
}
pub fn animation_index(script: AnimationScript, name: String) -> i32 {
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__animation_index__animation_script__string_ref(__skparam__script, __skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn animation_name(temp: Animation) -> String {
    let __skparam__temp = __skadapter__to_sklib_animation(temp);
    unsafe {
        let __skreturn = __sklib__animation_name__animation(__skparam__temp);
        __skadapter__to_string(__skreturn)
    }
}
pub fn animation_script_name(script: AnimationScript) -> String {
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    unsafe {
        let __skreturn = __sklib__animation_script_name__animation_script(__skparam__script);
        __skadapter__to_string(__skreturn)
    }
}
pub fn animation_script_named(name: String) -> AnimationScript {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__animation_script_named__string_ref(__skparam__name);
        __skadapter__to_animation_script(__skreturn)
    }
}
pub fn assign_animation_with_script(anim: Animation, script: AnimationScript, name: String) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__assign_animation__animation__animation_script__string_ref(__skparam__anim, __skparam__script, __skparam__name);
    }
}
pub fn assign_animation_with_script_and_sound(anim: Animation, script: AnimationScript, name: String, with_sound: bool) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__assign_animation__animation__animation_script__string_ref__bool(__skparam__anim, __skparam__script, __skparam__name, __skparam__with_sound);
    }
}
pub fn assign_animation_index_with_script(anim: Animation, script: AnimationScript, idx: i32) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        __sklib__assign_animation__animation__animation_script__int(__skparam__anim, __skparam__script, __skparam__idx);
    }
}
pub fn assign_animation_index_with_script_and_sound(anim: Animation, script: AnimationScript, idx: i32, with_sound: bool) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__assign_animation__animation__animation_script__int__bool(__skparam__anim, __skparam__script, __skparam__idx, __skparam__with_sound);
    }
}
pub fn assign_animation_with_script_named(anim: Animation, script_name: String, name: String) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__script_name = __skadapter__to_sklib_string(script_name);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__assign_animation__animation__string_ref__string_ref(__skparam__anim, __skparam__script_name, __skparam__name);
    }
}
pub fn assign_animation_with_script_named_and_sound(anim: Animation, script_name: String, name: String, with_sound: bool) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__script_name = __skadapter__to_sklib_string(script_name);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__assign_animation__animation__string_ref__string_ref__bool(__skparam__anim, __skparam__script_name, __skparam__name, __skparam__with_sound);
    }
}
pub fn assign_animation_index(anim: Animation, idx: i32) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        __sklib__assign_animation__animation__int(__skparam__anim, __skparam__idx);
    }
}
pub fn assign_animation_index_with_sound(anim: Animation, idx: i32, with_sound: bool) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__assign_animation__animation__int__bool(__skparam__anim, __skparam__idx, __skparam__with_sound);
    }
}
pub fn assign_animation(anim: Animation, name: String) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__assign_animation__animation__string(__skparam__anim, __skparam__name);
    }
}
pub fn assign_animation_with_sound(anim: Animation, name: String, with_sound: bool) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__assign_animation__animation__string__bool(__skparam__anim, __skparam__name, __skparam__with_sound);
    }
}
pub fn create_animation_from_index_with_sound(script: AnimationScript, idx: i32, with_sound: bool) -> Animation {
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        let __skreturn = __sklib__create_animation__animation_script__int__bool(__skparam__script, __skparam__idx, __skparam__with_sound);
        __skadapter__to_animation(__skreturn)
    }
}
pub fn create_animation(script: AnimationScript, name: String) -> Animation {
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__create_animation__animation_script__string_ref(__skparam__script, __skparam__name);
        __skadapter__to_animation(__skreturn)
    }
}
pub fn create_animation_with_sound(script: AnimationScript, name: String, with_sound: bool) -> Animation {
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        let __skreturn = __sklib__create_animation__animation_script__string_ref__bool(__skparam__script, __skparam__name, __skparam__with_sound);
        __skadapter__to_animation(__skreturn)
    }
}
pub fn create_animation_from_script_named(script_name: String, name: String) -> Animation {
    let __skparam__script_name = __skadapter__to_sklib_string(script_name);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__create_animation__string_ref__string_ref(__skparam__script_name, __skparam__name);
        __skadapter__to_animation(__skreturn)
    }
}
pub fn create_animation_from_script_named_with_sound(script_name: String, name: String, with_sound: bool) -> Animation {
    let __skparam__script_name = __skadapter__to_sklib_string(script_name);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        let __skreturn = __sklib__create_animation__string_ref__string_ref__bool(__skparam__script_name, __skparam__name, __skparam__with_sound);
        __skadapter__to_animation(__skreturn)
    }
}
pub fn free_all_animation_scripts() {
    unsafe {
        __sklib__free_all_animation_scripts();
    }
}
pub fn free_animation(ani: Animation) {
    let __skparam__ani = __skadapter__to_sklib_animation(ani);
    unsafe {
        __sklib__free_animation__animation(__skparam__ani);
    }
}
pub fn free_animation_script(script_to_free: AnimationScript) {
    let __skparam__script_to_free = __skadapter__to_sklib_animation_script(script_to_free);
    unsafe {
        __sklib__free_animation_script__animation_script(__skparam__script_to_free);
    }
}
pub fn free_animation_script_with_name(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__free_animation_script__string_ref(__skparam__name);
    }
}
pub fn has_animation_named(script: AnimationScript, name: String) -> bool {
    let __skparam__script = __skadapter__to_sklib_animation_script(script);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_animation_named__animation_script__string_ref(__skparam__script, __skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_animation_script(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_animation_script__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn load_animation_script(name: String, filename: String) -> AnimationScript {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        let __skreturn = __sklib__load_animation_script__string_ref__string_ref(__skparam__name, __skparam__filename);
        __skadapter__to_animation_script(__skreturn)
    }
}
pub fn restart_animation(anim: Animation) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    unsafe {
        __sklib__restart_animation__animation(__skparam__anim);
    }
}
pub fn restart_animation_with_sound(anim: Animation, with_sound: bool) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__restart_animation__animation__bool(__skparam__anim, __skparam__with_sound);
    }
}
pub fn update_animation_percent_with_sound(anim: Animation, pct: f32, with_sound: bool) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__update_animation__animation__float__bool(__skparam__anim, __skparam__pct, __skparam__with_sound);
    }
}
pub fn update_animation(anim: Animation) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    unsafe {
        __sklib__update_animation__animation(__skparam__anim);
    }
}
pub fn update_animation_percent(anim: Animation, pct: f32) {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    unsafe {
        __sklib__update_animation__animation__float(__skparam__anim, __skparam__pct);
    }
}
pub fn audio_ready() -> bool {
    unsafe {
        let __skreturn = __sklib__audio_ready();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn close_audio() {
    unsafe {
        __sklib__close_audio();
    }
}
pub fn open_audio() {
    unsafe {
        __sklib__open_audio();
    }
}
pub fn contains(text: String, subtext: String) -> bool {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__subtext = __skadapter__to_sklib_string(subtext);
    unsafe {
        let __skreturn = __sklib__contains__string_ref__string_ref(__skparam__text, __skparam__subtext);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn convert_to_double(text: String) -> f64 {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__convert_to_double__string_ref(__skparam__text);
        __skadapter__to_double(__skreturn)
    }
}
pub fn convert_to_integer(text: String) -> i32 {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__convert_to_integer__string_ref(__skparam__text);
        __skadapter__to_int(__skreturn)
    }
}
pub fn index_of(text: String, subtext: String) -> i32 {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__subtext = __skadapter__to_sklib_string(subtext);
    unsafe {
        let __skreturn = __sklib__index_of__string_ref__string_ref(__skparam__text, __skparam__subtext);
        __skadapter__to_int(__skreturn)
    }
}
pub fn is_double(text: String) -> bool {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__is_double__string_ref(__skparam__text);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_integer(text: String) -> bool {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__is_integer__string_ref(__skparam__text);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_number(text: String) -> bool {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__is_number__string_ref(__skparam__text);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn length_of(text: String) -> i32 {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__length_of__string_ref(__skparam__text);
        __skadapter__to_int(__skreturn)
    }
}
pub fn replace_all(text: String, substr: String, newtext: String) -> String {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__substr = __skadapter__to_sklib_string(substr);
    let __skparam__newtext = __skadapter__to_sklib_string(newtext);
    unsafe {
        let __skreturn = __sklib__replace_all__string_ref__string_ref__string_ref(__skparam__text, __skparam__substr, __skparam__newtext);
        __skadapter__to_string(__skreturn)
    }
}
pub fn split(text: String, delimiter: char) -> Vec<String> {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__delimiter = __skadapter__to_sklib_char(delimiter);
    unsafe {
        let __skreturn = __sklib__split__string_ref__char(__skparam__text, __skparam__delimiter);
        __skadapter__to_vector_string(__skreturn)
    }
}
pub fn to_lowercase(text: String) -> String {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__to_lowercase__string_ref(__skparam__text);
        __skadapter__to_string(__skreturn)
    }
}
pub fn to_uppercase(text: String) -> String {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__to_uppercase__string_ref(__skparam__text);
        __skadapter__to_string(__skreturn)
    }
}
pub fn trim(text: String) -> String {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__trim__string_ref(__skparam__text);
        __skadapter__to_string(__skreturn)
    }
}
pub fn free_resource_bundle(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__free_resource_bundle__string(__skparam__name);
    }
}
pub fn has_resource_bundle(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_resource_bundle__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn load_resource_bundle(name: String, filename: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        __sklib__load_resource_bundle__string_ref__string_ref(__skparam__name, __skparam__filename);
    }
}
pub fn camera_position() -> Point2D {
    unsafe {
        let __skreturn = __sklib__camera_position();
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn camera_x() -> f64 {
    unsafe {
        let __skreturn = __sklib__camera_x();
        __skadapter__to_double(__skreturn)
    }
}
pub fn camera_y() -> f64 {
    unsafe {
        let __skreturn = __sklib__camera_y();
        __skadapter__to_double(__skreturn)
    }
}
pub fn center_camera_on_vector(s: Sprite, offset: Vector2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__offset = __skadapter__to_sklib_vector_2d(offset);
    unsafe {
        __sklib__center_camera_on__sprite__vector_2d_ref(__skparam__s, __skparam__offset);
    }
}
pub fn center_camera_on(s: Sprite, offset_x: f64, offset_y: f64) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__offset_x = __skadapter__to_sklib_double(offset_x);
    let __skparam__offset_y = __skadapter__to_sklib_double(offset_y);
    unsafe {
        __sklib__center_camera_on__sprite__double__double(__skparam__s, __skparam__offset_x, __skparam__offset_y);
    }
}
pub fn move_camera_by_vector(offset: Vector2D) {
    let __skparam__offset = __skadapter__to_sklib_vector_2d(offset);
    unsafe {
        __sklib__move_camera_by__vector_2d_ref(__skparam__offset);
    }
}
pub fn move_camera_by(dx: f64, dy: f64) {
    let __skparam__dx = __skadapter__to_sklib_double(dx);
    let __skparam__dy = __skadapter__to_sklib_double(dy);
    unsafe {
        __sklib__move_camera_by__double__double(__skparam__dx, __skparam__dy);
    }
}
pub fn move_camera_to_point(pt: Point2D) {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        __sklib__move_camera_to__point_2d_ref(__skparam__pt);
    }
}
pub fn move_camera_to(x: f64, y: f64) {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__move_camera_to__double__double(__skparam__x, __skparam__y);
    }
}
pub fn point_in_window(wind: Window, pt: Point2D) -> bool {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__point_in_window__window__point_2d_ref(__skparam__wind, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_on_screen(pt: Point2D) -> bool {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__point_on_screen__point_2d_ref(__skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn rect_in_window(wind: Window, rect: Rectangle) -> bool {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__rect_in_window__window__rectangle_ref(__skparam__wind, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn rect_on_screen(rect: Rectangle) -> bool {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__rect_on_screen__rectangle_ref(__skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn screen_center() -> Point2D {
    unsafe {
        let __skreturn = __sklib__screen_center();
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn screen_rectangle() -> Rectangle {
    unsafe {
        let __skreturn = __sklib__screen_rectangle();
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn set_camera_position(pos: Point2D) {
    let __skparam__pos = __skadapter__to_sklib_point_2d(pos);
    unsafe {
        __sklib__set_camera_position__point_2d(__skparam__pos);
    }
}
pub fn set_camera_x(x: f64) {
    let __skparam__x = __skadapter__to_sklib_double(x);
    unsafe {
        __sklib__set_camera_x__double(__skparam__x);
    }
}
pub fn set_camera_y(y: f64) {
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__set_camera_y__double(__skparam__y);
    }
}
pub fn to_screen_point(pt: Point2D) -> Point2D {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__to_screen__point_2d_ref(__skparam__pt);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn to_screen_rectangle(rect: Rectangle) -> Rectangle {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__to_screen__rectangle_ref(__skparam__rect);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn to_screen_x(world_x: f64) -> f64 {
    let __skparam__world_x = __skadapter__to_sklib_double(world_x);
    unsafe {
        let __skreturn = __sklib__to_screen_x__double(__skparam__world_x);
        __skadapter__to_double(__skreturn)
    }
}
pub fn to_screen_y(world_y: f64) -> f64 {
    let __skparam__world_y = __skadapter__to_sklib_double(world_y);
    unsafe {
        let __skreturn = __sklib__to_screen_y__double(__skparam__world_y);
        __skadapter__to_double(__skreturn)
    }
}
pub fn to_world(pt: Point2D) -> Point2D {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__to_world__point_2d_ref(__skparam__pt);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn to_world_x(screen_x: f64) -> f64 {
    let __skparam__screen_x = __skadapter__to_sklib_double(screen_x);
    unsafe {
        let __skreturn = __sklib__to_world_x__double(__skparam__screen_x);
        __skadapter__to_double(__skreturn)
    }
}
pub fn to_world_y(screen_y: f64) -> f64 {
    let __skparam__screen_y = __skadapter__to_sklib_double(screen_y);
    unsafe {
        let __skreturn = __sklib__to_world_y__double(__skparam__screen_y);
        __skadapter__to_double(__skreturn)
    }
}
pub fn vector_world_to_screen() -> Vector2D {
    unsafe {
        let __skreturn = __sklib__vector_world_to_screen();
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn window_area(wind: Window) -> Rectangle {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__window_area__window(__skparam__wind);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn draw_circle_record(clr: Color, c: Circle) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        __sklib__draw_circle__color__circle_ref(__skparam__clr, __skparam__c);
    }
}
pub fn draw_circle_record_with_options(clr: Color, c: Circle, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_circle__color__circle_ref__drawing_options(__skparam__clr, __skparam__c, __skparam__opts);
    }
}
pub fn draw_circle(clr: Color, x: f64, y: f64, radius: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        __sklib__draw_circle__color__double__double__double(__skparam__clr, __skparam__x, __skparam__y, __skparam__radius);
    }
}
pub fn draw_circle_with_options(clr: Color, x: f64, y: f64, radius: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_circle__color__double__double__double__drawing_options(__skparam__clr, __skparam__x, __skparam__y, __skparam__radius, __skparam__opts);
    }
}
pub fn draw_circle_on_bitmap(destination: Bitmap, clr: Color, x: f64, y: f64, radius: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        __sklib__draw_circle_on_bitmap__bitmap__color__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__radius);
    }
}
pub fn draw_circle_on_bitmap_with_options(destination: Bitmap, clr: Color, x: f64, y: f64, radius: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_circle_on_bitmap__bitmap__color__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__radius, __skparam__opts);
    }
}
pub fn draw_circle_on_window(destination: Window, clr: Color, x: f64, y: f64, radius: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        __sklib__draw_circle_on_window__window__color__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__radius);
    }
}
pub fn draw_circle_on_window_with_options(destination: Window, clr: Color, x: f64, y: f64, radius: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_circle_on_window__window__color__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__radius, __skparam__opts);
    }
}
pub fn fill_circle_record(clr: Color, c: Circle) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        __sklib__fill_circle__color__circle_ref(__skparam__clr, __skparam__c);
    }
}
pub fn fill_circle_record_with_options(clr: Color, c: Circle, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_circle__color__circle_ref__drawing_options(__skparam__clr, __skparam__c, __skparam__opts);
    }
}
pub fn fill_circle(clr: Color, x: f64, y: f64, radius: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        __sklib__fill_circle__color__double__double__double(__skparam__clr, __skparam__x, __skparam__y, __skparam__radius);
    }
}
pub fn fill_circle_with_options(clr: Color, x: f64, y: f64, radius: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_circle__color__double__double__double__drawing_options(__skparam__clr, __skparam__x, __skparam__y, __skparam__radius, __skparam__opts);
    }
}
pub fn fill_circle_on_bitmap(destination: Bitmap, clr: Color, x: f64, y: f64, radius: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        __sklib__fill_circle_on_bitmap__bitmap__color__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__radius);
    }
}
pub fn fill_circle_on_bitmap_with_options(destination: Bitmap, clr: Color, x: f64, y: f64, radius: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_circle_on_bitmap__bitmap__color__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__radius, __skparam__opts);
    }
}
pub fn fill_circle_on_window(destination: Window, clr: Color, x: f64, y: f64, radius: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        __sklib__fill_circle_on_window__window__color__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__radius);
    }
}
pub fn fill_circle_on_window_with_options(destination: Window, clr: Color, x: f64, y: f64, radius: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_circle_on_window__window__color__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__radius, __skparam__opts);
    }
}
pub fn center_point_of_circle(c: Circle) -> Point2D {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__center_point__circle_ref(__skparam__c);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn circle_at(pt: Point2D, radius: f64) -> Circle {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        let __skreturn = __sklib__circle_at__point_2d_ref__double(__skparam__pt, __skparam__radius);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn circle_at_from_points(x: f64, y: f64, radius: f64) -> Circle {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        let __skreturn = __sklib__circle_at__double__double__double(__skparam__x, __skparam__y, __skparam__radius);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn circle_radius(c: Circle) -> f32 {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__circle_radius__circle(__skparam__c);
        __skadapter__to_float(__skreturn)
    }
}
pub fn circle_triangle_intersect(c: Circle, tri: Triangle) -> bool {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        let __skreturn = __sklib__circle_triangle_intersect__circle_ref__triangle_ref(__skparam__c, __skparam__tri);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn circle_triangle_intersect_get_closest_point(c: Circle, tri: Triangle, p: &mut Point2D) -> bool {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    let mut __skparam__p = __skadapter__to_sklib_point_2d((*p).clone());
    unsafe {
        let __skreturn = __sklib__circle_triangle_intersect__circle_ref__triangle_ref__point_2d_ref(__skparam__c, __skparam__tri, &mut __skparam__p);
        *p = __skadapter__to_point_2d(__skparam__p);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn circle_x(c: Circle) -> f32 {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__circle_x__circle_ref(__skparam__c);
        __skadapter__to_float(__skreturn)
    }
}
pub fn circle_y(c: Circle) -> f32 {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__circle_y__circle_ref(__skparam__c);
        __skadapter__to_float(__skreturn)
    }
}
pub fn circles_intersect(c1: Circle, c2: Circle) -> bool {
    let __skparam__c1 = __skadapter__to_sklib_circle(c1);
    let __skparam__c2 = __skadapter__to_sklib_circle(c2);
    unsafe {
        let __skreturn = __sklib__circles_intersect__circle__circle(__skparam__c1, __skparam__c2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn circles_intersect_using_values(c1_x: f64, c1_y: f64, c1_radius: f64, c2_x: f64, c2_y: f64, c2_radius: f64) -> bool {
    let __skparam__c1_x = __skadapter__to_sklib_double(c1_x);
    let __skparam__c1_y = __skadapter__to_sklib_double(c1_y);
    let __skparam__c1_radius = __skadapter__to_sklib_double(c1_radius);
    let __skparam__c2_x = __skadapter__to_sklib_double(c2_x);
    let __skparam__c2_y = __skadapter__to_sklib_double(c2_y);
    let __skparam__c2_radius = __skadapter__to_sklib_double(c2_radius);
    unsafe {
        let __skreturn = __sklib__circles_intersect__double__double__double__double__double__double(__skparam__c1_x, __skparam__c1_y, __skparam__c1_radius, __skparam__c2_x, __skparam__c2_y, __skparam__c2_radius);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn closest_point_on_circle(from_pt: Point2D, c: Circle) -> Point2D {
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__closest_point_on_circle__point_2d_ref__circle_ref(__skparam__from_pt, __skparam__c);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn closest_point_on_line_from_circle(c: Circle, l: Line) -> Point2D {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__closest_point_on_line_from_circle__circle_ref__line_ref(__skparam__c, __skparam__l);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn closest_point_on_rect_from_circle(c: Circle, rect: Rectangle) -> Point2D {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__closest_point_on_rect_from_circle__circle_ref__rectangle_ref(__skparam__c, __skparam__rect);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn closest_point_on_triangle_from_circle(c: Circle, tri: Triangle) -> Point2D {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        let __skreturn = __sklib__closest_point_on_triangle_from_circle__circle_ref__triangle_ref(__skparam__c, __skparam__tri);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn distant_point_on_circle(pt: Point2D, c: Circle) -> Point2D {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__distant_point_on_circle__point_2d_ref__circle_ref(__skparam__pt, __skparam__c);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn distant_point_on_circle_heading(pt: Point2D, c: Circle, heading: Vector2D, opposite_pt: &mut Point2D) -> bool {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__heading = __skadapter__to_sklib_vector_2d(heading);
    let mut __skparam__opposite_pt = __skadapter__to_sklib_point_2d((*opposite_pt).clone());
    unsafe {
        let __skreturn = __sklib__distant_point_on_circle_heading__point_2d_ref__circle_ref__vector_2d_ref__point_2d_ref(__skparam__pt, __skparam__c, __skparam__heading, &mut __skparam__opposite_pt);
        *opposite_pt = __skadapter__to_point_2d(__skparam__opposite_pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn ray_circle_intersect_distance(ray_origin: Point2D, ray_heading: Vector2D, c: Circle) -> f32 {
    let __skparam__ray_origin = __skadapter__to_sklib_point_2d(ray_origin);
    let __skparam__ray_heading = __skadapter__to_sklib_vector_2d(ray_heading);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__ray_circle_intersect_distance__point_2d_ref__vector_2d_ref__circle_ref(__skparam__ray_origin, __skparam__ray_heading, __skparam__c);
        __skadapter__to_float(__skreturn)
    }
}
pub fn tangent_points(from_pt: Point2D, c: Circle, p1: &mut Point2D, p2: &mut Point2D) -> bool {
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let mut __skparam__p1 = __skadapter__to_sklib_point_2d((*p1).clone());
    let mut __skparam__p2 = __skadapter__to_sklib_point_2d((*p2).clone());
    unsafe {
        let __skreturn = __sklib__tangent_points__point_2d_ref__circle_ref__point_2d_ref__point_2d_ref(__skparam__from_pt, __skparam__c, &mut __skparam__p1, &mut __skparam__p2);
        *p1 = __skadapter__to_point_2d(__skparam__p1);
        *p2 = __skadapter__to_point_2d(__skparam__p2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn widest_points(c: Circle, along: Vector2D, pt1: &mut Point2D, pt2: &mut Point2D) {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__along = __skadapter__to_sklib_vector_2d(along);
    let mut __skparam__pt1 = __skadapter__to_sklib_point_2d((*pt1).clone());
    let mut __skparam__pt2 = __skadapter__to_sklib_point_2d((*pt2).clone());
    unsafe {
        __sklib__widest_points__circle_ref__vector_2d_ref__point_2d_ref__point_2d_ref(__skparam__c, __skparam__along, &mut __skparam__pt1, &mut __skparam__pt2);
        *pt1 = __skadapter__to_point_2d(__skparam__pt1);
        *pt2 = __skadapter__to_point_2d(__skparam__pt2);
    }
}
pub fn current_clip() -> Rectangle {
    unsafe {
        let __skreturn = __sklib__current_clip();
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn current_clip_for_bitmap(bmp: Bitmap) -> Rectangle {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__current_clip__bitmap(__skparam__bmp);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn current_clip_for_window(wnd: Window) -> Rectangle {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        let __skreturn = __sklib__current_clip__window(__skparam__wnd);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn pop_clip_for_window(wnd: Window) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        __sklib__pop_clip__window(__skparam__wnd);
    }
}
pub fn pop_clip() {
    unsafe {
        __sklib__pop_clip();
    }
}
pub fn pop_clip_for_bitmap(bmp: Bitmap) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        __sklib__pop_clip__bitmap(__skparam__bmp);
    }
}
pub fn push_clip_for_window(wnd: Window, r: Rectangle) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__r = __skadapter__to_sklib_rectangle(r);
    unsafe {
        __sklib__push_clip__window__rectangle_ref(__skparam__wnd, __skparam__r);
    }
}
pub fn push_clip_for_bitmap(bmp: Bitmap, r: Rectangle) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__r = __skadapter__to_sklib_rectangle(r);
    unsafe {
        __sklib__push_clip__bitmap__rectangle_ref(__skparam__bmp, __skparam__r);
    }
}
pub fn push_clip(r: Rectangle) {
    let __skparam__r = __skadapter__to_sklib_rectangle(r);
    unsafe {
        __sklib__push_clip__rectangle_ref(__skparam__r);
    }
}
pub fn reset_clip_for_bitmap(bmp: Bitmap) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        __sklib__reset_clip__bitmap(__skparam__bmp);
    }
}
pub fn reset_clip() {
    unsafe {
        __sklib__reset_clip();
    }
}
pub fn reset_clip_for_window(wnd: Window) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        __sklib__reset_clip__window(__skparam__wnd);
    }
}
pub fn set_clip(r: Rectangle) {
    let __skparam__r = __skadapter__to_sklib_rectangle(r);
    unsafe {
        __sklib__set_clip__rectangle_ref(__skparam__r);
    }
}
pub fn set_clip_for_bitmap(bmp: Bitmap, r: Rectangle) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__r = __skadapter__to_sklib_rectangle(r);
    unsafe {
        __sklib__set_clip__bitmap__rectangle_ref(__skparam__bmp, __skparam__r);
    }
}
pub fn set_clip_for_window(wnd: Window, r: Rectangle) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__r = __skadapter__to_sklib_rectangle(r);
    unsafe {
        __sklib__set_clip__window__rectangle_ref(__skparam__wnd, __skparam__r);
    }
}
pub fn bitmap_circle_collision_at_point(bmp: Bitmap, pt: Point2D, circ: Circle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__circ = __skadapter__to_sklib_circle(circ);
    unsafe {
        let __skreturn = __sklib__bitmap_circle_collision__bitmap__point_2d_ref__circle_ref(__skparam__bmp, __skparam__pt, __skparam__circ);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_circle_collision(bmp: Bitmap, x: f64, y: f64, circ: Circle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__circ = __skadapter__to_sklib_circle(circ);
    unsafe {
        let __skreturn = __sklib__bitmap_circle_collision__bitmap__double__double__circle_ref(__skparam__bmp, __skparam__x, __skparam__y, __skparam__circ);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_circle_collision_for_cell_with_translation(bmp: Bitmap, cell: i32, translation: Matrix2D, circ: Circle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__translation = __skadapter__to_sklib_matrix_2d(translation);
    let __skparam__circ = __skadapter__to_sklib_circle(circ);
    unsafe {
        let __skreturn = __sklib__bitmap_circle_collision__bitmap__int__matrix_2d_ref__circle_ref(__skparam__bmp, __skparam__cell, __skparam__translation, __skparam__circ);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_circle_collision_for_cell_at_point(bmp: Bitmap, cell: i32, pt: Point2D, circ: Circle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__circ = __skadapter__to_sklib_circle(circ);
    unsafe {
        let __skreturn = __sklib__bitmap_circle_collision__bitmap__int__point_2d_ref__circle_ref(__skparam__bmp, __skparam__cell, __skparam__pt, __skparam__circ);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_circle_collision_for_cell(bmp: Bitmap, cell: i32, x: f64, y: f64, circ: Circle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__circ = __skadapter__to_sklib_circle(circ);
    unsafe {
        let __skreturn = __sklib__bitmap_circle_collision__bitmap__int__double__double__circle_ref(__skparam__bmp, __skparam__cell, __skparam__x, __skparam__y, __skparam__circ);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_collision(bmp1: Bitmap, x1: f64, y1: f64, bmp2: Bitmap, x2: f64, y2: f64) -> bool {
    let __skparam__bmp1 = __skadapter__to_sklib_bitmap(bmp1);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__bmp2 = __skadapter__to_sklib_bitmap(bmp2);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    unsafe {
        let __skreturn = __sklib__bitmap_collision__bitmap__double__double__bitmap__double__double(__skparam__bmp1, __skparam__x1, __skparam__y1, __skparam__bmp2, __skparam__x2, __skparam__y2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_collision_at_points(bmp1: Bitmap, pt1: Point2D, bmp2: Bitmap, pt2: Point2D) -> bool {
    let __skparam__bmp1 = __skadapter__to_sklib_bitmap(bmp1);
    let __skparam__pt1 = __skadapter__to_sklib_point_2d(pt1);
    let __skparam__bmp2 = __skadapter__to_sklib_bitmap(bmp2);
    let __skparam__pt2 = __skadapter__to_sklib_point_2d(pt2);
    unsafe {
        let __skreturn = __sklib__bitmap_collision__bitmap__point_2d_ref__bitmap__point_2d_ref(__skparam__bmp1, __skparam__pt1, __skparam__bmp2, __skparam__pt2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_collision_for_cells_with_translations(bmp1: Bitmap, cell1: i32, matrix1: Matrix2D, bmp2: Bitmap, cell2: i32, matrix2: Matrix2D) -> bool {
    let __skparam__bmp1 = __skadapter__to_sklib_bitmap(bmp1);
    let __skparam__cell1 = __skadapter__to_sklib_int(cell1);
    let __skparam__matrix1 = __skadapter__to_sklib_matrix_2d(matrix1);
    let __skparam__bmp2 = __skadapter__to_sklib_bitmap(bmp2);
    let __skparam__cell2 = __skadapter__to_sklib_int(cell2);
    let __skparam__matrix2 = __skadapter__to_sklib_matrix_2d(matrix2);
    unsafe {
        let __skreturn = __sklib__bitmap_collision__bitmap__int__matrix_2d_ref__bitmap__int__matrix_2d_ref(__skparam__bmp1, __skparam__cell1, __skparam__matrix1, __skparam__bmp2, __skparam__cell2, __skparam__matrix2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_collision_for_cells_at_points(bmp1: Bitmap, cell1: i32, pt1: Point2D, bmp2: Bitmap, cell2: i32, pt2: Point2D) -> bool {
    let __skparam__bmp1 = __skadapter__to_sklib_bitmap(bmp1);
    let __skparam__cell1 = __skadapter__to_sklib_int(cell1);
    let __skparam__pt1 = __skadapter__to_sklib_point_2d(pt1);
    let __skparam__bmp2 = __skadapter__to_sklib_bitmap(bmp2);
    let __skparam__cell2 = __skadapter__to_sklib_int(cell2);
    let __skparam__pt2 = __skadapter__to_sklib_point_2d(pt2);
    unsafe {
        let __skreturn = __sklib__bitmap_collision__bitmap__int__point_2d_ref__bitmap__int__point_2d_ref(__skparam__bmp1, __skparam__cell1, __skparam__pt1, __skparam__bmp2, __skparam__cell2, __skparam__pt2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_collision_for_cells(bmp1: Bitmap, cell1: i32, x1: f64, y1: f64, bmp2: Bitmap, cell2: i32, x2: f64, y2: f64) -> bool {
    let __skparam__bmp1 = __skadapter__to_sklib_bitmap(bmp1);
    let __skparam__cell1 = __skadapter__to_sklib_int(cell1);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__bmp2 = __skadapter__to_sklib_bitmap(bmp2);
    let __skparam__cell2 = __skadapter__to_sklib_int(cell2);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    unsafe {
        let __skreturn = __sklib__bitmap_collision__bitmap__int__double__double__bitmap__int__double__double(__skparam__bmp1, __skparam__cell1, __skparam__x1, __skparam__y1, __skparam__bmp2, __skparam__cell2, __skparam__x2, __skparam__y2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_point_collision_with_translation(bmp: Bitmap, translation: Matrix2D, pt: Point2D) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__translation = __skadapter__to_sklib_matrix_2d(translation);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__bitmap_point_collision__bitmap__matrix_2d_ref__point_2d_ref(__skparam__bmp, __skparam__translation, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_point_collision_at_point(bmp: Bitmap, bmp_pt: Point2D, pt: Point2D) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__bmp_pt = __skadapter__to_sklib_point_2d(bmp_pt);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__bitmap_point_collision__bitmap__point_2d_ref__point_2d_ref(__skparam__bmp, __skparam__bmp_pt, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_point_collision(bmp: Bitmap, bmp_x: f64, bmp_y: f64, x: f64, y: f64) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__bmp_x = __skadapter__to_sklib_double(bmp_x);
    let __skparam__bmp_y = __skadapter__to_sklib_double(bmp_y);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__bitmap_point_collision__bitmap__double__double__double__double(__skparam__bmp, __skparam__bmp_x, __skparam__bmp_y, __skparam__x, __skparam__y);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_point_collision_for_cell_with_translation(bmp: Bitmap, cell: i32, translation: Matrix2D, pt: Point2D) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__translation = __skadapter__to_sklib_matrix_2d(translation);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__bitmap_point_collision__bitmap__int__matrix_2d_ref__point_2d_ref(__skparam__bmp, __skparam__cell, __skparam__translation, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_point_collision_for_cell_at_point(bmp: Bitmap, cell: i32, bmp_pt: Point2D, pt: Point2D) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__bmp_pt = __skadapter__to_sklib_point_2d(bmp_pt);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__bitmap_point_collision__bitmap__int__point_2d_ref__point_2d_ref(__skparam__bmp, __skparam__cell, __skparam__bmp_pt, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_point_collision_for_cell(bmp: Bitmap, cell: i32, bmp_x: f64, bmp_y: f64, x: f64, y: f64) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__bmp_x = __skadapter__to_sklib_double(bmp_x);
    let __skparam__bmp_y = __skadapter__to_sklib_double(bmp_y);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__bitmap_point_collision__bitmap__int__double__double__double__double(__skparam__bmp, __skparam__cell, __skparam__bmp_x, __skparam__bmp_y, __skparam__x, __skparam__y);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_rectangle_collision_at_point(bmp: Bitmap, pt: Point2D, rect: Rectangle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__bitmap_rectangle_collision__bitmap__point_2d_ref__rectangle_ref(__skparam__bmp, __skparam__pt, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_rectangle_collision(bmp: Bitmap, x: f64, y: f64, rect: Rectangle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__bitmap_rectangle_collision__bitmap__double__double__rectangle_ref(__skparam__bmp, __skparam__x, __skparam__y, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_rectangle_collision_for_cell_with_translation(bmp: Bitmap, cell: i32, translation: Matrix2D, rect: Rectangle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__translation = __skadapter__to_sklib_matrix_2d(translation);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__bitmap_rectangle_collision__bitmap__int__matrix_2d_ref__rectangle_ref(__skparam__bmp, __skparam__cell, __skparam__translation, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_rectangle_collision_for_cell_at_point(bmp: Bitmap, cell: i32, pt: Point2D, rect: Rectangle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__bitmap_rectangle_collision__bitmap__int__point_2d_ref__rectangle_ref(__skparam__bmp, __skparam__cell, __skparam__pt, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_rectangle_collision_for_cell(bmp: Bitmap, cell: i32, x: f64, y: f64, rect: Rectangle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__bitmap_rectangle_collision__bitmap__int__double__double__rectangle_ref(__skparam__bmp, __skparam__cell, __skparam__x, __skparam__y, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_bitmap_collision(s: Sprite, bmp: Bitmap, x: f64, y: f64) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__sprite_bitmap_collision__sprite__bitmap__double__double(__skparam__s, __skparam__bmp, __skparam__x, __skparam__y);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_bitmap_collision_with_cell_at_point(s: Sprite, bmp: Bitmap, cell: i32, pt: Point2D) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__sprite_bitmap_collision__sprite__bitmap__int__point_2d_ref(__skparam__s, __skparam__bmp, __skparam__cell, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_bitmap_collision_with_cell(s: Sprite, bmp: Bitmap, cell: i32, x: f64, y: f64) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__sprite_bitmap_collision__sprite__bitmap__int__double__double(__skparam__s, __skparam__bmp, __skparam__cell, __skparam__x, __skparam__y);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_collision(s1: Sprite, s2: Sprite) -> bool {
    let __skparam__s1 = __skadapter__to_sklib_sprite(s1);
    let __skparam__s2 = __skadapter__to_sklib_sprite(s2);
    unsafe {
        let __skreturn = __sklib__sprite_collision__sprite__sprite(__skparam__s1, __skparam__s2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_point_collision(s: Sprite, pt: Point2D) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__sprite_point_collision__sprite__point_2d_ref(__skparam__s, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_rectangle_collision(s: Sprite, rect: Rectangle) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__sprite_rectangle_collision__sprite__rectangle_ref(__skparam__s, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn alpha_of(c: Color) -> i32 {
    let __skparam__c = __skadapter__to_sklib_color(c);
    unsafe {
        let __skreturn = __sklib__alpha_of__color(__skparam__c);
        __skadapter__to_int(__skreturn)
    }
}
pub fn blue_of(c: Color) -> i32 {
    let __skparam__c = __skadapter__to_sklib_color(c);
    unsafe {
        let __skreturn = __sklib__blue_of__color(__skparam__c);
        __skadapter__to_int(__skreturn)
    }
}
pub fn brightness_of(c: Color) -> f64 {
    let __skparam__c = __skadapter__to_sklib_color(c);
    unsafe {
        let __skreturn = __sklib__brightness_of__color(__skparam__c);
        __skadapter__to_double(__skreturn)
    }
}
pub fn color_alice_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_alice_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_antique_white() -> Color {
    unsafe {
        let __skreturn = __sklib__color_antique_white();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_aqua() -> Color {
    unsafe {
        let __skreturn = __sklib__color_aqua();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_aquamarine() -> Color {
    unsafe {
        let __skreturn = __sklib__color_aquamarine();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_azure() -> Color {
    unsafe {
        let __skreturn = __sklib__color_azure();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_beige() -> Color {
    unsafe {
        let __skreturn = __sklib__color_beige();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_bisque() -> Color {
    unsafe {
        let __skreturn = __sklib__color_bisque();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_black() -> Color {
    unsafe {
        let __skreturn = __sklib__color_black();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_blanched_almond() -> Color {
    unsafe {
        let __skreturn = __sklib__color_blanched_almond();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_blue_violet() -> Color {
    unsafe {
        let __skreturn = __sklib__color_blue_violet();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_bright_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_bright_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_brown() -> Color {
    unsafe {
        let __skreturn = __sklib__color_brown();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_burly_wood() -> Color {
    unsafe {
        let __skreturn = __sklib__color_burly_wood();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_cadet_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_cadet_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_chartreuse() -> Color {
    unsafe {
        let __skreturn = __sklib__color_chartreuse();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_chocolate() -> Color {
    unsafe {
        let __skreturn = __sklib__color_chocolate();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_coral() -> Color {
    unsafe {
        let __skreturn = __sklib__color_coral();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_cornflower_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_cornflower_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_cornsilk() -> Color {
    unsafe {
        let __skreturn = __sklib__color_cornsilk();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_crimson() -> Color {
    unsafe {
        let __skreturn = __sklib__color_crimson();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_cyan() -> Color {
    unsafe {
        let __skreturn = __sklib__color_cyan();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_cyan() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_cyan();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_goldenrod() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_goldenrod();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_gray() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_gray();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_khaki() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_khaki();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_magenta() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_magenta();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_olive_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_olive_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_orange() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_orange();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_orchid() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_orchid();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_red() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_red();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_salmon() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_salmon();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_sea_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_sea_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_slate_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_slate_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_slate_gray() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_slate_gray();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_turquoise() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_turquoise();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dark_violet() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dark_violet();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_deep_pink() -> Color {
    unsafe {
        let __skreturn = __sklib__color_deep_pink();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_deep_sky_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_deep_sky_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dim_gray() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dim_gray();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_dodger_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_dodger_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_firebrick() -> Color {
    unsafe {
        let __skreturn = __sklib__color_firebrick();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_floral_white() -> Color {
    unsafe {
        let __skreturn = __sklib__color_floral_white();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_forest_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_forest_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_fuchsia() -> Color {
    unsafe {
        let __skreturn = __sklib__color_fuchsia();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_gainsboro() -> Color {
    unsafe {
        let __skreturn = __sklib__color_gainsboro();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_ghost_white() -> Color {
    unsafe {
        let __skreturn = __sklib__color_ghost_white();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_gold() -> Color {
    unsafe {
        let __skreturn = __sklib__color_gold();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_goldenrod() -> Color {
    unsafe {
        let __skreturn = __sklib__color_goldenrod();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_gray() -> Color {
    unsafe {
        let __skreturn = __sklib__color_gray();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_green_yellow() -> Color {
    unsafe {
        let __skreturn = __sklib__color_green_yellow();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_honeydew() -> Color {
    unsafe {
        let __skreturn = __sklib__color_honeydew();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_hot_pink() -> Color {
    unsafe {
        let __skreturn = __sklib__color_hot_pink();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_indian_red() -> Color {
    unsafe {
        let __skreturn = __sklib__color_indian_red();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_indigo() -> Color {
    unsafe {
        let __skreturn = __sklib__color_indigo();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_ivory() -> Color {
    unsafe {
        let __skreturn = __sklib__color_ivory();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_khaki() -> Color {
    unsafe {
        let __skreturn = __sklib__color_khaki();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_lavender() -> Color {
    unsafe {
        let __skreturn = __sklib__color_lavender();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_lavender_blush() -> Color {
    unsafe {
        let __skreturn = __sklib__color_lavender_blush();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_lawn_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_lawn_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_lemon_chiffon() -> Color {
    unsafe {
        let __skreturn = __sklib__color_lemon_chiffon();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_coral() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_coral();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_cyan() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_cyan();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_goldenrod_yellow() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_goldenrod_yellow();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_gray() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_gray();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_pink() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_pink();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_salmon() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_salmon();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_sea_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_sea_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_sky_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_sky_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_slate_gray() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_slate_gray();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_steel_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_steel_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_light_yellow() -> Color {
    unsafe {
        let __skreturn = __sklib__color_light_yellow();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_lime() -> Color {
    unsafe {
        let __skreturn = __sklib__color_lime();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_lime_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_lime_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_linen() -> Color {
    unsafe {
        let __skreturn = __sklib__color_linen();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_magenta() -> Color {
    unsafe {
        let __skreturn = __sklib__color_magenta();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_maroon() -> Color {
    unsafe {
        let __skreturn = __sklib__color_maroon();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_aquamarine() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_aquamarine();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_orchid() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_orchid();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_purple() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_purple();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_sea_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_sea_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_slate_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_slate_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_spring_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_spring_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_turquoise() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_turquoise();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_medium_violet_red() -> Color {
    unsafe {
        let __skreturn = __sklib__color_medium_violet_red();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_midnight_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_midnight_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_mint_cream() -> Color {
    unsafe {
        let __skreturn = __sklib__color_mint_cream();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_misty_rose() -> Color {
    unsafe {
        let __skreturn = __sklib__color_misty_rose();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_moccasin() -> Color {
    unsafe {
        let __skreturn = __sklib__color_moccasin();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_navajo_white() -> Color {
    unsafe {
        let __skreturn = __sklib__color_navajo_white();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_navy() -> Color {
    unsafe {
        let __skreturn = __sklib__color_navy();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_old_lace() -> Color {
    unsafe {
        let __skreturn = __sklib__color_old_lace();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_olive() -> Color {
    unsafe {
        let __skreturn = __sklib__color_olive();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_olive_drab() -> Color {
    unsafe {
        let __skreturn = __sklib__color_olive_drab();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_orange() -> Color {
    unsafe {
        let __skreturn = __sklib__color_orange();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_orange_red() -> Color {
    unsafe {
        let __skreturn = __sklib__color_orange_red();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_orchid() -> Color {
    unsafe {
        let __skreturn = __sklib__color_orchid();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_pale_goldenrod() -> Color {
    unsafe {
        let __skreturn = __sklib__color_pale_goldenrod();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_pale_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_pale_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_pale_turquoise() -> Color {
    unsafe {
        let __skreturn = __sklib__color_pale_turquoise();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_pale_violet_red() -> Color {
    unsafe {
        let __skreturn = __sklib__color_pale_violet_red();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_papaya_whip() -> Color {
    unsafe {
        let __skreturn = __sklib__color_papaya_whip();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_peach_puff() -> Color {
    unsafe {
        let __skreturn = __sklib__color_peach_puff();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_peru() -> Color {
    unsafe {
        let __skreturn = __sklib__color_peru();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_pink() -> Color {
    unsafe {
        let __skreturn = __sklib__color_pink();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_plum() -> Color {
    unsafe {
        let __skreturn = __sklib__color_plum();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_powder_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_powder_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_purple() -> Color {
    unsafe {
        let __skreturn = __sklib__color_purple();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_red() -> Color {
    unsafe {
        let __skreturn = __sklib__color_red();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_rosy_brown() -> Color {
    unsafe {
        let __skreturn = __sklib__color_rosy_brown();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_royal_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_royal_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_saddle_brown() -> Color {
    unsafe {
        let __skreturn = __sklib__color_saddle_brown();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_salmon() -> Color {
    unsafe {
        let __skreturn = __sklib__color_salmon();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_sandy_brown() -> Color {
    unsafe {
        let __skreturn = __sklib__color_sandy_brown();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_sea_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_sea_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_sea_shell() -> Color {
    unsafe {
        let __skreturn = __sklib__color_sea_shell();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_sienna() -> Color {
    unsafe {
        let __skreturn = __sklib__color_sienna();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_silver() -> Color {
    unsafe {
        let __skreturn = __sklib__color_silver();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_sky_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_sky_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_slate_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_slate_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_slate_gray() -> Color {
    unsafe {
        let __skreturn = __sklib__color_slate_gray();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_snow() -> Color {
    unsafe {
        let __skreturn = __sklib__color_snow();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_spring_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_spring_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_steel_blue() -> Color {
    unsafe {
        let __skreturn = __sklib__color_steel_blue();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_swinburne_red() -> Color {
    unsafe {
        let __skreturn = __sklib__color_swinburne_red();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_tan() -> Color {
    unsafe {
        let __skreturn = __sklib__color_tan();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_teal() -> Color {
    unsafe {
        let __skreturn = __sklib__color_teal();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_thistle() -> Color {
    unsafe {
        let __skreturn = __sklib__color_thistle();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_to_string(c: Color) -> String {
    let __skparam__c = __skadapter__to_sklib_color(c);
    unsafe {
        let __skreturn = __sklib__color_to_string__color(__skparam__c);
        __skadapter__to_string(__skreturn)
    }
}
pub fn color_tomato() -> Color {
    unsafe {
        let __skreturn = __sklib__color_tomato();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_transparent() -> Color {
    unsafe {
        let __skreturn = __sklib__color_transparent();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_turquoise() -> Color {
    unsafe {
        let __skreturn = __sklib__color_turquoise();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_violet() -> Color {
    unsafe {
        let __skreturn = __sklib__color_violet();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_wheat() -> Color {
    unsafe {
        let __skreturn = __sklib__color_wheat();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_white() -> Color {
    unsafe {
        let __skreturn = __sklib__color_white();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_white_smoke() -> Color {
    unsafe {
        let __skreturn = __sklib__color_white_smoke();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_yellow() -> Color {
    unsafe {
        let __skreturn = __sklib__color_yellow();
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_yellow_green() -> Color {
    unsafe {
        let __skreturn = __sklib__color_yellow_green();
        __skadapter__to_color(__skreturn)
    }
}
pub fn green_of(c: Color) -> i32 {
    let __skparam__c = __skadapter__to_sklib_color(c);
    unsafe {
        let __skreturn = __sklib__green_of__color(__skparam__c);
        __skadapter__to_int(__skreturn)
    }
}
pub fn hsb_color(hue: f64, saturation: f64, brightness: f64) -> Color {
    let __skparam__hue = __skadapter__to_sklib_double(hue);
    let __skparam__saturation = __skadapter__to_sklib_double(saturation);
    let __skparam__brightness = __skadapter__to_sklib_double(brightness);
    unsafe {
        let __skreturn = __sklib__hsb_color__double__double__double(__skparam__hue, __skparam__saturation, __skparam__brightness);
        __skadapter__to_color(__skreturn)
    }
}
pub fn hue_of(c: Color) -> f64 {
    let __skparam__c = __skadapter__to_sklib_color(c);
    unsafe {
        let __skreturn = __sklib__hue_of__color(__skparam__c);
        __skadapter__to_double(__skreturn)
    }
}
pub fn random_color() -> Color {
    unsafe {
        let __skreturn = __sklib__random_color();
        __skadapter__to_color(__skreturn)
    }
}
pub fn random_rgb_color(alpha: i32) -> Color {
    let __skparam__alpha = __skadapter__to_sklib_int(alpha);
    unsafe {
        let __skreturn = __sklib__random_rgb_color__int(__skparam__alpha);
        __skadapter__to_color(__skreturn)
    }
}
pub fn red_of(c: Color) -> i32 {
    let __skparam__c = __skadapter__to_sklib_color(c);
    unsafe {
        let __skreturn = __sklib__red_of__color(__skparam__c);
        __skadapter__to_int(__skreturn)
    }
}
pub fn rgb_color_from_double(red: f64, green: f64, blue: f64) -> Color {
    let __skparam__red = __skadapter__to_sklib_double(red);
    let __skparam__green = __skadapter__to_sklib_double(green);
    let __skparam__blue = __skadapter__to_sklib_double(blue);
    unsafe {
        let __skreturn = __sklib__rgb_color__double__double__double(__skparam__red, __skparam__green, __skparam__blue);
        __skadapter__to_color(__skreturn)
    }
}
pub fn rgb_color(red: i32, green: i32, blue: i32) -> Color {
    let __skparam__red = __skadapter__to_sklib_int(red);
    let __skparam__green = __skadapter__to_sklib_int(green);
    let __skparam__blue = __skadapter__to_sklib_int(blue);
    unsafe {
        let __skreturn = __sklib__rgb_color__int__int__int(__skparam__red, __skparam__green, __skparam__blue);
        __skadapter__to_color(__skreturn)
    }
}
pub fn rgba_color_from_double(red: f64, green: f64, blue: f64, alpha: f64) -> Color {
    let __skparam__red = __skadapter__to_sklib_double(red);
    let __skparam__green = __skadapter__to_sklib_double(green);
    let __skparam__blue = __skadapter__to_sklib_double(blue);
    let __skparam__alpha = __skadapter__to_sklib_double(alpha);
    unsafe {
        let __skreturn = __sklib__rgba_color__double__double__double__double(__skparam__red, __skparam__green, __skparam__blue, __skparam__alpha);
        __skadapter__to_color(__skreturn)
    }
}
pub fn rgba_color(red: i32, green: i32, blue: i32, alpha: i32) -> Color {
    let __skparam__red = __skadapter__to_sklib_int(red);
    let __skparam__green = __skadapter__to_sklib_int(green);
    let __skparam__blue = __skadapter__to_sklib_int(blue);
    let __skparam__alpha = __skadapter__to_sklib_int(alpha);
    unsafe {
        let __skreturn = __sklib__rgba_color__int__int__int__int(__skparam__red, __skparam__green, __skparam__blue, __skparam__alpha);
        __skadapter__to_color(__skreturn)
    }
}
pub fn saturation_of(c: Color) -> f64 {
    let __skparam__c = __skadapter__to_sklib_color(c);
    unsafe {
        let __skreturn = __sklib__saturation_of__color(__skparam__c);
        __skadapter__to_double(__skreturn)
    }
}
pub fn string_to_color(str: String) -> Color {
    let __skparam__str = __skadapter__to_sklib_string(str);
    unsafe {
        let __skreturn = __sklib__string_to_color__string(__skparam__str);
        __skadapter__to_color(__skreturn)
    }
}
pub fn option_defaults() -> DrawingOptions {
    unsafe {
        let __skreturn = __sklib__option_defaults();
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_draw_to_bitmap(dest: Bitmap) -> DrawingOptions {
    let __skparam__dest = __skadapter__to_sklib_bitmap(dest);
    unsafe {
        let __skreturn = __sklib__option_draw_to__bitmap(__skparam__dest);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_draw_to_bitmap_with_options(dest: Bitmap, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__dest = __skadapter__to_sklib_bitmap(dest);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_draw_to__bitmap__drawing_options(__skparam__dest, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_draw_to_window(dest: Window) -> DrawingOptions {
    let __skparam__dest = __skadapter__to_sklib_window(dest);
    unsafe {
        let __skreturn = __sklib__option_draw_to__window(__skparam__dest);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_draw_to_window_with_options(dest: Window, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__dest = __skadapter__to_sklib_window(dest);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_draw_to__window__drawing_options(__skparam__dest, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_flip_x() -> DrawingOptions {
    unsafe {
        let __skreturn = __sklib__option_flip_x();
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_flip_x_with_options(opts: DrawingOptions) -> DrawingOptions {
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_flip_x__drawing_options(__skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_flip_xy() -> DrawingOptions {
    unsafe {
        let __skreturn = __sklib__option_flip_xy();
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_flip_xy_with_options(opts: DrawingOptions) -> DrawingOptions {
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_flip_xy__drawing_options(__skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_flip_y() -> DrawingOptions {
    unsafe {
        let __skreturn = __sklib__option_flip_y();
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_flip_y_with_options(opts: DrawingOptions) -> DrawingOptions {
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_flip_y__drawing_options(__skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_line_width(width: i32) -> DrawingOptions {
    let __skparam__width = __skadapter__to_sklib_int(width);
    unsafe {
        let __skreturn = __sklib__option_line_width__int(__skparam__width);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_line_width_with_options(width: i32, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__width = __skadapter__to_sklib_int(width);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_line_width__int__drawing_options(__skparam__width, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_part_bmp(x: f64, y: f64, w: f64, h: f64) -> DrawingOptions {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__w = __skadapter__to_sklib_double(w);
    let __skparam__h = __skadapter__to_sklib_double(h);
    unsafe {
        let __skreturn = __sklib__option_part_bmp__double__double__double__double(__skparam__x, __skparam__y, __skparam__w, __skparam__h);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_part_bmp_with_options(x: f64, y: f64, w: f64, h: f64, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__w = __skadapter__to_sklib_double(w);
    let __skparam__h = __skadapter__to_sklib_double(h);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_part_bmp__double__double__double__double__drawing_options(__skparam__x, __skparam__y, __skparam__w, __skparam__h, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_part_bmp_from_rectangle(part: Rectangle) -> DrawingOptions {
    let __skparam__part = __skadapter__to_sklib_rectangle(part);
    unsafe {
        let __skreturn = __sklib__option_part_bmp__rectangle(__skparam__part);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_part_bmp_from_rectangle_with_options(part: Rectangle, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__part = __skadapter__to_sklib_rectangle(part);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_part_bmp__rectangle__drawing_options(__skparam__part, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_rotate_bmp(angle: f64) -> DrawingOptions {
    let __skparam__angle = __skadapter__to_sklib_double(angle);
    unsafe {
        let __skreturn = __sklib__option_rotate_bmp__double(__skparam__angle);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_rotate_bmp_with_anchor(angle: f64, anchor_x: f64, anchor_y: f64) -> DrawingOptions {
    let __skparam__angle = __skadapter__to_sklib_double(angle);
    let __skparam__anchor_x = __skadapter__to_sklib_double(anchor_x);
    let __skparam__anchor_y = __skadapter__to_sklib_double(anchor_y);
    unsafe {
        let __skreturn = __sklib__option_rotate_bmp__double__double__double(__skparam__angle, __skparam__anchor_x, __skparam__anchor_y);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_rotate_bmp_with_anchor_and_options(angle: f64, anchor_x: f64, anchor_y: f64, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__angle = __skadapter__to_sklib_double(angle);
    let __skparam__anchor_x = __skadapter__to_sklib_double(anchor_x);
    let __skparam__anchor_y = __skadapter__to_sklib_double(anchor_y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_rotate_bmp__double__double__double__drawing_options(__skparam__angle, __skparam__anchor_x, __skparam__anchor_y, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_rotate_bmp_with_options(angle: f64, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__angle = __skadapter__to_sklib_double(angle);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_rotate_bmp__double__drawing_options(__skparam__angle, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_scale_bmp(scale_x: f64, scale_y: f64) -> DrawingOptions {
    let __skparam__scale_x = __skadapter__to_sklib_double(scale_x);
    let __skparam__scale_y = __skadapter__to_sklib_double(scale_y);
    unsafe {
        let __skreturn = __sklib__option_scale_bmp__double__double(__skparam__scale_x, __skparam__scale_y);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_scale_bmp_with_options(scale_x: f64, scale_y: f64, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__scale_x = __skadapter__to_sklib_double(scale_x);
    let __skparam__scale_y = __skadapter__to_sklib_double(scale_y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_scale_bmp__double__double__drawing_options(__skparam__scale_x, __skparam__scale_y, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_to_screen() -> DrawingOptions {
    unsafe {
        let __skreturn = __sklib__option_to_screen();
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_to_screen_with_options(opts: DrawingOptions) -> DrawingOptions {
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_to_screen__drawing_options(__skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_to_world() -> DrawingOptions {
    unsafe {
        let __skreturn = __sklib__option_to_world();
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_to_world_with_options(opts: DrawingOptions) -> DrawingOptions {
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_to_world__drawing_options(__skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_with_animation(anim: Animation) -> DrawingOptions {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    unsafe {
        let __skreturn = __sklib__option_with_animation__animation(__skparam__anim);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_with_animation_with_options(anim: Animation, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__anim = __skadapter__to_sklib_animation(anim);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_with_animation__animation__drawing_options(__skparam__anim, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_with_bitmap_cell(cell: i32) -> DrawingOptions {
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    unsafe {
        let __skreturn = __sklib__option_with_bitmap_cell__int(__skparam__cell);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn option_with_bitmap_cell_with_options(cell: i32, opts: DrawingOptions) -> DrawingOptions {
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__option_with_bitmap_cell__int__drawing_options(__skparam__cell, __skparam__opts);
        __skadapter__to_drawing_options(__skreturn)
    }
}
pub fn draw_ellipse_within_rectangle(clr: Color, rect: Rectangle) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__draw_ellipse__color__rectangle(__skparam__clr, __skparam__rect);
    }
}
pub fn draw_ellipse_within_rectangle_with_options(clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_ellipse__color__rectangle__drawing_options(__skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn draw_ellipse(clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__draw_ellipse__color__double__double__double__double(__skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn draw_ellipse_with_options(clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_ellipse__color__double__double__double__double__drawing_options(__skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn draw_ellipse_on_bitmap_within_rectangle(destination: Bitmap, clr: Color, rect: Rectangle) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__draw_ellipse_on_bitmap__bitmap__color__rectangle(__skparam__destination, __skparam__clr, __skparam__rect);
    }
}
pub fn draw_ellipse_on_bitmap_within_rectangle_with_options(destination: Bitmap, clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_ellipse_on_bitmap__bitmap__color__rectangle__drawing_options(__skparam__destination, __skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn draw_ellipse_on_bitmap(destination: Bitmap, clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__draw_ellipse_on_bitmap__bitmap__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn draw_ellipse_on_bitmap_with_options(destination: Bitmap, clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_ellipse_on_bitmap__bitmap__color__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn draw_ellipse_on_window_within_rectangle(destination: Window, clr: Color, rect: Rectangle) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__draw_ellipse_on_window__window__color__rectangle(__skparam__destination, __skparam__clr, __skparam__rect);
    }
}
pub fn draw_ellipse_on_window_within_rectangle_with_options(destination: Window, clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_ellipse_on_window__window__color__rectangle__drawing_options(__skparam__destination, __skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn draw_ellipse_on_window(destination: Window, clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__draw_ellipse_on_window__window__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn draw_ellipse_on_window_with_options(destination: Window, clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_ellipse_on_window__window__color__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn fill_ellipse_within_rectangle(clr: Color, rect: Rectangle) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__fill_ellipse__color__rectangle(__skparam__clr, __skparam__rect);
    }
}
pub fn fill_ellipse_within_rectangle_with_options(clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_ellipse__color__rectangle__drawing_options(__skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn fill_ellipse(clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__fill_ellipse__color__double__double__double__double(__skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn fill_ellipse_with_options(clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_ellipse__color__double__double__double__double__drawing_options(__skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn fill_ellipse_on_bitmap_within_rectangle(destination: Bitmap, clr: Color, rect: Rectangle) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__fill_ellipse_on_bitmap__bitmap__color__rectangle(__skparam__destination, __skparam__clr, __skparam__rect);
    }
}
pub fn fill_ellipse_on_bitmap_within_rectangle_with_options(destination: Bitmap, clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_ellipse_on_bitmap__bitmap__color__rectangle__drawing_options(__skparam__destination, __skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn fill_ellipse_on_bitmap(destination: Bitmap, clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__fill_ellipse_on_bitmap__bitmap__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn fill_ellipse_on_bitmap_with_options(destination: Bitmap, clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_ellipse_on_bitmap__bitmap__color__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn fill_ellipse_on_window_within_rectangle(destination: Window, clr: Color, rect: Rectangle) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__fill_ellipse_on_window__window__color__rectangle(__skparam__destination, __skparam__clr, __skparam__rect);
    }
}
pub fn fill_ellipse_on_window_within_rectangle_with_options(destination: Window, clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_ellipse_on_window__window__color__rectangle__drawing_options(__skparam__destination, __skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn fill_ellipse_on_window(destination: Window, clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__fill_ellipse_on_window__window__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn fill_ellipse_on_window_with_options(destination: Window, clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_ellipse_on_window__window__color__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn cosine(degrees: f32) -> f32 {
    let __skparam__degrees = __skadapter__to_sklib_float(degrees);
    unsafe {
        let __skreturn = __sklib__cosine__float(__skparam__degrees);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sine(degrees: f32) -> f32 {
    let __skparam__degrees = __skadapter__to_sklib_float(degrees);
    unsafe {
        let __skreturn = __sklib__sine__float(__skparam__degrees);
        __skadapter__to_float(__skreturn)
    }
}
pub fn tangent(degrees: f32) -> f32 {
    let __skparam__degrees = __skadapter__to_sklib_float(degrees);
    unsafe {
        let __skreturn = __sklib__tangent__float(__skparam__degrees);
        __skadapter__to_float(__skreturn)
    }
}
pub fn clear_screen_to_white() {
    unsafe {
        __sklib__clear_screen();
    }
}
pub fn clear_screen(clr: Color) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        __sklib__clear_screen__color(__skparam__clr);
    }
}
pub fn display_details(index: u32) -> Display {
    let __skparam__index = __skadapter__to_sklib_unsigned_int(index);
    unsafe {
        let __skreturn = __sklib__display_details__unsigned_int(__skparam__index);
        __skadapter__to_display(__skreturn)
    }
}
pub fn display_height(disp: Display) -> i32 {
    let __skparam__disp = __skadapter__to_sklib_display(disp);
    unsafe {
        let __skreturn = __sklib__display_height__display(__skparam__disp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn display_name(disp: Display) -> String {
    let __skparam__disp = __skadapter__to_sklib_display(disp);
    unsafe {
        let __skreturn = __sklib__display_name__display(__skparam__disp);
        __skadapter__to_string(__skreturn)
    }
}
pub fn display_width(disp: Display) -> i32 {
    let __skparam__disp = __skadapter__to_sklib_display(disp);
    unsafe {
        let __skreturn = __sklib__display_width__display(__skparam__disp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn display_x(disp: Display) -> i32 {
    let __skparam__disp = __skadapter__to_sklib_display(disp);
    unsafe {
        let __skreturn = __sklib__display_x__display(__skparam__disp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn display_y(disp: Display) -> i32 {
    let __skparam__disp = __skadapter__to_sklib_display(disp);
    unsafe {
        let __skreturn = __sklib__display_y__display(__skparam__disp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn number_of_displays() -> i32 {
    unsafe {
        let __skreturn = __sklib__number_of_displays();
        __skadapter__to_int(__skreturn)
    }
}
pub fn refresh_screen() {
    unsafe {
        __sklib__refresh_screen();
    }
}
pub fn refresh_screen_with_target_fps(target_fps: u32) {
    let __skparam__target_fps = __skadapter__to_sklib_unsigned_int(target_fps);
    unsafe {
        __sklib__refresh_screen__unsigned_int(__skparam__target_fps);
    }
}
pub fn save_bitmap(bmp: Bitmap, basename: String) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__basename = __skadapter__to_sklib_string(basename);
    unsafe {
        __sklib__save_bitmap__bitmap__string_ref(__skparam__bmp, __skparam__basename);
    }
}
pub fn screen_height() -> i32 {
    unsafe {
        let __skreturn = __sklib__screen_height();
        __skadapter__to_int(__skreturn)
    }
}
pub fn screen_width() -> i32 {
    unsafe {
        let __skreturn = __sklib__screen_width();
        __skadapter__to_int(__skreturn)
    }
}
pub fn take_screenshot(basename: String) {
    let __skparam__basename = __skadapter__to_sklib_string(basename);
    unsafe {
        __sklib__take_screenshot__string_ref(__skparam__basename);
    }
}
pub fn take_screenshot_of_window(wind: Window, basename: String) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__basename = __skadapter__to_sklib_string(basename);
    unsafe {
        __sklib__take_screenshot__window__string_ref(__skparam__wind, __skparam__basename);
    }
}
pub fn bitmap_bounding_circle(bmp: Bitmap, pt: Point2D) -> Circle {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__bitmap_bounding_circle__bitmap__point_2d_ref(__skparam__bmp, __skparam__pt);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn bitmap_bounding_rectangle(bmp: Bitmap) -> Rectangle {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_bounding_rectangle__bitmap(__skparam__bmp);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn bitmap_bounding_rectangle_at_location(bmp: Bitmap, x: f64, y: f64) -> Rectangle {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__bitmap_bounding_rectangle__bitmap__double__double(__skparam__bmp, __skparam__x, __skparam__y);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn bitmap_cell_center(bmp: Bitmap) -> Point2D {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_center__bitmap(__skparam__bmp);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn bitmap_cell_circle(bmp: Bitmap, x: f64, y: f64) -> Circle {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_circle__bitmap__double__double(__skparam__bmp, __skparam__x, __skparam__y);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn bitmap_cell_circle_at_point(bmp: Bitmap, pt: Point2D) -> Circle {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_circle__bitmap__point_2d(__skparam__bmp, __skparam__pt);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn bitmap_cell_circle_at_point_with_scale(bmp: Bitmap, pt: Point2D, scale: f64) -> Circle {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__scale = __skadapter__to_sklib_double(scale);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_circle__bitmap__point_2d__double(__skparam__bmp, __skparam__pt, __skparam__scale);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn bitmap_cell_columns(bmp: Bitmap) -> i32 {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_columns__bitmap(__skparam__bmp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn bitmap_cell_count(bmp: Bitmap) -> i32 {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_count__bitmap(__skparam__bmp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn bitmap_cell_height(bmp: Bitmap) -> i32 {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_height__bitmap(__skparam__bmp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn bitmap_cell_offset(src: Bitmap, cell: i32) -> Vector2D {
    let __skparam__src = __skadapter__to_sklib_bitmap(src);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_offset__bitmap__int(__skparam__src, __skparam__cell);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn bitmap_cell_rectangle(src: Bitmap) -> Rectangle {
    let __skparam__src = __skadapter__to_sklib_bitmap(src);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_rectangle__bitmap(__skparam__src);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn bitmap_cell_rectangle_at_point(src: Bitmap, pt: Point2D) -> Rectangle {
    let __skparam__src = __skadapter__to_sklib_bitmap(src);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_rectangle__bitmap__point_2d_ref(__skparam__src, __skparam__pt);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn bitmap_cell_rows(bmp: Bitmap) -> i32 {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_rows__bitmap(__skparam__bmp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn bitmap_cell_width(bmp: Bitmap) -> i32 {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_cell_width__bitmap(__skparam__bmp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn bitmap_center(bmp: Bitmap) -> Point2D {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_center__bitmap(__skparam__bmp);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn bitmap_filename(bmp: Bitmap) -> String {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_filename__bitmap(__skparam__bmp);
        __skadapter__to_string(__skreturn)
    }
}
pub fn bitmap_height(bmp: Bitmap) -> i32 {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_height__bitmap(__skparam__bmp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn bitmap_height_of_bitmap_named(name: String) -> i32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__bitmap_height__string(__skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn bitmap_name(bmp: Bitmap) -> String {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_name__bitmap(__skparam__bmp);
        __skadapter__to_string(__skreturn)
    }
}
pub fn bitmap_named(name: String) -> Bitmap {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__bitmap_named__string(__skparam__name);
        __skadapter__to_bitmap(__skreturn)
    }
}
pub fn bitmap_rectangle_of_cell(src: Bitmap, cell: i32) -> Rectangle {
    let __skparam__src = __skadapter__to_sklib_bitmap(src);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    unsafe {
        let __skreturn = __sklib__bitmap_rectangle_of_cell__bitmap__int(__skparam__src, __skparam__cell);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn bitmap_set_cell_details(bmp: Bitmap, width: i32, height: i32, columns: i32, rows: i32, count: i32) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__width = __skadapter__to_sklib_int(width);
    let __skparam__height = __skadapter__to_sklib_int(height);
    let __skparam__columns = __skadapter__to_sklib_int(columns);
    let __skparam__rows = __skadapter__to_sklib_int(rows);
    let __skparam__count = __skadapter__to_sklib_int(count);
    unsafe {
        __sklib__bitmap_set_cell_details__bitmap__int__int__int__int__int(__skparam__bmp, __skparam__width, __skparam__height, __skparam__columns, __skparam__rows, __skparam__count);
    }
}
pub fn bitmap_valid(bmp: Bitmap) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_valid__bitmap(__skparam__bmp);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_width(bmp: Bitmap) -> i32 {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_width__bitmap(__skparam__bmp);
        __skadapter__to_int(__skreturn)
    }
}
pub fn bitmap_width_of_bitmap_named(name: String) -> i32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__bitmap_width__string(__skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn clear_bitmap(bmp: Bitmap, clr: Color) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        __sklib__clear_bitmap__bitmap__color(__skparam__bmp, __skparam__clr);
    }
}
pub fn clear_bitmap_named(name: String, clr: Color) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        __sklib__clear_bitmap__string__color(__skparam__name, __skparam__clr);
    }
}
pub fn create_bitmap(name: String, width: i32, height: i32) -> Bitmap {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__width = __skadapter__to_sklib_int(width);
    let __skparam__height = __skadapter__to_sklib_int(height);
    unsafe {
        let __skreturn = __sklib__create_bitmap__string__int__int(__skparam__name, __skparam__width, __skparam__height);
        __skadapter__to_bitmap(__skreturn)
    }
}
pub fn draw_bitmap(bmp: Bitmap, x: f64, y: f64) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_bitmap__bitmap__double__double(__skparam__bmp, __skparam__x, __skparam__y);
    }
}
pub fn draw_bitmap_with_options(bmp: Bitmap, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_bitmap__bitmap__double__double__drawing_options(__skparam__bmp, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_bitmap_named(name: String, x: f64, y: f64) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_bitmap__string__double__double(__skparam__name, __skparam__x, __skparam__y);
    }
}
pub fn draw_bitmap_named_with_options(name: String, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_bitmap__string__double__double__drawing_options(__skparam__name, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_bitmap_on_bitmap_on_bitmap(destination: Bitmap, bmp: Bitmap, x: f64, y: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_bitmap_on_bitmap__bitmap__bitmap__double__double(__skparam__destination, __skparam__bmp, __skparam__x, __skparam__y);
    }
}
pub fn draw_bitmap_on_bitmap_on_bitmap_with_options(destination: Bitmap, bmp: Bitmap, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_bitmap_on_bitmap__bitmap__bitmap__double__double__drawing_options(__skparam__destination, __skparam__bmp, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_bitmap_on_window(destination: Window, bmp: Bitmap, x: f64, y: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_bitmap_on_window__window__bitmap__double__double(__skparam__destination, __skparam__bmp, __skparam__x, __skparam__y);
    }
}
pub fn draw_bitmap_on_window_with_options(destination: Window, bmp: Bitmap, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_bitmap_on_window__window__bitmap__double__double__drawing_options(__skparam__destination, __skparam__bmp, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn free_all_bitmaps() {
    unsafe {
        __sklib__free_all_bitmaps();
    }
}
pub fn free_bitmap(to_delete: Bitmap) {
    let __skparam__to_delete = __skadapter__to_sklib_bitmap(to_delete);
    unsafe {
        __sklib__free_bitmap__bitmap(__skparam__to_delete);
    }
}
pub fn has_bitmap(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_bitmap__string(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn load_bitmap(name: String, filename: String) -> Bitmap {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        let __skreturn = __sklib__load_bitmap__string__string(__skparam__name, __skparam__filename);
        __skadapter__to_bitmap(__skreturn)
    }
}
pub fn pixel_drawn_at_point_pt(bmp: Bitmap, pt: Point2D) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__pixel_drawn_at_point__bitmap__point_2d_ref(__skparam__bmp, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn pixel_drawn_at_point(bmp: Bitmap, x: f64, y: f64) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__pixel_drawn_at_point__bitmap__double__double(__skparam__bmp, __skparam__x, __skparam__y);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn pixel_drawn_at_point_in_cell_pt(bmp: Bitmap, cell: i32, pt: Point2D) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__pixel_drawn_at_point__bitmap__int__point_2d_ref(__skparam__bmp, __skparam__cell, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn pixel_drawn_at_point_in_cell(bmp: Bitmap, cell: i32, x: f64, y: f64) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__cell = __skadapter__to_sklib_int(cell);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__pixel_drawn_at_point__bitmap__int__double__double(__skparam__bmp, __skparam__cell, __skparam__x, __skparam__y);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn setup_collision_mask(bmp: Bitmap) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        __sklib__setup_collision_mask__bitmap(__skparam__bmp);
    }
}
pub fn process_events() {
    unsafe {
        __sklib__process_events();
    }
}
pub fn quit_requested() -> bool {
    unsafe {
        let __skreturn = __sklib__quit_requested();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn reset_quit() {
    unsafe {
        __sklib__reset_quit();
    }
}
pub fn add_column(width: i32) {
    let __skparam__width = __skadapter__to_sklib_int(width);
    unsafe {
        __sklib__add_column__int(__skparam__width);
    }
}
pub fn add_column_relative(width: f64) {
    let __skparam__width = __skadapter__to_sklib_double(width);
    unsafe {
        __sklib__add_column_relative__double(__skparam__width);
    }
}
pub fn bitmap_button(bmp: Bitmap) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_button__bitmap(__skparam__bmp);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_button_at_position(bmp: Bitmap, rect: Rectangle) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__bitmap_button__bitmap__rectangle_ref(__skparam__bmp, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_button_at_position_with_options(bmp: Bitmap, rect: Rectangle, opts: DrawingOptions) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__bitmap_button__bitmap__rectangle_ref__drawing_options(__skparam__bmp, __skparam__rect, __skparam__opts);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_button_with_options(bmp: Bitmap, opts: DrawingOptions) -> bool {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__bitmap_button__bitmap__drawing_options(__skparam__bmp, __skparam__opts);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_button_labeled(label_text: String, bmp: Bitmap) -> bool {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__bitmap_button__string_ref__bitmap(__skparam__label_text, __skparam__bmp);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn bitmap_button_labeled_with_options(label_text: String, bmp: Bitmap, opts: DrawingOptions) -> bool {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        let __skreturn = __sklib__bitmap_button__string_ref__bitmap__drawing_options(__skparam__label_text, __skparam__bmp, __skparam__opts);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn button_at_position(text: String, rect: Rectangle) -> bool {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__button__string_ref__rectangle_ref(__skparam__text, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn button(text: String) -> bool {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__button__string_ref(__skparam__text);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn button_labeled(label_text: String, text: String) -> bool {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        let __skreturn = __sklib__button__string_ref__string_ref(__skparam__label_text, __skparam__text);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn checkbox_at_position(text: String, value: bool, rect: Rectangle) -> bool {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__value = __skadapter__to_sklib_bool(value);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__checkbox__string_ref__bool_ref__rectangle_ref(__skparam__text, __skparam__value, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn checkbox(text: String, value: bool) -> bool {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__value = __skadapter__to_sklib_bool(value);
    unsafe {
        let __skreturn = __sklib__checkbox__string_ref__bool_ref(__skparam__text, __skparam__value);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn checkbox_labeled(label_text: String, text: String, value: bool) -> bool {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__value = __skadapter__to_sklib_bool(value);
    unsafe {
        let __skreturn = __sklib__checkbox__string_ref__string_ref__bool_ref(__skparam__label_text, __skparam__text, __skparam__value);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn color_slider_at_position(clr: Color, rect: Rectangle) -> Color {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__color_slider__color_ref__rectangle_ref(__skparam__clr, __skparam__rect);
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_slider(clr: Color) -> Color {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        let __skreturn = __sklib__color_slider__color_ref(__skparam__clr);
        __skadapter__to_color(__skreturn)
    }
}
pub fn color_slider_labeled(label_text: String, clr: Color) -> Color {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        let __skreturn = __sklib__color_slider__string_ref__color_ref(__skparam__label_text, __skparam__clr);
        __skadapter__to_color(__skreturn)
    }
}
pub fn disable_interface() {
    unsafe {
        __sklib__disable_interface();
    }
}
pub fn draw_interface() {
    unsafe {
        __sklib__draw_interface();
    }
}
pub fn enable_interface() {
    unsafe {
        __sklib__enable_interface();
    }
}
pub fn end_inset(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__end_inset__string_ref(__skparam__name);
    }
}
pub fn end_panel(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__end_panel__string_ref(__skparam__name);
    }
}
pub fn end_popup(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__end_popup__string_ref(__skparam__name);
    }
}
pub fn end_treenode(label_text: String) {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    unsafe {
        __sklib__end_treenode__string_ref(__skparam__label_text);
    }
}
pub fn enter_column() {
    unsafe {
        __sklib__enter_column();
    }
}
pub fn get_interface_label_width() -> i32 {
    unsafe {
        let __skreturn = __sklib__get_interface_label_width();
        __skadapter__to_int(__skreturn)
    }
}
pub fn header(label_text: String) -> bool {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    unsafe {
        let __skreturn = __sklib__header__string_ref(__skparam__label_text);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn hsb_color_slider_at_position(clr: Color, rect: Rectangle) -> Color {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__hsb_color_slider__color_ref__rectangle_ref(__skparam__clr, __skparam__rect);
        __skadapter__to_color(__skreturn)
    }
}
pub fn hsb_color_slider(clr: Color) -> Color {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        let __skreturn = __sklib__hsb_color_slider__color_ref(__skparam__clr);
        __skadapter__to_color(__skreturn)
    }
}
pub fn hsb_color_slider_labeled(label_text: String, clr: Color) -> Color {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        let __skreturn = __sklib__hsb_color_slider__string_ref__color_ref(__skparam__label_text, __skparam__clr);
        __skadapter__to_color(__skreturn)
    }
}
pub fn interface_enabled() -> bool {
    unsafe {
        let __skreturn = __sklib__interface_enabled();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn interface_style_panel(initial_rectangle: Rectangle) {
    let __skparam__initial_rectangle = __skadapter__to_sklib_rectangle(initial_rectangle);
    unsafe {
        __sklib__interface_style_panel__rectangle_ref(__skparam__initial_rectangle);
    }
}
pub fn label_element(text: String) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        __sklib__label_element__string_ref(__skparam__text);
    }
}
pub fn label_element_at_position(text: String, rect: Rectangle) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__label_element__string_ref__rectangle_ref(__skparam__text, __skparam__rect);
    }
}
pub fn last_element_changed() -> bool {
    unsafe {
        let __skreturn = __sklib__last_element_changed();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn last_element_confirmed() -> bool {
    unsafe {
        let __skreturn = __sklib__last_element_confirmed();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn leave_column() {
    unsafe {
        __sklib__leave_column();
    }
}
pub fn number_box_at_position(value: f32, step: f32, rect: Rectangle) -> f32 {
    let __skparam__value = __skadapter__to_sklib_float(value);
    let __skparam__step = __skadapter__to_sklib_float(step);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__number_box__float_ref__float__rectangle_ref(__skparam__value, __skparam__step, __skparam__rect);
        __skadapter__to_float(__skreturn)
    }
}
pub fn number_box(value: f32, step: f32) -> f32 {
    let __skparam__value = __skadapter__to_sklib_float(value);
    let __skparam__step = __skadapter__to_sklib_float(step);
    unsafe {
        let __skreturn = __sklib__number_box__float_ref__float(__skparam__value, __skparam__step);
        __skadapter__to_float(__skreturn)
    }
}
pub fn number_box_labeled(label_text: String, value: f32, step: f32) -> f32 {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__value = __skadapter__to_sklib_float(value);
    let __skparam__step = __skadapter__to_sklib_float(step);
    unsafe {
        let __skreturn = __sklib__number_box__string_ref__float_ref__float(__skparam__label_text, __skparam__value, __skparam__step);
        __skadapter__to_float(__skreturn)
    }
}
pub fn open_popup(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__open_popup__string_ref(__skparam__name);
    }
}
pub fn paragraph(text: String) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        __sklib__paragraph__string_ref(__skparam__text);
    }
}
pub fn paragraph_at_position(text: String, rect: Rectangle) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__paragraph__string_ref__rectangle_ref(__skparam__text, __skparam__rect);
    }
}
pub fn reset_layout() {
    unsafe {
        __sklib__reset_layout();
    }
}
pub fn set_interface_accent_color(clr: Color, contrast: f32) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__contrast = __skadapter__to_sklib_float(contrast);
    unsafe {
        __sklib__set_interface_accent_color__color__float(__skparam__clr, __skparam__contrast);
    }
}
pub fn set_interface_border_color(clr: Color) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        __sklib__set_interface_border_color__color(__skparam__clr);
    }
}
pub fn set_interface_colors_auto(main_clr: Color, accent_clr: Color, contrast: f32, accent_contrast: f32, border_contrast: f32) {
    let __skparam__main_clr = __skadapter__to_sklib_color(main_clr);
    let __skparam__accent_clr = __skadapter__to_sklib_color(accent_clr);
    let __skparam__contrast = __skadapter__to_sklib_float(contrast);
    let __skparam__accent_contrast = __skadapter__to_sklib_float(accent_contrast);
    let __skparam__border_contrast = __skadapter__to_sklib_float(border_contrast);
    unsafe {
        __sklib__set_interface_colors_auto__color__color__float__float__float(__skparam__main_clr, __skparam__accent_clr, __skparam__contrast, __skparam__accent_contrast, __skparam__border_contrast);
    }
}
pub fn set_interface_element_color(clr: Color, contrast: f32) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__contrast = __skadapter__to_sklib_float(contrast);
    unsafe {
        __sklib__set_interface_element_color__color__float(__skparam__clr, __skparam__contrast);
    }
}
pub fn set_interface_element_shadows(radius: i32, clr: Color, offset: Point2D) {
    let __skparam__radius = __skadapter__to_sklib_int(radius);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__offset = __skadapter__to_sklib_point_2d(offset);
    unsafe {
        __sklib__set_interface_element_shadows__int__color__point_2d(__skparam__radius, __skparam__clr, __skparam__offset);
    }
}
pub fn set_interface_font_font_as_string(fnt: String) {
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    unsafe {
        __sklib__set_interface_font__string_ref(__skparam__fnt);
    }
}
pub fn set_interface_font(fnt: Font) {
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    unsafe {
        __sklib__set_interface_font__font(__skparam__fnt);
    }
}
pub fn set_interface_font_size(size: i32) {
    let __skparam__size = __skadapter__to_sklib_int(size);
    unsafe {
        __sklib__set_interface_font_size__int(__skparam__size);
    }
}
pub fn set_interface_label_width(width: i32) {
    let __skparam__width = __skadapter__to_sklib_int(width);
    unsafe {
        __sklib__set_interface_label_width__int(__skparam__width);
    }
}
pub fn set_interface_panel_shadows(radius: i32, clr: Color, offset: Point2D) {
    let __skparam__radius = __skadapter__to_sklib_int(radius);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__offset = __skadapter__to_sklib_point_2d(offset);
    unsafe {
        __sklib__set_interface_panel_shadows__int__color__point_2d(__skparam__radius, __skparam__clr, __skparam__offset);
    }
}
pub fn set_interface_root_text_color(clr: Color) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        __sklib__set_interface_root_text_color__color(__skparam__clr);
    }
}
pub fn set_interface_shadows(radius: i32, clr: Color, offset: Point2D) {
    let __skparam__radius = __skadapter__to_sklib_int(radius);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__offset = __skadapter__to_sklib_point_2d(offset);
    unsafe {
        __sklib__set_interface_shadows__int__color__point_2d(__skparam__radius, __skparam__clr, __skparam__offset);
    }
}
pub fn set_interface_spacing(spacing: i32, padding: i32) {
    let __skparam__spacing = __skadapter__to_sklib_int(spacing);
    let __skparam__padding = __skadapter__to_sklib_int(padding);
    unsafe {
        __sklib__set_interface_spacing__int__int(__skparam__spacing, __skparam__padding);
    }
}
pub fn set_interface_style(style: InterfaceStyle) {
    let __skparam__style = __skadapter__to_sklib_interface_style(style);
    unsafe {
        __sklib__set_interface_style__interface_style(__skparam__style);
    }
}
pub fn set_interface_style_with_color(style: InterfaceStyle, clr: Color) {
    let __skparam__style = __skadapter__to_sklib_interface_style(style);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        __sklib__set_interface_style__interface_style__color(__skparam__style, __skparam__clr);
    }
}
pub fn set_interface_text_color(clr: Color) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        __sklib__set_interface_text_color__color(__skparam__clr);
    }
}
pub fn set_layout_height(height: i32) {
    let __skparam__height = __skadapter__to_sklib_int(height);
    unsafe {
        __sklib__set_layout_height__int(__skparam__height);
    }
}
pub fn single_line_layout() {
    unsafe {
        __sklib__single_line_layout();
    }
}
pub fn slider_at_position(value: f32, min_value: f32, max_value: f32, rect: Rectangle) -> f32 {
    let __skparam__value = __skadapter__to_sklib_float(value);
    let __skparam__min_value = __skadapter__to_sklib_float(min_value);
    let __skparam__max_value = __skadapter__to_sklib_float(max_value);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__slider__float_ref__float__float__rectangle_ref(__skparam__value, __skparam__min_value, __skparam__max_value, __skparam__rect);
        __skadapter__to_float(__skreturn)
    }
}
pub fn slider(value: f32, min_value: f32, max_value: f32) -> f32 {
    let __skparam__value = __skadapter__to_sklib_float(value);
    let __skparam__min_value = __skadapter__to_sklib_float(min_value);
    let __skparam__max_value = __skadapter__to_sklib_float(max_value);
    unsafe {
        let __skreturn = __sklib__slider__float_ref__float__float(__skparam__value, __skparam__min_value, __skparam__max_value);
        __skadapter__to_float(__skreturn)
    }
}
pub fn slider_labeled(label_text: String, value: f32, min_value: f32, max_value: f32) -> f32 {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__value = __skadapter__to_sklib_float(value);
    let __skparam__min_value = __skadapter__to_sklib_float(min_value);
    let __skparam__max_value = __skadapter__to_sklib_float(max_value);
    unsafe {
        let __skreturn = __sklib__slider__string_ref__float_ref__float__float(__skparam__label_text, __skparam__value, __skparam__min_value, __skparam__max_value);
        __skadapter__to_float(__skreturn)
    }
}
pub fn split_into_columns(count: i32) {
    let __skparam__count = __skadapter__to_sklib_int(count);
    unsafe {
        __sklib__split_into_columns__int(__skparam__count);
    }
}
pub fn split_into_columns_with_last_width(count: i32, last_width: i32) {
    let __skparam__count = __skadapter__to_sklib_int(count);
    let __skparam__last_width = __skadapter__to_sklib_int(last_width);
    unsafe {
        __sklib__split_into_columns__int__int(__skparam__count, __skparam__last_width);
    }
}
pub fn split_into_columns_relative_with_last_width(count: i32, last_width: f64) {
    let __skparam__count = __skadapter__to_sklib_int(count);
    let __skparam__last_width = __skadapter__to_sklib_double(last_width);
    unsafe {
        __sklib__split_into_columns_relative__int__double(__skparam__count, __skparam__last_width);
    }
}
pub fn start_custom_layout() {
    unsafe {
        __sklib__start_custom_layout();
    }
}
pub fn start_inset_at_position(name: String, rect: Rectangle) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__start_inset__string_ref__rectangle_ref(__skparam__name, __skparam__rect);
    }
}
pub fn start_inset(name: String, height: i32) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__height = __skadapter__to_sklib_int(height);
    unsafe {
        __sklib__start_inset__string_ref__int(__skparam__name, __skparam__height);
    }
}
pub fn start_panel(name: String, initial_rectangle: Rectangle) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__initial_rectangle = __skadapter__to_sklib_rectangle(initial_rectangle);
    unsafe {
        let __skreturn = __sklib__start_panel__string_ref__rectangle(__skparam__name, __skparam__initial_rectangle);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn start_popup(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__start_popup__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn start_treenode(label_text: String) -> bool {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    unsafe {
        let __skreturn = __sklib__start_treenode__string_ref(__skparam__label_text);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn text_box(value: String) -> String {
    let __skparam__value = __skadapter__to_sklib_string(value);
    unsafe {
        let __skreturn = __sklib__text_box__string_ref(__skparam__value);
        __skadapter__to_string(__skreturn)
    }
}
pub fn text_box_at_position(value: String, rect: Rectangle) -> String {
    let __skparam__value = __skadapter__to_sklib_string(value);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__text_box__string_ref__rectangle_ref(__skparam__value, __skparam__rect);
        __skadapter__to_string(__skreturn)
    }
}
pub fn text_box_labeled(label_text: String, value: String) -> String {
    let __skparam__label_text = __skadapter__to_sklib_string(label_text);
    let __skparam__value = __skadapter__to_sklib_string(value);
    unsafe {
        let __skreturn = __sklib__text_box__string_ref__string_ref(__skparam__label_text, __skparam__value);
        __skadapter__to_string(__skreturn)
    }
}
pub fn create_json() -> Json {
    unsafe {
        let __skreturn = __sklib__create_json();
        __skadapter__to_json(__skreturn)
    }
}
pub fn create_json_from_string(json_string: String) -> Json {
    let __skparam__json_string = __skadapter__to_sklib_string(json_string);
    unsafe {
        let __skreturn = __sklib__create_json__string(__skparam__json_string);
        __skadapter__to_json(__skreturn)
    }
}
pub fn free_all_json() {
    unsafe {
        __sklib__free_all_json();
    }
}
pub fn free_json(j: Json) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    unsafe {
        __sklib__free_json__json(__skparam__j);
    }
}
pub fn json_count_keys(j: Json) -> i32 {
    let __skparam__j = __skadapter__to_sklib_json(j);
    unsafe {
        let __skreturn = __sklib__json_count_keys__json(__skparam__j);
        __skadapter__to_int(__skreturn)
    }
}
pub fn json_from_color(clr: Color) -> Json {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        let __skreturn = __sklib__json_from_color__color(__skparam__clr);
        __skadapter__to_json(__skreturn)
    }
}
pub fn json_from_file(filename: String) -> Json {
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        let __skreturn = __sklib__json_from_file__string_ref(__skparam__filename);
        __skadapter__to_json(__skreturn)
    }
}
pub fn json_from_string(j_string: String) -> Json {
    let __skparam__j_string = __skadapter__to_sklib_string(j_string);
    unsafe {
        let __skreturn = __sklib__json_from_string__string_ref(__skparam__j_string);
        __skadapter__to_json(__skreturn)
    }
}
pub fn json_has_key(j: Json, key: String) -> bool {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    unsafe {
        let __skreturn = __sklib__json_has_key__json__string(__skparam__j, __skparam__key);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn json_read_array_of_double(j: Json, key: String, out_result: &mut Vec<f64>) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let mut __skparam__out_result = __skadapter__to_sklib_vector_double((*out_result).clone());
    unsafe {
        __sklib__json_read_array__json__string__vector_double_ref(__skparam__j, __skparam__key, &mut __skparam__out_result);
        *out_result = __skadapter__to_vector_double(__skparam__out_result);
    }
}
pub fn json_read_array_of_json(j: Json, key: String, out_result: &mut Vec<Json>) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let mut __skparam__out_result = __skadapter__to_sklib_vector_json((*out_result).clone());
    unsafe {
        __sklib__json_read_array__json__string__vector_json_ref(__skparam__j, __skparam__key, &mut __skparam__out_result);
        *out_result = __skadapter__to_vector_json(__skparam__out_result);
    }
}
pub fn json_read_array_of_string(j: Json, key: String, out_result: &mut Vec<String>) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let mut __skparam__out_result = __skadapter__to_sklib_vector_string((*out_result).clone());
    unsafe {
        __sklib__json_read_array__json__string__vector_string_ref(__skparam__j, __skparam__key, &mut __skparam__out_result);
        *out_result = __skadapter__to_vector_string(__skparam__out_result);
    }
}
pub fn json_read_array_of_bool(j: Json, key: String, out_result: &mut Vec<bool>) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let mut __skparam__out_result = __skadapter__to_sklib_vector_bool((*out_result).clone());
    unsafe {
        __sklib__json_read_array__json__string__vector_bool_ref(__skparam__j, __skparam__key, &mut __skparam__out_result);
        *out_result = __skadapter__to_vector_bool(__skparam__out_result);
    }
}
pub fn json_read_bool(j: Json, key: String) -> bool {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    unsafe {
        let __skreturn = __sklib__json_read_bool__json__string(__skparam__j, __skparam__key);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn json_read_number(j: Json, key: String) -> f32 {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    unsafe {
        let __skreturn = __sklib__json_read_number__json__string(__skparam__j, __skparam__key);
        __skadapter__to_float(__skreturn)
    }
}
pub fn json_read_number_as_double(j: Json, key: String) -> f64 {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    unsafe {
        let __skreturn = __sklib__json_read_number_as_double__json__string(__skparam__j, __skparam__key);
        __skadapter__to_double(__skreturn)
    }
}
pub fn json_read_number_as_int(j: Json, key: String) -> i32 {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    unsafe {
        let __skreturn = __sklib__json_read_number_as_int__json__string(__skparam__j, __skparam__key);
        __skadapter__to_int(__skreturn)
    }
}
pub fn json_read_object(j: Json, key: String) -> Json {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    unsafe {
        let __skreturn = __sklib__json_read_object__json__string(__skparam__j, __skparam__key);
        __skadapter__to_json(__skreturn)
    }
}
pub fn json_read_string(j: Json, key: String) -> String {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    unsafe {
        let __skreturn = __sklib__json_read_string__json__string(__skparam__j, __skparam__key);
        __skadapter__to_string(__skreturn)
    }
}
pub fn json_set_array_of_string(j: Json, key: String, value: Vec<String>) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_vector_string(value);
    unsafe {
        __sklib__json_set_array__json__string__vector_string(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_set_array_of_double(j: Json, key: String, value: Vec<f64>) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_vector_double(value);
    unsafe {
        __sklib__json_set_array__json__string__vector_double(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_set_array_of_bool(j: Json, key: String, value: Vec<bool>) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_vector_bool(value);
    unsafe {
        __sklib__json_set_array__json__string__vector_bool(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_set_array_of_json(j: Json, key: String, value: Vec<Json>) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_vector_json(value);
    unsafe {
        __sklib__json_set_array__json__string__vector_json(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_set_bool(j: Json, key: String, value: bool) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_bool(value);
    unsafe {
        __sklib__json_set_bool__json__string__bool(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_set_number_integer(j: Json, key: String, value: i32) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_int(value);
    unsafe {
        __sklib__json_set_number__json__string__int(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_set_number_double(j: Json, key: String, value: f64) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_double(value);
    unsafe {
        __sklib__json_set_number__json__string__double(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_set_number_float(j: Json, key: String, value: f32) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__json_set_number__json__string__float(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_set_object(j: Json, key: String, obj: Json) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__obj = __skadapter__to_sklib_json(obj);
    unsafe {
        __sklib__json_set_object__json__string__json(__skparam__j, __skparam__key, __skparam__obj);
    }
}
pub fn json_set_string(j: Json, key: String, value: String) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__key = __skadapter__to_sklib_string(key);
    let __skparam__value = __skadapter__to_sklib_string(value);
    unsafe {
        __sklib__json_set_string__json__string__string(__skparam__j, __skparam__key, __skparam__value);
    }
}
pub fn json_to_color(j: Json) -> Color {
    let __skparam__j = __skadapter__to_sklib_json(j);
    unsafe {
        let __skreturn = __sklib__json_to_color__json(__skparam__j);
        __skadapter__to_color(__skreturn)
    }
}
pub fn json_to_file(j: Json, filename: String) {
    let __skparam__j = __skadapter__to_sklib_json(j);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        __sklib__json_to_file__json__string_ref(__skparam__j, __skparam__filename);
    }
}
pub fn json_to_string(j: Json) -> String {
    let __skparam__j = __skadapter__to_sklib_json(j);
    unsafe {
        let __skreturn = __sklib__json_to_string__json(__skparam__j);
        __skadapter__to_string(__skreturn)
    }
}
pub fn any_key_pressed() -> bool {
    unsafe {
        let __skreturn = __sklib__any_key_pressed();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn deregister_callback_on_key_down(callback: KeyCallback) {
    let __skparam__callback = __skadapter__to_sklib_key_callback(callback);
    unsafe {
        __sklib__deregister_callback_on_key_down__key_callback_ptr(__skparam__callback);
    }
}
pub fn deregister_callback_on_key_typed(callback: KeyCallback) {
    let __skparam__callback = __skadapter__to_sklib_key_callback(callback);
    unsafe {
        __sklib__deregister_callback_on_key_typed__key_callback_ptr(__skparam__callback);
    }
}
pub fn deregister_callback_on_key_up(callback: KeyCallback) {
    let __skparam__callback = __skadapter__to_sklib_key_callback(callback);
    unsafe {
        __sklib__deregister_callback_on_key_up__key_callback_ptr(__skparam__callback);
    }
}
pub fn key_down(key: KeyCode) -> bool {
    let __skparam__key = __skadapter__to_sklib_key_code(key);
    unsafe {
        let __skreturn = __sklib__key_down__key_code(__skparam__key);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn key_name(key: KeyCode) -> String {
    let __skparam__key = __skadapter__to_sklib_key_code(key);
    unsafe {
        let __skreturn = __sklib__key_name__key_code(__skparam__key);
        __skadapter__to_string(__skreturn)
    }
}
pub fn key_released(key: KeyCode) -> bool {
    let __skparam__key = __skadapter__to_sklib_key_code(key);
    unsafe {
        let __skreturn = __sklib__key_released__key_code(__skparam__key);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn key_typed(key: KeyCode) -> bool {
    let __skparam__key = __skadapter__to_sklib_key_code(key);
    unsafe {
        let __skreturn = __sklib__key_typed__key_code(__skparam__key);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn key_up(key: KeyCode) -> bool {
    let __skparam__key = __skadapter__to_sklib_key_code(key);
    unsafe {
        let __skreturn = __sklib__key_up__key_code(__skparam__key);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn register_callback_on_key_down(callback: KeyCallback) {
    let __skparam__callback = __skadapter__to_sklib_key_callback(callback);
    unsafe {
        __sklib__register_callback_on_key_down__key_callback_ptr(__skparam__callback);
    }
}
pub fn register_callback_on_key_typed(callback: KeyCallback) {
    let __skparam__callback = __skadapter__to_sklib_key_callback(callback);
    unsafe {
        __sklib__register_callback_on_key_typed__key_callback_ptr(__skparam__callback);
    }
}
pub fn register_callback_on_key_up(callback: KeyCallback) {
    let __skparam__callback = __skadapter__to_sklib_key_callback(callback);
    unsafe {
        __sklib__register_callback_on_key_up__key_callback_ptr(__skparam__callback);
    }
}
pub fn draw_line_record(clr: Color, l: Line) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        __sklib__draw_line__color__line_ref(__skparam__clr, __skparam__l);
    }
}
pub fn draw_line_record_with_options(clr: Color, l: Line, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__l = __skadapter__to_sklib_line(l);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line__color__line_ref__drawing_options(__skparam__clr, __skparam__l, __skparam__opts);
    }
}
pub fn draw_line_point_to_point(clr: Color, from_pt: Point2D, to_pt: Point2D) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__to_pt = __skadapter__to_sklib_point_2d(to_pt);
    unsafe {
        __sklib__draw_line__color__point_2d_ref__point_2d_ref(__skparam__clr, __skparam__from_pt, __skparam__to_pt);
    }
}
pub fn draw_line_point_to_point_with_options(clr: Color, from_pt: Point2D, to_pt: Point2D, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__to_pt = __skadapter__to_sklib_point_2d(to_pt);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line__color__point_2d_ref__point_2d_ref__drawing_options_ref(__skparam__clr, __skparam__from_pt, __skparam__to_pt, __skparam__opts);
    }
}
pub fn draw_line(clr: Color, x1: f64, y1: f64, x2: f64, y2: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    unsafe {
        __sklib__draw_line__color__double__double__double__double(__skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2);
    }
}
pub fn draw_line_with_options(clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line__color__double__double__double__double__drawing_options_ref(__skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__opts);
    }
}
pub fn draw_line_on_bitmap_record(destination: Bitmap, clr: Color, l: Line) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        __sklib__draw_line_on_bitmap__bitmap__color__line_ref(__skparam__destination, __skparam__clr, __skparam__l);
    }
}
pub fn draw_line_on_bitmap_record_with_options(destination: Bitmap, clr: Color, l: Line, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__l = __skadapter__to_sklib_line(l);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line_on_bitmap__bitmap__color__line_ref__drawing_options(__skparam__destination, __skparam__clr, __skparam__l, __skparam__opts);
    }
}
pub fn draw_line_on_bitmap_point_to_point(destination: Bitmap, clr: Color, from_pt: Point2D, to_pt: Point2D) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__to_pt = __skadapter__to_sklib_point_2d(to_pt);
    unsafe {
        __sklib__draw_line_on_bitmap__bitmap__color__point_2d_ref__point_2d_ref(__skparam__destination, __skparam__clr, __skparam__from_pt, __skparam__to_pt);
    }
}
pub fn draw_line_on_bitmap_point_to_point_with_options(destination: Bitmap, clr: Color, from_pt: Point2D, to_pt: Point2D, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__to_pt = __skadapter__to_sklib_point_2d(to_pt);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line_on_bitmap__bitmap__color__point_2d_ref__point_2d_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__from_pt, __skparam__to_pt, __skparam__opts);
    }
}
pub fn draw_line_on_bitmap(destination: Bitmap, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    unsafe {
        __sklib__draw_line_on_bitmap__bitmap__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2);
    }
}
pub fn draw_line_on_bitmap_with_options(destination: Bitmap, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line_on_bitmap__bitmap__color__double__double__double__double__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__opts);
    }
}
pub fn draw_line_on_window_record(destination: Window, clr: Color, l: Line) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        __sklib__draw_line_on_window__window__color__line_ref(__skparam__destination, __skparam__clr, __skparam__l);
    }
}
pub fn draw_line_on_window_record_with_options(destination: Window, clr: Color, l: Line, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__l = __skadapter__to_sklib_line(l);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line_on_window__window__color__line_ref__drawing_options(__skparam__destination, __skparam__clr, __skparam__l, __skparam__opts);
    }
}
pub fn draw_line_on_window_point_to_point(destination: Window, clr: Color, from_pt: Point2D, to_pt: Point2D) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__to_pt = __skadapter__to_sklib_point_2d(to_pt);
    unsafe {
        __sklib__draw_line_on_window__window__color__point_2d_ref__point_2d_ref(__skparam__destination, __skparam__clr, __skparam__from_pt, __skparam__to_pt);
    }
}
pub fn draw_line_on_window_point_to_point_with_options(destination: Window, clr: Color, from_pt: Point2D, to_pt: Point2D, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__to_pt = __skadapter__to_sklib_point_2d(to_pt);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line_on_window__window__color__point_2d_ref__point_2d_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__from_pt, __skparam__to_pt, __skparam__opts);
    }
}
pub fn draw_line_on_window(destination: Window, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    unsafe {
        __sklib__draw_line_on_window__window__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2);
    }
}
pub fn draw_line_on_window_with_options(destination: Window, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_line_on_window__window__color__double__double__double__double__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__opts);
    }
}
pub fn closest_point_on_line(from_pt: Point2D, l: Line) -> Point2D {
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__closest_point_on_line__point_2d__line_ref(__skparam__from_pt, __skparam__l);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn closest_point_on_lines(from_pt: Point2D, lines: Vec<Line>, line_idx: &mut i32) -> Point2D {
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__lines = __skadapter__to_sklib_vector_line(lines);
    let mut __skparam__line_idx = __skadapter__to_sklib_int((*line_idx).clone());
    unsafe {
        let __skreturn = __sklib__closest_point_on_lines__point_2d__vector_line_ref__int_ref(__skparam__from_pt, __skparam__lines, &mut __skparam__line_idx);
        *line_idx = __skadapter__to_int(__skparam__line_idx);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn line_from_point_to_point(start: Point2D, end_pt: Point2D) -> Line {
    let __skparam__start = __skadapter__to_sklib_point_2d(start);
    let __skparam__end_pt = __skadapter__to_sklib_point_2d(end_pt);
    unsafe {
        let __skreturn = __sklib__line_from__point_2d_ref__point_2d_ref(__skparam__start, __skparam__end_pt);
        __skadapter__to_line(__skreturn)
    }
}
pub fn line_from_start_with_offset(start: Point2D, offset: Vector2D) -> Line {
    let __skparam__start = __skadapter__to_sklib_point_2d(start);
    let __skparam__offset = __skadapter__to_sklib_vector_2d(offset);
    unsafe {
        let __skreturn = __sklib__line_from__point_2d_ref__vector_2d_ref(__skparam__start, __skparam__offset);
        __skadapter__to_line(__skreturn)
    }
}
pub fn line_from_vector(v: Vector2D) -> Line {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__line_from__vector_2d_ref(__skparam__v);
        __skadapter__to_line(__skreturn)
    }
}
pub fn line_from(x1: f64, y1: f64, x2: f64, y2: f64) -> Line {
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    unsafe {
        let __skreturn = __sklib__line_from__double__double__double__double(__skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2);
        __skadapter__to_line(__skreturn)
    }
}
pub fn line_intersection_point(line1: Line, line2: Line, pt: &mut Point2D) -> bool {
    let __skparam__line1 = __skadapter__to_sklib_line(line1);
    let __skparam__line2 = __skadapter__to_sklib_line(line2);
    let mut __skparam__pt = __skadapter__to_sklib_point_2d((*pt).clone());
    unsafe {
        let __skreturn = __sklib__line_intersection_point__line_ref__line_ref__point_2d_ref(__skparam__line1, __skparam__line2, &mut __skparam__pt);
        *pt = __skadapter__to_point_2d(__skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn line_intersects_circle(l: Line, c: Circle) -> bool {
    let __skparam__l = __skadapter__to_sklib_line(l);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__line_intersects_circle__line_ref__circle_ref(__skparam__l, __skparam__c);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn line_intersects_lines(l: Line, lines: Vec<Line>) -> bool {
    let __skparam__l = __skadapter__to_sklib_line(l);
    let __skparam__lines = __skadapter__to_sklib_vector_line(lines);
    unsafe {
        let __skreturn = __sklib__line_intersects_lines__line_ref__vector_line_ref(__skparam__l, __skparam__lines);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn line_intersects_rect(l: Line, rect: Rectangle) -> bool {
    let __skparam__l = __skadapter__to_sklib_line(l);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__line_intersects_rect__line_ref__rectangle_ref(__skparam__l, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn line_length(l: Line) -> f32 {
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__line_length__line_ref(__skparam__l);
        __skadapter__to_float(__skreturn)
    }
}
pub fn line_length_squared(l: Line) -> f32 {
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__line_length_squared__line_ref(__skparam__l);
        __skadapter__to_float(__skreturn)
    }
}
pub fn line_mid_point(l: Line) -> Point2D {
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__line_mid_point__line_ref(__skparam__l);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn line_normal(l: Line) -> Vector2D {
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__line_normal__line_ref(__skparam__l);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn line_to_string(ln: Line) -> String {
    let __skparam__ln = __skadapter__to_sklib_line(ln);
    unsafe {
        let __skreturn = __sklib__line_to_string__line_ref(__skparam__ln);
        __skadapter__to_string(__skreturn)
    }
}
pub fn lines_from_rectangle(rect: Rectangle) -> Vec<Line> {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__lines_from__rectangle_ref(__skparam__rect);
        __skadapter__to_vector_line(__skreturn)
    }
}
pub fn lines_from_triangle(t: Triangle) -> Vec<Line> {
    let __skparam__t = __skadapter__to_sklib_triangle(t);
    unsafe {
        let __skreturn = __sklib__lines_from__triangle_ref(__skparam__t);
        __skadapter__to_vector_line(__skreturn)
    }
}
pub fn lines_intersect(l1: Line, l2: Line) -> bool {
    let __skparam__l1 = __skadapter__to_sklib_line(l1);
    let __skparam__l2 = __skadapter__to_sklib_line(l2);
    unsafe {
        let __skreturn = __sklib__lines_intersect__line_ref__line_ref(__skparam__l1, __skparam__l2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn close_log_process() {
    unsafe {
        __sklib__close_log_process();
    }
}
pub fn init_custom_logger(mode: LogMode) {
    let __skparam__mode = __skadapter__to_sklib_log_mode(mode);
    unsafe {
        __sklib__init_custom_logger__log_mode(__skparam__mode);
    }
}
pub fn init_custom_logger__name_override_mode(app_name: String, override_prev_log: bool, mode: LogMode) {
    let __skparam__app_name = __skadapter__to_sklib_string(app_name);
    let __skparam__override_prev_log = __skadapter__to_sklib_bool(override_prev_log);
    let __skparam__mode = __skadapter__to_sklib_log_mode(mode);
    unsafe {
        __sklib__init_custom_logger__string__bool__log_mode(__skparam__app_name, __skparam__override_prev_log, __skparam__mode);
    }
}
pub fn log(level: LogLevel, message: String) {
    let __skparam__level = __skadapter__to_sklib_log_level(level);
    let __skparam__message = __skadapter__to_sklib_string(message);
    unsafe {
        __sklib__log__log_level__string(__skparam__level, __skparam__message);
    }
}
pub fn apply_matrix_to_quad(matrix: Matrix2D, q: &mut Quad) {
    let __skparam__matrix = __skadapter__to_sklib_matrix_2d(matrix);
    let mut __skparam__q = __skadapter__to_sklib_quad((*q).clone());
    unsafe {
        __sklib__apply_matrix__matrix_2d_ref__quad_ref(__skparam__matrix, &mut __skparam__q);
        *q = __skadapter__to_quad(__skparam__q);
    }
}
pub fn apply_matrix_to_triangle(m: Matrix2D, tri: &mut Triangle) {
    let __skparam__m = __skadapter__to_sklib_matrix_2d(m);
    let mut __skparam__tri = __skadapter__to_sklib_triangle((*tri).clone());
    unsafe {
        __sklib__apply_matrix__matrix_2d_ref__triangle_ref(__skparam__m, &mut __skparam__tri);
        *tri = __skadapter__to_triangle(__skparam__tri);
    }
}
pub fn identity_matrix() -> Matrix2D {
    unsafe {
        let __skreturn = __sklib__identity_matrix();
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn matrix_inverse(m: Matrix2D) -> Matrix2D {
    let __skparam__m = __skadapter__to_sklib_matrix_2d(m);
    unsafe {
        let __skreturn = __sklib__matrix_inverse__matrix_2d_ref(__skparam__m);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn matrix_multiply_point(m: Matrix2D, pt: Point2D) -> Point2D {
    let __skparam__m = __skadapter__to_sklib_matrix_2d(m);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__matrix_multiply__matrix_2d_ref__point_2d_ref(__skparam__m, __skparam__pt);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn matrix_multiply_matrix(m1: Matrix2D, m2: Matrix2D) -> Matrix2D {
    let __skparam__m1 = __skadapter__to_sklib_matrix_2d(m1);
    let __skparam__m2 = __skadapter__to_sklib_matrix_2d(m2);
    unsafe {
        let __skreturn = __sklib__matrix_multiply__matrix_2d_ref__matrix_2d_ref(__skparam__m1, __skparam__m2);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn matrix_multiply_vector(m: Matrix2D, v: Vector2D) -> Vector2D {
    let __skparam__m = __skadapter__to_sklib_matrix_2d(m);
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__matrix_multiply__matrix_2d_ref__vector_2d_ref(__skparam__m, __skparam__v);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn matrix_to_string(matrix: Matrix2D) -> String {
    let __skparam__matrix = __skadapter__to_sklib_matrix_2d(matrix);
    unsafe {
        let __skreturn = __sklib__matrix_to_string__matrix_2d_ref(__skparam__matrix);
        __skadapter__to_string(__skreturn)
    }
}
pub fn rotation_matrix(deg: f64) -> Matrix2D {
    let __skparam__deg = __skadapter__to_sklib_double(deg);
    unsafe {
        let __skreturn = __sklib__rotation_matrix__double(__skparam__deg);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn scale_matrix_from_point(scale: Point2D) -> Matrix2D {
    let __skparam__scale = __skadapter__to_sklib_point_2d(scale);
    unsafe {
        let __skreturn = __sklib__scale_matrix__point_2d_ref(__skparam__scale);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn scale_matrix_from_vector(scale: Vector2D) -> Matrix2D {
    let __skparam__scale = __skadapter__to_sklib_vector_2d(scale);
    unsafe {
        let __skreturn = __sklib__scale_matrix__vector_2d_ref(__skparam__scale);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn scale_matrix(scale: f64) -> Matrix2D {
    let __skparam__scale = __skadapter__to_sklib_double(scale);
    unsafe {
        let __skreturn = __sklib__scale_matrix__double(__skparam__scale);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn scale_rotate_translate_matrix(scale: Point2D, deg: f64, translate: Point2D) -> Matrix2D {
    let __skparam__scale = __skadapter__to_sklib_point_2d(scale);
    let __skparam__deg = __skadapter__to_sklib_double(deg);
    let __skparam__translate = __skadapter__to_sklib_point_2d(translate);
    unsafe {
        let __skreturn = __sklib__scale_rotate_translate_matrix__point_2d_ref__double__point_2d_ref(__skparam__scale, __skparam__deg, __skparam__translate);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn translation_matrix_to_point(pt: Point2D) -> Matrix2D {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__translation_matrix__point_2d_ref(__skparam__pt);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn translation_matrix_from_vector(pt: Vector2D) -> Matrix2D {
    let __skparam__pt = __skadapter__to_sklib_vector_2d(pt);
    unsafe {
        let __skreturn = __sklib__translation_matrix__vector_2d_ref(__skparam__pt);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn translation_matrix(dx: f64, dy: f64) -> Matrix2D {
    let __skparam__dx = __skadapter__to_sklib_double(dx);
    let __skparam__dy = __skadapter__to_sklib_double(dy);
    unsafe {
        let __skreturn = __sklib__translation_matrix__double__double(__skparam__dx, __skparam__dy);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn hide_mouse() {
    unsafe {
        __sklib__hide_mouse();
    }
}
pub fn mouse_clicked(button: MouseButton) -> bool {
    let __skparam__button = __skadapter__to_sklib_mouse_button(button);
    unsafe {
        let __skreturn = __sklib__mouse_clicked__mouse_button(__skparam__button);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn mouse_down(button: MouseButton) -> bool {
    let __skparam__button = __skadapter__to_sklib_mouse_button(button);
    unsafe {
        let __skreturn = __sklib__mouse_down__mouse_button(__skparam__button);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn mouse_movement() -> Vector2D {
    unsafe {
        let __skreturn = __sklib__mouse_movement();
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn mouse_position() -> Point2D {
    unsafe {
        let __skreturn = __sklib__mouse_position();
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn mouse_position_vector() -> Vector2D {
    unsafe {
        let __skreturn = __sklib__mouse_position_vector();
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn mouse_shown() -> bool {
    unsafe {
        let __skreturn = __sklib__mouse_shown();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn mouse_up(button: MouseButton) -> bool {
    let __skparam__button = __skadapter__to_sklib_mouse_button(button);
    unsafe {
        let __skreturn = __sklib__mouse_up__mouse_button(__skparam__button);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn mouse_wheel_scroll() -> Vector2D {
    unsafe {
        let __skreturn = __sklib__mouse_wheel_scroll();
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn mouse_x() -> f32 {
    unsafe {
        let __skreturn = __sklib__mouse_x();
        __skadapter__to_float(__skreturn)
    }
}
pub fn mouse_y() -> f32 {
    unsafe {
        let __skreturn = __sklib__mouse_y();
        __skadapter__to_float(__skreturn)
    }
}
pub fn move_mouse(x: f64, y: f64) {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__move_mouse__double__double(__skparam__x, __skparam__y);
    }
}
pub fn move_mouse_to_point(point: Point2D) {
    let __skparam__point = __skadapter__to_sklib_point_2d(point);
    unsafe {
        __sklib__move_mouse__point_2d(__skparam__point);
    }
}
pub fn show_mouse() {
    unsafe {
        __sklib__show_mouse();
    }
}
pub fn show_mouse_with_boolean(show: bool) {
    let __skparam__show = __skadapter__to_sklib_bool(show);
    unsafe {
        __sklib__show_mouse__bool(__skparam__show);
    }
}
pub fn fade_music_in_named(name: String, ms: i32) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__ms = __skadapter__to_sklib_int(ms);
    unsafe {
        __sklib__fade_music_in__string_ref__int(__skparam__name, __skparam__ms);
    }
}
pub fn fade_music_in_named_with_times(name: String, times: i32, ms: i32) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__times = __skadapter__to_sklib_int(times);
    let __skparam__ms = __skadapter__to_sklib_int(ms);
    unsafe {
        __sklib__fade_music_in__string_ref__int__int(__skparam__name, __skparam__times, __skparam__ms);
    }
}
pub fn fade_music_in(data: Music, ms: i32) {
    let __skparam__data = __skadapter__to_sklib_music(data);
    let __skparam__ms = __skadapter__to_sklib_int(ms);
    unsafe {
        __sklib__fade_music_in__music__int(__skparam__data, __skparam__ms);
    }
}
pub fn fade_music_in_with_times(data: Music, times: i32, ms: i32) {
    let __skparam__data = __skadapter__to_sklib_music(data);
    let __skparam__times = __skadapter__to_sklib_int(times);
    let __skparam__ms = __skadapter__to_sklib_int(ms);
    unsafe {
        __sklib__fade_music_in__music__int__int(__skparam__data, __skparam__times, __skparam__ms);
    }
}
pub fn fade_music_out(ms: i32) {
    let __skparam__ms = __skadapter__to_sklib_int(ms);
    unsafe {
        __sklib__fade_music_out__int(__skparam__ms);
    }
}
pub fn free_all_music() {
    unsafe {
        __sklib__free_all_music();
    }
}
pub fn free_music(effect: Music) {
    let __skparam__effect = __skadapter__to_sklib_music(effect);
    unsafe {
        __sklib__free_music__music(__skparam__effect);
    }
}
pub fn has_music(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_music__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn load_music(name: String, filename: String) -> Music {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        let __skreturn = __sklib__load_music__string_ref__string_ref(__skparam__name, __skparam__filename);
        __skadapter__to_music(__skreturn)
    }
}
pub fn music_filename(data: Music) -> String {
    let __skparam__data = __skadapter__to_sklib_music(data);
    unsafe {
        let __skreturn = __sklib__music_filename__music(__skparam__data);
        __skadapter__to_string(__skreturn)
    }
}
pub fn music_name(data: Music) -> String {
    let __skparam__data = __skadapter__to_sklib_music(data);
    unsafe {
        let __skreturn = __sklib__music_name__music(__skparam__data);
        __skadapter__to_string(__skreturn)
    }
}
pub fn music_named(name: String) -> Music {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__music_named__string_ref(__skparam__name);
        __skadapter__to_music(__skreturn)
    }
}
pub fn music_playing() -> bool {
    unsafe {
        let __skreturn = __sklib__music_playing();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn music_valid(m: Music) -> bool {
    let __skparam__m = __skadapter__to_sklib_music(m);
    unsafe {
        let __skreturn = __sklib__music_valid__music(__skparam__m);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn music_volume() -> f64 {
    unsafe {
        let __skreturn = __sklib__music_volume();
        __skadapter__to_double(__skreturn)
    }
}
pub fn pause_music() {
    unsafe {
        __sklib__pause_music();
    }
}
pub fn play_music_named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__play_music__string_ref(__skparam__name);
    }
}
pub fn play_music_named_with_times(name: String, times: i32) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__times = __skadapter__to_sklib_int(times);
    unsafe {
        __sklib__play_music__string_ref__int(__skparam__name, __skparam__times);
    }
}
pub fn play_music(data: Music) {
    let __skparam__data = __skadapter__to_sklib_music(data);
    unsafe {
        __sklib__play_music__music(__skparam__data);
    }
}
pub fn play_music_with_times(data: Music, times: i32) {
    let __skparam__data = __skadapter__to_sklib_music(data);
    let __skparam__times = __skadapter__to_sklib_int(times);
    unsafe {
        __sklib__play_music__music__int(__skparam__data, __skparam__times);
    }
}
pub fn play_music_with_times_and_volume(data: Music, times: i32, volume: f64) {
    let __skparam__data = __skadapter__to_sklib_music(data);
    let __skparam__times = __skadapter__to_sklib_int(times);
    let __skparam__volume = __skadapter__to_sklib_double(volume);
    unsafe {
        __sklib__play_music__music__int__double(__skparam__data, __skparam__times, __skparam__volume);
    }
}
pub fn resume_music() {
    unsafe {
        __sklib__resume_music();
    }
}
pub fn set_music_volume(volume: f64) {
    let __skparam__volume = __skadapter__to_sklib_double(volume);
    unsafe {
        __sklib__set_music_volume__double(__skparam__volume);
    }
}
pub fn stop_music() {
    unsafe {
        __sklib__stop_music();
    }
}
pub fn accept_all_new_connections() -> bool {
    unsafe {
        let __skreturn = __sklib__accept_all_new_connections();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn accept_new_connection(server: ServerSocket) -> bool {
    let __skparam__server = __skadapter__to_sklib_server_socket(server);
    unsafe {
        let __skreturn = __sklib__accept_new_connection__server_socket(__skparam__server);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn broadcast_message(a_msg: String, svr: ServerSocket) {
    let __skparam__a_msg = __skadapter__to_sklib_string(a_msg);
    let __skparam__svr = __skadapter__to_sklib_server_socket(svr);
    unsafe {
        __sklib__broadcast_message__string_ref__server_socket(__skparam__a_msg, __skparam__svr);
    }
}
pub fn broadcast_message_to_all(a_msg: String) {
    let __skparam__a_msg = __skadapter__to_sklib_string(a_msg);
    unsafe {
        __sklib__broadcast_message__string_ref(__skparam__a_msg);
    }
}
pub fn broadcast_message_to_server_named(a_msg: String, name: String) {
    let __skparam__a_msg = __skadapter__to_sklib_string(a_msg);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__broadcast_message__string_ref__string_ref(__skparam__a_msg, __skparam__name);
    }
}
pub fn check_network_activity() {
    unsafe {
        __sklib__check_network_activity();
    }
}
pub fn clear_messages_from_name(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__clear_messages__string_ref(__skparam__name);
    }
}
pub fn clear_messages_from_connection(a_connection: Connection) {
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        __sklib__clear_messages__connection(__skparam__a_connection);
    }
}
pub fn clear_messages_from_server(svr: ServerSocket) {
    let __skparam__svr = __skadapter__to_sklib_server_socket(svr);
    unsafe {
        __sklib__clear_messages__server_socket(__skparam__svr);
    }
}
pub fn close_all_connections() {
    unsafe {
        __sklib__close_all_connections();
    }
}
pub fn close_all_servers() {
    unsafe {
        __sklib__close_all_servers();
    }
}
pub fn close_connection(a_connection: Connection) -> bool {
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        let __skreturn = __sklib__close_connection__connection(__skparam__a_connection);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn close_connection_named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__close_connection__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn close_message(msg: Message) {
    let __skparam__msg = __skadapter__to_sklib_message(msg);
    unsafe {
        __sklib__close_message__message(__skparam__msg);
    }
}
pub fn close_server_named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__close_server__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn close_server(svr: ServerSocket) -> bool {
    let __skparam__svr = __skadapter__to_sklib_server_socket(svr);
    unsafe {
        let __skreturn = __sklib__close_server__server_socket(__skparam__svr);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn connection_count_named(name: String) -> u32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__connection_count__string_ref(__skparam__name);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn connection_count(server: ServerSocket) -> u32 {
    let __skparam__server = __skadapter__to_sklib_server_socket(server);
    unsafe {
        let __skreturn = __sklib__connection_count__server_socket(__skparam__server);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn connection_ip(a_connection: Connection) -> u32 {
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        let __skreturn = __sklib__connection_ip__connection(__skparam__a_connection);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn connection_ip_from_name(name: String) -> u32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__connection_ip__string_ref(__skparam__name);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn connection_named(name: String) -> Connection {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__connection_named__string_ref(__skparam__name);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn connection_port(a_connection: Connection) -> u16 {
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        let __skreturn = __sklib__connection_port__connection(__skparam__a_connection);
        __skadapter__to_unsigned_short(__skreturn)
    }
}
pub fn connection_port_from_name(name: String) -> u16 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__connection_port__string_ref(__skparam__name);
        __skadapter__to_unsigned_short(__skreturn)
    }
}
pub fn create_server_with_port(name: String, port: u16) -> ServerSocket {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    unsafe {
        let __skreturn = __sklib__create_server__string_ref__unsigned_short(__skparam__name, __skparam__port);
        __skadapter__to_server_socket(__skreturn)
    }
}
pub fn create_server_with_port_and_protocol(name: String, port: u16, protocol: ConnectionType) -> ServerSocket {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    let __skparam__protocol = __skadapter__to_sklib_connection_type(protocol);
    unsafe {
        let __skreturn = __sklib__create_server__string_ref__unsigned_short__connection_type(__skparam__name, __skparam__port, __skparam__protocol);
        __skadapter__to_server_socket(__skreturn)
    }
}
pub fn dec_to_hex(a_dec: u32) -> String {
    let __skparam__a_dec = __skadapter__to_sklib_unsigned_int(a_dec);
    unsafe {
        let __skreturn = __sklib__dec_to_hex__unsigned_int(__skparam__a_dec);
        __skadapter__to_string(__skreturn)
    }
}
pub fn fetch_new_connection(server: ServerSocket) -> Connection {
    let __skparam__server = __skadapter__to_sklib_server_socket(server);
    unsafe {
        let __skreturn = __sklib__fetch_new_connection__server_socket(__skparam__server);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn has_connection(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_connection__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_messages() -> bool {
    unsafe {
        let __skreturn = __sklib__has_messages();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_messages_on_connection(con: Connection) -> bool {
    let __skparam__con = __skadapter__to_sklib_connection(con);
    unsafe {
        let __skreturn = __sklib__has_messages__connection(__skparam__con);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_messages_on_name(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_messages__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_messages_on_server(svr: ServerSocket) -> bool {
    let __skparam__svr = __skadapter__to_sklib_server_socket(svr);
    unsafe {
        let __skreturn = __sklib__has_messages__server_socket(__skparam__svr);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_new_connections() -> bool {
    unsafe {
        let __skreturn = __sklib__has_new_connections();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_server(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_server__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn hex_str_to_ipv4(a_hex: String) -> String {
    let __skparam__a_hex = __skadapter__to_sklib_string(a_hex);
    unsafe {
        let __skreturn = __sklib__hex_str_to_ipv4__string_ref(__skparam__a_hex);
        __skadapter__to_string(__skreturn)
    }
}
pub fn hex_to_dec_string(a_hex: String) -> String {
    let __skparam__a_hex = __skadapter__to_sklib_string(a_hex);
    unsafe {
        let __skreturn = __sklib__hex_to_dec_string__string_ref(__skparam__a_hex);
        __skadapter__to_string(__skreturn)
    }
}
pub fn ipv4_to_dec(a_ip: String) -> u32 {
    let __skparam__a_ip = __skadapter__to_sklib_string(a_ip);
    unsafe {
        let __skreturn = __sklib__ipv4_to_dec__string_ref(__skparam__a_ip);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn ipv4_to_hex(a_ip: String) -> String {
    let __skparam__a_ip = __skadapter__to_sklib_string(a_ip);
    unsafe {
        let __skreturn = __sklib__ipv4_to_hex__string_ref(__skparam__a_ip);
        __skadapter__to_string(__skreturn)
    }
}
pub fn ipv4_to_str(ip: u32) -> String {
    let __skparam__ip = __skadapter__to_sklib_unsigned_int(ip);
    unsafe {
        let __skreturn = __sklib__ipv4_to_str__unsigned_int(__skparam__ip);
        __skadapter__to_string(__skreturn)
    }
}
pub fn is_connection_open(con: Connection) -> bool {
    let __skparam__con = __skadapter__to_sklib_connection(con);
    unsafe {
        let __skreturn = __sklib__is_connection_open__connection(__skparam__con);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_connection_open_from_name(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__is_connection_open__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn last_connection_named(name: String) -> Connection {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__last_connection__string_ref(__skparam__name);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn last_connection(server: ServerSocket) -> Connection {
    let __skparam__server = __skadapter__to_sklib_server_socket(server);
    unsafe {
        let __skreturn = __sklib__last_connection__server_socket(__skparam__server);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn message_connection(msg: Message) -> Connection {
    let __skparam__msg = __skadapter__to_sklib_message(msg);
    unsafe {
        let __skreturn = __sklib__message_connection__message(__skparam__msg);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn message_count_on_server(svr: ServerSocket) -> u32 {
    let __skparam__svr = __skadapter__to_sklib_server_socket(svr);
    unsafe {
        let __skreturn = __sklib__message_count__server_socket(__skparam__svr);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn message_count_on_connection(a_connection: Connection) -> u32 {
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        let __skreturn = __sklib__message_count__connection(__skparam__a_connection);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn message_count_from_name(name: String) -> u32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__message_count__string_ref(__skparam__name);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn message_data(msg: Message) -> String {
    let __skparam__msg = __skadapter__to_sklib_message(msg);
    unsafe {
        let __skreturn = __sklib__message_data__message(__skparam__msg);
        __skadapter__to_string(__skreturn)
    }
}
pub fn message_data_bytes(msg: Message) -> Vec<i8> {
    let __skparam__msg = __skadapter__to_sklib_message(msg);
    unsafe {
        let __skreturn = __sklib__message_data_bytes__message(__skparam__msg);
        __skadapter__to_vector_int8_t(__skreturn)
    }
}
pub fn message_host(msg: Message) -> String {
    let __skparam__msg = __skadapter__to_sklib_message(msg);
    unsafe {
        let __skreturn = __sklib__message_host__message(__skparam__msg);
        __skadapter__to_string(__skreturn)
    }
}
pub fn message_port(msg: Message) -> u16 {
    let __skparam__msg = __skadapter__to_sklib_message(msg);
    unsafe {
        let __skreturn = __sklib__message_port__message(__skparam__msg);
        __skadapter__to_unsigned_short(__skreturn)
    }
}
pub fn message_protocol(msg: Message) -> ConnectionType {
    let __skparam__msg = __skadapter__to_sklib_message(msg);
    unsafe {
        let __skreturn = __sklib__message_protocol__message(__skparam__msg);
        __skadapter__to_connection_type(__skreturn)
    }
}
pub fn my_ip() -> String {
    unsafe {
        let __skreturn = __sklib__my_ip();
        __skadapter__to_string(__skreturn)
    }
}
pub fn name_for_connection(host: String, port: u32) -> String {
    let __skparam__host = __skadapter__to_sklib_string(host);
    let __skparam__port = __skadapter__to_sklib_unsigned_int(port);
    unsafe {
        let __skreturn = __sklib__name_for_connection__string__unsigned_int(__skparam__host, __skparam__port);
        __skadapter__to_string(__skreturn)
    }
}
pub fn new_connection_count(server: ServerSocket) -> i32 {
    let __skparam__server = __skadapter__to_sklib_server_socket(server);
    unsafe {
        let __skreturn = __sklib__new_connection_count__server_socket(__skparam__server);
        __skadapter__to_int(__skreturn)
    }
}
pub fn open_connection(name: String, host: String, port: u16) -> Connection {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__host = __skadapter__to_sklib_string(host);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    unsafe {
        let __skreturn = __sklib__open_connection__string_ref__string_ref__unsigned_short(__skparam__name, __skparam__host, __skparam__port);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn open_connection_with_protocol(name: String, host: String, port: u16, protocol: ConnectionType) -> Connection {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__host = __skadapter__to_sklib_string(host);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    let __skparam__protocol = __skadapter__to_sklib_connection_type(protocol);
    unsafe {
        let __skreturn = __sklib__open_connection__string_ref__string_ref__unsigned_short__connection_type(__skparam__name, __skparam__host, __skparam__port, __skparam__protocol);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn read_message() -> Message {
    unsafe {
        let __skreturn = __sklib__read_message();
        __skadapter__to_message(__skreturn)
    }
}
pub fn read_message_from_connection(a_connection: Connection) -> Message {
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        let __skreturn = __sklib__read_message__connection(__skparam__a_connection);
        __skadapter__to_message(__skreturn)
    }
}
pub fn read_message_from_name(name: String) -> Message {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__read_message__string_ref(__skparam__name);
        __skadapter__to_message(__skreturn)
    }
}
pub fn read_message_from_server(svr: ServerSocket) -> Message {
    let __skparam__svr = __skadapter__to_sklib_server_socket(svr);
    unsafe {
        let __skreturn = __sklib__read_message__server_socket(__skparam__svr);
        __skadapter__to_message(__skreturn)
    }
}
pub fn read_message_data_from_name(name: String) -> String {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__read_message_data__string_ref(__skparam__name);
        __skadapter__to_string(__skreturn)
    }
}
pub fn read_message_data_from_connection(a_connection: Connection) -> String {
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        let __skreturn = __sklib__read_message_data__connection(__skparam__a_connection);
        __skadapter__to_string(__skreturn)
    }
}
pub fn read_message_data_from_server(svr: ServerSocket) -> String {
    let __skparam__svr = __skadapter__to_sklib_server_socket(svr);
    unsafe {
        let __skreturn = __sklib__read_message_data__server_socket(__skparam__svr);
        __skadapter__to_string(__skreturn)
    }
}
pub fn reconnect(a_connection: Connection) {
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        __sklib__reconnect__connection(__skparam__a_connection);
    }
}
pub fn reconnect_from_name(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__reconnect__string_ref(__skparam__name);
    }
}
pub fn release_all_connections() {
    unsafe {
        __sklib__release_all_connections();
    }
}
pub fn reset_new_connection_count(server: ServerSocket) {
    let __skparam__server = __skadapter__to_sklib_server_socket(server);
    unsafe {
        __sklib__reset_new_connection_count__server_socket(__skparam__server);
    }
}
pub fn retrieve_connection_named(name: String, idx: i32) -> Connection {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__retrieve_connection__string_ref__int(__skparam__name, __skparam__idx);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn retrieve_connection(server: ServerSocket, idx: i32) -> Connection {
    let __skparam__server = __skadapter__to_sklib_server_socket(server);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__retrieve_connection__server_socket__int(__skparam__server, __skparam__idx);
        __skadapter__to_connection(__skreturn)
    }
}
pub fn send_message_to_connection(a_msg: String, a_connection: Connection) -> bool {
    let __skparam__a_msg = __skadapter__to_sklib_string(a_msg);
    let __skparam__a_connection = __skadapter__to_sklib_connection(a_connection);
    unsafe {
        let __skreturn = __sklib__send_message_to__string_ref__connection(__skparam__a_msg, __skparam__a_connection);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn send_message_to_name(a_msg: String, name: String) -> bool {
    let __skparam__a_msg = __skadapter__to_sklib_string(a_msg);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__send_message_to__string_ref__string_ref(__skparam__a_msg, __skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn server_has_new_connection_named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__server_has_new_connection__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn server_has_new_connection(server: ServerSocket) -> bool {
    let __skparam__server = __skadapter__to_sklib_server_socket(server);
    unsafe {
        let __skreturn = __sklib__server_has_new_connection__server_socket(__skparam__server);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn server_named(name: String) -> ServerSocket {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__server_named__string_ref(__skparam__name);
        __skadapter__to_server_socket(__skreturn)
    }
}
pub fn set_udp_packet_size(udp_packet_size: u32) {
    let __skparam__udp_packet_size = __skadapter__to_sklib_unsigned_int(udp_packet_size);
    unsafe {
        __sklib__set_udp_packet_size__unsigned_int(__skparam__udp_packet_size);
    }
}
pub fn udp_packet_size() -> u32 {
    unsafe {
        let __skreturn = __sklib__udp_packet_size();
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn draw_pixel_at_point(clr: Color, pt: Point2D) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        __sklib__draw_pixel__color__point_2d_ref(__skparam__clr, __skparam__pt);
    }
}
pub fn draw_pixel_at_point_with_options(clr: Color, pt: Point2D, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_pixel__color__point_2d_ref__drawing_options(__skparam__clr, __skparam__pt, __skparam__opts);
    }
}
pub fn draw_pixel(clr: Color, x: f64, y: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_pixel__color__double__double(__skparam__clr, __skparam__x, __skparam__y);
    }
}
pub fn draw_pixel_with_options(clr: Color, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_pixel__color__double__double__drawing_options(__skparam__clr, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_pixel_on_bitmap_at_point(destination: Bitmap, clr: Color, pt: Point2D) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        __sklib__draw_pixel_on_bitmap__bitmap__color__point_2d_ref(__skparam__destination, __skparam__clr, __skparam__pt);
    }
}
pub fn draw_pixel_on_bitmap_at_point_with_options(destination: Bitmap, clr: Color, pt: Point2D, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_pixel_on_bitmap__bitmap__color__point_2d_ref__drawing_options(__skparam__destination, __skparam__clr, __skparam__pt, __skparam__opts);
    }
}
pub fn draw_pixel_on_bitmap(destination: Bitmap, clr: Color, x: f64, y: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_pixel_on_bitmap__bitmap__color__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y);
    }
}
pub fn draw_pixel_on_bitmap_with_options(destination: Bitmap, clr: Color, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_pixel_on_bitmap__bitmap__color__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_pixel_on_window_at_point(destination: Window, clr: Color, pt: Point2D) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        __sklib__draw_pixel_on_window__window__color__point_2d_ref(__skparam__destination, __skparam__clr, __skparam__pt);
    }
}
pub fn draw_pixel_on_window_at_point_with_options(destination: Window, clr: Color, pt: Point2D, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_pixel_on_window__window__color__point_2d_ref__drawing_options(__skparam__destination, __skparam__clr, __skparam__pt, __skparam__opts);
    }
}
pub fn draw_pixel_on_window(destination: Window, clr: Color, x: f64, y: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_pixel_on_window__window__color__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y);
    }
}
pub fn draw_pixel_on_window_with_options(destination: Window, clr: Color, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_pixel_on_window__window__color__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn get_pixel_from_bitmap_at_point(bmp: Bitmap, pt: Point2D) -> Color {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__get_pixel__bitmap__point_2d_ref(__skparam__bmp, __skparam__pt);
        __skadapter__to_color(__skreturn)
    }
}
pub fn get_pixel_from_bitmap(bmp: Bitmap, x: f64, y: f64) -> Color {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__get_pixel__bitmap__double__double(__skparam__bmp, __skparam__x, __skparam__y);
        __skadapter__to_color(__skreturn)
    }
}
pub fn get_pixel_at_point(pt: Point2D) -> Color {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__get_pixel__point_2d_ref(__skparam__pt);
        __skadapter__to_color(__skreturn)
    }
}
pub fn get_pixel(x: f64, y: f64) -> Color {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__get_pixel__double__double(__skparam__x, __skparam__y);
        __skadapter__to_color(__skreturn)
    }
}
pub fn get_pixel_from_window_at_point(wnd: Window, pt: Point2D) -> Color {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__get_pixel__window__point_2d_ref(__skparam__wnd, __skparam__pt);
        __skadapter__to_color(__skreturn)
    }
}
pub fn get_pixel_from_window(wnd: Window, x: f64, y: f64) -> Color {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__get_pixel__window__double__double(__skparam__wnd, __skparam__x, __skparam__y);
        __skadapter__to_color(__skreturn)
    }
}
pub fn get_pixel_from_window_at_point_from_window(destination: Window, pt: Point2D) -> Color {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__get_pixel_from_window__window__point_2d_ref(__skparam__destination, __skparam__pt);
        __skadapter__to_color(__skreturn)
    }
}
pub fn get_pixel_from_window_from_window(destination: Window, x: f64, y: f64) -> Color {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__get_pixel_from_window__window__double__double(__skparam__destination, __skparam__x, __skparam__y);
        __skadapter__to_color(__skreturn)
    }
}
pub fn point_at(x: f64, y: f64) -> Point2D {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__point_at__double__double(__skparam__x, __skparam__y);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn point_at_origin() -> Point2D {
    unsafe {
        let __skreturn = __sklib__point_at_origin();
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn point_in_circle(pt: Point2D, c: Circle) -> bool {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__point_in_circle__point_2d_ref__circle_ref(__skparam__pt, __skparam__c);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_in_circle_with_values(ptx: f64, pty: f64, cx: f64, cy: f64, radius: f64) -> bool {
    let __skparam__ptx = __skadapter__to_sklib_double(ptx);
    let __skparam__pty = __skadapter__to_sklib_double(pty);
    let __skparam__cx = __skadapter__to_sklib_double(cx);
    let __skparam__cy = __skadapter__to_sklib_double(cy);
    let __skparam__radius = __skadapter__to_sklib_double(radius);
    unsafe {
        let __skreturn = __sklib__point_in_circle__double__double__double__double__double(__skparam__ptx, __skparam__pty, __skparam__cx, __skparam__cy, __skparam__radius);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_in_quad(pt: Point2D, q: Quad) -> bool {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        let __skreturn = __sklib__point_in_quad__point_2d_ref__quad_ref(__skparam__pt, __skparam__q);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_in_rectangle(pt: Point2D, rect: Rectangle) -> bool {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__point_in_rectangle__point_2d_ref__rectangle_ref(__skparam__pt, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_in_rectangle_with_values(ptx: f64, pty: f64, rect_x: f64, rect_y: f64, rect_width: f64, rect_height: f64) -> bool {
    let __skparam__ptx = __skadapter__to_sklib_double(ptx);
    let __skparam__pty = __skadapter__to_sklib_double(pty);
    let __skparam__rect_x = __skadapter__to_sklib_double(rect_x);
    let __skparam__rect_y = __skadapter__to_sklib_double(rect_y);
    let __skparam__rect_width = __skadapter__to_sklib_double(rect_width);
    let __skparam__rect_height = __skadapter__to_sklib_double(rect_height);
    unsafe {
        let __skreturn = __sklib__point_in_rectangle__double__double__double__double__double__double(__skparam__ptx, __skparam__pty, __skparam__rect_x, __skparam__rect_y, __skparam__rect_width, __skparam__rect_height);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_in_triangle(pt: Point2D, tri: Triangle) -> bool {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        let __skreturn = __sklib__point_in_triangle__point_2d_ref__triangle_ref(__skparam__pt, __skparam__tri);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_line_distance(pt: Point2D, l: Line) -> f32 {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__point_line_distance__point_2d_ref__line_ref(__skparam__pt, __skparam__l);
        __skadapter__to_float(__skreturn)
    }
}
pub fn point_offset_by(start_point: Point2D, offset: Vector2D) -> Point2D {
    let __skparam__start_point = __skadapter__to_sklib_point_2d(start_point);
    let __skparam__offset = __skadapter__to_sklib_vector_2d(offset);
    unsafe {
        let __skreturn = __sklib__point_offset_by__point_2d_ref__vector_2d_ref(__skparam__start_point, __skparam__offset);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn point_offset_from_origin(offset: Vector2D) -> Point2D {
    let __skparam__offset = __skadapter__to_sklib_vector_2d(offset);
    unsafe {
        let __skreturn = __sklib__point_offset_from_origin__vector_2d_ref(__skparam__offset);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn point_on_line(pt: Point2D, l: Line) -> bool {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__point_on_line__point_2d_ref__line_ref(__skparam__pt, __skparam__l);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_on_line_with_proximity(pt: Point2D, l: Line, proximity: f32) -> bool {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__l = __skadapter__to_sklib_line(l);
    let __skparam__proximity = __skadapter__to_sklib_float(proximity);
    unsafe {
        let __skreturn = __sklib__point_on_line__point_2d_ref__line_ref__float(__skparam__pt, __skparam__l, __skparam__proximity);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn point_point_angle(pt1: Point2D, pt2: Point2D) -> f32 {
    let __skparam__pt1 = __skadapter__to_sklib_point_2d(pt1);
    let __skparam__pt2 = __skadapter__to_sklib_point_2d(pt2);
    unsafe {
        let __skreturn = __sklib__point_point_angle__point_2d_ref__point_2d_ref(__skparam__pt1, __skparam__pt2);
        __skadapter__to_float(__skreturn)
    }
}
pub fn point_point_distance(pt1: Point2D, pt2: Point2D) -> f32 {
    let __skparam__pt1 = __skadapter__to_sklib_point_2d(pt1);
    let __skparam__pt2 = __skadapter__to_sklib_point_2d(pt2);
    unsafe {
        let __skreturn = __sklib__point_point_distance__point_2d_ref__point_2d_ref(__skparam__pt1, __skparam__pt2);
        __skadapter__to_float(__skreturn)
    }
}
pub fn point_to_string(pt: Point2D) -> String {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__point_to_string__point_2d_ref(__skparam__pt);
        __skadapter__to_string(__skreturn)
    }
}
pub fn random_bitmap_point(bmp: Bitmap) -> Point2D {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        let __skreturn = __sklib__random_bitmap_point__bitmap(__skparam__bmp);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn random_screen_point() -> Point2D {
    unsafe {
        let __skreturn = __sklib__random_screen_point();
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn random_window_point(wind: Window) -> Point2D {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__random_window_point__window(__skparam__wind);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn same_point(pt1: Point2D, pt2: Point2D) -> bool {
    let __skparam__pt1 = __skadapter__to_sklib_point_2d(pt1);
    let __skparam__pt2 = __skadapter__to_sklib_point_2d(pt2);
    unsafe {
        let __skreturn = __sklib__same_point__point_2d_ref__point_2d_ref(__skparam__pt1, __skparam__pt2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn quad_from_points(p1: Point2D, p2: Point2D, p3: Point2D, p4: Point2D) -> Quad {
    let __skparam__p1 = __skadapter__to_sklib_point_2d(p1);
    let __skparam__p2 = __skadapter__to_sklib_point_2d(p2);
    let __skparam__p3 = __skadapter__to_sklib_point_2d(p3);
    let __skparam__p4 = __skadapter__to_sklib_point_2d(p4);
    unsafe {
        let __skreturn = __sklib__quad_from__point_2d_ref__point_2d_ref__point_2d_ref__point_2d_ref(__skparam__p1, __skparam__p2, __skparam__p3, __skparam__p4);
        __skadapter__to_quad(__skreturn)
    }
}
pub fn quad_from_rectangle(rect: Rectangle) -> Quad {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__quad_from__rectangle_ref(__skparam__rect);
        __skadapter__to_quad(__skreturn)
    }
}
pub fn quad_from_rectangle_with_transformation(rect: Rectangle, transform: Matrix2D) -> Quad {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__transform = __skadapter__to_sklib_matrix_2d(transform);
    unsafe {
        let __skreturn = __sklib__quad_from__rectangle_ref__matrix_2d_ref(__skparam__rect, __skparam__transform);
        __skadapter__to_quad(__skreturn)
    }
}
pub fn quad_from(x_top_left: f64, y_top_left: f64, x_top_right: f64, y_top_right: f64, x_bottom_left: f64, y_bottom_left: f64, x_bottom_right: f64, y_bottom_right: f64) -> Quad {
    let __skparam__x_top_left = __skadapter__to_sklib_double(x_top_left);
    let __skparam__y_top_left = __skadapter__to_sklib_double(y_top_left);
    let __skparam__x_top_right = __skadapter__to_sklib_double(x_top_right);
    let __skparam__y_top_right = __skadapter__to_sklib_double(y_top_right);
    let __skparam__x_bottom_left = __skadapter__to_sklib_double(x_bottom_left);
    let __skparam__y_bottom_left = __skadapter__to_sklib_double(y_bottom_left);
    let __skparam__x_bottom_right = __skadapter__to_sklib_double(x_bottom_right);
    let __skparam__y_bottom_right = __skadapter__to_sklib_double(y_bottom_right);
    unsafe {
        let __skreturn = __sklib__quad_from__double__double__double__double__double__double__double__double(__skparam__x_top_left, __skparam__y_top_left, __skparam__x_top_right, __skparam__y_top_right, __skparam__x_bottom_left, __skparam__y_bottom_left, __skparam__x_bottom_right, __skparam__y_bottom_right);
        __skadapter__to_quad(__skreturn)
    }
}
pub fn quads_intersect(q1: Quad, q2: Quad) -> bool {
    let __skparam__q1 = __skadapter__to_sklib_quad(q1);
    let __skparam__q2 = __skadapter__to_sklib_quad(q2);
    unsafe {
        let __skreturn = __sklib__quads_intersect__quad_ref__quad_ref(__skparam__q1, __skparam__q2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn set_quad_point(q: &mut Quad, idx: i32, value: Point2D) {
    let mut __skparam__q = __skadapter__to_sklib_quad((*q).clone());
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    let __skparam__value = __skadapter__to_sklib_point_2d(value);
    unsafe {
        __sklib__set_quad_point__quad_ref__int__point_2d_ref(&mut __skparam__q, __skparam__idx, __skparam__value);
        *q = __skadapter__to_quad(__skparam__q);
    }
}
pub fn triangles_from(q: Quad) -> Vec<Triangle> {
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        let __skreturn = __sklib__triangles_from__quad_ref(__skparam__q);
        __skadapter__to_vector_triangle(__skreturn)
    }
}
pub fn rnd_range(min: i32, max: i32) -> i32 {
    let __skparam__min = __skadapter__to_sklib_int(min);
    let __skparam__max = __skadapter__to_sklib_int(max);
    unsafe {
        let __skreturn = __sklib__rnd__int__int(__skparam__min, __skparam__max);
        __skadapter__to_int(__skreturn)
    }
}
pub fn rnd() -> f32 {
    unsafe {
        let __skreturn = __sklib__rnd();
        __skadapter__to_float(__skreturn)
    }
}
pub fn rnd_int(ubound: i32) -> i32 {
    let __skparam__ubound = __skadapter__to_sklib_int(ubound);
    unsafe {
        let __skreturn = __sklib__rnd__int(__skparam__ubound);
        __skadapter__to_int(__skreturn)
    }
}
pub fn has_gpio() -> bool {
    unsafe {
        let __skreturn = __sklib__has_gpio();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn raspi_cleanup() {
    unsafe {
        __sklib__raspi_cleanup();
    }
}
pub fn raspi_get_mode(pin: Pins) -> PinModes {
    let __skparam__pin = __skadapter__to_sklib_pins(pin);
    unsafe {
        let __skreturn = __sklib__raspi_get_mode__pins(__skparam__pin);
        __skadapter__to_pin_modes(__skreturn)
    }
}
pub fn raspi_init() {
    unsafe {
        __sklib__raspi_init();
    }
}
pub fn raspi_read(pin: Pins) -> PinValues {
    let __skparam__pin = __skadapter__to_sklib_pins(pin);
    unsafe {
        let __skreturn = __sklib__raspi_read__pins(__skparam__pin);
        __skadapter__to_pin_values(__skreturn)
    }
}
pub fn raspi_set_mode(pin: Pins, mode: PinModes) {
    let __skparam__pin = __skadapter__to_sklib_pins(pin);
    let __skparam__mode = __skadapter__to_sklib_pin_modes(mode);
    unsafe {
        __sklib__raspi_set_mode__pins__pin_modes(__skparam__pin, __skparam__mode);
    }
}
pub fn raspi_set_pull_up_down(pin: Pins, pud: PullUpDown) {
    let __skparam__pin = __skadapter__to_sklib_pins(pin);
    let __skparam__pud = __skadapter__to_sklib_pull_up_down(pud);
    unsafe {
        __sklib__raspi_set_pull_up_down__pins__pull_up_down(__skparam__pin, __skparam__pud);
    }
}
pub fn raspi_set_pwm_dutycycle(pin: Pins, dutycycle: i32) {
    let __skparam__pin = __skadapter__to_sklib_pins(pin);
    let __skparam__dutycycle = __skadapter__to_sklib_int(dutycycle);
    unsafe {
        __sklib__raspi_set_pwm_dutycycle__pins__int(__skparam__pin, __skparam__dutycycle);
    }
}
pub fn raspi_set_pwm_frequency(pin: Pins, frequency: i32) {
    let __skparam__pin = __skadapter__to_sklib_pins(pin);
    let __skparam__frequency = __skadapter__to_sklib_int(frequency);
    unsafe {
        __sklib__raspi_set_pwm_frequency__pins__int(__skparam__pin, __skparam__frequency);
    }
}
pub fn raspi_set_pwm_range(pin: Pins, range: i32) {
    let __skparam__pin = __skadapter__to_sklib_pins(pin);
    let __skparam__range = __skadapter__to_sklib_int(range);
    unsafe {
        __sklib__raspi_set_pwm_range__pins__int(__skparam__pin, __skparam__range);
    }
}
pub fn raspi_write(pin: Pins, value: PinValues) {
    let __skparam__pin = __skadapter__to_sklib_pins(pin);
    let __skparam__value = __skadapter__to_sklib_pin_values(value);
    unsafe {
        __sklib__raspi_write__pins__pin_values(__skparam__pin, __skparam__value);
    }
}
pub fn draw_quad(clr: Color, q: Quad) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        __sklib__draw_quad__color__quad_ref(__skparam__clr, __skparam__q);
    }
}
pub fn draw_quad_with_options(clr: Color, q: Quad, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_quad__color__quad_ref__drawing_options_ref(__skparam__clr, __skparam__q, __skparam__opts);
    }
}
pub fn draw_quad_on_bitmap(destination: Bitmap, clr: Color, q: Quad) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        __sklib__draw_quad_on_bitmap__bitmap__color__quad_ref(__skparam__destination, __skparam__clr, __skparam__q);
    }
}
pub fn draw_quad_on_bitmap_with_options(destination: Bitmap, clr: Color, q: Quad, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_quad_on_bitmap__bitmap__color__quad_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__q, __skparam__opts);
    }
}
pub fn draw_quad_on_window(destination: Window, clr: Color, q: Quad) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        __sklib__draw_quad_on_window__window__color__quad_ref(__skparam__destination, __skparam__clr, __skparam__q);
    }
}
pub fn draw_quad_on_window_with_options(destination: Window, clr: Color, q: Quad, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_quad_on_window__window__color__quad_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__q, __skparam__opts);
    }
}
pub fn draw_rectangle_record(clr: Color, rect: Rectangle) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__draw_rectangle__color__rectangle_ref(__skparam__clr, __skparam__rect);
    }
}
pub fn draw_rectangle_record_with_options(clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_rectangle__color__rectangle_ref__drawing_options_ref(__skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn draw_rectangle(clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__draw_rectangle__color__double__double__double__double(__skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn draw_rectangle_with_options(clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_rectangle__color__double__double__double__double__drawing_options_ref(__skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn draw_rectangle_on_bitmap_record(destination: Bitmap, clr: Color, rect: Rectangle) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__draw_rectangle_on_bitmap__bitmap__color__rectangle_ref(__skparam__destination, __skparam__clr, __skparam__rect);
    }
}
pub fn draw_rectangle_on_bitmap_record_with_options(destination: Bitmap, clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_rectangle_on_bitmap__bitmap__color__rectangle_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn draw_rectangle_on_bitmap(destination: Bitmap, clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__draw_rectangle_on_bitmap__bitmap__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn draw_rectangle_on_bitmap_with_options(destination: Bitmap, clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_rectangle_on_bitmap__bitmap__color__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn draw_rectangle_on_window_record(destination: Window, clr: Color, rect: Rectangle) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__draw_rectangle_on_window__window__color__rectangle_ref(__skparam__destination, __skparam__clr, __skparam__rect);
    }
}
pub fn draw_rectangle_on_window_record_with_options(destination: Window, clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_rectangle_on_window__window__color__rectangle_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn draw_rectangle_on_window(destination: Window, clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__draw_rectangle_on_window__window__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn draw_rectangle_on_window_with_options(destination: Window, clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_rectangle_on_window__window__color__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn fill_quad(clr: Color, q: Quad) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        __sklib__fill_quad__color__quad_ref(__skparam__clr, __skparam__q);
    }
}
pub fn fill_quad_with_options(clr: Color, q: Quad, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_quad__color__quad_ref__drawing_options_ref(__skparam__clr, __skparam__q, __skparam__opts);
    }
}
pub fn fill_quad_on_bitmap(destination: Bitmap, clr: Color, q: Quad) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        __sklib__fill_quad_on_bitmap__bitmap__color__quad_ref(__skparam__destination, __skparam__clr, __skparam__q);
    }
}
pub fn fill_quad_on_bitmap_with_options(destination: Bitmap, clr: Color, q: Quad, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_quad_on_bitmap__bitmap__color__quad_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__q, __skparam__opts);
    }
}
pub fn fill_quad_on_window(destination: Window, clr: Color, q: Quad) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        __sklib__fill_quad_on_window__window__color__quad_ref(__skparam__destination, __skparam__clr, __skparam__q);
    }
}
pub fn fill_quad_on_window_with_options(destination: Window, clr: Color, q: Quad, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__q = __skadapter__to_sklib_quad(q);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_quad_on_window__window__color__quad_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__q, __skparam__opts);
    }
}
pub fn fill_rectangle_record(clr: Color, rect: Rectangle) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__fill_rectangle__color__rectangle_ref(__skparam__clr, __skparam__rect);
    }
}
pub fn fill_rectangle_record_with_options(clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_rectangle__color__rectangle_ref__drawing_options_ref(__skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn fill_rectangle(clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__fill_rectangle__color__double__double__double__double(__skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn fill_rectangle_with_options(clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_rectangle__color__double__double__double__double__drawing_options_ref(__skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn fill_rectangle_on_bitmap_record(destination: Bitmap, clr: Color, rect: Rectangle) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__fill_rectangle_on_bitmap__bitmap__color__rectangle_ref(__skparam__destination, __skparam__clr, __skparam__rect);
    }
}
pub fn fill_rectangle_on_bitmap_record_with_options(destination: Bitmap, clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_rectangle_on_bitmap__bitmap__color__rectangle_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn fill_rectangle_on_bitmap(destination: Bitmap, clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__fill_rectangle_on_bitmap__bitmap__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn fill_rectangle_on_bitmap_with_options(destination: Bitmap, clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_rectangle_on_bitmap__bitmap__color__double__double__double__double__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn fill_rectangle_on_window_record(destination: Window, clr: Color, rect: Rectangle) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__fill_rectangle_on_window__window__color__rectangle_ref(__skparam__destination, __skparam__clr, __skparam__rect);
    }
}
pub fn fill_rectangle_on_window_record_with_options(destination: Window, clr: Color, rect: Rectangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_rectangle_on_window__window__color__rectangle_ref__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__rect, __skparam__opts);
    }
}
pub fn fill_rectangle_on_window(destination: Window, clr: Color, x: f64, y: f64, width: f64, height: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        __sklib__fill_rectangle_on_window__window__color__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height);
    }
}
pub fn fill_rectangle_on_window_with_options(destination: Window, clr: Color, x: f64, y: f64, width: f64, height: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_rectangle_on_window__window__color__double__double__double__double__drawing_options_ref(__skparam__destination, __skparam__clr, __skparam__x, __skparam__y, __skparam__width, __skparam__height, __skparam__opts);
    }
}
pub fn inset_rectangle(rect: Rectangle, inset_amount: f32) -> Rectangle {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__inset_amount = __skadapter__to_sklib_float(inset_amount);
    unsafe {
        let __skreturn = __sklib__inset_rectangle__rectangle_ref__float(__skparam__rect, __skparam__inset_amount);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn intersection(rect1: Rectangle, rect2: Rectangle) -> Rectangle {
    let __skparam__rect1 = __skadapter__to_sklib_rectangle(rect1);
    let __skparam__rect2 = __skadapter__to_sklib_rectangle(rect2);
    unsafe {
        let __skreturn = __sklib__intersection__rectangle_ref__rectangle_ref(__skparam__rect1, __skparam__rect2);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_around_circle(c: Circle) -> Rectangle {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    unsafe {
        let __skreturn = __sklib__rectangle_around__circle_ref(__skparam__c);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_around_line(l: Line) -> Rectangle {
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__rectangle_around__line_ref(__skparam__l);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_around_quad(q: Quad) -> Rectangle {
    let __skparam__q = __skadapter__to_sklib_quad(q);
    unsafe {
        let __skreturn = __sklib__rectangle_around__quad_ref(__skparam__q);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_around_triangle(t: Triangle) -> Rectangle {
    let __skparam__t = __skadapter__to_sklib_triangle(t);
    unsafe {
        let __skreturn = __sklib__rectangle_around__triangle_ref(__skparam__t);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_bottom(rect: Rectangle) -> f32 {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__rectangle_bottom__rectangle_ref(__skparam__rect);
        __skadapter__to_float(__skreturn)
    }
}
pub fn rectangle_center(rect: Rectangle) -> Point2D {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__rectangle_center__rectangle_ref(__skparam__rect);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn rectangle_from_point_and_size(pt: Point2D, width: f64, height: f64) -> Rectangle {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        let __skreturn = __sklib__rectangle_from__point_2d__double__double(__skparam__pt, __skparam__width, __skparam__height);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_from_points(pt1: Point2D, pt2: Point2D) -> Rectangle {
    let __skparam__pt1 = __skadapter__to_sklib_point_2d(pt1);
    let __skparam__pt2 = __skadapter__to_sklib_point_2d(pt2);
    unsafe {
        let __skreturn = __sklib__rectangle_from__point_2d__point_2d(__skparam__pt1, __skparam__pt2);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_from(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__width = __skadapter__to_sklib_double(width);
    let __skparam__height = __skadapter__to_sklib_double(height);
    unsafe {
        let __skreturn = __sklib__rectangle_from__double__double__double__double(__skparam__x, __skparam__y, __skparam__width, __skparam__height);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_left(rect: Rectangle) -> f32 {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__rectangle_left__rectangle_ref(__skparam__rect);
        __skadapter__to_float(__skreturn)
    }
}
pub fn rectangle_offset_by(rect: Rectangle, offset: Vector2D) -> Rectangle {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__offset = __skadapter__to_sklib_vector_2d(offset);
    unsafe {
        let __skreturn = __sklib__rectangle_offset_by__rectangle_ref__vector_2d_ref(__skparam__rect, __skparam__offset);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn rectangle_right(rect: Rectangle) -> f32 {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__rectangle_right__rectangle_ref(__skparam__rect);
        __skadapter__to_float(__skreturn)
    }
}
pub fn rectangle_to_string(rect: Rectangle) -> String {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__rectangle_to_string__rectangle_ref(__skparam__rect);
        __skadapter__to_string(__skreturn)
    }
}
pub fn rectangle_top(rect: Rectangle) -> f32 {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__rectangle_top__rectangle_ref(__skparam__rect);
        __skadapter__to_float(__skreturn)
    }
}
pub fn rectangles_intersect(rect1: Rectangle, rect2: Rectangle) -> bool {
    let __skparam__rect1 = __skadapter__to_sklib_rectangle(rect1);
    let __skparam__rect2 = __skadapter__to_sklib_rectangle(rect2);
    unsafe {
        let __skreturn = __sklib__rectangles_intersect__rectangle_ref__rectangle_ref(__skparam__rect1, __skparam__rect2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn deregister_free_notifier(handler: FreeNotifier) {
    let __skparam__handler = __skadapter__to_sklib_free_notifier(handler);
    unsafe {
        __sklib__deregister_free_notifier__free_notifier_ptr(__skparam__handler);
    }
}
pub fn path_to_resource(filename: String, kind: ResourceKind) -> String {
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    let __skparam__kind = __skadapter__to_sklib_resource_kind(kind);
    unsafe {
        let __skreturn = __sklib__path_to_resource__string_ref__resource_kind(__skparam__filename, __skparam__kind);
        __skadapter__to_string(__skreturn)
    }
}
pub fn path_to_resources() -> String {
    unsafe {
        let __skreturn = __sklib__path_to_resources();
        __skadapter__to_string(__skreturn)
    }
}
pub fn path_to_resources_for_kind(kind: ResourceKind) -> String {
    let __skparam__kind = __skadapter__to_sklib_resource_kind(kind);
    unsafe {
        let __skreturn = __sklib__path_to_resources__resource_kind(__skparam__kind);
        __skadapter__to_string(__skreturn)
    }
}
pub fn register_free_notifier(r#fn: FreeNotifier) {
    let __skparam__fn = __skadapter__to_sklib_free_notifier(r#fn);
    unsafe {
        __sklib__register_free_notifier__free_notifier_ptr(__skparam__fn);
    }
}
pub fn set_resources_path(path: String) {
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        __sklib__set_resources_path__string_ref(__skparam__path);
    }
}
pub fn fade_all_sound_effects_out(ms: i32) {
    let __skparam__ms = __skadapter__to_sklib_int(ms);
    unsafe {
        __sklib__fade_all_sound_effects_out__int(__skparam__ms);
    }
}
pub fn fade_sound_effect_out(effect: SoundEffect, ms: i32) {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    let __skparam__ms = __skadapter__to_sklib_int(ms);
    unsafe {
        __sklib__fade_sound_effect_out__sound_effect__int(__skparam__effect, __skparam__ms);
    }
}
pub fn free_all_sound_effects() {
    unsafe {
        __sklib__free_all_sound_effects();
    }
}
pub fn free_sound_effect(effect: SoundEffect) {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    unsafe {
        __sklib__free_sound_effect__sound_effect(__skparam__effect);
    }
}
pub fn has_sound_effect(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_sound_effect__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn load_sound_effect(name: String, filename: String) -> SoundEffect {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        let __skreturn = __sklib__load_sound_effect__string_ref__string_ref(__skparam__name, __skparam__filename);
        __skadapter__to_sound_effect(__skreturn)
    }
}
pub fn play_sound_effect_named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__play_sound_effect__string_ref(__skparam__name);
    }
}
pub fn play_sound_effect_named_with_volume(name: String, volume: f64) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__volume = __skadapter__to_sklib_double(volume);
    unsafe {
        __sklib__play_sound_effect__string_ref__double(__skparam__name, __skparam__volume);
    }
}
pub fn play_sound_effect_named_with_times(name: String, times: i32) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__times = __skadapter__to_sklib_int(times);
    unsafe {
        __sklib__play_sound_effect__string_ref__int(__skparam__name, __skparam__times);
    }
}
pub fn play_sound_effect_named_with_times_and_volume(name: String, times: i32, volume: f64) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__times = __skadapter__to_sklib_int(times);
    let __skparam__volume = __skadapter__to_sklib_double(volume);
    unsafe {
        __sklib__play_sound_effect__string_ref__int__double(__skparam__name, __skparam__times, __skparam__volume);
    }
}
pub fn play_sound_effect(effect: SoundEffect) {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    unsafe {
        __sklib__play_sound_effect__sound_effect(__skparam__effect);
    }
}
pub fn play_sound_effect_with_volume(effect: SoundEffect, volume: f64) {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    let __skparam__volume = __skadapter__to_sklib_double(volume);
    unsafe {
        __sklib__play_sound_effect__sound_effect__double(__skparam__effect, __skparam__volume);
    }
}
pub fn play_sound_effect_with_times(effect: SoundEffect, times: i32) {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    let __skparam__times = __skadapter__to_sklib_int(times);
    unsafe {
        __sklib__play_sound_effect__sound_effect__int(__skparam__effect, __skparam__times);
    }
}
pub fn play_sound_effect_with_times_and_volume(effect: SoundEffect, times: i32, volume: f64) {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    let __skparam__times = __skadapter__to_sklib_int(times);
    let __skparam__volume = __skadapter__to_sklib_double(volume);
    unsafe {
        __sklib__play_sound_effect__sound_effect__int__double(__skparam__effect, __skparam__times, __skparam__volume);
    }
}
pub fn sound_effect_filename(effect: SoundEffect) -> String {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    unsafe {
        let __skreturn = __sklib__sound_effect_filename__sound_effect(__skparam__effect);
        __skadapter__to_string(__skreturn)
    }
}
pub fn sound_effect_name(effect: SoundEffect) -> String {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    unsafe {
        let __skreturn = __sklib__sound_effect_name__sound_effect(__skparam__effect);
        __skadapter__to_string(__skreturn)
    }
}
pub fn sound_effect_named(name: String) -> SoundEffect {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sound_effect_named__string_ref(__skparam__name);
        __skadapter__to_sound_effect(__skreturn)
    }
}
pub fn sound_effect_playing_named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sound_effect_playing__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sound_effect_playing(effect: SoundEffect) -> bool {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    unsafe {
        let __skreturn = __sklib__sound_effect_playing__sound_effect(__skparam__effect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sound_effect_valid(effect: SoundEffect) -> bool {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    unsafe {
        let __skreturn = __sklib__sound_effect_valid__sound_effect(__skparam__effect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn stop_sound_effect_named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__stop_sound_effect__string_ref(__skparam__name);
    }
}
pub fn stop_sound_effect(effect: SoundEffect) {
    let __skparam__effect = __skadapter__to_sklib_sound_effect(effect);
    unsafe {
        __sklib__stop_sound_effect__sound_effect(__skparam__effect);
    }
}
pub fn call_for_all_sprites_with_value(r#fn: SpriteFloatFunction, val: f32) {
    let __skparam__fn = __skadapter__to_sklib_sprite_float_function(r#fn);
    let __skparam__val = __skadapter__to_sklib_float(val);
    unsafe {
        __sklib__call_for_all_sprites__sprite_float_function_ptr__float(__skparam__fn, __skparam__val);
    }
}
pub fn call_for_all_sprites(r#fn: SpriteFunction) {
    let __skparam__fn = __skadapter__to_sklib_sprite_function(r#fn);
    unsafe {
        __sklib__call_for_all_sprites__sprite_function_ptr(__skparam__fn);
    }
}
pub fn call_on_sprite_event(handler: SpriteEventHandler) {
    let __skparam__handler = __skadapter__to_sklib_sprite_event_handler(handler);
    unsafe {
        __sklib__call_on_sprite_event__sprite_event_handler_ptr(__skparam__handler);
    }
}
pub fn center_point_of_sprite(s: Sprite) -> Point2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__center_point__sprite(__skparam__s);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn create_sprite(layer: Bitmap) -> Sprite {
    let __skparam__layer = __skadapter__to_sklib_bitmap(layer);
    unsafe {
        let __skreturn = __sklib__create_sprite__bitmap(__skparam__layer);
        __skadapter__to_sprite(__skreturn)
    }
}
pub fn create_sprite_with_animation(layer: Bitmap, ani: AnimationScript) -> Sprite {
    let __skparam__layer = __skadapter__to_sklib_bitmap(layer);
    let __skparam__ani = __skadapter__to_sklib_animation_script(ani);
    unsafe {
        let __skreturn = __sklib__create_sprite__bitmap__animation_script(__skparam__layer, __skparam__ani);
        __skadapter__to_sprite(__skreturn)
    }
}
pub fn create_sprite_with_bitmap_named(bitmap_name: String) -> Sprite {
    let __skparam__bitmap_name = __skadapter__to_sklib_string(bitmap_name);
    unsafe {
        let __skreturn = __sklib__create_sprite__string_ref(__skparam__bitmap_name);
        __skadapter__to_sprite(__skreturn)
    }
}
pub fn create_sprite_named(name: String, layer: Bitmap) -> Sprite {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__layer = __skadapter__to_sklib_bitmap(layer);
    unsafe {
        let __skreturn = __sklib__create_sprite__string_ref__bitmap(__skparam__name, __skparam__layer);
        __skadapter__to_sprite(__skreturn)
    }
}
pub fn create_sprite_named_with_animation(name: String, layer: Bitmap, ani: AnimationScript) -> Sprite {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__layer = __skadapter__to_sklib_bitmap(layer);
    let __skparam__ani = __skadapter__to_sklib_animation_script(ani);
    unsafe {
        let __skreturn = __sklib__create_sprite__string_ref__bitmap__animation_script(__skparam__name, __skparam__layer, __skparam__ani);
        __skadapter__to_sprite(__skreturn)
    }
}
pub fn create_sprite_with_bitmap_and_animation_named(bitmap_name: String, animation_name: String) -> Sprite {
    let __skparam__bitmap_name = __skadapter__to_sklib_string(bitmap_name);
    let __skparam__animation_name = __skadapter__to_sklib_string(animation_name);
    unsafe {
        let __skreturn = __sklib__create_sprite__string_ref__string_ref(__skparam__bitmap_name, __skparam__animation_name);
        __skadapter__to_sprite(__skreturn)
    }
}
pub fn create_sprite_pack(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__create_sprite_pack__string_ref(__skparam__name);
    }
}
pub fn current_sprite_pack() -> String {
    unsafe {
        let __skreturn = __sklib__current_sprite_pack();
        __skadapter__to_string(__skreturn)
    }
}
pub fn draw_all_sprites() {
    unsafe {
        __sklib__draw_all_sprites();
    }
}
pub fn draw_sprite_offset_by(s: Sprite, offset: Vector2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__offset = __skadapter__to_sklib_vector_2d(offset);
    unsafe {
        __sklib__draw_sprite__sprite__vector_2d_ref(__skparam__s, __skparam__offset);
    }
}
pub fn draw_sprite(s: Sprite) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        __sklib__draw_sprite__sprite(__skparam__s);
    }
}
pub fn draw_sprite_offset_x_y(s: Sprite, x_offset: f64, y_offset: f64) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__x_offset = __skadapter__to_sklib_double(x_offset);
    let __skparam__y_offset = __skadapter__to_sklib_double(y_offset);
    unsafe {
        __sklib__draw_sprite__sprite__double__double(__skparam__s, __skparam__x_offset, __skparam__y_offset);
    }
}
pub fn free_all_sprites() {
    unsafe {
        __sklib__free_all_sprites();
    }
}
pub fn free_sprite(s: Sprite) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        __sklib__free_sprite__sprite(__skparam__s);
    }
}
pub fn free_sprite_pack(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__free_sprite_pack__string_ref(__skparam__name);
    }
}
pub fn has_sprite(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_sprite__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_sprite_pack(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_sprite_pack__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn move_sprite(s: Sprite) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        __sklib__move_sprite__sprite(__skparam__s);
    }
}
pub fn move_sprite_by_vector(s: Sprite, distance: Vector2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__distance = __skadapter__to_sklib_vector_2d(distance);
    unsafe {
        __sklib__move_sprite__sprite__vector_2d_ref(__skparam__s, __skparam__distance);
    }
}
pub fn move_sprite_by_vector_percent(s: Sprite, distance: Vector2D, pct: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__distance = __skadapter__to_sklib_vector_2d(distance);
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    unsafe {
        __sklib__move_sprite__sprite__vector_2d_ref__float(__skparam__s, __skparam__distance, __skparam__pct);
    }
}
pub fn move_sprite_percent(s: Sprite, pct: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    unsafe {
        __sklib__move_sprite__sprite__float(__skparam__s, __skparam__pct);
    }
}
pub fn move_sprite_to(s: Sprite, x: f64, y: f64) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__move_sprite_to__sprite__double__double(__skparam__s, __skparam__x, __skparam__y);
    }
}
pub fn select_sprite_pack(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__select_sprite_pack__string_ref(__skparam__name);
    }
}
pub fn sprite_add_layer(s: Sprite, new_layer: Bitmap, layer_name: String) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__new_layer = __skadapter__to_sklib_bitmap(new_layer);
    let __skparam__layer_name = __skadapter__to_sklib_string(layer_name);
    unsafe {
        let __skreturn = __sklib__sprite_add_layer__sprite__bitmap__string_ref(__skparam__s, __skparam__new_layer, __skparam__layer_name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_add_to_velocity(s: Sprite, value: Vector2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_vector_2d(value);
    unsafe {
        __sklib__sprite_add_to_velocity__sprite__vector_2d_ref(__skparam__s, __skparam__value);
    }
}
pub fn sprite_add_value(s: Sprite, name: String) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__sprite_add_value__sprite__string_ref(__skparam__s, __skparam__name);
    }
}
pub fn sprite_add_value_with_default(s: Sprite, name: String, init_val: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__init_val = __skadapter__to_sklib_float(init_val);
    unsafe {
        __sklib__sprite_add_value__sprite__string_ref__float(__skparam__s, __skparam__name, __skparam__init_val);
    }
}
pub fn sprite_anchor_point(s: Sprite) -> Point2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_anchor_point__sprite(__skparam__s);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn sprite_anchor_position(s: Sprite) -> Point2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_anchor_position__sprite(__skparam__s);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn sprite_animation_has_ended(s: Sprite) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_animation_has_ended__sprite(__skparam__s);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_animation_name(s: Sprite) -> String {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_animation_name__sprite(__skparam__s);
        __skadapter__to_string(__skreturn)
    }
}
pub fn sprite_at(s: Sprite, pt: Point2D) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__sprite_at__sprite__point_2d_ref(__skparam__s, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_bring_layer_forward(s: Sprite, visible_layer: i32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__visible_layer = __skadapter__to_sklib_int(visible_layer);
    unsafe {
        __sklib__sprite_bring_layer_forward__sprite__int(__skparam__s, __skparam__visible_layer);
    }
}
pub fn sprite_bring_layer_to_front(s: Sprite, visible_layer: i32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__visible_layer = __skadapter__to_sklib_int(visible_layer);
    unsafe {
        __sklib__sprite_bring_layer_to_front__sprite__int(__skparam__s, __skparam__visible_layer);
    }
}
pub fn sprite_call_on_event(s: Sprite, handler: SpriteEventHandler) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__handler = __skadapter__to_sklib_sprite_event_handler(handler);
    unsafe {
        __sklib__sprite_call_on_event__sprite__sprite_event_handler_ptr(__skparam__s, __skparam__handler);
    }
}
pub fn sprite_circle(s: Sprite) -> Circle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_circle__sprite(__skparam__s);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn sprite_collision_bitmap(s: Sprite) -> Bitmap {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_collision_bitmap__sprite(__skparam__s);
        __skadapter__to_bitmap(__skreturn)
    }
}
pub fn sprite_collision_circle(s: Sprite) -> Circle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_collision_circle__sprite(__skparam__s);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn sprite_collision_kind(s: Sprite) -> CollisionTestKind {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_collision_kind__sprite(__skparam__s);
        __skadapter__to_collision_test_kind(__skreturn)
    }
}
pub fn sprite_collision_rectangle(s: Sprite) -> Rectangle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_collision_rectangle__sprite(__skparam__s);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn sprite_current_cell(s: Sprite) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_current_cell__sprite(__skparam__s);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_current_cell_rectangle(s: Sprite) -> Rectangle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_current_cell_rectangle__sprite(__skparam__s);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn sprite_dx(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_dx__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_dy(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_dy__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_has_value(s: Sprite, name: String) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_has_value__sprite__string(__skparam__s, __skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_heading(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_heading__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_height(s: Sprite) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_height__sprite(__skparam__s);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_hide_layer_named(s: Sprite, name: String) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__sprite_hide_layer__sprite__string_ref(__skparam__s, __skparam__name);
    }
}
pub fn sprite_hide_layer(s: Sprite, id: i32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__id = __skadapter__to_sklib_int(id);
    unsafe {
        __sklib__sprite_hide_layer__sprite__int(__skparam__s, __skparam__id);
    }
}
pub fn sprite_layer_named(s: Sprite, name: String) -> Bitmap {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_layer__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_bitmap(__skreturn)
    }
}
pub fn sprite_layer_at_index(s: Sprite, idx: i32) -> Bitmap {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_layer__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_bitmap(__skreturn)
    }
}
pub fn sprite_layer_circle_named(s: Sprite, name: String) -> Circle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_layer_circle__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn sprite_layer_circle_at_index(s: Sprite, idx: i32) -> Circle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_layer_circle__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_circle(__skreturn)
    }
}
pub fn sprite_layer_count(s: Sprite) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_layer_count__sprite(__skparam__s);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_layer_height_named(s: Sprite, name: String) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_layer_height__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_layer_height(s: Sprite, idx: i32) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_layer_height__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_layer_index(s: Sprite, name: String) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_layer_index__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_layer_name(s: Sprite, idx: i32) -> String {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_layer_name__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_string(__skreturn)
    }
}
pub fn sprite_layer_offset_named(s: Sprite, name: String) -> Vector2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_layer_offset__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn sprite_layer_offset(s: Sprite, idx: i32) -> Vector2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_layer_offset__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn sprite_layer_rectangle_named(s: Sprite, name: String) -> Rectangle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_layer_rectangle__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn sprite_layer_rectangle_at_index(s: Sprite, idx: i32) -> Rectangle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_layer_rectangle__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn sprite_layer_width_named(s: Sprite, name: String) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_layer_width__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_layer_width(s: Sprite, idx: i32) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_layer_width__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_location_matrix(s: Sprite) -> Matrix2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_location_matrix__sprite(__skparam__s);
        __skadapter__to_matrix_2d(__skreturn)
    }
}
pub fn sprite_mass(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_mass__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_move_from_anchor_point(s: Sprite) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_move_from_anchor_point__sprite(__skparam__s);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_move_to_taking_seconds(s: Sprite, pt: Point2D, taking_seconds: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__taking_seconds = __skadapter__to_sklib_float(taking_seconds);
    unsafe {
        __sklib__sprite_move_to__sprite__point_2d_ref__float(__skparam__s, __skparam__pt, __skparam__taking_seconds);
    }
}
pub fn sprite_name(s: Sprite) -> String {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_name__sprite(__skparam__s);
        __skadapter__to_string(__skreturn)
    }
}
pub fn sprite_named(name: String) -> Sprite {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_named__string_ref(__skparam__name);
        __skadapter__to_sprite(__skreturn)
    }
}
pub fn sprite_offscreen(s: Sprite) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_offscreen__sprite(__skparam__s);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_on_screen_at_point(s: Sprite, pt: Point2D) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__sprite_on_screen_at__sprite__point_2d_ref(__skparam__s, __skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_on_screen_at(s: Sprite, x: f64, y: f64) -> bool {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__sprite_on_screen_at__sprite__double__double(__skparam__s, __skparam__x, __skparam__y);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn sprite_position(s: Sprite) -> Point2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_position__sprite(__skparam__s);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn sprite_replay_animation(s: Sprite) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        __sklib__sprite_replay_animation__sprite(__skparam__s);
    }
}
pub fn sprite_replay_animation_with_sound(s: Sprite, with_sound: bool) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__sprite_replay_animation__sprite__bool(__skparam__s, __skparam__with_sound);
    }
}
pub fn sprite_rotation(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_rotation__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_scale(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_scale__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_screen_rectangle(s: Sprite) -> Rectangle {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_screen_rectangle__sprite(__skparam__s);
        __skadapter__to_rectangle(__skreturn)
    }
}
pub fn sprite_send_layer_backward(s: Sprite, visible_layer: i32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__visible_layer = __skadapter__to_sklib_int(visible_layer);
    unsafe {
        __sklib__sprite_send_layer_backward__sprite__int(__skparam__s, __skparam__visible_layer);
    }
}
pub fn sprite_send_layer_to_back(s: Sprite, visible_layer: i32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__visible_layer = __skadapter__to_sklib_int(visible_layer);
    unsafe {
        __sklib__sprite_send_layer_to_back__sprite__int(__skparam__s, __skparam__visible_layer);
    }
}
pub fn sprite_set_anchor_point(s: Sprite, pt: Point2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        __sklib__sprite_set_anchor_point__sprite__point_2d_ref(__skparam__s, __skparam__pt);
    }
}
pub fn sprite_set_collision_bitmap(s: Sprite, bmp: Bitmap) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        __sklib__sprite_set_collision_bitmap__sprite__bitmap(__skparam__s, __skparam__bmp);
    }
}
pub fn sprite_set_collision_kind(s: Sprite, value: CollisionTestKind) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_collision_test_kind(value);
    unsafe {
        __sklib__sprite_set_collision_kind__sprite__collision_test_kind(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_dx(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_dx__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_dy(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_dy__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_heading(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_heading__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_layer_offset_named(s: Sprite, name: String, value: Vector2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__value = __skadapter__to_sklib_vector_2d(value);
    unsafe {
        __sklib__sprite_set_layer_offset__sprite__string_ref__vector_2d_ref(__skparam__s, __skparam__name, __skparam__value);
    }
}
pub fn sprite_set_layer_offset_at_index(s: Sprite, idx: i32, value: Vector2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    let __skparam__value = __skadapter__to_sklib_vector_2d(value);
    unsafe {
        __sklib__sprite_set_layer_offset__sprite__int__vector_2d_ref(__skparam__s, __skparam__idx, __skparam__value);
    }
}
pub fn sprite_set_mass(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_mass__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_move_from_anchor_point(s: Sprite, value: bool) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_bool(value);
    unsafe {
        __sklib__sprite_set_move_from_anchor_point__sprite__bool(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_position(s: Sprite, value: Point2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_point_2d(value);
    unsafe {
        __sklib__sprite_set_position__sprite__point_2d_ref(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_rotation(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_rotation__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_scale(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_scale__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_speed(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_speed__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_value_named(s: Sprite, name: String, val: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__val = __skadapter__to_sklib_float(val);
    unsafe {
        __sklib__sprite_set_value__sprite__string_ref__float(__skparam__s, __skparam__name, __skparam__val);
    }
}
pub fn sprite_set_velocity(s: Sprite, value: Vector2D) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_vector_2d(value);
    unsafe {
        __sklib__sprite_set_velocity__sprite__vector_2d_ref(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_x(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_x__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_set_y(s: Sprite, value: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__value = __skadapter__to_sklib_float(value);
    unsafe {
        __sklib__sprite_set_y__sprite__float(__skparam__s, __skparam__value);
    }
}
pub fn sprite_show_layer_named(s: Sprite, name: String) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_show_layer__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_show_layer(s: Sprite, id: i32) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__id = __skadapter__to_sklib_int(id);
    unsafe {
        let __skreturn = __sklib__sprite_show_layer__sprite__int(__skparam__s, __skparam__id);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_speed(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_speed__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_start_animation_named(s: Sprite, named: String) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__named = __skadapter__to_sklib_string(named);
    unsafe {
        __sklib__sprite_start_animation__sprite__string_ref(__skparam__s, __skparam__named);
    }
}
pub fn sprite_start_animation_named_with_sound(s: Sprite, named: String, with_sound: bool) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__named = __skadapter__to_sklib_string(named);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__sprite_start_animation__sprite__string_ref__bool(__skparam__s, __skparam__named, __skparam__with_sound);
    }
}
pub fn sprite_start_animation(s: Sprite, idx: i32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        __sklib__sprite_start_animation__sprite__int(__skparam__s, __skparam__idx);
    }
}
pub fn sprite_start_animation_with_sound(s: Sprite, idx: i32, with_sound: bool) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__sprite_start_animation__sprite__int__bool(__skparam__s, __skparam__idx, __skparam__with_sound);
    }
}
pub fn sprite_stop_calling_on_event(s: Sprite, handler: SpriteEventHandler) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__handler = __skadapter__to_sklib_sprite_event_handler(handler);
    unsafe {
        __sklib__sprite_stop_calling_on_event__sprite__sprite_event_handler_ptr(__skparam__s, __skparam__handler);
    }
}
pub fn sprite_toggle_layer_visible_named(s: Sprite, name: String) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__sprite_toggle_layer_visible__sprite__string_ref(__skparam__s, __skparam__name);
    }
}
pub fn sprite_toggle_layer_visible(s: Sprite, id: i32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__id = __skadapter__to_sklib_int(id);
    unsafe {
        __sklib__sprite_toggle_layer_visible__sprite__int(__skparam__s, __skparam__id);
    }
}
pub fn sprite_value(s: Sprite, name: String) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_value__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_value_count(s: Sprite) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_value_count__sprite(__skparam__s);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_velocity(s: Sprite) -> Vector2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_velocity__sprite(__skparam__s);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn sprite_visible_index_of_layer_named(s: Sprite, name: String) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__sprite_visible_index_of_layer__sprite__string_ref(__skparam__s, __skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_visible_index_of_layer(s: Sprite, id: i32) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__id = __skadapter__to_sklib_int(id);
    unsafe {
        let __skreturn = __sklib__sprite_visible_index_of_layer__sprite__int(__skparam__s, __skparam__id);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_visible_layer(s: Sprite, idx: i32) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_visible_layer__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_visible_layer_count(s: Sprite) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_visible_layer_count__sprite(__skparam__s);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_visible_layer_id(s: Sprite, idx: i32) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__idx = __skadapter__to_sklib_int(idx);
    unsafe {
        let __skreturn = __sklib__sprite_visible_layer_id__sprite__int(__skparam__s, __skparam__idx);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_width(s: Sprite) -> i32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_width__sprite(__skparam__s);
        __skadapter__to_int(__skreturn)
    }
}
pub fn sprite_x(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_x__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn sprite_y(s: Sprite) -> f32 {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        let __skreturn = __sklib__sprite_y__sprite(__skparam__s);
        __skadapter__to_float(__skreturn)
    }
}
pub fn stop_calling_on_sprite_event(handler: SpriteEventHandler) {
    let __skparam__handler = __skadapter__to_sklib_sprite_event_handler(handler);
    unsafe {
        __sklib__stop_calling_on_sprite_event__sprite_event_handler_ptr(__skparam__handler);
    }
}
pub fn update_all_sprites() {
    unsafe {
        __sklib__update_all_sprites();
    }
}
pub fn update_all_sprites_percent(pct: f32) {
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    unsafe {
        __sklib__update_all_sprites__float(__skparam__pct);
    }
}
pub fn update_sprite(s: Sprite) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        __sklib__update_sprite__sprite(__skparam__s);
    }
}
pub fn update_sprite_with_sound(s: Sprite, with_sound: bool) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__update_sprite__sprite__bool(__skparam__s, __skparam__with_sound);
    }
}
pub fn update_sprite_percent(s: Sprite, pct: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    unsafe {
        __sklib__update_sprite__sprite__float(__skparam__s, __skparam__pct);
    }
}
pub fn update_sprite_percent_with_sound(s: Sprite, pct: f32, with_sound: bool) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__update_sprite__sprite__float__bool(__skparam__s, __skparam__pct, __skparam__with_sound);
    }
}
pub fn update_sprite_animation(s: Sprite) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    unsafe {
        __sklib__update_sprite_animation__sprite(__skparam__s);
    }
}
pub fn update_sprite_animation_with_sound(s: Sprite, with_sound: bool) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__update_sprite_animation__sprite__bool(__skparam__s, __skparam__with_sound);
    }
}
pub fn update_sprite_animation_percent(s: Sprite, pct: f32) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    unsafe {
        __sklib__update_sprite_animation__sprite__float(__skparam__s, __skparam__pct);
    }
}
pub fn update_sprite_animation_percent_with_sound(s: Sprite, pct: f32, with_sound: bool) {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pct = __skadapter__to_sklib_float(pct);
    let __skparam__with_sound = __skadapter__to_sklib_bool(with_sound);
    unsafe {
        __sklib__update_sprite_animation__sprite__float__bool(__skparam__s, __skparam__pct, __skparam__with_sound);
    }
}
pub fn vector_from_center_sprite_to_point_point(s: Sprite, pt: Point2D) -> Vector2D {
    let __skparam__s = __skadapter__to_sklib_sprite(s);
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    unsafe {
        let __skreturn = __sklib__vector_from_center_sprite_to_point__sprite__point_2d_ref(__skparam__s, __skparam__pt);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_from_to(s1: Sprite, s2: Sprite) -> Vector2D {
    let __skparam__s1 = __skadapter__to_sklib_sprite(s1);
    let __skparam__s2 = __skadapter__to_sklib_sprite(s2);
    unsafe {
        let __skreturn = __sklib__vector_from_to__sprite__sprite(__skparam__s1, __skparam__s2);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn read_char() -> char {
    unsafe {
        let __skreturn = __sklib__read_char();
        __skadapter__to_char(__skreturn)
    }
}
pub fn read_line() -> String {
    unsafe {
        let __skreturn = __sklib__read_line();
        __skadapter__to_string(__skreturn)
    }
}
pub fn terminal_has_input() -> bool {
    unsafe {
        let __skreturn = __sklib__terminal_has_input();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn write_char(data: char) {
    let __skparam__data = __skadapter__to_sklib_char(data);
    unsafe {
        __sklib__write__char(__skparam__data);
    }
}
pub fn write_double(data: f64) {
    let __skparam__data = __skadapter__to_sklib_double(data);
    unsafe {
        __sklib__write__double(__skparam__data);
    }
}
pub fn write_int(data: i32) {
    let __skparam__data = __skadapter__to_sklib_int(data);
    unsafe {
        __sklib__write__int(__skparam__data);
    }
}
pub fn write(text: String) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    unsafe {
        __sklib__write__string(__skparam__text);
    }
}
pub fn write_line_char(data: char) {
    let __skparam__data = __skadapter__to_sklib_char(data);
    unsafe {
        __sklib__write_line__char(__skparam__data);
    }
}
pub fn write_line_empty() {
    unsafe {
        __sklib__write_line();
    }
}
pub fn write_line_double(data: f64) {
    let __skparam__data = __skadapter__to_sklib_double(data);
    unsafe {
        __sklib__write_line__double(__skparam__data);
    }
}
pub fn write_line_int(data: i32) {
    let __skparam__data = __skadapter__to_sklib_int(data);
    unsafe {
        __sklib__write_line__int(__skparam__data);
    }
}
pub fn write_line(line: String) {
    let __skparam__line = __skadapter__to_sklib_string(line);
    unsafe {
        __sklib__write_line__string(__skparam__line);
    }
}
pub fn draw_text_font_as_string(text: String, clr: Color, fnt: String, font_size: i32, x: f64, y: f64) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text__string_ref__color_ref__string_ref__int__double__double(__skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_with_options_font_as_string(text: String, clr: Color, fnt: String, font_size: i32, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref(__skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_text_no_font_no_size(text: String, clr: Color, x: f64, y: f64) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text__string_ref__color_ref__double__double(__skparam__text, __skparam__clr, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_no_font_no_size_with_options(text: String, clr: Color, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text__string_ref__color_ref__double__double__drawing_options_ref(__skparam__text, __skparam__clr, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_text(text: String, clr: Color, fnt: Font, font_size: i32, x: f64, y: f64) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text__string_ref__color_ref__font__int__double__double(__skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_with_options(text: String, clr: Color, fnt: Font, font_size: i32, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text__string_ref__color_ref__font__int__double__double__drawing_options_ref(__skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_text_on_bitmap_font_as_string(bmp: Bitmap, text: String, clr: Color, fnt: String, font_size: i32, x: f64, y: f64) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__string_ref__int__double__double(__skparam__bmp, __skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_on_bitmap_with_options_font_as_string(bmp: Bitmap, text: String, clr: Color, fnt: String, font_size: i32, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref(__skparam__bmp, __skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_text_on_bitmap_no_font_no_size(bmp: Bitmap, text: String, clr: Color, x: f64, y: f64) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__double__double(__skparam__bmp, __skparam__text, __skparam__clr, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_on_bitmap_no_font_no_size_with_options(bmp: Bitmap, text: String, clr: Color, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__double__double__drawing_options_ref(__skparam__bmp, __skparam__text, __skparam__clr, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_text_on_bitmap(bmp: Bitmap, text: String, clr: Color, fnt: Font, font_size: i32, x: f64, y: f64) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__font__int__double__double(__skparam__bmp, __skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_on_bitmap_with_options(bmp: Bitmap, text: String, clr: Color, fnt: Font, font_size: i32, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text_on_bitmap__bitmap__string_ref__color_ref__font__int__double__double__drawing_options_ref(__skparam__bmp, __skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_text_on_window_font_as_string(wnd: Window, text: String, clr: Color, fnt: String, font_size: i32, x: f64, y: f64) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text_on_window__window__string_ref__color_ref__string_ref__int__double__double(__skparam__wnd, __skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_on_window_with_options_font_as_string(wnd: Window, text: String, clr: Color, fnt: String, font_size: i32, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text_on_window__window__string_ref__color_ref__string_ref__int__double__double__drawing_options_ref(__skparam__wnd, __skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_text_on_window_no_font_no_size(wnd: Window, text: String, clr: Color, x: f64, y: f64) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text_on_window__window__string_ref__color_ref__double__double(__skparam__wnd, __skparam__text, __skparam__clr, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_on_window_no_font_no_size_with_options(wnd: Window, text: String, clr: Color, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text_on_window__window__string_ref__color_ref__double__double__drawing_options_ref(__skparam__wnd, __skparam__text, __skparam__clr, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn draw_text_on_window(wnd: Window, text: String, clr: Color, fnt: Font, font_size: i32, x: f64, y: f64) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        __sklib__draw_text_on_window__window__string_ref__color_ref__font__int__double__double(__skparam__wnd, __skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y);
    }
}
pub fn draw_text_on_window_with_options(wnd: Window, text: String, clr: Color, fnt: Font, font_size: i32, x: f64, y: f64, opts: DrawingOptions) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_text_on_window__window__string_ref__color_ref__font__int__double__double__drawing_options_ref(__skparam__wnd, __skparam__text, __skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__x, __skparam__y, __skparam__opts);
    }
}
pub fn font_has_size_name_as_string(name: String, font_size: i32) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        let __skreturn = __sklib__font_has_size__string_ref__int(__skparam__name, __skparam__font_size);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn font_has_size(fnt: Font, font_size: i32) -> bool {
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        let __skreturn = __sklib__font_has_size__font__int(__skparam__fnt, __skparam__font_size);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn font_load_size_name_as_string(name: String, font_size: i32) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        __sklib__font_load_size__string_ref__int(__skparam__name, __skparam__font_size);
    }
}
pub fn font_load_size(fnt: Font, font_size: i32) {
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        __sklib__font_load_size__font__int(__skparam__fnt, __skparam__font_size);
    }
}
pub fn font_named(name: String) -> Font {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__font_named__string(__skparam__name);
        __skadapter__to_font(__skreturn)
    }
}
pub fn free_all_fonts() {
    unsafe {
        __sklib__free_all_fonts();
    }
}
pub fn free_font(fnt: Font) {
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    unsafe {
        __sklib__free_font__font(__skparam__fnt);
    }
}
pub fn get_font_style_name_as_string(name: String) -> FontStyle {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__get_font_style__string_ref(__skparam__name);
        __skadapter__to_font_style(__skreturn)
    }
}
pub fn get_font_style(fnt: Font) -> FontStyle {
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    unsafe {
        let __skreturn = __sklib__get_font_style__font(__skparam__fnt);
        __skadapter__to_font_style(__skreturn)
    }
}
pub fn get_system_font() -> Font {
    unsafe {
        let __skreturn = __sklib__get_system_font();
        __skadapter__to_font(__skreturn)
    }
}
pub fn has_font(fnt: Font) -> bool {
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    unsafe {
        let __skreturn = __sklib__has_font__font(__skparam__fnt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn has_font_name_as_string(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_font__string(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn load_font(name: String, filename: String) -> Font {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        let __skreturn = __sklib__load_font__string_ref__string_ref(__skparam__name, __skparam__filename);
        __skadapter__to_font(__skreturn)
    }
}
pub fn set_font_style_name_as_string(name: String, style: FontStyle) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__style = __skadapter__to_sklib_font_style(style);
    unsafe {
        __sklib__set_font_style__string_ref__font_style(__skparam__name, __skparam__style);
    }
}
pub fn set_font_style(fnt: Font, style: FontStyle) {
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__style = __skadapter__to_sklib_font_style(style);
    unsafe {
        __sklib__set_font_style__font__font_style(__skparam__fnt, __skparam__style);
    }
}
pub fn text_height_font_named(text: String, fnt: String, font_size: i32) -> i32 {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        let __skreturn = __sklib__text_height__string_ref__string_ref__int(__skparam__text, __skparam__fnt, __skparam__font_size);
        __skadapter__to_int(__skreturn)
    }
}
pub fn text_height(text: String, fnt: Font, font_size: i32) -> i32 {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        let __skreturn = __sklib__text_height__string_ref__font__int(__skparam__text, __skparam__fnt, __skparam__font_size);
        __skadapter__to_int(__skreturn)
    }
}
pub fn text_width_font_named(text: String, fnt: String, font_size: i32) -> i32 {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__fnt = __skadapter__to_sklib_string(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        let __skreturn = __sklib__text_width__string_ref__string_ref__int(__skparam__text, __skparam__fnt, __skparam__font_size);
        __skadapter__to_int(__skreturn)
    }
}
pub fn text_width(text: String, fnt: Font, font_size: i32) -> i32 {
    let __skparam__text = __skadapter__to_sklib_string(text);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        let __skreturn = __sklib__text_width__string_ref__font__int(__skparam__text, __skparam__fnt, __skparam__font_size);
        __skadapter__to_int(__skreturn)
    }
}
pub fn draw_collected_text(clr: Color, fnt: Font, font_size: i32, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__fnt = __skadapter__to_sklib_font(fnt);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_collected_text__color__font__int__drawing_options_ref(__skparam__clr, __skparam__fnt, __skparam__font_size, __skparam__opts);
    }
}
pub fn end_reading_text() {
    unsafe {
        __sklib__end_reading_text();
    }
}
pub fn end_reading_text_in_window(wind: Window) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        __sklib__end_reading_text__window(__skparam__wind);
    }
}
pub fn reading_text() -> bool {
    unsafe {
        let __skreturn = __sklib__reading_text();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn reading_text_in_window(wind: Window) -> bool {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__reading_text__window(__skparam__wind);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn start_reading_text(rect: Rectangle) {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__start_reading_text__rectangle(__skparam__rect);
    }
}
pub fn start_reading_text_with_initial_text(rect: Rectangle, initial_text: String) {
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__initial_text = __skadapter__to_sklib_string(initial_text);
    unsafe {
        __sklib__start_reading_text__rectangle__string(__skparam__rect, __skparam__initial_text);
    }
}
pub fn start_reading_text_in_window(wind: Window, rect: Rectangle) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        __sklib__start_reading_text__window__rectangle(__skparam__wind, __skparam__rect);
    }
}
pub fn start_reading_text_in_window_with_initial_text(wind: Window, rect: Rectangle, initial_text: String) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__initial_text = __skadapter__to_sklib_string(initial_text);
    unsafe {
        __sklib__start_reading_text__window__rectangle__string(__skparam__wind, __skparam__rect, __skparam__initial_text);
    }
}
pub fn text_entry_cancelled() -> bool {
    unsafe {
        let __skreturn = __sklib__text_entry_cancelled();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn text_entry_cancelled_in_window(wind: Window) -> bool {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__text_entry_cancelled__window(__skparam__wind);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn text_input() -> String {
    unsafe {
        let __skreturn = __sklib__text_input();
        __skadapter__to_string(__skreturn)
    }
}
pub fn text_input_in_window(wind: Window) -> String {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__text_input__window(__skparam__wind);
        __skadapter__to_string(__skreturn)
    }
}
pub fn create_timer(name: String) -> Timer {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__create_timer__string(__skparam__name);
        __skadapter__to_timer(__skreturn)
    }
}
pub fn free_all_timers() {
    unsafe {
        __sklib__free_all_timers();
    }
}
pub fn free_timer(to_free: Timer) {
    let __skparam__to_free = __skadapter__to_sklib_timer(to_free);
    unsafe {
        __sklib__free_timer__timer(__skparam__to_free);
    }
}
pub fn has_timer__named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__has_timer__string(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn pause_timer__named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__pause_timer__string(__skparam__name);
    }
}
pub fn pause_timer(to_pause: Timer) {
    let __skparam__to_pause = __skadapter__to_sklib_timer(to_pause);
    unsafe {
        __sklib__pause_timer__timer(__skparam__to_pause);
    }
}
pub fn reset_timer__named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__reset_timer__string(__skparam__name);
    }
}
pub fn reset_timer(tmr: Timer) {
    let __skparam__tmr = __skadapter__to_sklib_timer(tmr);
    unsafe {
        __sklib__reset_timer__timer(__skparam__tmr);
    }
}
pub fn resume_timer__named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__resume_timer__string(__skparam__name);
    }
}
pub fn resume_timer(to_resume: Timer) {
    let __skparam__to_resume = __skadapter__to_sklib_timer(to_resume);
    unsafe {
        __sklib__resume_timer__timer(__skparam__to_resume);
    }
}
pub fn start_timer__named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__start_timer__string(__skparam__name);
    }
}
pub fn start_timer(to_start: Timer) {
    let __skparam__to_start = __skadapter__to_sklib_timer(to_start);
    unsafe {
        __sklib__start_timer__timer(__skparam__to_start);
    }
}
pub fn stop_timer__named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__stop_timer__string(__skparam__name);
    }
}
pub fn stop_timer(to_stop: Timer) {
    let __skparam__to_stop = __skadapter__to_sklib_timer(to_stop);
    unsafe {
        __sklib__stop_timer__timer(__skparam__to_stop);
    }
}
pub fn timer_named(name: String) -> Timer {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__timer_named__string(__skparam__name);
        __skadapter__to_timer(__skreturn)
    }
}
pub fn timer_paused__named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__timer_paused__string(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn timer_paused(to_get: Timer) -> bool {
    let __skparam__to_get = __skadapter__to_sklib_timer(to_get);
    unsafe {
        let __skreturn = __sklib__timer_paused__timer(__skparam__to_get);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn timer_started__named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__timer_started__string(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn timer_started(to_get: Timer) -> bool {
    let __skparam__to_get = __skadapter__to_sklib_timer(to_get);
    unsafe {
        let __skreturn = __sklib__timer_started__timer(__skparam__to_get);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn timer_ticks__named(name: String) -> u32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__timer_ticks__string(__skparam__name);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn timer_ticks(to_get: Timer) -> u32 {
    let __skparam__to_get = __skadapter__to_sklib_timer(to_get);
    unsafe {
        let __skreturn = __sklib__timer_ticks__timer(__skparam__to_get);
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn draw_triangle_record(clr: Color, tri: Triangle) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        __sklib__draw_triangle__color__triangle_ref(__skparam__clr, __skparam__tri);
    }
}
pub fn draw_triangle_record_with_options(clr: Color, tri: Triangle, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_triangle__color__triangle_ref__drawing_options(__skparam__clr, __skparam__tri, __skparam__opts);
    }
}
pub fn draw_triangle(clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    unsafe {
        __sklib__draw_triangle__color__double__double__double__double__double__double(__skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3);
    }
}
pub fn draw_triangle_with_options(clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_triangle__color__double__double__double__double__double__double__drawing_options(__skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3, __skparam__opts);
    }
}
pub fn draw_triangle_on_bitmap_record(destination: Bitmap, clr: Color, tri: Triangle) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        __sklib__draw_triangle_on_bitmap__bitmap__color__triangle_ref(__skparam__destination, __skparam__clr, __skparam__tri);
    }
}
pub fn draw_triangle_on_bitmap_record_with_options(destination: Bitmap, clr: Color, tri: Triangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_triangle_on_bitmap__bitmap__color__triangle_ref__drawing_options(__skparam__destination, __skparam__clr, __skparam__tri, __skparam__opts);
    }
}
pub fn draw_triangle_on_bitmap(destination: Bitmap, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    unsafe {
        __sklib__draw_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3);
    }
}
pub fn draw_triangle_on_bitmap_with_options(destination: Bitmap, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3, __skparam__opts);
    }
}
pub fn draw_triangle_on_window_record(destination: Window, clr: Color, tri: Triangle) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        __sklib__draw_triangle_on_window__window__color__triangle_ref(__skparam__destination, __skparam__clr, __skparam__tri);
    }
}
pub fn draw_triangle_on_window_record_with_options(destination: Window, clr: Color, tri: Triangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_triangle_on_window__window__color__triangle_ref__drawing_options(__skparam__destination, __skparam__clr, __skparam__tri, __skparam__opts);
    }
}
pub fn draw_triangle_on_window(destination: Window, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    unsafe {
        __sklib__draw_triangle_on_window__window__color__double__double__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3);
    }
}
pub fn draw_triangle_on_window_with_options(destination: Window, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__draw_triangle_on_window__window__color__double__double__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3, __skparam__opts);
    }
}
pub fn fill_triangle_record(clr: Color, tri: Triangle) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        __sklib__fill_triangle__color__triangle_ref(__skparam__clr, __skparam__tri);
    }
}
pub fn fill_triangle_record_with_options(clr: Color, tri: Triangle, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_triangle__color__triangle_ref__drawing_options(__skparam__clr, __skparam__tri, __skparam__opts);
    }
}
pub fn fill_triangle(clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    unsafe {
        __sklib__fill_triangle__color__double__double__double__double__double__double(__skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3);
    }
}
pub fn fill_triangle_with_options(clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: DrawingOptions) {
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_triangle__color__double__double__double__double__double__double__drawing_options(__skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3, __skparam__opts);
    }
}
pub fn fill_triangle_on_bitmap_record(destination: Bitmap, clr: Color, tri: Triangle) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        __sklib__fill_triangle_on_bitmap__bitmap__color__triangle_ref(__skparam__destination, __skparam__clr, __skparam__tri);
    }
}
pub fn fill_triangle_on_bitmap_record_with_options(destination: Bitmap, clr: Color, tri: Triangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_triangle_on_bitmap__bitmap__color__triangle_ref__drawing_options(__skparam__destination, __skparam__clr, __skparam__tri, __skparam__opts);
    }
}
pub fn fill_triangle_on_bitmap(destination: Bitmap, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    unsafe {
        __sklib__fill_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3);
    }
}
pub fn fill_triangle_on_bitmap_with_options(destination: Bitmap, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_bitmap(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_triangle_on_bitmap__bitmap__color__double__double__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3, __skparam__opts);
    }
}
pub fn fill_triangle_on_window_record(destination: Window, clr: Color, tri: Triangle) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        __sklib__fill_triangle_on_window__window__color__triangle_ref(__skparam__destination, __skparam__clr, __skparam__tri);
    }
}
pub fn fill_triangle_on_window_record_with_options(destination: Window, clr: Color, tri: Triangle, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_triangle_on_window__window__color__triangle_ref__drawing_options(__skparam__destination, __skparam__clr, __skparam__tri, __skparam__opts);
    }
}
pub fn fill_triangle_on_window(destination: Window, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    unsafe {
        __sklib__fill_triangle_on_window__window__color__double__double__double__double__double__double(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3);
    }
}
pub fn fill_triangle_on_window_with_options(destination: Window, clr: Color, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, opts: DrawingOptions) {
    let __skparam__destination = __skadapter__to_sklib_window(destination);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    let __skparam__opts = __skadapter__to_sklib_drawing_options(opts);
    unsafe {
        __sklib__fill_triangle_on_window__window__color__double__double__double__double__double__double__drawing_options(__skparam__destination, __skparam__clr, __skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3, __skparam__opts);
    }
}
pub fn triangle_barycenter(tri: Triangle) -> Point2D {
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        let __skreturn = __sklib__triangle_barycenter__triangle_ref(__skparam__tri);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn triangle_from(p1: Point2D, p2: Point2D, p3: Point2D) -> Triangle {
    let __skparam__p1 = __skadapter__to_sklib_point_2d(p1);
    let __skparam__p2 = __skadapter__to_sklib_point_2d(p2);
    let __skparam__p3 = __skadapter__to_sklib_point_2d(p3);
    unsafe {
        let __skreturn = __sklib__triangle_from__point_2d_ref__point_2d_ref__point_2d_ref(__skparam__p1, __skparam__p2, __skparam__p3);
        __skadapter__to_triangle(__skreturn)
    }
}
pub fn triangle_from__from_coordinates(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Triangle {
    let __skparam__x1 = __skadapter__to_sklib_double(x1);
    let __skparam__y1 = __skadapter__to_sklib_double(y1);
    let __skparam__x2 = __skadapter__to_sklib_double(x2);
    let __skparam__y2 = __skadapter__to_sklib_double(y2);
    let __skparam__x3 = __skadapter__to_sklib_double(x3);
    let __skparam__y3 = __skadapter__to_sklib_double(y3);
    unsafe {
        let __skreturn = __sklib__triangle_from__double__double__double__double__double__double(__skparam__x1, __skparam__y1, __skparam__x2, __skparam__y2, __skparam__x3, __skparam__y3);
        __skadapter__to_triangle(__skreturn)
    }
}
pub fn triangle_rectangle_intersect(tri: Triangle, rect: Rectangle) -> bool {
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__triangle_rectangle_intersect__triangle_ref__rectangle_ref(__skparam__tri, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn triangle_to_string(tri: Triangle) -> String {
    let __skparam__tri = __skadapter__to_sklib_triangle(tri);
    unsafe {
        let __skreturn = __sklib__triangle_to_string__triangle_ref(__skparam__tri);
        __skadapter__to_string(__skreturn)
    }
}
pub fn triangles_intersect(t1: Triangle, t2: Triangle) -> bool {
    let __skparam__t1 = __skadapter__to_sklib_triangle(t1);
    let __skparam__t2 = __skadapter__to_sklib_triangle(t2);
    unsafe {
        let __skreturn = __sklib__triangles_intersect__triangle_ref__triangle_ref(__skparam__t1, __skparam__t2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn current_ticks() -> u32 {
    unsafe {
        let __skreturn = __sklib__current_ticks();
        __skadapter__to_unsigned_int(__skreturn)
    }
}
pub fn delay(milliseconds: i32) {
    let __skparam__milliseconds = __skadapter__to_sklib_int(milliseconds);
    unsafe {
        __sklib__delay__int(__skparam__milliseconds);
    }
}
pub fn display_dialog(title: String, msg: String, output_font: Font, font_size: i32) {
    let __skparam__title = __skadapter__to_sklib_string(title);
    let __skparam__msg = __skadapter__to_sklib_string(msg);
    let __skparam__output_font = __skadapter__to_sklib_font(output_font);
    let __skparam__font_size = __skadapter__to_sklib_int(font_size);
    unsafe {
        __sklib__display_dialog__string_ref__string_ref__font__int(__skparam__title, __skparam__msg, __skparam__output_font, __skparam__font_size);
    }
}
pub fn file_as_string(filename: String, kind: ResourceKind) -> String {
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    let __skparam__kind = __skadapter__to_sklib_resource_kind(kind);
    unsafe {
        let __skreturn = __sklib__file_as_string__string__resource_kind(__skparam__filename, __skparam__kind);
        __skadapter__to_string(__skreturn)
    }
}
pub fn angle_between(v1: Vector2D, v2: Vector2D) -> f64 {
    let __skparam__v1 = __skadapter__to_sklib_vector_2d(v1);
    let __skparam__v2 = __skadapter__to_sklib_vector_2d(v2);
    unsafe {
        let __skreturn = __sklib__angle_between__vector_2d_ref__vector_2d_ref(__skparam__v1, __skparam__v2);
        __skadapter__to_double(__skreturn)
    }
}
pub fn dot_product(v1: Vector2D, v2: Vector2D) -> f64 {
    let __skparam__v1 = __skadapter__to_sklib_vector_2d(v1);
    let __skparam__v2 = __skadapter__to_sklib_vector_2d(v2);
    unsafe {
        let __skreturn = __sklib__dot_product__vector_2d_ref__vector_2d_ref(__skparam__v1, __skparam__v2);
        __skadapter__to_double(__skreturn)
    }
}
pub fn is_zero_vector(v: Vector2D) -> bool {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__is_zero_vector__vector_2d_ref(__skparam__v);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn ray_intersection_point(from_pt: Point2D, heading: Vector2D, l: Line, pt: &mut Point2D) -> bool {
    let __skparam__from_pt = __skadapter__to_sklib_point_2d(from_pt);
    let __skparam__heading = __skadapter__to_sklib_vector_2d(heading);
    let __skparam__l = __skadapter__to_sklib_line(l);
    let mut __skparam__pt = __skadapter__to_sklib_point_2d((*pt).clone());
    unsafe {
        let __skreturn = __sklib__ray_intersection_point__point_2d_ref__vector_2d_ref__line_ref__point_2d_ref(__skparam__from_pt, __skparam__heading, __skparam__l, &mut __skparam__pt);
        *pt = __skadapter__to_point_2d(__skparam__pt);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn unit_vector(v: Vector2D) -> Vector2D {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__unit_vector__vector_2d_ref(__skparam__v);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_add(v1: Vector2D, v2: Vector2D) -> Vector2D {
    let __skparam__v1 = __skadapter__to_sklib_vector_2d(v1);
    let __skparam__v2 = __skadapter__to_sklib_vector_2d(v2);
    unsafe {
        let __skreturn = __sklib__vector_add__vector_2d_ref__vector_2d_ref(__skparam__v1, __skparam__v2);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_angle(v: Vector2D) -> f64 {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__vector_angle__vector_2d(__skparam__v);
        __skadapter__to_double(__skreturn)
    }
}
pub fn vector_from_angle(angle: f64, magnitude: f64) -> Vector2D {
    let __skparam__angle = __skadapter__to_sklib_double(angle);
    let __skparam__magnitude = __skadapter__to_sklib_double(magnitude);
    unsafe {
        let __skreturn = __sklib__vector_from_angle__double__double(__skparam__angle, __skparam__magnitude);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_from_line(l: Line) -> Vector2D {
    let __skparam__l = __skadapter__to_sklib_line(l);
    unsafe {
        let __skreturn = __sklib__vector_from_line__line_ref(__skparam__l);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_from_point_to_rect(pt: Point2D, rect: Rectangle) -> Vector2D {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__vector_from_point_to_rect__point_2d_ref__rectangle_ref(__skparam__pt, __skparam__rect);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_in_rect(v: Vector2D, rect: Rectangle) -> bool {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    unsafe {
        let __skreturn = __sklib__vector_in_rect__vector_2d_ref__rectangle_ref(__skparam__v, __skparam__rect);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn vector_invert(v: Vector2D) -> Vector2D {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__vector_invert__vector_2d_ref(__skparam__v);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_limit(v: Vector2D, limit: f64) -> Vector2D {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    let __skparam__limit = __skadapter__to_sklib_double(limit);
    unsafe {
        let __skreturn = __sklib__vector_limit__vector_2d_ref__double(__skparam__v, __skparam__limit);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_magnitude(v: Vector2D) -> f64 {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__vector_magnitude__vector_2d_ref(__skparam__v);
        __skadapter__to_double(__skreturn)
    }
}
pub fn vector_magnitude_squared(v: Vector2D) -> f64 {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__vector_magnitude_squared__vector_2d_ref(__skparam__v);
        __skadapter__to_double(__skreturn)
    }
}
pub fn vector_multiply(v1: Vector2D, s: f64) -> Vector2D {
    let __skparam__v1 = __skadapter__to_sklib_vector_2d(v1);
    let __skparam__s = __skadapter__to_sklib_double(s);
    unsafe {
        let __skreturn = __sklib__vector_multiply__vector_2d_ref__double(__skparam__v1, __skparam__s);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_normal(v: Vector2D) -> Vector2D {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__vector_normal__vector_2d_ref(__skparam__v);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_out_of_circle_from_circle(src: Circle, bounds: Circle, velocity: Vector2D) -> Vector2D {
    let __skparam__src = __skadapter__to_sklib_circle(src);
    let __skparam__bounds = __skadapter__to_sklib_circle(bounds);
    let __skparam__velocity = __skadapter__to_sklib_vector_2d(velocity);
    unsafe {
        let __skreturn = __sklib__vector_out_of_circle_from_circle__circle_ref__circle_ref__vector_2d_ref(__skparam__src, __skparam__bounds, __skparam__velocity);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_out_of_circle_from_point(pt: Point2D, c: Circle, velocity: Vector2D) -> Vector2D {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__velocity = __skadapter__to_sklib_vector_2d(velocity);
    unsafe {
        let __skreturn = __sklib__vector_out_of_circle_from_point__point_2d_ref__circle_ref__vector_2d_ref(__skparam__pt, __skparam__c, __skparam__velocity);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_out_of_rect_from_circle(c: Circle, rect: Rectangle, velocity: Vector2D) -> Vector2D {
    let __skparam__c = __skadapter__to_sklib_circle(c);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__velocity = __skadapter__to_sklib_vector_2d(velocity);
    unsafe {
        let __skreturn = __sklib__vector_out_of_rect_from_circle__circle_ref__rectangle_ref__vector_2d_ref(__skparam__c, __skparam__rect, __skparam__velocity);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_out_of_rect_from_point(pt: Point2D, rect: Rectangle, velocity: Vector2D) -> Vector2D {
    let __skparam__pt = __skadapter__to_sklib_point_2d(pt);
    let __skparam__rect = __skadapter__to_sklib_rectangle(rect);
    let __skparam__velocity = __skadapter__to_sklib_vector_2d(velocity);
    unsafe {
        let __skreturn = __sklib__vector_out_of_rect_from_point__point_2d_ref__rectangle_ref__vector_2d_ref(__skparam__pt, __skparam__rect, __skparam__velocity);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_out_of_rect_from_rect(src: Rectangle, bounds: Rectangle, velocity: Vector2D) -> Vector2D {
    let __skparam__src = __skadapter__to_sklib_rectangle(src);
    let __skparam__bounds = __skadapter__to_sklib_rectangle(bounds);
    let __skparam__velocity = __skadapter__to_sklib_vector_2d(velocity);
    unsafe {
        let __skreturn = __sklib__vector_out_of_rect_from_rect__rectangle_ref__rectangle_ref__vector_2d_ref(__skparam__src, __skparam__bounds, __skparam__velocity);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_point_to_point(start: Point2D, end_pt: Point2D) -> Vector2D {
    let __skparam__start = __skadapter__to_sklib_point_2d(start);
    let __skparam__end_pt = __skadapter__to_sklib_point_2d(end_pt);
    unsafe {
        let __skreturn = __sklib__vector_point_to_point__point_2d_ref__point_2d_ref(__skparam__start, __skparam__end_pt);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_subtract(v1: Vector2D, v2: Vector2D) -> Vector2D {
    let __skparam__v1 = __skadapter__to_sklib_vector_2d(v1);
    let __skparam__v2 = __skadapter__to_sklib_vector_2d(v2);
    unsafe {
        let __skreturn = __sklib__vector_subtract__vector_2d_ref__vector_2d_ref(__skparam__v1, __skparam__v2);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_to_point(p1: Point2D) -> Vector2D {
    let __skparam__p1 = __skadapter__to_sklib_point_2d(p1);
    unsafe {
        let __skreturn = __sklib__vector_to__point_2d_ref(__skparam__p1);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_to(x: f64, y: f64) -> Vector2D {
    let __skparam__x = __skadapter__to_sklib_double(x);
    let __skparam__y = __skadapter__to_sklib_double(y);
    unsafe {
        let __skreturn = __sklib__vector_to__double__double(__skparam__x, __skparam__y);
        __skadapter__to_vector_2d(__skreturn)
    }
}
pub fn vector_to_string(v: Vector2D) -> String {
    let __skparam__v = __skadapter__to_sklib_vector_2d(v);
    unsafe {
        let __skreturn = __sklib__vector_to_string__vector_2d_ref(__skparam__v);
        __skadapter__to_string(__skreturn)
    }
}
pub fn vectors_equal(v1: Vector2D, v2: Vector2D) -> bool {
    let __skparam__v1 = __skadapter__to_sklib_vector_2d(v1);
    let __skparam__v2 = __skadapter__to_sklib_vector_2d(v2);
    unsafe {
        let __skreturn = __sklib__vectors_equal__vector_2d_ref__vector_2d(__skparam__v1, __skparam__v2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn vectors_not_equal(v1: Vector2D, v2: Vector2D) -> bool {
    let __skparam__v1 = __skadapter__to_sklib_vector_2d(v1);
    let __skparam__v2 = __skadapter__to_sklib_vector_2d(v2);
    unsafe {
        let __skreturn = __sklib__vectors_not_equal__vector_2d_ref__vector_2d(__skparam__v1, __skparam__v2);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn download_bitmap(name: String, url: String, port: u16) -> Bitmap {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__url = __skadapter__to_sklib_string(url);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    unsafe {
        let __skreturn = __sklib__download_bitmap__string_ref__string_ref__unsigned_short(__skparam__name, __skparam__url, __skparam__port);
        __skadapter__to_bitmap(__skreturn)
    }
}
pub fn download_font(name: String, url: String, port: u16) -> Font {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__url = __skadapter__to_sklib_string(url);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    unsafe {
        let __skreturn = __sklib__download_font__string_ref__string_ref__unsigned_short(__skparam__name, __skparam__url, __skparam__port);
        __skadapter__to_font(__skreturn)
    }
}
pub fn download_music(name: String, url: String, port: u16) -> Music {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__url = __skadapter__to_sklib_string(url);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    unsafe {
        let __skreturn = __sklib__download_music__string_ref__string_ref__unsigned_short(__skparam__name, __skparam__url, __skparam__port);
        __skadapter__to_music(__skreturn)
    }
}
pub fn download_sound_effect(name: String, url: String, port: u16) -> SoundEffect {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__url = __skadapter__to_sklib_string(url);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    unsafe {
        let __skreturn = __sklib__download_sound_effect__string_ref__string_ref__unsigned_short(__skparam__name, __skparam__url, __skparam__port);
        __skadapter__to_sound_effect(__skreturn)
    }
}
pub fn free_response(response: HttpResponse) {
    let __skparam__response = __skadapter__to_sklib_http_response(response);
    unsafe {
        __sklib__free_response__http_response(__skparam__response);
    }
}
pub fn http_get(url: String, port: u16) -> HttpResponse {
    let __skparam__url = __skadapter__to_sklib_string(url);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    unsafe {
        let __skreturn = __sklib__http_get__string_ref__unsigned_short(__skparam__url, __skparam__port);
        __skadapter__to_http_response(__skreturn)
    }
}
pub fn http_post_with_headers(url: String, port: u16, body: String, headers: Vec<String>) -> HttpResponse {
    let __skparam__url = __skadapter__to_sklib_string(url);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    let __skparam__body = __skadapter__to_sklib_string(body);
    let __skparam__headers = __skadapter__to_sklib_vector_string(headers);
    unsafe {
        let __skreturn = __sklib__http_post__string_ref__unsigned_short__string_ref__vector_string_ref(__skparam__url, __skparam__port, __skparam__body, __skparam__headers);
        __skadapter__to_http_response(__skreturn)
    }
}
pub fn http_post(url: String, port: u16, body: String) -> HttpResponse {
    let __skparam__url = __skadapter__to_sklib_string(url);
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    let __skparam__body = __skadapter__to_sklib_string(body);
    unsafe {
        let __skreturn = __sklib__http_post__string_ref__unsigned_short__string(__skparam__url, __skparam__port, __skparam__body);
        __skadapter__to_http_response(__skreturn)
    }
}
pub fn http_response_to_string(response: HttpResponse) -> String {
    let __skparam__response = __skadapter__to_sklib_http_response(response);
    unsafe {
        let __skreturn = __sklib__http_response_to_string__http_response(__skparam__response);
        __skadapter__to_string(__skreturn)
    }
}
pub fn save_response_to_file(response: HttpResponse, path: String) {
    let __skparam__response = __skadapter__to_sklib_http_response(response);
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        __sklib__save_response_to_file__http_response__string(__skparam__response, __skparam__path);
    }
}
pub fn has_incoming_requests(server: WebServer) -> bool {
    let __skparam__server = __skadapter__to_sklib_web_server(server);
    unsafe {
        let __skreturn = __sklib__has_incoming_requests__web_server(__skparam__server);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_delete_request_for(request: HttpRequest, path: String) -> bool {
    let __skparam__request = __skadapter__to_sklib_http_request(request);
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        let __skreturn = __sklib__is_delete_request_for__http_request__string_ref(__skparam__request, __skparam__path);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_get_request_for(request: HttpRequest, path: String) -> bool {
    let __skparam__request = __skadapter__to_sklib_http_request(request);
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        let __skreturn = __sklib__is_get_request_for__http_request__string_ref(__skparam__request, __skparam__path);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_options_request_for(request: HttpRequest, path: String) -> bool {
    let __skparam__request = __skadapter__to_sklib_http_request(request);
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        let __skreturn = __sklib__is_options_request_for__http_request__string_ref(__skparam__request, __skparam__path);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_post_request_for(request: HttpRequest, path: String) -> bool {
    let __skparam__request = __skadapter__to_sklib_http_request(request);
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        let __skreturn = __sklib__is_post_request_for__http_request__string_ref(__skparam__request, __skparam__path);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_put_request_for(request: HttpRequest, path: String) -> bool {
    let __skparam__request = __skadapter__to_sklib_http_request(request);
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        let __skreturn = __sklib__is_put_request_for__http_request__string_ref(__skparam__request, __skparam__path);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_request_for(request: HttpRequest, method: HttpMethod, path: String) -> bool {
    let __skparam__request = __skadapter__to_sklib_http_request(request);
    let __skparam__method = __skadapter__to_sklib_http_method(method);
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        let __skreturn = __sklib__is_request_for__http_request__http_method__string_ref(__skparam__request, __skparam__method, __skparam__path);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_trace_request_for(request: HttpRequest, path: String) -> bool {
    let __skparam__request = __skadapter__to_sklib_http_request(request);
    let __skparam__path = __skadapter__to_sklib_string(path);
    unsafe {
        let __skreturn = __sklib__is_trace_request_for__http_request__string_ref(__skparam__request, __skparam__path);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn next_web_request(server: WebServer) -> HttpRequest {
    let __skparam__server = __skadapter__to_sklib_web_server(server);
    unsafe {
        let __skreturn = __sklib__next_web_request__web_server(__skparam__server);
        __skadapter__to_http_request(__skreturn)
    }
}
pub fn request_body(r: HttpRequest) -> String {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    unsafe {
        let __skreturn = __sklib__request_body__http_request(__skparam__r);
        __skadapter__to_string(__skreturn)
    }
}
pub fn request_has_query_parameter(r: HttpRequest, name: String) -> bool {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__request_has_query_parameter__http_request__string_ref(__skparam__r, __skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn request_headers(r: HttpRequest) -> Vec<String> {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    unsafe {
        let __skreturn = __sklib__request_headers__http_request(__skparam__r);
        __skadapter__to_vector_string(__skreturn)
    }
}
pub fn request_method(r: HttpRequest) -> HttpMethod {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    unsafe {
        let __skreturn = __sklib__request_method__http_request(__skparam__r);
        __skadapter__to_http_method(__skreturn)
    }
}
pub fn request_query_parameter(r: HttpRequest, name: String, default_value: String) -> String {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__default_value = __skadapter__to_sklib_string(default_value);
    unsafe {
        let __skreturn = __sklib__request_query_parameter__http_request__string_ref__string_ref(__skparam__r, __skparam__name, __skparam__default_value);
        __skadapter__to_string(__skreturn)
    }
}
pub fn request_query_string(r: HttpRequest) -> String {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    unsafe {
        let __skreturn = __sklib__request_query_string__http_request(__skparam__r);
        __skadapter__to_string(__skreturn)
    }
}
pub fn request_uri(r: HttpRequest) -> String {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    unsafe {
        let __skreturn = __sklib__request_uri__http_request(__skparam__r);
        __skadapter__to_string(__skreturn)
    }
}
pub fn request_uri_stubs(r: HttpRequest) -> Vec<String> {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    unsafe {
        let __skreturn = __sklib__request_uri_stubs__http_request(__skparam__r);
        __skadapter__to_vector_string(__skreturn)
    }
}
pub fn send_css_file_response(r: HttpRequest, filename: String) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        __sklib__send_css_file_response__http_request__string_ref(__skparam__r, __skparam__filename);
    }
}
pub fn send_file_response(r: HttpRequest, filename: String, content_type: String) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    let __skparam__content_type = __skadapter__to_sklib_string(content_type);
    unsafe {
        __sklib__send_file_response__http_request__string_ref__string_ref(__skparam__r, __skparam__filename, __skparam__content_type);
    }
}
pub fn send_html_file_response(r: HttpRequest, filename: String) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        __sklib__send_html_file_response__http_request__string_ref(__skparam__r, __skparam__filename);
    }
}
pub fn send_javascript_file_response(r: HttpRequest, filename: String) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__filename = __skadapter__to_sklib_string(filename);
    unsafe {
        __sklib__send_javascript_file_response__http_request__string_ref(__skparam__r, __skparam__filename);
    }
}
pub fn send_response_empty(r: HttpRequest) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    unsafe {
        __sklib__send_response__http_request(__skparam__r);
    }
}
pub fn send_response(r: HttpRequest, message: String) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__message = __skadapter__to_sklib_string(message);
    unsafe {
        __sklib__send_response__http_request__string_ref(__skparam__r, __skparam__message);
    }
}
pub fn send_response_json_with_status(r: HttpRequest, code: HttpStatusCode) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__code = __skadapter__to_sklib_http_status_code(code);
    unsafe {
        __sklib__send_response__http_request__http_status_code(__skparam__r, __skparam__code);
    }
}
pub fn send_response_with_status(r: HttpRequest, code: HttpStatusCode, message: String) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__code = __skadapter__to_sklib_http_status_code(code);
    let __skparam__message = __skadapter__to_sklib_string(message);
    unsafe {
        __sklib__send_response__http_request__http_status_code__string_ref(__skparam__r, __skparam__code, __skparam__message);
    }
}
pub fn send_response_with_status_and_content_type(r: HttpRequest, code: HttpStatusCode, message: String, content_type: String) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__code = __skadapter__to_sklib_http_status_code(code);
    let __skparam__message = __skadapter__to_sklib_string(message);
    let __skparam__content_type = __skadapter__to_sklib_string(content_type);
    unsafe {
        __sklib__send_response__http_request__http_status_code__string_ref__string_ref(__skparam__r, __skparam__code, __skparam__message, __skparam__content_type);
    }
}
pub fn send_response_with_status_and_content_type_and_headers(r: HttpRequest, code: HttpStatusCode, message: String, content_type: String, headers: Vec<String>) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__code = __skadapter__to_sklib_http_status_code(code);
    let __skparam__message = __skadapter__to_sklib_string(message);
    let __skparam__content_type = __skadapter__to_sklib_string(content_type);
    let __skparam__headers = __skadapter__to_sklib_vector_string(headers);
    unsafe {
        __sklib__send_response__http_request__http_status_code__string_ref__string_ref__vector_string_ref(__skparam__r, __skparam__code, __skparam__message, __skparam__content_type, __skparam__headers);
    }
}
pub fn send_response_json(r: HttpRequest, j: Json) {
    let __skparam__r = __skadapter__to_sklib_http_request(r);
    let __skparam__j = __skadapter__to_sklib_json(j);
    unsafe {
        __sklib__send_response__http_request__json(__skparam__r, __skparam__j);
    }
}
pub fn split_uri_stubs(uri: String) -> Vec<String> {
    let __skparam__uri = __skadapter__to_sklib_string(uri);
    unsafe {
        let __skreturn = __sklib__split_uri_stubs__string_ref(__skparam__uri);
        __skadapter__to_vector_string(__skreturn)
    }
}
pub fn start_web_server_with_default_port() -> WebServer {
    unsafe {
        let __skreturn = __sklib__start_web_server();
        __skadapter__to_web_server(__skreturn)
    }
}
pub fn start_web_server(port: u16) -> WebServer {
    let __skparam__port = __skadapter__to_sklib_unsigned_short(port);
    unsafe {
        let __skreturn = __sklib__start_web_server__unsigned_short(__skparam__port);
        __skadapter__to_web_server(__skreturn)
    }
}
pub fn stop_web_server(server: WebServer) {
    let __skparam__server = __skadapter__to_sklib_web_server(server);
    unsafe {
        __sklib__stop_web_server__web_server(__skparam__server);
    }
}
pub fn clear_window(wind: Window, clr: Color) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__clr = __skadapter__to_sklib_color(clr);
    unsafe {
        __sklib__clear_window__window__color(__skparam__wind, __skparam__clr);
    }
}
pub fn close_all_windows() {
    unsafe {
        __sklib__close_all_windows();
    }
}
pub fn close_current_window() {
    unsafe {
        __sklib__close_current_window();
    }
}
pub fn close_window_named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__close_window__string_ref(__skparam__name);
    }
}
pub fn close_window(wind: Window) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        __sklib__close_window__window(__skparam__wind);
    }
}
pub fn current_window() -> Window {
    unsafe {
        let __skreturn = __sklib__current_window();
        __skadapter__to_window(__skreturn)
    }
}
pub fn current_window_has_border() -> bool {
    unsafe {
        let __skreturn = __sklib__current_window_has_border();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn current_window_height() -> i32 {
    unsafe {
        let __skreturn = __sklib__current_window_height();
        __skadapter__to_int(__skreturn)
    }
}
pub fn current_window_is_fullscreen() -> bool {
    unsafe {
        let __skreturn = __sklib__current_window_is_fullscreen();
        __skadapter__to_bool(__skreturn)
    }
}
pub fn current_window_position() -> Point2D {
    unsafe {
        let __skreturn = __sklib__current_window_position();
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn current_window_toggle_border() {
    unsafe {
        __sklib__current_window_toggle_border();
    }
}
pub fn current_window_toggle_fullscreen() {
    unsafe {
        __sklib__current_window_toggle_fullscreen();
    }
}
pub fn current_window_width() -> i32 {
    unsafe {
        let __skreturn = __sklib__current_window_width();
        __skadapter__to_int(__skreturn)
    }
}
pub fn current_window_x() -> i32 {
    unsafe {
        let __skreturn = __sklib__current_window_x();
        __skadapter__to_int(__skreturn)
    }
}
pub fn current_window_y() -> i32 {
    unsafe {
        let __skreturn = __sklib__current_window_y();
        __skadapter__to_int(__skreturn)
    }
}
pub fn has_window(caption: String) -> bool {
    let __skparam__caption = __skadapter__to_sklib_string(caption);
    unsafe {
        let __skreturn = __sklib__has_window__string(__skparam__caption);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn is_current_window(wind: Window) -> bool {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__is_current_window__window(__skparam__wind);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn move_current_window_to(x: i32, y: i32) {
    let __skparam__x = __skadapter__to_sklib_int(x);
    let __skparam__y = __skadapter__to_sklib_int(y);
    unsafe {
        __sklib__move_current_window_to__int__int(__skparam__x, __skparam__y);
    }
}
pub fn move_window_to_named(name: String, x: i32, y: i32) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    let __skparam__x = __skadapter__to_sklib_int(x);
    let __skparam__y = __skadapter__to_sklib_int(y);
    unsafe {
        __sklib__move_window_to__string_ref__int__int(__skparam__name, __skparam__x, __skparam__y);
    }
}
pub fn move_window_to(wind: Window, x: i32, y: i32) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__x = __skadapter__to_sklib_int(x);
    let __skparam__y = __skadapter__to_sklib_int(y);
    unsafe {
        __sklib__move_window_to__window__int__int(__skparam__wind, __skparam__x, __skparam__y);
    }
}
pub fn open_window(caption: String, width: i32, height: i32) -> Window {
    let __skparam__caption = __skadapter__to_sklib_string(caption);
    let __skparam__width = __skadapter__to_sklib_int(width);
    let __skparam__height = __skadapter__to_sklib_int(height);
    unsafe {
        let __skreturn = __sklib__open_window__string__int__int(__skparam__caption, __skparam__width, __skparam__height);
        __skadapter__to_window(__skreturn)
    }
}
pub fn refresh_window(wind: Window) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        __sklib__refresh_window__window(__skparam__wind);
    }
}
pub fn refresh_window_with_target_fps(wind: Window, target_fps: u32) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__target_fps = __skadapter__to_sklib_unsigned_int(target_fps);
    unsafe {
        __sklib__refresh_window__window__unsigned_int(__skparam__wind, __skparam__target_fps);
    }
}
pub fn resize_current_window(width: i32, height: i32) {
    let __skparam__width = __skadapter__to_sklib_int(width);
    let __skparam__height = __skadapter__to_sklib_int(height);
    unsafe {
        __sklib__resize_current_window__int__int(__skparam__width, __skparam__height);
    }
}
pub fn resize_window(wnd: Window, width: i32, height: i32) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    let __skparam__width = __skadapter__to_sklib_int(width);
    let __skparam__height = __skadapter__to_sklib_int(height);
    unsafe {
        __sklib__resize_window__window__int__int(__skparam__wnd, __skparam__width, __skparam__height);
    }
}
pub fn set_current_window_named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__set_current_window__string_ref(__skparam__name);
    }
}
pub fn set_current_window(wind: Window) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        __sklib__set_current_window__window(__skparam__wind);
    }
}
pub fn window_caption(wind: Window) -> String {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__window_caption__window(__skparam__wind);
        __skadapter__to_string(__skreturn)
    }
}
pub fn window_close_requested_named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__window_close_requested__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn window_close_requested(wind: Window) -> bool {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__window_close_requested__window(__skparam__wind);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn window_has_border_named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__window_has_border__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn window_has_border(wnd: Window) -> bool {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        let __skreturn = __sklib__window_has_border__window(__skparam__wnd);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn window_has_focus(wind: Window) -> bool {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__window_has_focus__window(__skparam__wind);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn window_height_named(name: String) -> i32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__window_height__string_ref(__skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn window_height(wind: Window) -> i32 {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__window_height__window(__skparam__wind);
        __skadapter__to_int(__skreturn)
    }
}
pub fn window_is_fullscreen_named(name: String) -> bool {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__window_is_fullscreen__string_ref(__skparam__name);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn window_is_fullscreen(wnd: Window) -> bool {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        let __skreturn = __sklib__window_is_fullscreen__window(__skparam__wnd);
        __skadapter__to_bool(__skreturn)
    }
}
pub fn window_named(caption: String) -> Window {
    let __skparam__caption = __skadapter__to_sklib_string(caption);
    unsafe {
        let __skreturn = __sklib__window_named__string(__skparam__caption);
        __skadapter__to_window(__skreturn)
    }
}
pub fn window_position_named(name: String) -> Point2D {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__window_position__string_ref(__skparam__name);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn window_position(wnd: Window) -> Point2D {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        let __skreturn = __sklib__window_position__window(__skparam__wnd);
        __skadapter__to_point_2d(__skreturn)
    }
}
pub fn window_set_icon(wind: Window, bmp: Bitmap) {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    let __skparam__bmp = __skadapter__to_sklib_bitmap(bmp);
    unsafe {
        __sklib__window_set_icon__window__bitmap(__skparam__wind, __skparam__bmp);
    }
}
pub fn window_toggle_border_named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__window_toggle_border__string_ref(__skparam__name);
    }
}
pub fn window_toggle_border(wnd: Window) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        __sklib__window_toggle_border__window(__skparam__wnd);
    }
}
pub fn window_toggle_fullscreen_named(name: String) {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        __sklib__window_toggle_fullscreen__string_ref(__skparam__name);
    }
}
pub fn window_toggle_fullscreen(wnd: Window) {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        __sklib__window_toggle_fullscreen__window(__skparam__wnd);
    }
}
pub fn window_width_named(name: String) -> i32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__window_width__string_ref(__skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn window_width(wind: Window) -> i32 {
    let __skparam__wind = __skadapter__to_sklib_window(wind);
    unsafe {
        let __skreturn = __sklib__window_width__window(__skparam__wind);
        __skadapter__to_int(__skreturn)
    }
}
pub fn window_with_focus() -> Window {
    unsafe {
        let __skreturn = __sklib__window_with_focus();
        __skadapter__to_window(__skreturn)
    }
}
pub fn window_x_named(name: String) -> i32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__window_x__string_ref(__skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn window_x(wnd: Window) -> i32 {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        let __skreturn = __sklib__window_x__window(__skparam__wnd);
        __skadapter__to_int(__skreturn)
    }
}
pub fn window_y_named(name: String) -> i32 {
    let __skparam__name = __skadapter__to_sklib_string(name);
    unsafe {
        let __skreturn = __sklib__window_y__string_ref(__skparam__name);
        __skadapter__to_int(__skreturn)
    }
}
pub fn window_y(wnd: Window) -> i32 {
    let __skparam__wnd = __skadapter__to_sklib_window(wnd);
    unsafe {
        let __skreturn = __sklib__window_y__window(__skparam__wnd);
        __skadapter__to_int(__skreturn)
    }
}
