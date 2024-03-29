struct I2c{

}

pub enum I2cState {
    WR = 0,
    RD = 1 << 0,
    Addr10bit = 1 << 2,
    NoStart = 1 << 4,
    IgnoreNack = 1 << 5,
    NoReadAck = 1 << 6,
    NoStop = 1 << 7,
}

impl From<I2cState> for usize {
    fn from(value:I2cState ) -> usize {
        value as usize
    }
}

impl From<usize> for I2cState {
    fn from(value:usize ) -> I2cState {
        match value {
            0 => I2cState::WR,
            0x1 => I2cState::RD,
            0x4 => I2cState::Addr10bit,
            0x10 => I2cState::NoStart,
            0x20 => I2cState::IgnoreNack,
            0x40 => I2cState::NoReadAck,
            0x80 => I2cState::NoStop,
            _ => unreachable!(),
        }
    }
}

pub struct I2cMsg
{
    addr:u16,
    flags:u16,
    len:usize,
    buf:Option<*mut u8>,
}

impl I2cMsg {
    pub fn init(addr:u16, flags:u16, len:usize, buf:Option<*mut u8>) -> I2cMsg {
            I2cMsg {addr, flags, len ,buf}
    }
    pub fn flags(&self) -> u16 {
        self.flags
    }
    pub fn addr(&self) -> u16 {
        self.addr
    }
    pub fn len(&self) -> usize{
        self.len
    }
    pub fn buf_offset(&self, bytes:isize, data:u8){
        if self.buf.is_none(){
            return;
        }
        let buf = self.buf.unwrap();
        unsafe {
            let offset_ptr = buf.offset(bytes);
            *offset_ptr = data;
        };

    }
}

pub struct I2cPrivData
{
    msgs:I2cMsg,
    number:usize,
}

impl I2cPrivData {
    pub fn init(msgs:I2cMsg, number:usize) -> I2cPrivData {
        I2cPrivData {msgs, number}
    }
    pub fn msgs(&mut self) -> &mut I2cMsg{
        &mut self.msgs
    }
    pub fn number(&self) -> usize{
        self.number
    }
}