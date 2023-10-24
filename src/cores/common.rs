//----------------------------------------------------------------------------------
// mostly rcore.h, but also some types
//----------------------------------------------------------------------------------

/// Maximum capacity for filepath
pub const MAX_FILE_CAPACITY: usize = 8192;
/// Maximum length for filepaths (Linux PATH_MAX default value)
pub const MAX_FILEPATH_LENGTH: usize = 4096;
/// Maximum number of keyboard keys supported
pub const MAX_KEYBOARD_KEYS: usize = 512;
/// Maximum number of mouse buttons supported
pub const MAX_MOUSE_BUTTONS: usize = 8;
/// Maximum number of gamepads supported
pub const MAX_GAMEPADS: usize = 4;
/// Maximum number of axis supported (per gamepad)
pub const MAX_GAMEPAD_AXIS: usize = 8;
/// Maximum number of buttons supported (per gamepad)
pub const MAX_GAMEPAD_BUTTONS: usize = 32;
/// Maximum number of touch points supported
pub const MAX_TOUCH_POINTS: usize = 8;
/// Maximum number of keys in the key input queue
pub const MAX_KEY_PRESSED_QUEUE: usize = 16;
/// Maximum number of characters in the char input queue
pub const MAX_CHAR_PRESSED_QUEUE: usize = 16;
/// Maximum size allocated for decompression in MB
pub const MAX_DECOMPRESSION_SIZE: usize = 64;

// todo
// // Flags operation macros
// #define FLAG_SET(n, f) ((n) |= (f))
// #define FLAG_CLEAR(n, f) ((n) &= ~(f))
// #define FLAG_TOGGLE(n, f) ((n) ^= (f))
// #define FLAG_CHECK(n, f) ((n) & (f))

/// Matrix type (OpenGL style 4x4 - right handed, column major)
pub struct Matrix {
    // first row
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    // second row
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    // third row
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    // fourth row
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}

/// Vector2 type
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct Window {
    /// Window text title const pointer
    pub title: String, // todo &str?
    /// Configuration flags (bit based), keeps window state
    pub flags: isize, // todo bitflags?
    /// Check if window has been initialized successfully
    pub ready: bool,
    /// Check if fullscreen mode is enabled
    pub fullscreen: bool,
    /// Check if window set for closing
    pub shouldClose: bool,
    /// Check if window has been resized last frame
    pub revisedLastFrame: bool,
    /// Wait for events before ending frame
    pub eventWaiting: bool,

    /// Window position (required on fullscreen toggle)
    pub position: Point,
    /// Window previous position (required on borderless windowed toggle)
    pub previousPosition: Point,
    /// Display width and height (monitor, device-screen, LCD, ...)
    pub display: Size,
    /// Screen width and height (used render area)
    pub screen: Size,
    /// Screen previous width and height (required on borderless windowed toggle)
    pub previousScreen: Size,
    /// Current render width and height (depends on active fbo)
    pub currentFbo: Size,
    /// Framebuffer width and height (render area, including black bars if required)
    pub render: Size,
    /// Offset from render area (must be divided by 2)
    pub renderOffset: Point,
    /// Screen minimum width and height (for resizable window)
    pub screenMin: Size,
    /// Screen maximum width and height (for resizable window)
    pub screenMax: Size,
    /// Matrix to scale screen (framebuffer rendering)
    pub screenScale: Matrix,

    /// Store dropped files paths pointers (provided by GLFW)
    pub dropFilepaths: Vec<String>, // todo previously char**
    /// Count dropped files strings
    pub dropFileCount: usize,
}

pub struct Storage {
    /// Base path for data storage
    pub basePath: String,
}

// todo, types probably,
// likely bools instead of i8
// maybe usize instead of isize
pub struct Keyboard {
    /// Default exit key
    pub exitKey: isize,
    /// Registers current frame key state
    pub currentKeyState: [i8; MAX_KEYBOARD_KEYS],
    /// Registers previous frame key state
    pub previousKeyState: [i8; MAX_KEYBOARD_KEYS],

