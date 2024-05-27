pub struct Config;

impl Config {
    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 600;
    pub const WIDTH_UNIT: i32 = (Self::WIDTH / 8) as i32;
    pub const HEIGHT_UNIT: i32 = (Self::HEIGHT / 8) as i32;
}
