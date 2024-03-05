use hidapi::{HidDevice, HidError, HidResult};

pub struct FT260 {
    device: HidDevice,
}

impl FT260 {
    pub fn new() -> Result<Self, HidError> {
        let api = hidapi::HidApi::new()?;

        let (vid, pid) = (0x0403, 0x6030);

        Ok(FT260 {
            device: api.open(vid, pid).unwrap()
        })
    }

    pub fn enable_uart(
        &mut self
    ) -> HidResult<()> {
        let mut data = [0u8; 3];
        data[0] = 0xA1;
        data[1] = 0x03;
        data[2] = 0x04;

        self.device.send_feature_report(&data)
    }

    pub fn reset_uart(
        &mut self
    ) -> HidResult<()> {
        let mut data = [0u8; 2];
        data[0] = 0xA1;
        data[1] = 0x40;

        self.device.send_feature_report(&data)
    }

    pub fn configure_uart(
        &mut self,
        baud_rate: u32,
        data_bits: u8,
        parity: u8,
        stop_bits: u8,
        breaking: u8,
    ) -> HidResult<()>{
        let mut data = [0u8; 11];
        data[0] = 0xA1;
        data[1] = 0x41;
        data[2] = 0x04; // Placeholder for flow_ctrl

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

        self.device.send_feature_report(&data)
    }

    pub fn receive_data(&mut self) -> Result<Vec<u8>, ()> {
        let mut buf = [0; 64];

        self.device.read_timeout(&mut buf, 1).expect("TODO: panic message");

        Ok(buf[2..(buf[1] as usize + 2)].to_vec())
    }

    pub fn write_usb(&mut self, data: &[u8]) -> HidResult<usize> {
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

        self.device.write(payload.as_slice())
    }

}

