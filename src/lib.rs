#![feature(const_trait_impl)]

use std::{collections::HashMap, sync::RwLock};

use dynamic::ext::*;
use once_cell::sync::OnceCell;
use skyline::hooks::InlineCtx;
use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};

const fn default_true() -> bool {true}
const fn default_one() -> usize {1}
const fn default_none<T>() -> Option<T> { None }

#[cfg(not(test))]
#[repr(C)]
struct SomeControllerStruct {
    pub padding: [u8; 0x10],
    pub controller: &'static mut Controller,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum Button {
    Attack      = 0x1,
    Special     = 0x2,
    Jump        = 0x4,
    Guard       = 0x8,
    Catch       = 0x10,
    Smash       = 0x20,
    JumpMini    = 0x40,
    CStickOn    = 0x80,
    StockShare  = 0x100,
    AttackRaw   = 0x200,
    AppealHi    = 0x400,
    SpecialRaw  = 0x800,
    AppealLw    = 0x1000,
    AppealSL    = 0x2000,
    AppealSR    = 0x4000,
    FlickJump   = 0x8000,
    GuardHold   = 0x10000,
    SpecialRaw2 = 0x20000,
    SpecialAll  = 0x20802,
    AttackAll   = 0x201,
    AppealAll   = 0x7400,
}

impl Button {
    pub fn bits(&self) -> Buttons {
        Buttons::from_bits(*self as i32).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TriggerAction {
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    LeftStick,
    RightStick,
    ButtonA,
    ButtonB,
    ButtonX,
    ButtonY,
    ButtonZL,
    ButtonZR,
    ButtonR,
    ButtonL,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TriggerKind {
    #[default]
    Holding,
    JustPressed,
    JustReleased
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trigger {
    action: TriggerAction,
    #[serde(default)]
    kind: TriggerKind
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Macro {
    pub trigger: Trigger,
    #[serde(default)]
    pub name: String,
    pub steps: Vec<FrameInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sticks {
    pub lstick_x: i8,
    pub lstick_y: i8,
    pub rstick_x: i8,
    pub rstick_y: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FrameInput {
    pub buttons: Vec<Button>,
    #[serde(default = "default_none")]
    pub sticks: Option<Sticks>,
    #[serde(default = "default_one")]
    pub hold_frames: usize,
}

#[derive(Debug, Clone, Default)]
pub struct MacroState {
    /// which macro we are currently executing (or none)
    macro_idx: Option<usize>,
    /// which step we're on for the selected macro
    macro_step: usize,
    /// remaining frames for holding the current macro step
    remaining_step_frames: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MacroConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub macros: Vec<Macro>,
    #[serde(default = "default_true")]
    pub reload: bool,
}
