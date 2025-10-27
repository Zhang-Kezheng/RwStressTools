use bytebuffer::ByteBuffer;
use serde::Serialize;
#[derive(Debug, Serialize, Clone)]
pub struct AoaGateway {
    header: u32,
    length: u16,
    pub dev_id: [u8; 6],
    cmd: u8,
    sn: u8,
    jiami: u8,
    pub data: Vec<u8>,
    check_sum: u8,
}

impl AoaGateway {
    pub fn get_instance(data: Vec<u8>) -> Option<AoaGateway> {
        let mut byte_buffer = ByteBuffer::from(data);
        let header = byte_buffer.read_u32().unwrap();
        if header != 0x02030405 {
            return None;
        }

        let length = byte_buffer.read_u16().unwrap();

        let aoa_gateway = AoaGateway {
            header: 0x02030405,
            length,
            dev_id: byte_buffer
                .read_bytes(6)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
            cmd: byte_buffer.read_u8().unwrap(),
            sn: byte_buffer.read_u8().unwrap(),
            jiami: byte_buffer.read_u8().unwrap(),
            data: byte_buffer.read_bytes((length as usize) - 16).unwrap(),
            check_sum: byte_buffer.read_u8().unwrap(),
        };
        Some(aoa_gateway)
    }
}
#[derive(Debug, Serialize, Clone)]
pub struct AoaTag {
    pub mac: [u8; 6],
    length: u8,
    fix: u8,
    manufacturer_id: u16,
    package_id: u8,
    pub command: u8,
    pub user_data: [u8; 3],
    crc: u8,
    df_field: [u8; 20],
    pub rssi: i8,
}

impl AoaTag {
    pub fn get_instance(data: &[u8; 38]) -> AoaTag {
        let mut byte_buffer = ByteBuffer::from_bytes(data);
        AoaTag {
            mac: byte_buffer
                .read_bytes(6)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
            length: byte_buffer.read_u8().unwrap(),
            fix: byte_buffer.read_u8().unwrap(),
            manufacturer_id: byte_buffer.read_u16().unwrap(),
            package_id: byte_buffer.read_u8().unwrap(),
            command: byte_buffer.read_u8().unwrap(),
            user_data: byte_buffer
                .read_bytes(3)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
            crc: byte_buffer.read_u8().unwrap(),
            df_field: byte_buffer
                .read_bytes(20)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
            rssi: byte_buffer.read_i8().unwrap(),
        }
    }
}
