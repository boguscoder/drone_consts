#![no_std]

pub mod telemetry {
    use num_enum::TryFromPrimitive;
    use strum_macros::{AsRefStr, EnumIter};

    #[derive(Debug, EnumIter, AsRefStr, PartialEq, Clone, Copy, TryFromPrimitive)]
    #[repr(u8)]
    pub enum Category {
        None = 0,
        Imu,
        Baro,
        Rc,
        Attitude,
        Pid,
        Mix,
        Dshot,
        Dump,
    }

    #[derive(Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
    #[repr(u8)]
    pub enum Mode {
        Stream = 0,
        Store,
    }

    #[derive(Copy, Clone)]
    pub struct TelemetryState {
        pub mode: Mode,
        pub category: Category,
    }

    impl TelemetryState {
        pub const fn to_u16(self) -> u16 {
            ((self.mode as u16) << 8) | (self.category as u16)
        }

        pub fn from_u16(val: u16) -> Self {
            let mode = Mode::try_from((val >> 8) as u8).unwrap_or(Mode::Stream);
            let category = Category::try_from((val & 0xFF) as u8).unwrap_or(Category::None);

            Self { mode, category }
        }
    }
}
