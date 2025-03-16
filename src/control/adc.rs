use embassy_rp::adc::{Adc, Channel};
use heapless::Vec;

use super::CommandError;

pub struct Pins<'d> {
    pub adc: Adc<'d, embassy_rp::adc::Async>,
    pub vdd: Channel<'d>,
    pub vin: Channel<'d>,
}

#[derive(defmt::Format)]
pub enum Command {
    ReadVdd, // 0x50
    ReadVin, // 0x51
}

impl Command {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, CommandError> {
        //defmt::println!("ADC COMMAND {:x}", buf);
        match buf {
            [0x50] => Ok(Self::ReadVdd),
            [0x51] => Ok(Self::ReadVin),
            _ => Err(CommandError::Invalid),
        }
    }
}

impl super::ControllerCommand for Command {
    async fn handle(&self, controller: &mut super::Controller) -> Result<Vec<u8, 256>, CommandError> {
        let adc = &mut controller.adc.adc;
        let value = match self {
            Command::ReadVdd => adc.read(&mut controller.adc.vdd).await,
            Command::ReadVin => adc.read(&mut controller.adc.vin).await,
        }
        .map_err(|_| CommandError::Message("ADC Read Error"))?;

        Ok(Vec::from_slice(&value.to_le_bytes()).unwrap())
    }
}