    // NOTE: Since key press logic involves comparing prev vs cur key state, we need to handle key repeats specially
    /// Registers key repeats for current frame.
    pub keyRepeatInFrame: [i8; MAX_KEYBOARD_KEYS],

    /// Input keys queue
    pub keyPressedQueue: [isize; MAX_KEY_PRESSED_QUEUE],
    /// Input keys queue count
    pub keyPressedQueueCount: isize,

    /// Input characters queue (unicode)
    pub charPressedQueue: [isize; MAX_KEY_PRESSED_QUEUE],
    /// Input characters queue count
    pub charPressedQueueCount: isize,
}

pub struct Mouse {
    /// Mouse offset
    pub offset: Vector2,
    /// Mouse scaling
    pub scale: Vector2,
    /// Mouse position on screen
    pub currentPosition: Vector2,
    /// Previous mouse position
    pub previousPosition: Vector2,

    /// Tracks current mouse cursor
    pub cursor: isize,
    /// Track if cursor is hidden
    pub cursorHidden: bool,
    /// Tracks if cursor is inside client area
    pub cursorOnScreen: bool,

    /// Registers current mouse button state
    pub currentButtonState: [i8; MAX_MOUSE_BUTTONS],
    /// Registers previous mouse button state
    pub previousButtonState: [i8; MAX_MOUSE_BUTTONS],
    /// Registers current mouse wheel variation
    pub currentWheelMove: Vector2,
    /// Registers previous mouse wheel variation
    pub previousWheelMove: Vector2,
}

pub struct Touch {
    /// Number of touch points active
    pub pointCount: isize,
    /// Point identifiers
    pub pointId: [isize; MAX_TOUCH_POINTS],
    /// Touch position on screen  
    pub position: [Vector2; MAX_TOUCH_POINTS],
    /// Registers current touch state
    pub currentTouchState: [i8; MAX_TOUCH_POINTS],
    /// Registers previous touch state
    pub previousTouchState: [i8; MAX_TOUCH_POINTS],
}

pub struct Gamepad {
    /// Register last gamepad button pressed
    pub lastButtonPressed: isize,
    /// Register number of available gamepad axis
    pub axisCount: [isize; MAX_GAMEPADS],
    /// Flag to know if gamepad is ready                
    pub ready: [bool; MAX_GAMEPADS],
    /// Gamepad name holder
    pub name: [[i8; MAX_GAMEPADS]; 64],
    /// Current gamepad buttons state
    pub currentButtonState: [[i8; MAX_GAMEPADS]; MAX_GAMEPAD_BUTTONS],
    /// Previous gamepad buttons state  
    pub previousButtonState: [[i8; MAX_GAMEPADS]; MAX_GAMEPAD_BUTTONS],
    /// Gamepad axis state
    pub axisState: [[f32; MAX_GAMEPADS]; MAX_GAMEPAD_AXIS],
}

pub struct Input {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
    pub touch: Touch,
    pub gamepad: Gamepad,
}

pub struct Time {
    /// Current time measure
    pub current: f64,
    /// Previous time measure
    pub previous: f64,
    /// Time measure for frame update
    pub update: f64,
    /// Time measure for frame draw
    pub draw: f64,
    /// Time measure for one frame
    pub frame: f64,
    /// Desired time for one frame, if 0 not applied
    pub target: f64,
    /// Base time measure for hi-res timer (PLATFORM_ANDROID, PLATFORM_DRM)
    pub base: u64,
    /// Frame counter
    pub frameCounter: u32,
}

/// Core global state context data
pub struct CoreData {
    pub window: Window,
    pub storage: Storage,
    pub input: Input,
    pub time: Time,
}

pub fn IsWindowFullscreen(core: &CoreData) -> bool {
    return core.window.fullscreen;
}
