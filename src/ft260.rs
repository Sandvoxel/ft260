use std::io;
use std::time::Duration;
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Direction, Recipient, request_type, RequestType};
use crate::util::open_device;

pub struct FT260 {
    device: Device<Context>,
    device_desc: DeviceDescriptor,
    handle: DeviceHandle<Context>,
    timeout: Duration
}

impl FT260 {
    pub fn new() -> Option<Self> {
        match Context::new() {
            Ok(mut context) => match open_device(&mut context, 0x0403, 0x6030) {
                Some((device, device_desc, handle)) => {
                    return Some(FT260{
                        device,
                        device_desc,
                        handle,
                        timeout: Duration::from_millis(20),
                    })
                }
                None => {
                    println!("could not find device {:04x}:{:04x}", 0403, 6030);
                    None
                },
            },
            Err(e) => panic!("could not initialize libusb: {}", e),
        }
    }

    pub fn enable_uart(
        &mut self
    ) -> rusb::Result<usize> {
        let mut data = [0u8; 64];
        data[0] = 0xA1;
        data[1] = 0x03;
        data[2] = 0x03;

        self.handle.write_control(
            request_type(Direction::Out, RequestType::Class, Recipient::Interface),
            0x09,
            0x03A1,
            0x00,
            &data,
            self.timeout
        )
    }

    pub fn configure_uart(
        &mut self,
        baud_rate: u32,
        data_bits: u8,
        parity: u8,
        stop_bits: u8,
        breaking: u8,
    ) -> rusb::Result<usize> {
        let mut data = [0u8; 64];
        data[0] = 0xA1;
        data[1] = 0x41;
        data[2] = 0x00; // Placeholder for flow_ctrl

        // Convert baud rate to little-endian bytes
        let baud_bytes = baud_rate.to_le_bytes();
        data[3] = baud_bytes[0];
        data[4] = baud_bytes[1];
        data[5] = baud_bytes[2];
        data[6] = baud_bytes[3];

        data[7] = data_bits;
        data[8] = parity;
        data[9] = stop_bits;
        data[10] = breaking;

        self.handle.write_control(
            request_type(Direction::Out, RequestType::Class, Recipient::Interface),
            0x09,
            0x03A1,
            0x00,
            &data,
            self.timeout
        )
    }

    pub fn receive_data(&mut self) -> Result<Vec<u8>, rusb::Error> {
        let mut buf = [0; 64];
        match self.handle.read_interrupt(0x81, &mut buf, self.timeout) {
            Ok(_len) => {
                // Convert the relevant part of the buffer into a Vec and return it
                Ok(buf[2..(buf[1] as usize + 2)].to_vec())
            },
            Err(err) => Err(err),
        }
    }

    pub fn write_usb(&mut self, data: &[u8]) -> rusb::Result<usize> {
        // Calculate the length of the data
        let length = data.len() as u8;
        // Determine the appropriate report ID based on the length of the data
        let report_id = match length {
            0..=3 => 0xF0,   // 4 bytes data payload
            4..=7 => 0xF1,   // 8 bytes data payload
            8..=11 => 0xF2,  // 12 bytes data payload
            12..=15 => 0xF3, // 16 bytes data payload
            16..=19 => 0xF4, // 20 bytes data payload
            20..=23 => 0xF5, // 24 bytes data payload
            24..=27 => 0xF6, // 28 bytes data payload
            28..=31 => 0xF7, // 32 bytes data payload
            32..=35 => 0xF8, // 36 bytes data payload
            36..=39 => 0xF9, // 40 bytes data payload
            40..=43 => 0xFA, // 44 bytes data payload
            44..=47 => 0xFB, // 48 bytes data payload
            48..=51 => 0xFC, // 52 bytes data payload
            52..=55 => 0xFD, // 56 bytes data payload
            56..=59 => 0xFE, // 60 bytes data payload
            _ => panic!("Payload size exceeds maximum supported size (60 bytes)"),
        };
        // Construct the data payload with the appropriate report ID and length
        let mut payload = vec![report_id, length];
        payload.extend_from_slice(data);
        payload.resize(64, 0);

        let value: u16 = (0x03u16) << 8 | (report_id as u16);

        self.handle.write_interrupt(0x02, payload.as_slice(), self.timeout)?;

        Ok(0)
    }

}

