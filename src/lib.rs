#![no_std]

pub mod types;
use types::*;

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;
use types::Reg00Values;

pub const SGM41511_ADDR: SevenBitAddress = 0x6B;

#[repr(u8)]
pub enum Register {
    Reg00 = 0x00,
    Reg01 = 0x01,
    Reg02 = 0x02,
    Reg03 = 0x03, // Pre-Charge and Termination Current Settings
    Reg04 = 0x04,
    Reg05 = 0x05,
    Reg06 = 0x06,
    Reg07 = 0x07,
    Reg08 = 0x08, // Status Bits, Read Only
    Reg09 = 0x09, // Fault Bits, Read Only
    Reg0a = 0x0a,
    Reg0b = 0x0b,
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Voltage {
    Unattached = 0x00,
    _5v = 0x10,
    _9v = 0x20,
    _12v = 0x30,
    _15v = 0x40,
    _18v = 0x50,
    _20v = 0x60,
    Reserved = 0x70, // placeholder for other reserved values
}

impl From<u8> for Voltage {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Voltage::Unattached,
            0x10 => Voltage::_5v,
            0x20 => Voltage::_9v,
            0x30 => Voltage::_12v,
            0x40 => Voltage::_15v,
            0x50 => Voltage::_18v,
            0x60 => Voltage::_20v,
            _ => Voltage::Reserved,
        }
    }
}

impl<'a> Into<&'a str> for Voltage {
    fn into(self) -> &'a str {
        match self {
            Voltage::Unattached => "Unattached",
            Voltage::_5v => "5V",
            Voltage::_9v => "9V",
            Voltage::_12v => "12V",
            Voltage::_15v => "15V",
            Voltage::_18v => "18V",
            Voltage::_20v => "20V",
            Voltage::Reserved => "Reserved",
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Current {
    _0_5a = 0x00,
    _0_7a = 0x01,
    _1_0a = 0x02,
    _1_25a = 0x03,
    _1_5a = 0x04,
    _1_75a = 0x05,
    _2_0a = 0x06,
    _2_25a = 0x07,
    _2_5a = 0x08,
    _2_75a = 0x09,
    _3_0a = 0x0A,
    _3_25a = 0x0B,
    _3_5a = 0x0C,
    _4_0a = 0x0D,
    _4_5a = 0x0E,
    _5_0a = 0x0F,
}

impl From<u8> for Current {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Current::_0_5a,
            0x01 => Current::_0_7a,
            0x02 => Current::_1_0a,
            0x03 => Current::_1_25a,
            0x04 => Current::_1_5a,
            0x05 => Current::_1_75a,
            0x06 => Current::_2_0a,
            0x07 => Current::_2_25a,
            0x08 => Current::_2_5a,
            0x09 => Current::_2_75a,
            0x0A => Current::_3_0a,
            0x0B => Current::_3_25a,
            0x0C => Current::_3_5a,
            0x0D => Current::_4_0a,
            0x0E => Current::_4_5a,
            0x0F => Current::_5_0a,
            _ => unreachable!(),
        }
    }
}

