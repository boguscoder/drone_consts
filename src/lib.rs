#![no_std]

pub mod telemetry {
    use num_enum::TryFromPrimitive;
    use strum_macros::{AsRefStr, EnumIter};

    #[derive(Debug, EnumIter, AsRefStr, Eq, PartialEq, Clone, Copy, TryFromPrimitive)]
    #[repr(u8)]
    pub enum Mode {
        None = 0,
        Imu,
        Baro,
        Rc,
        Attitude,
        Pid,
        Mix,
        Dshot,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Command {
        SetTelemetryMode(Mode),
        DumpFlash,
    }

    impl From<Command> for u8 {
        fn from(val: Command) -> u8 {
            match val {
                Command::SetTelemetryMode(cat) => (cat as u8) << 4,
                Command::DumpFlash => 0x01,
            }
        }
    }

    impl TryFrom<u8> for Command {
        type Error = &'static str;

        fn try_from(val: u8) -> Result<Self, Self::Error> {
            match val & 0x0F {
                0x00 => {
                    let cat = Mode::try_from(val >> 4).map_err(|_| "Invalid Category")?;
                    Ok(Command::SetTelemetryMode(cat))
                }
                0x01 => {
                    if (val >> 4) != 0 {
                        return Err("DumpFlash expects no payload");
                    }
                    Ok(Command::DumpFlash)
                }
                _ => Err("Unknown Command ID"),
            }
        }
    }
}
