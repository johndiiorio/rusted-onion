use std::convert::TryInto;
use thiserror::Error;

pub struct Packet<'a> {
    ip_header: &'a [u8],
    udp_header: &'a [u8],
    udp_data: &'a [u8],
}

const IP_ADDRESS_LENGTH: u16 = 20;

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("Eror creating packet: {message}")]
    CreatePacket { message: String },
}

impl<'a> Packet<'a> {
    pub fn new(start_index: usize, bytes: &'a Vec<u8>) -> Result<Packet<'a>, PacketError> {
        let ip_header_end_index = start_index + IP_ADDRESS_LENGTH as usize;
        let udp_header_end_index = ip_header_end_index + 8;

        if ip_header_end_index > bytes.len() {
            return Err(PacketError::CreatePacket {
                message: String::from("Out of bounds with start_index"),
            });
        }

        let ip_header = &bytes[start_index..ip_header_end_index];
        let udp_header = &bytes[ip_header_end_index..udp_header_end_index];
        let udp_data_length = udp_total_length(udp_header) - 8;
        let udp_data =
            &bytes[udp_header_end_index..udp_header_end_index + udp_data_length as usize];

        Ok(Packet {
            ip_header,
            udp_header,
            udp_data,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_ip_header() && self.is_valid_udp_header()
    }

    pub fn get_packet_length(&self) -> u16 {
        IP_ADDRESS_LENGTH + self.get_udp_total_length()
    }

    pub fn get_data(&self) -> &[u8] {
        self.udp_data
    }

    fn get_udp_total_length(&self) -> u16 {
        udp_total_length(self.udp_header)
    }

    fn get_source_address(&self) -> &[u8; 4] {
        self.ip_header[12..16]
            .try_into()
            .expect("Source address has incorrect length")
    }

    fn get_destination_address(&self) -> &[u8; 4] {
        self.ip_header[16..20]
            .try_into()
            .expect("Destination address has incorrect length")
    }

    fn get_protocol(&self) -> u8 {
        self.ip_header[9]
    }

    fn get_source_port(&self) -> u16 {
        let source_port_bytes = &self.udp_header[0..2];
        u16::from_be_bytes([source_port_bytes[0], source_port_bytes[1]])
    }

    fn get_destination_port(&self) -> u16 {
        let destination_port_bytes = &self.udp_header[2..4];
        u16::from_be_bytes([destination_port_bytes[0], destination_port_bytes[1]])
    }

    fn get_udp_checksum(&self) -> u16 {
        let udp_checksum_bytes = &self.udp_header[6..8];
        u16::from_be_bytes([udp_checksum_bytes[0], udp_checksum_bytes[1]])
    }

    fn is_valid_ip_header(&self) -> bool {
        let version_and_ihl = self.ip_header[0];
        let version_and_ihl_bit_string = format!("{:08b}", version_and_ihl);
        if !version_and_ihl_bit_string.eq("01000101") {
            return false;
        }
        let source_ip_address = self.get_source_address();
        if !ip_address_to_decimal(source_ip_address).eq("10.1.1.10") {
            return false;
        }
        let destination_ip_address = self.get_destination_address();
        if !ip_address_to_decimal(destination_ip_address).eq("10.1.1.200") {
            return false;
        }
        verify_ip_checksum(self.ip_header)
    }

    fn is_valid_udp_header(&self) -> bool {
        if self.get_destination_port() != 42069 {
            return false;
        }
        self.verify_udp_checksum()
    }

    fn verify_udp_checksum(&self) -> bool {
        let source_address = self.get_source_address();
        let destination_adddress = self.get_destination_address();
        let udp_total_length = self.get_udp_total_length();

        let mut psuedo_header: Vec<u16> = vec![
            u16::from_be_bytes([source_address[0], source_address[1]]),
            u16::from_be_bytes([source_address[2], source_address[3]]),
            u16::from_be_bytes([destination_adddress[0], destination_adddress[1]]),
            u16::from_be_bytes([destination_adddress[2], destination_adddress[3]]),
            self.get_protocol() as u16,
            udp_total_length,
            self.get_source_port(),
            self.get_destination_port(),
            udp_total_length,
        ];
        let mut udp_data = self.get_data().to_vec();
        if udp_data.len() % 2 != 0 {
            udp_data.push(0);
        }
        for i in (0..udp_data.len()).step_by(2) {
            let index = i as usize;
            let part = &udp_data[index..index + 2];
            let part_as_u16 = u16::from_be_bytes([part[0], part[1]]);
            psuedo_header.push(part_as_u16);
        }

        let mut sum: u16 = 0;
        psuedo_header.iter().for_each(|header_item| {
            // Add to sum, adding 1 on overflow
            let (new_sum, overflow) = sum.overflowing_add(*header_item);
            sum = new_sum;
            if overflow {
                sum += 1;
            }
        });
        !sum == self.get_udp_checksum()
    }
}

fn udp_total_length(udp_header: &[u8]) -> u16 {
    let data_length_bytes = &udp_header[4..6];
    u16::from_be_bytes([data_length_bytes[0], data_length_bytes[1]])
}

fn ip_address_to_decimal(address: &[u8; 4]) -> String {
    let decimals: Vec<String> = address.iter().map(|byte| byte.to_string()).collect();
    decimals.join(".")
}

fn verify_ip_checksum(header: &[u8]) -> bool {
    let mut sum: u16 = 0;
    for i in (0..IP_ADDRESS_LENGTH as usize).step_by(2) {
        let part = &header[i..i + 2];
        let part_as_u16 = u16::from_be_bytes([part[0], part[1]]);

        // Add to sum, adding 1 on overflow
        let (new_sum, overflow) = sum.overflowing_add(part_as_u16);
        sum = new_sum;
        if overflow {
            sum += 1;
        }
    }
    !sum == 0
}
