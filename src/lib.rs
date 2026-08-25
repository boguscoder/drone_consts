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
}
