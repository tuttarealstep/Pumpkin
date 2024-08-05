use pumpkin_macros::packet;
use serde::Serialize;

use crate::VarInt32;

#[derive(Serialize)]
#[packet(0x40)]
pub struct CSyncPlayerPostion {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    flags: i8,
    teleport_id: VarInt32,
}

impl CSyncPlayerPostion {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: i8,
        teleport_id: VarInt32,
    ) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
            flags,
            teleport_id,
        }
    }
}
