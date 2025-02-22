use std::fmt;
use ggez::input::keyboard::KeyCode;
use crate::runtime::input::InputComponent;
pub type Void_Fn = fn();
pub type Bool_KeyCode_Fn = fn(KeyCode) -> bool;