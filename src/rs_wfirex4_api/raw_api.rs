use std::io::{Read, Write};
use std::net::TcpStream;
const IR_HEADER: u8 = 0xAA;
const IR_ORDER: u16 = 0x1100;
struct SendData {
    header: [u8; 1],  // 固定ヘッダ(0xAA)
    len: [u8; 2],     // 送信データ(Payload)の長さ
    payload: Payload, // 送信データ
    crc: [u8; 1],     // 送信データのチェックサム
}
struct Payload {
    order: [u8; 2],   // 固定ヘッダ(0x11,0x00)=赤外線照射命令
    len: [u8; 2],     // 赤外線波形データの長さ
    ir_data: Vec<u8>, // 赤外線波形データ
}
impl SendData {
    fn new(ir_data: &Vec<u8>) -> Self {
        let payload = Payload::new(ir_data);
        let crc = calculate_crc(&payload.to_bytes());

        SendData {
            header: [IR_HEADER],
            len: (payload.len() as u16).to_be_bytes(),
            payload: payload,
            crc: [crc],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.header);
        bytes.extend(&self.len);
        bytes.extend(&self.payload.to_bytes());
        bytes.extend(&self.crc);
        bytes
    }
}
impl Payload {
    fn new(ir_data: &Vec<u8>) -> Self {
        Payload {
            order: IR_ORDER.to_be_bytes(),
            len: (ir_data.len() as u16).to_be_bytes(),
            ir_data: ir_data.to_vec(),
        }
    }
    fn len(&self) -> usize {
        self.order.len() + self.len.len() + self.ir_data.len()
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.order);
        bytes.extend(&self.len);
        bytes.extend(&self.ir_data);
        bytes
    }
}
#[derive(Default)]
struct RecvData {
    header: [u8; 1], // AA
    len: [u8; 2],    // 00 02
    order: [u8; 2],  // 11 00
    crc: [u8; 1],    // D1
}
pub fn get_payload(ir_data: &Vec<u8>) -> Vec<u8> {
    let send_data = SendData::new(ir_data);
    send_data.to_bytes()
}
fn recv_response(mut socket: TcpStream) -> std::io::Result<()> {
    let mut recv: RecvData = RecvData::default();
    socket.read_exact(&mut recv.header)?;
    if recv.header[0] != IR_HEADER {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Recv Header",
        ));
    }
    socket.read_exact(&mut recv.len)?;
    let recv_len: u16 = u16::from_be_bytes(recv.len);
    if recv_len != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Recv Length",
        ));
    }
    socket.read_exact(&mut recv.order)?;
    let recv_order: u16 = u16::from_be_bytes(recv.order);
    if recv_order != IR_ORDER {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Recv Order",
        ));
    }
    socket.read_exact(&mut recv.crc)?;
    if recv.crc[0] != calculate_crc(&recv.order.to_vec()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Recv CRC",
        ));
    }
    Ok(())
}
pub fn send_ir_data(server: &String, ir_data: &Vec<u8>) -> std::io::Result<()> {
    let mut socket = TcpStream::connect(server)?;

    socket.write(&get_payload(ir_data)).map_err(|e| {
        std::io::Error::new(
            e.kind(),
            format!("Failed to send data to {}: {}", server, e),
        )
    })?;
    recv_response(socket).map_err(|e| {
        std::io::Error::new(
            e.kind(),
            format!("Failed to receive response from {}: {}", server, e),
        )
    })?;
    Ok(())
}
fn calculate_crc(data: &Vec<u8>) -> u8 {
    let mut crc: u8 = 0;
    for i in 0..data.len() {
        crc = crc8_calc(crc, &data[i]);
    }
    return crc;
}
fn crc8_calc(_crc: u8, byte: &u8) -> u8 {
    let mut crc = _crc ^ byte;
    for _i in 0..8 {
        if crc & 0x80 > 0 {
            crc = (crc << 1) ^ 0x85;
        } else {
            crc = crc << 1;
        }
    }
    return crc & 0xFF;
}
