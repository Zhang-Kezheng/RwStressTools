use bytebuffer::ByteBuffer;
use serde::Serialize;
use std::sync::RwLock;
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
static SN: RwLock<u8> = RwLock::new(0);
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

    pub fn new(data: &[u8], device_id: [u8; 6]) -> AoaGateway {
        let length = data.len() + 16;
        let mut sn = *SN.write().unwrap();
        sn += 1;
        let mut gateway = AoaGateway {
            header: 0x02030405,
            length: length as u16,
            dev_id: device_id,
            cmd: 0x01,
            sn,
            jiami: 1,
            data: data.to_vec(),
            check_sum: 0,
        };
        gateway.check_sum = gateway.check();
        gateway
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = ByteBuffer::new();
        buffer.write_u32(self.header);
        buffer.write_u16(self.length);
        buffer.write_bytes(self.dev_id.as_slice());
        buffer.write_u8(self.cmd);
        buffer.write_u8(self.sn);
        buffer.write_u8(self.jiami);
        buffer.write_bytes(self.data.as_slice());
        buffer.write_u8(self.check_sum);
        return Vec::from(buffer.as_bytes());
    }
    fn check(&self) -> u8 {
        let sum: i32 = self
            .to_bytes()
            .iter()
            .map(|&b| b as i32) // 关键：&b 解引用为 u8，再转为 i32
            .sum();
        ((sum - self.to_bytes()[self.to_bytes().len() - 1] as i32) % 256) as u8
    }
}
#[derive(Debug, Serialize, Clone)]
pub struct AoaTag {
    pub mac: [u8; 6],
    pub length: u8,
    pub fix: u8,
    pub manufacturer_id: u16,
    pub package_id: u8,
    pub command: u8,
    pub user_data: [u8; 3],
    pub crc: i16,
    pub df_field: [u8; 20],
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
            crc: byte_buffer.read_i16().unwrap(),
            df_field: byte_buffer
                .read_bytes(20)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
            rssi: byte_buffer.read_i8().unwrap(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = ByteBuffer::new();
        buffer.write_bytes(self.mac.as_slice());
        buffer.write_u8(self.length);
        buffer.write_u8(self.fix);
        buffer.write_u16(self.manufacturer_id);
        buffer.write_u8(self.package_id);
        buffer.write_u8(self.command);
        buffer.write_bytes(self.user_data.as_slice());
        buffer.write_i16(self.crc);

        buffer.write_bytes(self.df_field.as_slice());
        buffer.write_i8(self.rssi);
        Vec::from(buffer.as_bytes())
    }
}
