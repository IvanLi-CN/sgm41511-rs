#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputCurrentLimit {
    // 100 mA
    _100mA = 0x00,
    // 200 mA
    _200mA = 0x01,
    // 300 mA
    _300mA = 0x02,
    // 400 mA
    _400mA = 0x03,
    // 500 mA
    _500mA = 0x04,
    // 600 mA
    _600mA = 0x05,
    // 700 mA
    _700mA = 0x06,
    // 800 mA
    _800mA = 0x07,
    // 900 mA
    _900mA = 0x08,
    // 1000 mA
    _1000mA = 0x09,
    // 1100 mA
    _1100mA = 0x0a,
    // 1200 mA
    _1200mA = 0x0b,
    // 1300 mA
    _1300mA = 0x0c,
    // 1400 mA
    _1400mA = 0x0d,
    // 1500 mA
    _1500mA = 0x0e,
    // 1600 mA
    _1600mA = 0x0f,
    // 1700 mA
    _1700mA = 0x10,
    // 1800 mA
    _1800mA = 0x11,
    // 1900 mA
    _1900mA = 0x12,
    // 2000 mA
    _2000mA = 0x13,
    // 2100 mA
    _2100mA = 0x14,
    // 2200 mA
    _2200mA = 0x15,
    // 2300 mA
    _2300mA = 0x16,
    // 2400 mA
    _2400mA = 0x17,
    // 2500 mA
    _2500mA = 0x18,
    // 2600 mA
    _2600mA = 0x19,
    // 2700 mA
    _2700mA = 0x1a,
    // 2800 mA
    _2800mA = 0x1b,
    // 2900 mA
    _2900mA = 0x1c,
    // 3000 mA
    _3000mA = 0x1d,
    // 3100 mA
    _3100mA = 0x1e,
    // 3200 mA
    _3200mA = 0x1f,
}

