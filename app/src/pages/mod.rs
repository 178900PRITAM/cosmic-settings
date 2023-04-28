// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod desktop;
pub mod networking;
pub mod sound;
pub mod system;
pub mod time;
pub mod input;

#[derive(Clone, Debug)]
pub enum Message {
    About(system::about::Message),
    DateAndTime(time::date::Message),
    Desktop(desktop::Message),
    Input(input::Message),
    External { id: String, message: Vec<u8> },
}

// TODO handle page event dispatch here?
// Anything requiring list of builtin pages