impl<'a> Into<&'a str> for Current {
    fn into(self) -> &'a str {
        match self {
            Current::_0_5a => "0.5A",
            Current::_0_7a => "0.7A",
            Current::_1_0a => "1.0A",
            Current::_1_25a => "1.25A",
            Current::_1_5a => "1.5A",
            Current::_1_75a => "1.75A",
            Current::_2_0a => "2.0A",
            Current::_2_25a => "2.25A",
            Current::_2_5a => "2.5A",
            Current::_2_75a => "2.75A",
            Current::_3_0a => "3.0A",
            Current::_3_25a => "3.25A",
            Current::_3_5a => "3.5A",
            Current::_4_0a => "4.0A",
            Current::_4_5a => "4.5A",
            Current::_5_0a => "5.0A",
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrcPdo {
    NotSelected = 0x00,
    _5v = 0x10,
    _9v = 0x20,
    _12v = 0x30,
    _15v = 0x80,
    _18v = 0x90,
    _20v = 0xa0,
    Reserved = 0xf0, // placeholder for other reserved values
}

impl From<u8> for SrcPdo {
    fn from(value: u8) -> Self {
        match value {
            0x00 => SrcPdo::NotSelected,
            0x10 => SrcPdo::_5v,
            0x20 => SrcPdo::_9v,
            0x30 => SrcPdo::_12v,
            0x80 => SrcPdo::_15v,
            0x90 => SrcPdo::_18v,
            0xa0 => SrcPdo::_20v,
            _ => SrcPdo::Reserved,
        }
    }
}

impl<'a> Into<&'a str> for SrcPdo {
    fn into(self) -> &'a str {
        match self {
            SrcPdo::NotSelected => "NotSelected",
            SrcPdo::_5v => "5V",
            SrcPdo::_9v => "9V",
            SrcPdo::_12v => "12V",
            SrcPdo::_15v => "15V",
            SrcPdo::_18v => "18V",
            SrcPdo::_20v => "20V",
            SrcPdo::Reserved => "Reserved",
        }
    }
}

pub struct SGM41511<I2C> {
    i2c: I2C,
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "SGM41511",),
    async(feature = "async", keep_self)
)]
impl<I2C, E> SGM41511<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    #[inline(always)]
    pub async fn read_register(&mut self, register: Register) -> Result<u8, E> {
        let mut data = [0u8; 1];
        self.i2c
            .write_read(SGM41511_ADDR, &[register as u8], &mut data)
            .await?;
        Ok(data[0])
    }

    #[inline(always)]
    pub async fn write_register(&mut self, register: Register, value: u8) -> Result<(), E> {
        self.i2c
            .write(SGM41511_ADDR, &[register as u8, value])
            .await
    }

    #[inline(always)]
    pub async fn get_device_revision(&mut self) -> Result<Option<u8>, E> {
        let data = self.read_register(Register::Reg0b).await?;

        if data & 0x7c == 0x14 {
            return Ok(Some(data & 0x03));
        }

        Ok(None)
    }

    #[inline(always)]
    pub async fn get_reg00(&mut self) -> Result<Reg00Values, E> {
        let data = self.read_register(Register::Reg00).await?;
        Ok(Reg00Values::from(data))
    }

    #[inline(always)]
    pub async fn set_reg00(&mut self, value: Reg00Values) -> Result<(), E> {
        self.write_register(Register::Reg00, value.into()).await
    }

    #[inline(always)]
    pub async fn get_reg01(&mut self) -> Result<Reg01Values, E> {
        let data = self.read_register(Register::Reg01).await?;
        Ok(Reg01Values::from(data))
    }

    #[inline(always)]
    pub async fn set_reg01(&mut self, value: Reg01Values) -> Result<(), E> {
        self.write_register(Register::Reg01, value.into()).await
    }

    #[inline(always)]
    pub async fn get_reg02(&mut self) -> Result<Reg02Values, E> {
        let data = self.read_register(Register::Reg02).await?;
        Ok(Reg02Values::from(data))
    }

    #[inline(always)]
    pub async fn set_reg02(&mut self, value: Reg02Values) -> Result<(), E> {
        self.write_register(Register::Reg02, value.into()).await
    }

    #[inline(always)]
    pub async fn get_reg03(&mut self) -> Result<Reg03Values, E> {
        let data = self.read_register(Register::Reg03).await?;
        Ok(Reg03Values::from(data))
    }

    #[inline(always)]
    pub async fn set_reg03(&mut self, value: Reg03Values) -> Result<(), E> {
        self.write_register(Register::Reg03, value.into()).await
    }

    #[inline(always)]
    pub async fn get_reg04(&mut self) -> Result<Reg04Values, E> {
        let data = self.read_register(Register::Reg04).await?;
        Ok(Reg04Values::from(data))
    }

    #[inline(always)]
    pub async fn set_reg04(&mut self, value: Reg04Values) -> Result<(), E> {
        self.write_register(Register::Reg04, value.into()).await
    }

    #[inline(always)]
    pub async fn get_reg05(&mut self) -> Result<Reg05Values, E> {
        let data = self.read_register(Register::Reg05).await?;
        Ok(Reg05Values::from(data))
    }

    #[inline(always)]
    pub async fn set_reg05(&mut self, value: Reg05Values) -> Result<(), E> {
        self.write_register(Register::Reg05, value.into()).await
    }

    #[inline(always)]
    pub async fn get_reg06(&mut self) -> Result<Reg06Values, E> {
        let data = self.read_register(Register::Reg06).await?;
        Ok(Reg06Values::from(data))
    }

    #[inline(always)]
    pub async fn set_reg06(&mut self, value: Reg06Values) -> Result<(), E> {
        self.write_register(Register::Reg06, value.into()).await
    }

    #[inline(always)]
    pub async fn get_reg07(&mut self) -> Result<Reg07Values, E> {
        let data = self.read_register(Register::Reg07).await?;
        Ok(Reg07Values::from(data))
    }

    #[inline(always)]
    pub async fn set_reg07(&mut self, value: Reg07Values) -> Result<(), E> {
        self.write_register(Register::Reg07, value.into()).await
    }

    #[inline(always)]
    pub async fn get_reg08(&mut self) -> Result<Reg08Values, E> {
        let data = self.read_register(Register::Reg08).await?;
        Ok(Reg08Values::from(data))
    }

    #[inline(always)]
    pub async fn get_reg09(&mut self) -> Result<Reg09Values, E> {
        let data = self.read_register(Register::Reg09).await?;
        Ok(Reg09Values::from(data))
    }

    #[inline(always)]
    pub async fn get_reg0a(&mut self) -> Result<Reg0aValues, E> {
        let data = self.read_register(Register::Reg0a).await?;
        Ok(Reg0aValues::from(data))
    }

    #[inline(always)]
    pub async fn set_reg0a(&mut self, value: Reg0aValues) -> Result<(), E> {
        self.write_register(Register::Reg0a, value.into()).await
    }

    #[inline(always)]
    pub async fn set_interrupt_masks(&mut self, vindpm: bool, iindpm: bool) -> Result<(), E> {
        self.write_register(Register::Reg0b, (vindpm as u8) << 1 | (iindpm as u8)).await
    }

    #[inline(always)]
    pub async fn reset_register(&mut self) -> Result<(), E> {
        self.write_register(Register::Reg0b, 0x80).await
    }
}
