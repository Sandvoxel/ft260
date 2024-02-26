// Enum to represent the possible chip modes
#[derive(Debug)]
pub enum ChipMode {
    DCNF0,
    DCNF1,
}

// Enum to represent the possible clock control values
#[derive(Debug)]
pub enum ClkCtl {
    MHz12,
    MHz24,
    MHz48,
}

// Enum to represent the possible suspend status values
#[derive(Debug)]
pub enum SuspendStatus {
    NotSuspended,
    Suspended,
}

// Enum to represent the possible PWREN status values
#[derive(Debug)]
pub enum PwrenStatus {
    NotReady,
    Ready,
}

// Enum to represent the possible I2C enable values
#[derive(Debug)]
pub enum I2cEnable {
    Disabled,
    Enabled,
}

// Enum to represent the possible UART mode values
#[derive(Debug)]
pub enum UartMode {
    Off,
    RtsCts,
    DtrDsr,
    XonXoff,
    NoFlowControl,
}

// Enum to represent the possible HID-over-I2C enable values
#[derive(Debug)]
pub enum HidOverI2cEnable {
    NotConfigured,
    Configured,
}

// Enum to represent the possible GPIO2 function values
#[derive(Debug)]
pub enum Gpio2Function {
    Gpio,
    SuspOut,
    PwrenLowActive,
    TxLed,
}

// Enum to represent the possible GPIOA function values
#[derive(Debug)]
pub enum GpioAFunction {
    Gpio,
    TxActive,
    TxLed,
}

// Enum to represent the possible GPIOG function values
#[derive(Debug)]
pub enum GpioGFunction {
    Gpio,
    PwrenLowActive,
    RxLed,
    BcdDet,
}

// Enum to represent the possible suspend output polarity values
#[derive(Debug)]
pub enum SuspendOutPol {
    ActiveHigh,
    ActiveLow,
}

// Enum to represent the possible interrupt condition values
#[derive(Debug)]
pub enum IntrCond {
    RisingEdge,
    LevelHigh,
    FallingEdge,
    LevelLow,
}

// Enum to represent the possible interrupt level duration values
#[derive(Debug)]
pub enum IntrLevelDuration {
    Ms1,
    Ms5,
    Ms30,
}

// Struct to represent the ft260 data
#[derive(Debug)]
pub struct Ft260Config {
    pub report_id: u8,
    pub chip_mode: ChipMode,
    pub clk_ctl: ClkCtl,
    pub suspend_status: SuspendStatus,
    pub pwren_status: PwrenStatus,
    pub i2c_enable: bool,
    pub uart_mode: UartMode,
    pub hid_over_i2c_enable: bool,
    pub gpio2_function: Gpio2Function,
    pub gpio_a_function: GpioAFunction,
    pub gpio_g_function: GpioGFunction,
    pub suspend_out_pol: SuspendOutPol,
    pub enable_wakeup_int: bool,
    pub intr_cond: IntrCond,
    pub intr_level_duration: IntrLevelDuration,
    pub enable_power_saving: bool,
}

impl Ft260Config {
    // Method to decode byte slice into Ft260Data
    pub fn from_bytes(data_slice: &[u8]) -> Self {
        Ft260Config {
            report_id: data_slice[0],
            chip_mode: if data_slice[1] & 0b00000001 != 0 { ChipMode::DCNF0 } else { ChipMode::DCNF1 },
            clk_ctl: match data_slice[2] {
                0 => ClkCtl::MHz12,
                1 => ClkCtl::MHz24,
                2 => ClkCtl::MHz48,
                _ => unreachable!(),
            },
            suspend_status: if data_slice[3] == 0 { SuspendStatus::NotSuspended } else { SuspendStatus::Suspended },
            pwren_status: if data_slice[4] == 0 { PwrenStatus::NotReady } else { PwrenStatus::Ready },
            i2c_enable: data_slice[5] != 0,
            uart_mode: match data_slice[6] {
                0 => UartMode::Off,
                1 => UartMode::RtsCts,
                2 => UartMode::DtrDsr,
                3 => UartMode::XonXoff,
                4 => UartMode::NoFlowControl,
                _ => unreachable!(),
            },
            hid_over_i2c_enable: data_slice[7] != 0,
            gpio2_function: match data_slice[8] {
                0 => Gpio2Function::Gpio,
                1 => Gpio2Function::SuspOut,
                2 => Gpio2Function::PwrenLowActive,
                4 => Gpio2Function::TxLed,
                _ => unreachable!(),
            },
            gpio_a_function: match data_slice[9] {
                0 => GpioAFunction::Gpio,
                3 => GpioAFunction::TxActive,
                4 => GpioAFunction::TxLed,
                _ => unreachable!(),
            },
            gpio_g_function: match data_slice[10] {
                0 => GpioGFunction::Gpio,
                2 => GpioGFunction::PwrenLowActive,
                5 => GpioGFunction::RxLed,
                6 => GpioGFunction::BcdDet,
                _ => unreachable!(),
            },
            suspend_out_pol: if data_slice[11] == 0 { SuspendOutPol::ActiveHigh } else { SuspendOutPol::ActiveLow },
            enable_wakeup_int: data_slice[12] != 0,
            intr_cond: match data_slice[13] & 0b11 {
                0b00 => IntrCond::RisingEdge,
                0b01 => IntrCond::LevelHigh,
                0b10 => IntrCond::FallingEdge,
                0b11 => IntrCond::LevelLow,
                _ => unreachable!(),
            },
            intr_level_duration: match (data_slice[13] >> 2) & 0b11 {
                0b01 => IntrLevelDuration::Ms1,
                0b10 => IntrLevelDuration::Ms5,
                0b11 => IntrLevelDuration::Ms30,
                _ => unreachable!(),
            },
            enable_power_saving: data_slice[14] != 0,
        }
    }



}