use crate::commands::Command;
use crate::error::{Error, SelfTestError};
use crate::types::RawSensorData;
use embedded_hal as hal;
use hal::blocking::delay::DelayMs;
use hal::blocking::i2c::{Read, Write, WriteRead};
use sensirion_i2c::{crc8, i2c};

const SGP41_I2C_ADDRESS: u8 = 0x59;

#[derive(Debug, Default)]
pub struct Sgp41<I2C, D> {
    i2c: I2C,
    delay: D,
    // useful in case of presence heat sources on the PCB (battery charger, motor, etc)
    temperature_offset: i16,
}

impl<I2C, D, E> Sgp41<I2C, D>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    D: DelayMs<u32>,
{
    pub fn new(i2c: I2C, delay: D) -> Self {
        Sgp41 {
            i2c,
            delay,
            temperature_offset: 0,
        }
    }

    /// This command starts the conditioning, i.e., the VOC pixel will be
    /// operated at the default temperature and humidity (+25 deg.C, 50% rH) as it is by calling the
    /// measure_raw command  while  the  NOx  pixel  will
    /// be  operated  at  a  different  temperature  for  conditioning.  This
    /// command returns only the measured raw signal of the VOC pixel SRAW_VOC as u16.
    pub fn execute_conditioning(&mut self) -> Result<u16, Error<E>> {
        let mut buf = [0; 3];
        self.read_cmd_args(Command::ExecuteConditioning, &[0x8000, 0x6666], &mut buf)?;
        Ok(u16::from_be_bytes([buf[0], buf[1]]))
    }

    pub fn measure_raw(&mut self) -> Result<RawSensorData, Error<E>> {
        let mut buf = [0; 6];
        self.read_cmd_args(Command::MeasureRawSignals, &[0x8000, 0x6666], &mut buf)?;
        let voc_ticks = u16::from_be_bytes([buf[0], buf[1]]);
        let nox_ticks = u16::from_be_bytes([buf[3], buf[4]]);
        let data = RawSensorData {
            voc_ticks,
            nox_ticks,
        };
        Ok(data)
    }

    pub fn measure_raw_compensated(
        &mut self,
        humidity: u8,
        temperature: i16,
    ) -> Result<RawSensorData, Error<E>> {
        assert!(humidity <= 100 && temperature >= -45 && temperature <= 130);
        let humidity_ticks = humidity as u16 * u16::MAX / 100;
        let temperature_ticks =
            (temperature + 45 + self.temperature_offset) as u16 * u16::MAX / 175;

        let mut buf = [0; 6];
        self.read_cmd_args(
            Command::MeasureRawSignals,
            &[humidity_ticks, temperature_ticks],
            &mut buf,
        )?;
        let voc_ticks = u16::from_be_bytes([buf[0], buf[1]]);
        let nox_ticks = u16::from_be_bytes([buf[3], buf[4]]);
        let data = RawSensorData {
            voc_ticks,
            nox_ticks,
        };
        Ok(data)
    }

    /// This command triggers the built-in self-test checking for integrity
    /// of both hotplate and MOX material and returns the result of this
    /// test as 2 bytes (+ 1 CRC byte).
    pub fn execute_self_test(&mut self) -> Result<(), Error<E>> {
        let mut buf = [0; 3];
        self.read_cmd(Command::ExecuteConditioning, &mut buf)?;
        // There is only two significant bits
        let err = u16::from_be_bytes([buf[0], buf[1]]) & 0b11;
        match err {
            0b00 => Ok(()),
            0b01 => Err(Error::SelfTest(SelfTestError::Voc)),
            0b10 => Err(Error::SelfTest(SelfTestError::Nox)),
            0b11 => Err(Error::SelfTest(SelfTestError::All)),
            _ => Err(Error::SelfTest(SelfTestError::Undefined)),
        }
    }

    /// This command turns the hotplate off and stops the measurement.
    /// Subsequently, the sensor enters the idle mode.
    pub fn turn_heater_off(&mut self) -> Result<(), Error<E>> {
        self.write_cmd(Command::TurnHeaterOff)
    }

