use crate::vec::Vec2;

use core::time::Duration;


#[derive(Copy,Clone, Debug, Hash, PartialEq, Eq)]
pub enum Event{
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    /// Logical key, translated by modifiers, and reified by input character
    KeyTyped(char), 
    MouseMove(Vec2<f32>),
    MouseButtonPressed(u32),
    MouseButtonReleased(u32),
    MouseScroll(f32),
    WindowStateChanged(WindowEvent),
    Periodic(Duration),
}


#[derive(Copy,Clone, Debug, Hash, PartialEq, Eq)]
pub enum WindowState{
    Closed,
    Iconified,
    Floating,
    Maximized,
    Fullscren
}

#[derive(Copy,Clone, Debug, Hash, PartialEq, Eq)]
pub struct WindowEvent{
    pub previous_state: WindowState,
    pub new_state: WindowState,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum KeyCode{
    Char(char),
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LMeta,
    RMeta,
    LAlt,
    RAlt,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrtScreen,
    Insert,
    Delete,
    NumLock,
    ScrollLock,
    CapsLock,
    NumpadKey(char),
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    NumpadUp,
    NumpadDown,
    NumpadLeft,
    NumpadRight,
    PageUp,
    PageDown,
    Home,
    End,
}