/// Convert from u8 to InputCurrentLimit
///
/// Range: 0x00 - 0x1f
impl From<u8> for InputCurrentLimit {
    fn from(value: u8) -> Self {
        match value {
            0x00 => InputCurrentLimit::_100mA,
            0x01 => InputCurrentLimit::_200mA,
            0x02 => InputCurrentLimit::_300mA,
            0x03 => InputCurrentLimit::_400mA,
            0x04 => InputCurrentLimit::_500mA,
            0x05 => InputCurrentLimit::_600mA,
            0x06 => InputCurrentLimit::_700mA,
            0x07 => InputCurrentLimit::_800mA,
            0x08 => InputCurrentLimit::_900mA,
            0x09 => InputCurrentLimit::_1000mA,
            0x0a => InputCurrentLimit::_1100mA,
            0x0b => InputCurrentLimit::_1200mA,
            0x0c => InputCurrentLimit::_1300mA,
            0x0d => InputCurrentLimit::_1400mA,
            0x0e => InputCurrentLimit::_1500mA,
            0x0f => InputCurrentLimit::_1600mA,
            0x10 => InputCurrentLimit::_1700mA,
            0x11 => InputCurrentLimit::_1800mA,
            0x12 => InputCurrentLimit::_1900mA,
            0x13 => InputCurrentLimit::_2000mA,
            0x14 => InputCurrentLimit::_2100mA,
            0x15 => InputCurrentLimit::_2200mA,
            0x16 => InputCurrentLimit::_2300mA,
            0x17 => InputCurrentLimit::_2400mA,
            0x18 => InputCurrentLimit::_2500mA,
            0x19 => InputCurrentLimit::_2600mA,
            0x1a => InputCurrentLimit::_2700mA,
            0x1b => InputCurrentLimit::_2800mA,
            0x1c => InputCurrentLimit::_2900mA,
            0x1d => InputCurrentLimit::_3000mA,
            0x1e => InputCurrentLimit::_3100mA,
            0x1f => InputCurrentLimit::_3200mA,
            _ => InputCurrentLimit::_3200mA,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg00Values {
    // In HIZ mode, the VBUS pin is effectively
    // disconnected from internal circuit. Some
    // leakage current may exist.
    pub en_hiz: bool,
    // These bits turn on or off the function of the
    // STAT open-drain output pin (charge status
    // indicator).
    pub en_ichg_mon: bool,
    /// Input Current Limit Value. 100mA - 3200mA
    pub input_milliamps_limit: InputCurrentLimit,
}

/// Converts an `u8` to `Reg00Values`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values = Reg00Values::from(0b00010111);
/// assert_eq!(values, Reg00Values { en_hiz: false, en_ichg_mon: true, input_milliamps_limit: InputCurrentLimit::_2400mA });
///
/// let values = Reg00Values::from(0b11111111);
/// assert_eq!(values, Reg00Values { en_hiz: true, en_ichg_mon: false, input_milliamps_limit: InputCurrentLimit::_3200mA });
///
/// let values = Reg00Values::from(0b00000000);
/// assert_eq!(values, Reg00Values { en_hiz: false, en_ichg_mon: true, input_milliamps_limit: InputCurrentLimit::_100mA });
/// ```
impl From<u8> for Reg00Values {
    fn from(value: u8) -> Self {
        Reg00Values {
            en_hiz: value & 0x80 != 0,
            en_ichg_mon: value & 0x60 == 0,
            input_milliamps_limit: InputCurrentLimit::from(value & 0x1f),
        }
    }
}

/// Converts `Reg00Values` to `u8`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values: u8 = Reg00Values { en_hiz: false, en_ichg_mon: true, input_milliamps_limit: InputCurrentLimit::_2400mA }.into();
/// assert_eq!(values, 0b00010111);
///
/// let values: u8 = Reg00Values { en_hiz: true, en_ichg_mon: false, input_milliamps_limit: InputCurrentLimit::_3200mA }.into();
/// assert_eq!(values, 0b11111111);
///
/// let values: u8 = Reg00Values { en_hiz: false, en_ichg_mon: true, input_milliamps_limit: InputCurrentLimit::_100mA }.into();
/// assert_eq!(values, 0b00000000);
/// ```
impl<'a> Into<u8> for Reg00Values {
    fn into(self) -> u8 {
        let mut value = 0u8;
        if self.en_hiz {
            value |= 0x80;
        }
        if !self.en_ichg_mon {
            value |= 0x60;
        }
        value |= self.input_milliamps_limit as u8;
        value
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinSystemVoltage {
    // 2.6V
    _2_6V = 0b000,
    // 2.8V
    _2_8V = 0b001,
    // 3.0V
    _3_0V = 0b010,
    // 3.2V
    _3_2V = 0b011,
    // 3.4V
    _3_4V = 0b100,
    // 3.5V
    _3_5V = 0b101,
    // 3.6V
    _3_6V = 0b110,
    // 3.7V
    _3_7V = 0b111,
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinBatteryVoltageForOtG {
    // 2.95V
    _2_95V = 0,
    // 2.6V
    _2_6V = 1,
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg01Values {
    pub pfm_disabled: bool,
    // I2C watchdog timer reset
    pub watchdog_reset: bool,
    pub otg_enabled: bool,
    pub charge_enabled: bool,
    // Minimum system voltage
    pub sys_min_voltage: MinSystemVoltage,
    // Minimum battery voltage for OTG mode
    pub min_bat_sel: MinBatteryVoltageForOtG,
}

/// Converts `u8` to `Reg01Values`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values = Reg01Values::from(0b00011010);
/// assert_eq!(values, Reg01Values {
///     pfm_disabled: false,
///     watchdog_reset: false,
///     otg_enabled: false,
///     charge_enabled: true,
///     sys_min_voltage: MinSystemVoltage::_3_5V,
///     min_bat_sel: MinBatteryVoltageForOtG::_2_95V,
/// });
///
/// let values = Reg01Values::from(0b11111111);
/// assert_eq!(values, Reg01Values {
///     pfm_disabled: true,
///     watchdog_reset: true,
///     otg_enabled: true,
///     charge_enabled: true,
///     sys_min_voltage: MinSystemVoltage::_3_7V,
///     min_bat_sel: MinBatteryVoltageForOtG::_2_6V,
/// });
///
/// let values = Reg01Values::from(0b00000000);
/// assert_eq!(values, Reg01Values {
///     pfm_disabled: false,
///     watchdog_reset: false,
///     otg_enabled: false,
///     charge_enabled: false,
///     sys_min_voltage: MinSystemVoltage::_2_6V,
///     min_bat_sel: MinBatteryVoltageForOtG::_2_95V,
/// });
/// ```
impl From<u8> for Reg01Values {
    fn from(value: u8) -> Self {
        Reg01Values {
            pfm_disabled: value & 0x80 != 0,
            watchdog_reset: value & 0x40 != 0,
            otg_enabled: value & 0x20 != 0,
            charge_enabled: value & 0x10 != 0,
            sys_min_voltage: match (value & 0x0e) >> 1 {
                0b000 => MinSystemVoltage::_2_6V,
                0b001 => MinSystemVoltage::_2_8V,
                0b010 => MinSystemVoltage::_3_0V,
                0b011 => MinSystemVoltage::_3_2V,
                0b100 => MinSystemVoltage::_3_4V,
                0b101 => MinSystemVoltage::_3_5V,
                0b110 => MinSystemVoltage::_3_6V,
                _ => MinSystemVoltage::_3_7V,
            },
            min_bat_sel: match value & 0x01 {
                0 => MinBatteryVoltageForOtG::_2_95V,
                _ => MinBatteryVoltageForOtG::_2_6V,
            },
        }
    }
}

/// Converts `Reg01Values` to `u8`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values: u8 = Reg01Values {
///     pfm_disabled: false,
///     watchdog_reset: false,
///     otg_enabled: false,
///     charge_enabled: true,
///     sys_min_voltage: MinSystemVoltage::_3_5V,
///     min_bat_sel: MinBatteryVoltageForOtG::_2_95V
/// }.into();
/// assert_eq!(values, 0b00011010);
///
/// let values: u8 = Reg01Values {
///     pfm_disabled: true,
///     watchdog_reset: true,
///     otg_enabled: true,
///     charge_enabled: true,
///     sys_min_voltage: MinSystemVoltage::_3_7V,
///     min_bat_sel: MinBatteryVoltageForOtG::_2_6V
/// }.into();
/// assert_eq!(values, 0b11111111);
///
/// let values: u8 = Reg01Values {
///     pfm_disabled: false,
///     watchdog_reset: false,
///     otg_enabled: false,
///     charge_enabled: false,
///     sys_min_voltage: MinSystemVoltage::_2_6V,
///     min_bat_sel: MinBatteryVoltageForOtG::_2_95V
/// }.into();
/// assert_eq!(values, 0b00000000);
/// ```
impl<'a> Into<u8> for Reg01Values {
    fn into(self) -> u8 {
        let mut value = 0u8;
        if self.pfm_disabled {
            value |= 0x80;
        }
        if self.watchdog_reset {
            value |= 0x40;
        }
        if self.otg_enabled {
            value |= 0x20;
        }
        if self.charge_enabled {
            value |= 0x10;
        }
        value |= (self.sys_min_voltage as u8) << 1;
        value |= self.min_bat_sel as u8;

        value
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BoostCurrentLimit {
    // 0.5A
    _0_5A = 0,
    // 1.25A
    _1_25A = 1,
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Q1FullOnMode {
    // Use higher R_DSON if I_INPDM < 750 mA (for better accuracy)
    Accuracy = 0,
    // Use lower R_DSON always (fully ON for better efficiency)
    Efficiency = 1,
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeCurrent {
    // 0 mA
    _0mA = 0x00,
    // 60 mA
    _60mA = 0x01,
    // 120 mA
    _120mA = 0x02,
    // 180 mA
    _180mA = 0x03,
    // 240 mA
    _240mA = 0x04,
    // 300 mA
    _300mA = 0x05,
    // 360 mA
    _360mA = 0x06,
    // 420 mA
    _420mA = 0x07,
    // 480 mA
    _480mA = 0x08,
    // 540 mA
    _540mA = 0x09,
    // 600 mA
    _600mA = 0x0a,
    // 660 mA
    _660mA = 0x0b,
    // 720 mA
    _720mA = 0x0c,
    // 780 mA
    _780mA = 0x0d,
    // 840 mA
    _840mA = 0x0e,
    // 900 mA
    _900mA = 0x0f,
    // 960 mA
    _960mA = 0x10,
    // 1020 mA
    _1020mA = 0x11,
    // 1080 mA
    _1080mA = 0x12,
    // 1140 mA
    _1140mA = 0x13,
    // 1200 mA
    _1200mA = 0x14,
    // 1260 mA
    _1260mA = 0x15,
    // 1320 mA
    _1320mA = 0x16,
    // 1380 mA
    _1380mA = 0x17,
    // 1440 mA
    _1440mA = 0x18,
    // 1500 mA
    _1500mA = 0x19,
    // 1560 mA
    _1560mA = 0x1a,
    // 1620 mA
    _1620mA = 0x1b,
    // 1680 mA
    _1680mA = 0x1c,
    // 1740 mA
    _1740mA = 0x1d,
    // 1800 mA
    _1800mA = 0x1e,
    // 1860 mA
    _1860mA = 0x1f,
    // 1920 mA
    _1920mA = 0x20,
    // 1980 mA
    _1980mA = 0x21,
    // 2040 mA
    _2040mA = 0x22,
    // 2100 mA
    _2100mA = 0x23,
    // 2160 mA
    _2160mA = 0x24,
    // 2220 mA
    _2220mA = 0x25,
    // 2280 mA
    _2280mA = 0x26,
    // 2340 mA
    _2340mA = 0x27,
    // 2400 mA
    _2400mA = 0x28,
    // 2460 mA
    _2460mA = 0x29,
    // 2520 mA
    _2520mA = 0x2a,
    // 2580 mA
    _2580mA = 0x2b,
    // 2640 mA
    _2640mA = 0x2c,
    // 2700 mA
    _2700mA = 0x2d,
    // 2760 mA
    _2760mA = 0x2e,
    // 2820 mA
    _2820mA = 0x2f,
    // 2880 mA
    _2880mA = 0x30,
    // 2940 mA
    _2940mA = 0x31,
    // 3000 mA
    _3000mA = 0x32,
}

/// Converts an `u8` to `ChargeCurrent`
///
/// Range: 0x00 - 0x32
/// Values above 0x32 will be mapped to 0x32
impl From<u8> for ChargeCurrent {
    fn from(value: u8) -> Self {
        match value & 0x3f {
            0x00 => ChargeCurrent::_0mA,
            0x01 => ChargeCurrent::_60mA,
            0x02 => ChargeCurrent::_120mA,
            0x03 => ChargeCurrent::_180mA,
            0x04 => ChargeCurrent::_240mA,
            0x05 => ChargeCurrent::_300mA,
            0x06 => ChargeCurrent::_360mA,
            0x07 => ChargeCurrent::_420mA,
            0x08 => ChargeCurrent::_480mA,
            0x09 => ChargeCurrent::_540mA,
            0x0a => ChargeCurrent::_600mA,
            0x0b => ChargeCurrent::_660mA,
            0x0c => ChargeCurrent::_720mA,
            0x0d => ChargeCurrent::_780mA,
            0x0e => ChargeCurrent::_840mA,
            0x0f => ChargeCurrent::_900mA,
            0x10 => ChargeCurrent::_960mA,
            0x11 => ChargeCurrent::_1020mA,
            0x12 => ChargeCurrent::_1080mA,
            0x13 => ChargeCurrent::_1140mA,
            0x14 => ChargeCurrent::_1200mA,
            0x15 => ChargeCurrent::_1260mA,
            0x16 => ChargeCurrent::_1320mA,
            0x17 => ChargeCurrent::_1380mA,
            0x18 => ChargeCurrent::_1440mA,
            0x19 => ChargeCurrent::_1500mA,
            0x1a => ChargeCurrent::_1560mA,
            0x1b => ChargeCurrent::_1620mA,
            0x1c => ChargeCurrent::_1680mA,
            0x1d => ChargeCurrent::_1740mA,
            0x1e => ChargeCurrent::_1800mA,
            0x1f => ChargeCurrent::_1860mA,
            0x20 => ChargeCurrent::_1920mA,
            0x21 => ChargeCurrent::_1980mA,
            0x22 => ChargeCurrent::_2040mA,
            0x23 => ChargeCurrent::_2100mA,
            0x24 => ChargeCurrent::_2160mA,
            0x25 => ChargeCurrent::_2220mA,
            0x26 => ChargeCurrent::_2280mA,
            0x27 => ChargeCurrent::_2340mA,
            0x28 => ChargeCurrent::_2400mA,
            0x29 => ChargeCurrent::_2460mA,
            0x2a => ChargeCurrent::_2520mA,
            0x2b => ChargeCurrent::_2580mA,
            0x2c => ChargeCurrent::_2640mA,
            0x2d => ChargeCurrent::_2700mA,
            0x2e => ChargeCurrent::_2760mA,
            0x2f => ChargeCurrent::_2820mA,
            0x30 => ChargeCurrent::_2880mA,
            0x31 => ChargeCurrent::_2940mA,
            0x32 => ChargeCurrent::_3000mA,
            _ => ChargeCurrent::_3000mA,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg02Values {
    pub boost_current_limit: BoostCurrentLimit,
    pub q1_full_on: Q1FullOnMode,
    pub charge_current: ChargeCurrent,
}

/// Converts `u8` to `Reg02Values`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values = Reg02Values::from(0b10100010);
/// assert_eq!(values, Reg02Values { boost_current_limit: BoostCurrentLimit::_1_25A, q1_full_on: Q1FullOnMode::Accuracy, charge_current: ChargeCurrent::_2040mA });
///
/// let values = Reg02Values::from(0b00000000);
/// assert_eq!(values, Reg02Values { boost_current_limit: BoostCurrentLimit::_0_5A, q1_full_on: Q1FullOnMode::Accuracy, charge_current: ChargeCurrent::_0mA });
///
/// let values = Reg02Values::from(0b11110010);
/// assert_eq!(values, Reg02Values { boost_current_limit: BoostCurrentLimit::_1_25A, q1_full_on: Q1FullOnMode::Efficiency, charge_current: ChargeCurrent::_3000mA });
/// ```
impl From<u8> for Reg02Values {
    fn from(value: u8) -> Self {
        let boost_current_limit = match (value & 0x80) >> 7 {
            0 => BoostCurrentLimit::_0_5A,
            _ => BoostCurrentLimit::_1_25A,
        };
        let q1_full_on = match (value & 0x60) >> 6 {
            0 => Q1FullOnMode::Accuracy,
            _ => Q1FullOnMode::Efficiency,
        };
        let charge_current = (value & 0x3f).into();
        Self {
            boost_current_limit,
            q1_full_on,
            charge_current,
        }
    }
}

/// Converts `Reg02Values` to `u8`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values: u8 = Reg02Values { boost_current_limit: BoostCurrentLimit::_1_25A, q1_full_on: Q1FullOnMode::Accuracy, charge_current: ChargeCurrent::_2040mA }.into();
/// assert_eq!(values, 0b10100010);
///
/// let values: u8 = Reg02Values { boost_current_limit: BoostCurrentLimit::_0_5A, q1_full_on: Q1FullOnMode::Accuracy, charge_current: ChargeCurrent::_0mA }.into();
/// assert_eq!(values, 0b00000000);
///
/// let values: u8 = Reg02Values { boost_current_limit: BoostCurrentLimit::_1_25A, q1_full_on: Q1FullOnMode::Efficiency, charge_current: ChargeCurrent::_3000mA }.into();
/// assert_eq!(values, 0b11110010);
/// ```
impl Into<u8> for Reg02Values {
    fn into(self) -> u8 {
        let mut value = 0u8;
        if self.boost_current_limit == BoostCurrentLimit::_1_25A {
            value |= 0x80;
        }
        if self.q1_full_on == Q1FullOnMode::Efficiency {
            value |= 0x60;
        }

        value |= self.charge_current as u8;
        value
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PreChargeCurrent {
    // 60 mA
    _60mA = 0x00,
    // 120 mA
    _120mA = 0x01,
    // 180 mA
    _180mA = 0x02,
    // 240 mA
    _240mA = 0x03,
    // 300 mA
    _300mA = 0x04,
    // 360 mA
    _360mA = 0x05,
    // 420 mA
    _420mA = 0x06,
    // 480 mA
    _480mA = 0x07,
    // 540 mA
    _540mA = 0x08,
    // 600 mA
    _600mA = 0x09,
    // 660 mA
    _660mA = 0x0a,
    // 720 mA
    _720mA = 0x0b,
    // 780 mA
    _780mA = 0x0c,
}

/// Converts `u8` to `PreChargeCurrent`
///
/// Range: 0x00 - 0x0c (60 - 780 mA)
/// Values above 0x0c (780mA) are clamped to 0x0c (780mA)
impl From<u8> for PreChargeCurrent {
    fn from(value: u8) -> Self {
        match value {
            0x00 => PreChargeCurrent::_60mA,
            0x01 => PreChargeCurrent::_120mA,
            0x02 => PreChargeCurrent::_180mA,
            0x03 => PreChargeCurrent::_240mA,
            0x04 => PreChargeCurrent::_300mA,
            0x05 => PreChargeCurrent::_360mA,
            0x06 => PreChargeCurrent::_420mA,
            0x07 => PreChargeCurrent::_480mA,
            0x08 => PreChargeCurrent::_540mA,
            0x09 => PreChargeCurrent::_600mA,
            0x0a => PreChargeCurrent::_660mA,
            0x0b => PreChargeCurrent::_720mA,
            0x0c => PreChargeCurrent::_780mA,
            _ => PreChargeCurrent::_780mA,
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TermChargeCurrent {
    // 60 mA
    _60mA = 0x00,
    // 120 mA
    _120mA = 0x01,
    // 180 mA
    _180mA = 0x02,
    // 240 mA
    _240mA = 0x03,
    // 300 mA
    _300mA = 0x04,
    // 360 mA
    _360mA = 0x05,
    // 420 mA
    _420mA = 0x06,
    // 480 mA
    _480mA = 0x07,
    // 540 mA
    _540mA = 0x08,
    // 600 mA
    _600mA = 0x09,
    // 660 mA
    _660mA = 0x0a,
    // 720 mA
    _720mA = 0x0b,
    // 780 mA
    _780mA = 0x0c,
    // 840 mA
    _840mA = 0x0d,
    // 900 mA
    _900mA = 0x0e,
}

/// Converts `u8` to `TermChargeCurrent`
///
/// Range: 0x00 - 0x0e (60 - 900 mA)
/// Values above 0x0e (900mA) are clamped to 0x0e (900mA)
impl From<u8> for TermChargeCurrent {
    fn from(value: u8) -> Self {
        match value {
            0x00 => TermChargeCurrent::_60mA,
            0x01 => TermChargeCurrent::_120mA,
            0x02 => TermChargeCurrent::_180mA,
            0x03 => TermChargeCurrent::_240mA,
            0x04 => TermChargeCurrent::_300mA,
            0x05 => TermChargeCurrent::_360mA,
            0x06 => TermChargeCurrent::_420mA,
            0x07 => TermChargeCurrent::_480mA,
            0x08 => TermChargeCurrent::_540mA,
            0x09 => TermChargeCurrent::_600mA,
            0x0a => TermChargeCurrent::_660mA,
            0x0b => TermChargeCurrent::_720mA,
            0x0c => TermChargeCurrent::_780mA,
            0x0d => TermChargeCurrent::_840mA,
            0x0e => TermChargeCurrent::_900mA,
            _ => TermChargeCurrent::_900mA,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg03Values {
    pub pre_charge_current: PreChargeCurrent,
    pub term_charge_current: TermChargeCurrent,
}

/// Converts an `u8` to `Reg03Values`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::{PreChargeCurrent, TermChargeCurrent, Reg03Values};
/// let values = Reg03Values::from(0b00100010);
/// assert_eq!(values, Reg03Values { pre_charge_current: PreChargeCurrent::_180mA, term_charge_current: TermChargeCurrent::_180mA });
impl From<u8> for Reg03Values {
    fn from(value: u8) -> Self {
        Reg03Values {
            pre_charge_current: PreChargeCurrent::from(value >> 4),
            term_charge_current: TermChargeCurrent::from(value & 0x0f),
        }
    }
}

impl Into<u8> for Reg03Values {
    fn into(self) -> u8 {
        (self.pre_charge_current as u8) << 4 | (self.term_charge_current as u8)
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeVoltageLimit {
    // 3.856 V
    _3_856V = 0x00,
    // 3.888 V
    _3_888V = 0x01,
    // 3.920 V
    _3_920V = 0x02,
    // 3.952 V
    _3_952V = 0x03,
    // 3.984 V
    _3_984V = 0x04,
    // 4.016 V
    _4_016V = 0x05,
    // 4.048 V
    _4_048V = 0x06,
    // 4.080 V
    _4_080V = 0x07,
    // 4.112 V
    _4_112V = 0x08,
    // 4.144 V
    _4_144V = 0x09,
    // 4.176 V
    _4_176V = 0x0a,
    // 4.208 V
    _4_208V = 0x0b,
    // 4.240 V
    _4_240V = 0x0c,
    // 4.272 V
    _4_272V = 0x0d,
    // 4.304 V
    _4_304V = 0x0e,
    // 4.352 V (Special Value) 0b01111
    _4_352V = 0x0f,
    // 4.368 V
    _4_368V = 0x10,
    // 4.400 V
    _4_400V = 0x11,
    // 4.432 V
    _4_432V = 0x12,
    // 4.464 V
    _4_464V = 0x13,
    // 4.496 V
    _4_496V = 0x14,
    // 4.528 V
    _4_528V = 0x15,
    // 4.560 V
    _4_560V = 0x16,
    // 4.592 V
    _4_592V = 0x17,
    // 4.624 V
    _4_624V = 0x18,
}

/// Converts `u8` to `ChargeVoltageLimit`
///
/// Range: 0x00 - 0x18 (3.856 - 4.624 V)
/// Step: 32 mV
/// Values above 0x18 (4.624 V) are clamped to 0x18 (4.624 V)
impl From<u8> for ChargeVoltageLimit {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ChargeVoltageLimit::_3_856V,
            0x01 => ChargeVoltageLimit::_3_888V,
            0x02 => ChargeVoltageLimit::_3_920V,
            0x03 => ChargeVoltageLimit::_3_952V,
            0x04 => ChargeVoltageLimit::_3_984V,
            0x05 => ChargeVoltageLimit::_4_016V,
            0x06 => ChargeVoltageLimit::_4_048V,
            0x07 => ChargeVoltageLimit::_4_080V,
            0x08 => ChargeVoltageLimit::_4_112V,
            0x09 => ChargeVoltageLimit::_4_144V,
            0x0a => ChargeVoltageLimit::_4_176V,
            0x0b => ChargeVoltageLimit::_4_208V,
            0x0c => ChargeVoltageLimit::_4_240V,
            0x0d => ChargeVoltageLimit::_4_272V,
            0x0e => ChargeVoltageLimit::_4_304V,
            0x0f => ChargeVoltageLimit::_4_352V,
            0x10 => ChargeVoltageLimit::_4_368V,
            0x11 => ChargeVoltageLimit::_4_400V,
            0x12 => ChargeVoltageLimit::_4_432V,
            0x13 => ChargeVoltageLimit::_4_464V,
            0x14 => ChargeVoltageLimit::_4_496V,
            0x15 => ChargeVoltageLimit::_4_528V,
            0x16 => ChargeVoltageLimit::_4_560V,
            0x17 => ChargeVoltageLimit::_4_592V,
            0x18 => ChargeVoltageLimit::_4_624V,
            _ => ChargeVoltageLimit::_4_624V,
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TopOffTimer {
    // Disabled
    Disabled = 0x00,
    // 15 minutes
    _15Minutes = 0x01,
    // 30 minutes
    _30Minutes = 0x02,
    // 45 minutes
    _45Minutes = 0x03,
}

/// Converts `u8` to `TopOffTimer`
///
/// Range: 0x00 - 0x03
impl From<u8> for TopOffTimer {
    fn from(value: u8) -> Self {
        match value {
            0x00 => TopOffTimer::Disabled,
            0x01 => TopOffTimer::_15Minutes,
            0x02 => TopOffTimer::_30Minutes,
            0x03 => TopOffTimer::_45Minutes,
            _ => unreachable!("Invalid value for TopOffTimer. Range: 0x00 - 0x03"),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BatteryRechargeThreshold {
    // 100 mV below VREG(Charge Voltage Limit)
    _100mV = 0x00,
    // 200 mV below VREG(Charge Voltage Limit)
    _200mV = 0x01,
}

/// Converts `u8` to `BatteryRechargeThreshold`
///
/// Range: 0x00 - 0x01
impl From<u8> for BatteryRechargeThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BatteryRechargeThreshold::_100mV,
            0x01 => BatteryRechargeThreshold::_200mV,
            _ => unreachable!("Invalid value for BatteryRechargeThreshold. Range: 0x00 - 0x01"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg04Values {
    pub charge_voltage_limit: ChargeVoltageLimit,
    pub top_off_timer: TopOffTimer,
    pub battery_recharge_threshold: BatteryRechargeThreshold,
}

/// Converts `u8` to `Reg04Values`
///
/// Example:
///
/// ```rust
/// use sgm41511::types::*;
///
/// let values: u8 = Reg04Values { charge_voltage_limit: ChargeVoltageLimit::_4_208V, top_off_timer: TopOffTimer::Disabled, battery_recharge_threshold: BatteryRechargeThreshold::_100mV }.into();
/// assert_eq!(values, 0b01011000);
///
/// let values: u8 = Reg04Values { charge_voltage_limit: ChargeVoltageLimit::_3_856V, top_off_timer: TopOffTimer::Disabled, battery_recharge_threshold: BatteryRechargeThreshold::_100mV }.into();
/// assert_eq!(values, 0b00000000);
///
/// let values: u8 = Reg04Values { charge_voltage_limit: ChargeVoltageLimit::_4_624V, top_off_timer: TopOffTimer::_45Minutes, battery_recharge_threshold: BatteryRechargeThreshold::_200mV }.into();
/// assert_eq!(values, 0b11000111);
///
/// let values: u8 = Reg04Values { charge_voltage_limit: ChargeVoltageLimit::_4_352V, top_off_timer: TopOffTimer::Disabled, battery_recharge_threshold: BatteryRechargeThreshold::_100mV }.into();
/// assert_eq!(values, 0b01111000);
/// ```
impl Into<u8> for Reg04Values {
    fn into(self) -> u8 {
        let mut value = 0u8;
        value |= (self.charge_voltage_limit as u8) << 3;
        value |= (self.top_off_timer as u8) << 1;
        value |= self.battery_recharge_threshold as u8;
        value
    }
}

/// Converts `u8` to `Reg04Values`
///
/// Example:
///
/// ```rust
/// use sgm41511::types::*;
///
/// let values: u8 = Reg04Values { charge_voltage_limit: ChargeVoltageLimit::_4_208V, top_off_timer: TopOffTimer::Disabled, battery_recharge_threshold: BatteryRechargeThreshold::_100mV }.into();
/// assert_eq!(values, 0b01011000);
///
/// let values: u8 = Reg04Values { charge_voltage_limit: ChargeVoltageLimit::_3_856V, top_off_timer: TopOffTimer::Disabled, battery_recharge_threshold: BatteryRechargeThreshold::_100mV }.into();
/// assert_eq!(values, 0b00000000);
///
/// let values: u8 = Reg04Values { charge_voltage_limit: ChargeVoltageLimit::_4_624V, top_off_timer: TopOffTimer::_45Minutes, battery_recharge_threshold: BatteryRechargeThreshold::_200mV }.into();
/// assert_eq!(values, 0b11000111);
///
/// let values: u8 = Reg04Values { charge_voltage_limit: ChargeVoltageLimit::_4_352V, top_off_timer: TopOffTimer::Disabled, battery_recharge_threshold: BatteryRechargeThreshold::_100mV }.into();
/// assert_eq!(values, 0b01111000);
/// ```
impl From<u8> for Reg04Values {
    fn from(value: u8) -> Self {
        Reg04Values {
            charge_voltage_limit: ChargeVoltageLimit::from((value & 0xf8) >> 3),
            top_off_timer: TopOffTimer::from((value & 0x02) >> 1),
            battery_recharge_threshold: BatteryRechargeThreshold::from(value & 0x01),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WatchDogTimerSetting {
    Disabled = 0x00,
    _40Seconds = 0x01,
    _80Seconds = 0x02,
    _160Seconds = 0x03,
}

/// Converts `u8` to `WatchDogTimerSetting`
///
/// Range: 0x00 - 0x03
impl From<u8> for WatchDogTimerSetting {
    fn from(value: u8) -> Self {
        match value {
            0x00 => WatchDogTimerSetting::Disabled,
            0x01 => WatchDogTimerSetting::_40Seconds,
            0x02 => WatchDogTimerSetting::_80Seconds,
            0x03 => WatchDogTimerSetting::_160Seconds,
            _ => unreachable!("Invalid value for WatchDogTimerSetting. Range: 0x00 - 0x03"),
        }
    }
}

impl Into<u8> for WatchDogTimerSetting {
    fn into(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeTimerSetting {
    _4Hours = 0x00,
    _6Hours = 0x01,
}

/// Converts `u8` to `ChargeTimerSetting`
///
/// Range: 0x00 - 0x01
impl From<u8> for ChargeTimerSetting {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ChargeTimerSetting::_4Hours,
            0x01 => ChargeTimerSetting::_6Hours,
            _ => unreachable!("Invalid value for ChargeTimerSetting. Range: 0x00 - 0x01"),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThermalRegulationThreshold {
    // 80℃
    _80DegreeC = 0x00,
    // 120℃
    _120DegreeC = 0x01,
}

/// Converts `u8` to `ThermalRegulationThreshold`
///
/// Range: 0x00 - 0x01
impl From<u8> for ThermalRegulationThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ThermalRegulationThreshold::_80DegreeC,
            0x01 => ThermalRegulationThreshold::_120DegreeC,
            _ => unreachable!("Invalid value for ThermalRegulationThreshold. Range: 0x00 - 0x01"),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum JEITAChargingCurrent {
    // 50% of I_CHG
    _50Percent = 0x00,
    // 20% of I_CHG
    _20Percent = 0x01,
}

/// Converts `u8` to `JEITAChargingCurrent`
///
/// Range: 0x00 - 0x01
impl From<u8> for JEITAChargingCurrent {
    fn from(value: u8) -> Self {
        match value {
            0x00 => JEITAChargingCurrent::_50Percent,
            0x01 => JEITAChargingCurrent::_20Percent,
            _ => unreachable!("Invalid value for JEITAChargingCurrent. Range: 0x00 - 0x01"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg05Values {
    pub term_enabled: bool,
    pub watchdog_timer_setting: WatchDogTimerSetting,
    pub timer_enabled: bool,
    pub charge_timer_setting: ChargeTimerSetting,
    pub thermal_regulation_threshold: ThermalRegulationThreshold,
    pub jeita_charging_current: JEITAChargingCurrent,
}

/// Converts `u8` to `Reg05Values`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values = Reg05Values::from(0b10011111);
/// assert_eq!(values, Reg05Values {
///     term_enabled: true,
///     watchdog_timer_setting: WatchDogTimerSetting::_40Seconds,
///     timer_enabled: true,
///     charge_timer_setting: ChargeTimerSetting::_6Hours,
/// thermal_regulation_threshold: ThermalRegulationThreshold::_120DegreeC,
///     jeita_charging_current: JEITAChargingCurrent::_20Percent,
/// });
///
/// let values = Reg05Values::from(0b00000000);
/// assert_eq!(values, Reg05Values {
///     term_enabled: false,
///     watchdog_timer_setting: WatchDogTimerSetting::Disabled,
///     timer_enabled: false,
///     charge_timer_setting: ChargeTimerSetting::_4Hours,
///     thermal_regulation_threshold: ThermalRegulationThreshold::_80DegreeC,
///     jeita_charging_current: JEITAChargingCurrent::_50Percent,
/// });
///
/// let values = Reg05Values::from(0b11111111);
/// assert_eq!(values, Reg05Values {
///     term_enabled: true,
///     watchdog_timer_setting: WatchDogTimerSetting::_160Seconds,
///     timer_enabled: true,
///     charge_timer_setting: ChargeTimerSetting::_6Hours,
///     thermal_regulation_threshold: ThermalRegulationThreshold::_120DegreeC,
///     jeita_charging_current: JEITAChargingCurrent::_20Percent,
/// });
/// ```
impl From<u8> for Reg05Values {
    fn from(value: u8) -> Self {
        Reg05Values {
            term_enabled: value & 0x80 != 0,
            watchdog_timer_setting: WatchDogTimerSetting::from((value & 0x30) >> 4),
            timer_enabled: value & 0x30 != 0,
            charge_timer_setting: ChargeTimerSetting::from((value & 0x04) >> 2),
            thermal_regulation_threshold: ThermalRegulationThreshold::from((value & 0x02) >> 1),
            jeita_charging_current: JEITAChargingCurrent::from(value & 0x01),
        }
    }
}

/// Converts `Reg05Values` to `u8`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values: u8 = Reg05Values {
///     term_enabled: true,
///     watchdog_timer_setting: WatchDogTimerSetting::_40Seconds,
///     timer_enabled: true,
///     charge_timer_setting: ChargeTimerSetting::_6Hours,
///     thermal_regulation_threshold: ThermalRegulationThreshold::_120DegreeC,
///     jeita_charging_current: JEITAChargingCurrent::_20Percent,
/// }.into();
/// assert_eq!(values, 0b10011111);
///
/// let values: u8 = Reg05Values {
///     term_enabled: false,
///     watchdog_timer_setting: WatchDogTimerSetting::Disabled,
///     timer_enabled: false,
///     charge_timer_setting: ChargeTimerSetting::_4Hours,
///     thermal_regulation_threshold: ThermalRegulationThreshold::_80DegreeC,
///     jeita_charging_current: JEITAChargingCurrent::_50Percent,
/// }.into();
/// assert_eq!(values, 0b00000000);
///
/// let values: u8 = Reg05Values {
///     term_enabled: true,
///     watchdog_timer_setting: WatchDogTimerSetting::_160Seconds,
///     timer_enabled: true,
///     charge_timer_setting: ChargeTimerSetting::_6Hours,
///     thermal_regulation_threshold: ThermalRegulationThreshold::_120DegreeC,
///     jeita_charging_current: JEITAChargingCurrent::_20Percent,
/// }.into();
/// assert_eq!(values, 0b10111111);
/// ```
impl<'a> Into<u8> for Reg05Values {
    fn into(self) -> u8 {
        let mut value = 0u8;
        if self.term_enabled {
            value |= 0x80;
        }
        value |= (self.watchdog_timer_setting as u8) << 4;
        if self.timer_enabled {
            value |= 0x08;
        }
        value |= (self.charge_timer_setting as u8) << 2;
        value |= (self.thermal_regulation_threshold as u8) << 1;
        value |= self.jeita_charging_current as u8;
        value
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OVPThreshold {
    // 5.5 V
    _5_5V = 0x00,
    // 6.5 V (5V Input)
    _6_5V = 0x01,
    // 10.5 V (9V Input)
    _10_5V = 0x02,
    // 14V (12V Input)
    _14V = 0x03,
}

/// Converts `OVPThreshold` to `u8`
///
/// Range: 0x00 - 0x03
impl From<u8> for OVPThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x00 => OVPThreshold::_5_5V,
            0x01 => OVPThreshold::_6_5V,
            0x02 => OVPThreshold::_10_5V,
            0x03 => OVPThreshold::_14V,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BoostModeVoltage {
    // 4.85V
    _4_85V = 0x00,
    // 5.00V
    _5_00V = 0x01,
    // 5.15V
    _5_15V = 0x02,
    // 5.30V
    _5_30V = 0x03,
}

/// Converts `BoostModeVoltage` to `u8`
///
/// Range: 0x00 - 0x03
impl From<u8> for BoostModeVoltage {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BoostModeVoltage::_4_85V,
            0x01 => BoostModeVoltage::_5_00V,
            0x02 => BoostModeVoltage::_5_15V,
            0x03 => BoostModeVoltage::_5_30V,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VINDPMThreshold {
    // 3.9V
    _3_9V = 0x00,
    // 4.0V
    _4_0V = 0x01,
    // 4.1V
    _4_1V = 0x02,
    // 4.2V
    _4_2V = 0x03,
    // 4.3V
    _4_3V = 0x04,
    // 4.4V
    _4_4V = 0x05,
    // 4.5V
    _4_5V = 0x06,
    // 4.6V
    _4_6V = 0x07,
    // 4.7V
    _4_7V = 0x08,
    // 4.8V
    _4_8V = 0x09,
    // 4.9V
    _4_9V = 0x0a,
    // 5.0V
    _5_0V = 0x0b,
    // 5.1V
    _5_1V = 0x0c,
    // 5.2V
    _5_2V = 0x0d,
    // 5.3V
    _5_3V = 0x0e,
    // 5.4V
    _5_4V = 0x0f,
}

/// Converts `VINDPMThreshold` to `u8`
///
/// Range: 0x00 - 0x0f
impl From<u8> for VINDPMThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x00 => VINDPMThreshold::_3_9V,
            0x01 => VINDPMThreshold::_4_0V,
            0x02 => VINDPMThreshold::_4_1V,
            0x03 => VINDPMThreshold::_4_2V,
            0x04 => VINDPMThreshold::_4_3V,
            0x05 => VINDPMThreshold::_4_4V,
            0x06 => VINDPMThreshold::_4_5V,
            0x07 => VINDPMThreshold::_4_6V,
            0x08 => VINDPMThreshold::_4_7V,
            0x09 => VINDPMThreshold::_4_8V,
            0x0a => VINDPMThreshold::_4_9V,
            0x0b => VINDPMThreshold::_5_0V,
            0x0c => VINDPMThreshold::_5_1V,
            0x0d => VINDPMThreshold::_5_2V,
            0x0e => VINDPMThreshold::_5_3V,
            0x0f => VINDPMThreshold::_5_4V,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg06Values {
    pub ovp_threshold: OVPThreshold,
    // BOOSTV
    pub boost_mode_voltage: BoostModeVoltage,
    pub vindpm_threshold: VINDPMThreshold,
}

/// Converts `u8` to `Reg06Values`
///
/// Examples
///
/// ```rust
/// use sgm41511::types::*;
///
/// let values = Reg06Values::from(0b01100110);
/// assert_eq!(values, Reg06Values { ovp_threshold: OVPThreshold::_6_5V, boost_mode_voltage: BoostModeVoltage::_5_15V, vindpm_threshold: VINDPMThreshold::_4_5V });
///
/// let values = Reg06Values::from(0b00000000);
/// assert_eq!(values, Reg06Values { ovp_threshold: OVPThreshold::_5_5V, boost_mode_voltage: BoostModeVoltage::_4_85V, vindpm_threshold: VINDPMThreshold::_3_9V });
///
/// let values = Reg06Values::from(0b11111111);
/// assert_eq!(values, Reg06Values { ovp_threshold: OVPThreshold::_14V, boost_mode_voltage: BoostModeVoltage::_5_30V, vindpm_threshold: VINDPMThreshold::_5_4V });
/// ```
impl From<u8> for Reg06Values {
    fn from(value: u8) -> Self {
        Reg06Values {
            ovp_threshold: OVPThreshold::from((value & 0xc0) >> 6),
            boost_mode_voltage: BoostModeVoltage::from((value & 0x30) >> 4),
            vindpm_threshold: VINDPMThreshold::from((value & 0x0f) & 0x0f),
        }
    }
}

/// Converts `Reg06Values` to `u8`
///
/// Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let value: u8 = Reg06Values { ovp_threshold: OVPThreshold::_6_5V, boost_mode_voltage: BoostModeVoltage::_5_15V, vindpm_threshold: VINDPMThreshold::_4_5V }.into();
/// assert_eq!(value, 0b01100110);
///
/// let value: u8 = Reg06Values { ovp_threshold: OVPThreshold::_5_5V, boost_mode_voltage: BoostModeVoltage::_4_85V, vindpm_threshold: VINDPMThreshold::_3_9V }.into();
/// assert_eq!(value, 0b00000000);
///
/// let value: u8 = Reg06Values { ovp_threshold: OVPThreshold::_14V, boost_mode_voltage: BoostModeVoltage::_5_30V, vindpm_threshold: VINDPMThreshold::_5_4V }.into();
/// assert_eq!(value, 0b11111111);
/// ```
impl Into<u8> for Reg06Values {
    fn into(self) -> u8 {
        let mut value = 0u8;
        value |= (self.ovp_threshold as u8) << 6;
        value |= (self.boost_mode_voltage as u8) << 4;
        value |= (self.vindpm_threshold as u8) << 0;
        value
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum JEITAVoltageSetting {
    LowerOf4_1V = 0x00,
    VReg = 0x01,
}

/// Converts `u8` to `JEITAVoltageSetting`
///
/// Range: 0x00 - 0x01
impl From<u8> for JEITAVoltageSetting {
    fn from(value: u8) -> Self {
        match value {
            0x00 => JEITAVoltageSetting::LowerOf4_1V,
            0x01 => JEITAVoltageSetting::VReg,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VDPMBatteryVoltageTracking {
    // Disable (V_INDPM set by register)
    Disabled = 0x00,
    // V_BAT + 200mV
    _200mV = 0x01,
    // V_BAT + 250mV
    _250mV = 0x02,
    // V_BAT + 300mV
    _300mV = 0x03,
}

/// Converts `u8` to `VDPMBatteryVoltageTracking`
///
/// Range: 0x00 - 0x03
impl From<u8> for VDPMBatteryVoltageTracking {
    fn from(value: u8) -> Self {
        match value {
            0x00 => VDPMBatteryVoltageTracking::Disabled,
            0x01 => VDPMBatteryVoltageTracking::_200mV,
            0x02 => VDPMBatteryVoltageTracking::_250mV,
            0x03 => VDPMBatteryVoltageTracking::_300mV,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg07Values {
    // Input Current Limit Detection
    pub iindet_enabled: bool,
    // Enable Half Clock Rate Safety Timer
    pub tmr2x_enabled: bool,
    // Disable BATFET
    pub batfet_disabled: bool,
    // JEITA Charging Voltage (45℃ - 60℃)
    pub jeita_voltage_setting: JEITAVoltageSetting,
    // BATFET Turn Off Delay Control
    pub batfet_delay: bool,
    // Enable BATFET Reset
    pub batfet_reset_enabled: bool,
    // Dynamic VINDPM Tracking
    pub vdpm_battery_tracking: VDPMBatteryVoltageTracking,
}

/// Converts `u8` to `Reg07Values`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
///
/// let values = Reg07Values::from(0b01001100);
/// assert_eq!(values, Reg07Values {
///     iindet_enabled: false,
///     tmr2x_enabled: true,
///     batfet_disabled: false,
///     jeita_voltage_setting: JEITAVoltageSetting::LowerOf4_1V,
///     batfet_delay: true,
///     batfet_reset_enabled: true,
///     vdpm_battery_tracking: VDPMBatteryVoltageTracking::Disabled,
/// });
///
/// let values = Reg07Values::from(0b00000000);
/// assert_eq!(values, Reg07Values {
///     iindet_enabled: false,
///     tmr2x_enabled: false,
///     batfet_disabled: false,
///     jeita_voltage_setting: JEITAVoltageSetting::LowerOf4_1V,
///     batfet_delay: false,
///     batfet_reset_enabled: false,
///     vdpm_battery_tracking: VDPMBatteryVoltageTracking::Disabled,
/// });
///
/// let values = Reg07Values::from(0b11111111);
/// assert_eq!(values, Reg07Values {
///     iindet_enabled: true,
///     tmr2x_enabled: true,
///     batfet_disabled: true,
///     jeita_voltage_setting: JEITAVoltageSetting::VReg,
///     batfet_delay: true,
///     batfet_reset_enabled: true,
///     vdpm_battery_tracking: VDPMBatteryVoltageTracking::_300mV,
/// });
/// ```
impl From<u8> for Reg07Values {
    fn from(value: u8) -> Self {
        Reg07Values {
            iindet_enabled: (value & 0b10000000) != 0,
            tmr2x_enabled: (value & 0b01000000) != 0,
            batfet_disabled: (value & 0b00100000) != 0,
            jeita_voltage_setting: JEITAVoltageSetting::from((value & 0b00010000) >> 4),
            batfet_delay: (value & 0b00001000) != 0,
            batfet_reset_enabled: (value & 0b00000100) != 0,
            vdpm_battery_tracking: VDPMBatteryVoltageTracking::from((value & 0b00000011) as u8),
        }
    }
}

/// Converts `Reg07Values` to `u8`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
///
/// let values: u8 = Reg07Values {
///     iindet_enabled: false,
///     tmr2x_enabled: true,
///     batfet_disabled: false,
///     jeita_voltage_setting: JEITAVoltageSetting::LowerOf4_1V,
///     batfet_delay: true,
///     batfet_reset_enabled: true,
///     vdpm_battery_tracking: VDPMBatteryVoltageTracking::Disabled,
/// }.into();
/// assert_eq!(values, 0b01001100);
///
/// let values: u8 = Reg07Values {
///     iindet_enabled: false,
///     tmr2x_enabled: false,
///     batfet_disabled: false,
///     jeita_voltage_setting: JEITAVoltageSetting::LowerOf4_1V,
///     batfet_delay: false,
///     batfet_reset_enabled: false,
///     vdpm_battery_tracking: VDPMBatteryVoltageTracking::Disabled,
/// }.into();
/// assert_eq!(values, 0b00000000);
///
/// let values: u8 = Reg07Values {
///     iindet_enabled: true,
///     tmr2x_enabled: true,
///     batfet_disabled: true,
///     jeita_voltage_setting: JEITAVoltageSetting::VReg,
///     batfet_delay: true,
///     batfet_reset_enabled: true,
///     vdpm_battery_tracking: VDPMBatteryVoltageTracking::_300mV,
/// }.into();
/// assert_eq!(values, 0b11111111);
/// ```
impl Into<u8> for Reg07Values {
    fn into(self) -> u8 {
        let mut value = 0u8;
        if self.iindet_enabled {
            value |= 0b10000000;
        }
        if self.tmr2x_enabled {
            value |= 0b01000000;
        }
        if self.batfet_disabled {
            value |= 0b00100000;
        }
        value |= (self.jeita_voltage_setting as u8) << 4;
        if self.batfet_delay {
            value |= 0b00001000;
        }
        if self.batfet_reset_enabled {
            value |= 0b00000100;
        }
        value |= self.vdpm_battery_tracking as u8;
        value
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUSStatus {
    NoInput = 0x00,
    USBHostSDP = 0x01,
    Adaptor2_4A = 0x02,
    OTG = 0x03,
}

/// Converts u8 to `VBUSStatus`
///
/// Range 0x00 - 0x03
impl From<u8> for VBUSStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => VBUSStatus::NoInput,
            0x01 => VBUSStatus::USBHostSDP,
            0x02 => VBUSStatus::Adaptor2_4A,
            0x03 => VBUSStatus::OTG,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeStatus {
    Disabled = 0x00,
    Pre = 0x01,
    Fast = 0x02,
    Terminated = 0x03,
}

/// Converts u8 to `ChargeStatus`
///
/// Range 0x00 - 0x03
impl From<u8> for ChargeStatus {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ChargeStatus::Disabled,
            0x01 => ChargeStatus::Pre,
            0x02 => ChargeStatus::Fast,
            0x03 => ChargeStatus::Terminated,
            _ => unreachable!(),
        }
    }
}

/// Status Bits, Read Only
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg08Values {
    pub vbus_status: VBUSStatus,
    pub charge_status: ChargeStatus,
    // input Power Status (VBUS in good voltage range and not poor)
    pub pg_status: bool,
    // Thermal Regulation Status
    pub therm_status: bool,
    // System Voltage Regulation Status
    pub vsys_status: bool,
}

/// Converts `u8` to `Reg08Values`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values = Reg08Values::from(0b00101010);
/// assert_eq!(values, Reg08Values {
///     vbus_status: VBUSStatus::USBHostSDP,
///     charge_status: ChargeStatus::Pre,
///     pg_status: false,
///     therm_status: true,
///     vsys_status: false,
/// });
///
/// let values = Reg08Values::from(0b01010101);
/// assert_eq!(values, Reg08Values {
///     vbus_status: VBUSStatus::Adaptor2_4A,
///     charge_status: ChargeStatus::Fast,
///     pg_status: true,
///     therm_status: false,
///     vsys_status: true,
/// });
/// ```
impl From<u8> for Reg08Values {
    fn from(value: u8) -> Self {
        Reg08Values {
            vbus_status: VBUSStatus::from((value & 0xe0) >> 5),
            charge_status: ChargeStatus::from((value & 0x18) >> 3),
            pg_status: value & 0x04 != 0,
            therm_status: value & 0x02 != 0,
            vsys_status: value & 0x01 != 0,
        }
    }
}

/// Converts `Reg08Values` to `u8`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values: u8 = Reg08Values::from(0b00101010).into();
/// assert_eq!(values, 0b00101010);
///
/// let values: u8 = Reg08Values::from(0b01010101).into();
/// assert_eq!(values, 0b01010101);
/// ```
impl Into<u8> for Reg08Values {
    fn into(self) -> u8 {
        (self.vbus_status as u8) << 5
            | (self.charge_status as u8) << 3
            | if self.pg_status { 0x04 } else { 0x00 }
            | if self.therm_status { 0x02 } else { 0x00 }
            | if self.vsys_status { 0x01 } else { 0x00 }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeFault {
    Normal = 0x00,
    InputFault = 0x01,
    ThermalShutdown = 0x02,
    ChhargeSafetyTimerExpired = 0x03,
}

/// Converts u8 to `ChargeFault`
///
/// Range 0x00 - 0x03
impl From<u8> for ChargeFault {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ChargeFault::Normal,
            0x01 => ChargeFault::InputFault,
            0x02 => ChargeFault::ThermalShutdown,
            0x03 => ChargeFault::ChhargeSafetyTimerExpired,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NtcFault {
    Normal = 0x00,
    Warm = 0x02,
    // Cool (Buck mode only)
    Cool = 0x03,
    Cold = 0x05,
    Hot = 0x06,
}

/// Converts u8 to `NtcFault`
///
/// Range 0x00 - 0x04
impl From<u8> for NtcFault {
    fn from(value: u8) -> Self {
        match value {
            0x00 => NtcFault::Normal,
            0x02 => NtcFault::Warm,
            0x03 => NtcFault::Cool,
            0x05 => NtcFault::Cold,
            0x06 => NtcFault::Hot,
            _ => unreachable!(),
        }
    }
}

/// Fault Bits, Read Only
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg09Values {
    /// Watchdog Fault Status
    ///
    /// Watchdog timer expired
    pub watchdog_fault: bool,
    /// Boost Mode Fault Status
    ///
    /// VBUS is overloaded in OTG, or VBUS OVP, or battery voltage is too
    /// low (any condition that prevents Boost starting)
    pub boost_fault: bool,
    pub charge_fault: ChargeFault,
    /// Battery Fault Status
    ///
    /// Battery over voltage (BATOVP)
    pub bat_fault: bool,
    pub ntc_fault: NtcFault,
}

/// Converts `u8` to `Reg09Values`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values = Reg09Values::from(0b11111110);
/// assert_eq!(values, Reg09Values {
///     watchdog_fault: true,
///     boost_fault: true,
///     charge_fault: ChargeFault::ChhargeSafetyTimerExpired,
///     bat_fault: true,
///     ntc_fault: NtcFault::Hot,
/// });
///
/// let values = Reg09Values::from(0b00000000);
/// assert_eq!(values, Reg09Values {
///     watchdog_fault: false,
///     boost_fault: false,
///     charge_fault: ChargeFault::Normal,
///     bat_fault: false,
///     ntc_fault: NtcFault::Normal,
/// });
///
/// let values = Reg09Values::from(0b10101010);
/// assert_eq!(values, Reg09Values {
///     watchdog_fault: true,
///     boost_fault: false,
///     charge_fault: ChargeFault::ThermalShutdown,
///     bat_fault: true,
///     ntc_fault: NtcFault::Warm,
/// });
/// ```
impl From<u8> for Reg09Values {
    fn from(value: u8) -> Self {
        Reg09Values {
            watchdog_fault: value & 0x80 != 0,
            boost_fault: value & 0x40 != 0,
            charge_fault: ChargeFault::from((value & 0x30) >> 4),
            bat_fault: value & 0x08 != 0,
            ntc_fault: NtcFault::from(value & 0x07),
        }
    }
}

/// Converts `Reg09Values` to `u8`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values: u8 = Reg09Values {
///     watchdog_fault: true,
///     boost_fault: true,
///     charge_fault: ChargeFault::ChhargeSafetyTimerExpired,
///     bat_fault: true,
///     ntc_fault: NtcFault::Hot,
/// }.into();
/// assert_eq!(values, 0b11111110);
///
/// let values: u8 = Reg09Values {
///     watchdog_fault: false,
///     boost_fault: false,
///     charge_fault: ChargeFault::Normal,
///     bat_fault: false,
///     ntc_fault: NtcFault::Normal,
/// }.into();
/// assert_eq!(values, 0b00000000);
///
/// let values: u8 = Reg09Values {
///     watchdog_fault: true,
///     boost_fault: false,
///     charge_fault: ChargeFault::ThermalShutdown,
///     bat_fault: true,
///     ntc_fault: NtcFault::Warm,
/// }.into();
/// assert_eq!(values, 0b10101010);
///
/// ```
impl Into<u8> for Reg09Values {
    fn into(self) -> u8 {
        let mut value = 0u8;
        if self.watchdog_fault {
            value |= 0x80;
        }
        if self.boost_fault {
            value |= 0x40;
        }
        value |= (self.charge_fault as u8) << 4;
        if self.bat_fault {
            value |= 0x08;
        }
        value |= self.ntc_fault as u8;
        value
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reg0aValues {
    // Good Input Source Detected
    pub vbus_gd: bool,
    // Input Voltage Regulation (Dynamic Power Management)
    pub vindpm_status: bool,
    // Input Current Regulation (Dynamic Power Management)
    pub iindpm_status: bool,
    // Input Current Limit Detection
    pub iindet_enabled: bool,
    // Active Top-Off Timer Counting Status
    pub topoff_active: bool,
    // Input Over-Voltage Status (AC adaptor is the input source)
    pub acov_status: bool,
    // VINDPM Event Delection Interrupt Mask
    pub vindpm_int_mask: bool,
    // IINDPM Event Delection Interrupt Mask
    pub iindpm_int_mask: bool,
}

/// Converts `u8` to `Reg0aValues`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values = Reg0aValues::from(0b10101010);
/// assert_eq!(values, Reg0aValues {
///     vbus_gd: true,
///     vindpm_status: false,
///     iindpm_status: true,
///     iindet_enabled: false,
///     topoff_active: true,
///     acov_status: false,
///     vindpm_int_mask: true,
///     iindpm_int_mask: false
/// });
///
/// let values = Reg0aValues::from(0b01010101);
/// assert_eq!(values, Reg0aValues {
///     vbus_gd: false,
///     vindpm_status: true,
///     iindpm_status: false,
///     iindet_enabled: true,
///     topoff_active: false,
///     acov_status: true,
///     vindpm_int_mask: false,
///     iindpm_int_mask: true
/// });
/// ```
impl From<u8> for Reg0aValues {
    fn from(value: u8) -> Self {
        Reg0aValues {
            vbus_gd: value & 0x80 != 0,
            vindpm_status: value & 0x40 != 0,
            iindpm_status: value & 0x20 != 0,
            iindet_enabled: value & 0x10 != 0,
            topoff_active: value & 0x08 != 0,
            acov_status: value & 0x04 != 0,
            vindpm_int_mask: value & 0x02 != 0,
            iindpm_int_mask: value & 0x01 != 0,
        }
    }
}

/// Converts `Reg0aValues` to `u8`
///
/// # Examples
///
/// ```rust
/// use sgm41511::types::*;
/// let values: u8 = Reg0aValues {
///     vbus_gd: true,
///     vindpm_status: false,
///     iindpm_status: true,
///     iindet_enabled: false,
///     topoff_active: true,
///     acov_status: false,
///     vindpm_int_mask: true,
///     iindpm_int_mask: false
/// }.into();
/// assert_eq!(values, 0b10101010);
///
/// let values: u8 = Reg0aValues {
///     vbus_gd: false,
///     vindpm_status: true,
///     iindpm_status: false,
///     iindet_enabled: true,
///     topoff_active: false,
///     acov_status: true,
///     vindpm_int_mask: false,
///     iindpm_int_mask: true
/// }.into();
/// assert_eq!(values, 0b01010101);
/// ```
impl Into<u8> for Reg0aValues {
    fn into(self) -> u8 {
        let mut value = 0u8;
        if self.vbus_gd {
            value |= 0x80;
        }
        if self.vindpm_status {
            value |= 0x40;
        }
        if self.iindpm_status {
            value |= 0x20;
        }
        if self.iindet_enabled {
            value |= 0x10;
        }
        if self.topoff_active {
            value |= 0x08;
        }
        if self.acov_status {
            value |= 0x04;
        }
        if self.vindpm_int_mask {
            value |= 0x02;
        }
        if self.iindpm_int_mask {
            value |= 0x01;
        }
        value
    }
}