    pub fn get_serial_number(&mut self) -> Result<u64, Error<E>> {
        let mut buf = [0; 9];
        self.read_cmd(Command::GetSerialNumber, &mut buf)?;

        let serial = u64::from(buf[0]) << 40
            | u64::from(buf[1]) << 32
            | u64::from(buf[3]) << 24
            | u64::from(buf[4]) << 16
            | u64::from(buf[6]) << 8
            | u64::from(buf[7]);
        Ok(serial)
    }

    /// This command is a general call resetting all devices connected to
    /// the same I2C bus. The first byte refers to the general call address
    /// and  the  second  byte  refers  to  the  reset  command.  After  calling
    /// this command, the SGP41 will restart entering the idle mode.
    pub fn soft_reset(&mut self) -> Result<(), Error<E>> {
        self.write_cmd(Command::SoftReset)
    }

    pub fn set_temperature_offset(&mut self, offset: i8) {
        self.temperature_offset += offset as i16;
    }

    /// Writes command without additional arguments.
    fn write_cmd(&mut self, cmd: Command) -> Result<(), Error<E>> {
        let (command, delay) = cmd.as_tuple();
        i2c::write_command(&mut self.i2c, SGP41_I2C_ADDRESS, command).map_err(Error::I2c)?;
        self.delay.delay_ms(delay);
        Ok(())
    }

    /// Writes command with additional arguments.
    fn write_cmd_args(&mut self, cmd: Command, args: &[u16]) -> Result<(), Error<E>> {
        let (command, delay) = cmd.as_tuple();

        let mut buf = [0; 8];
        assert!(command.to_ne_bytes().len() + args.len() * 3 <= buf.len());

        buf[0..2].copy_from_slice(&command.to_be_bytes());

        let mut i = 2;
        for arg in args {
            let end = i + 2;
            let be_arg = arg.to_be_bytes();
            buf[i..end].copy_from_slice(&be_arg);
            buf[end] = crc8::calculate(&be_arg);
            i += 3;
        }

        self.i2c
            .write(SGP41_I2C_ADDRESS, &buf[0..i])
            .map_err(Error::I2c)?;
        self.delay.delay_ms(delay);
        Ok(())
    }

    /// Read command execution result.
    fn read_cmd(&mut self, cmd: Command, data: &mut [u8]) -> Result<(), Error<E>> {
        self.write_cmd(cmd)?;
        i2c::read_words_with_crc(&mut self.i2c, SGP41_I2C_ADDRESS, data)?;
        Ok(())
    }

    /// Read command with args execution result.
    fn read_cmd_args(
        &mut self,
        cmd: Command,
        args: &[u16],
        data: &mut [u8],
    ) -> Result<(), Error<E>> {
        self.write_cmd_args(cmd, args)?;
        i2c::read_words_with_crc(&mut self.i2c, SGP41_I2C_ADDRESS, data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use embedded_hal_mock as hal;

    use self::hal::delay::MockNoop as DelayMock;
    use self::hal::i2c::{Mock as I2cMock, Transaction};
    use super::*;

    /// Test the get_serial_number function
    #[test]
    fn test_get_serial_number() {
        // Arrange
        let (cmd, _) = Command::GetSerialNumber.as_tuple();
        let expectations = [
            Transaction::write(SGP41_I2C_ADDRESS, cmd.to_be_bytes().to_vec()),
            Transaction::read(
                SGP41_I2C_ADDRESS,
                vec![0xbe, 0xef, 0x92, 0xbe, 0xef, 0x92, 0xbe, 0xef, 0x92],
            ),
        ];
        let mock = I2cMock::new(&expectations);
        let mut sensor = Sgp41::new(mock, DelayMock);
        // Act
        let serial = sensor.get_serial_number().unwrap();
        // Assert
        assert_eq!(serial, 0xbeefbeefbeef);
    }
}