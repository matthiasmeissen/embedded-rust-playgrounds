
use core::i32;

#[derive(Debug, PartialEq, Clone)]
pub struct Calibration {
    pub x: i32,
    pub x_min: i32,
    pub x_max: i32,
    pub cx: i32,

    pub y: i32,
    pub y_min: i32,
    pub y_max: i32,
    pub cy: i32,

    pub z: i32,
    pub z_min: i32,
    pub z_max: i32,
    pub cz: i32,

    pub is_calibrating: bool,
}

impl Calibration {
    pub fn new() -> Self {
        Self { 
            x: 0, 
            x_min: i32::MAX, 
            x_max: i32::MIN, 
            cx: 0,

            y: 0, 
            y_min: i32::MAX, 
            y_max: i32::MIN, 
            cy: 0,

            z: 0, 
            z_min: i32::MAX, 
            z_max: i32::MIN, 
            cz: 0,

            is_calibrating: true 
        }
    }

    pub fn update(&mut self, x: i32, y: i32, z: i32) {
        if x > self.x_max { self.x_max = x };
        if x < self.x_min { self.x_min = x };

        if y > self.y_max { self.y_max = y };
        if y < self.y_min { self.y_min = y };

        if z > self.z_max { self.z_max = z };
        if z < self.z_min { self.z_min = z };

        self.x = (self.x_min + self.x_max) / 2;
        self.y = (self.y_min + self.y_max) / 2;
        self.z = (self.z_min + self.z_max) / 2;
    }

    pub fn calculate_offset(&mut self, x: i32, y: i32, z: i32) {
        (self.cx, self.cy, self.cz) = (x - self.x, y - self.y, z - self.z);
    }
}