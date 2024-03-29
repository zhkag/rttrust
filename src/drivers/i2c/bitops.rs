use alloc::vec::Vec;

use crate::tick;
use crate::Box;
pub trait I2cBitOps
{
    fn set_sda(&mut self, state:usize);
    fn set_scl(&mut self, state:usize);
    fn get_sda(&mut self) -> usize;
    fn get_scl(&mut self) -> usize;
    fn udelay(&self, us:usize);
}
pub struct I2cBit
{
    data:Option<*mut ()>,
    delay_us:usize,
    timeout:usize,
    retries:usize,
    ops: Option<Box<dyn I2cBitOps>>,
}

impl I2cBit {
    pub fn new() -> I2cBit{
        I2cBit{data:None, delay_us:1, timeout:100, retries: 0, ops:None}
    }

    pub fn init(data:Option<*mut ()>, delay_us:usize, timeout:usize) -> I2cBit {
        I2cBit{data, delay_us, timeout, retries: 0, ops:None}
    }
    pub fn ops(&mut self) -> &mut Box<dyn I2cBitOps>{
        self.ops.as_mut().unwrap()
    }

    pub fn restart(&mut self)-> Result<(),Error>{
        if let Some(ops) = &mut self.ops {
            ops.set_sda(1);
        }
        self.scl_h()?;
        if let Some(ops) = &mut self.ops {
            ops.udelay((self.delay_us + 1) >> 1);
            ops.set_sda(0);
            ops.udelay((self.delay_us + 1) >> 1);
            ops.set_scl(0);
            return Ok(())
        }
        return Err(Error::Intr)
    }
    pub fn start(&mut self)-> Result<(),Error>{
        if let Some(ops) = &mut self.ops {
            ops.set_sda(0);
            ops.udelay((self.delay_us + 1) >> 1);
            ops.set_scl(0);
            return Ok(())
        }
        return Err(Error::Intr)
    }

    pub fn stop(&mut self)-> Result<(),Error>{
        if let Some(ops) = &mut self.ops {
            ops.set_sda(0);
            ops.udelay((self.delay_us + 1) >> 1);
        }
        self.scl_h()?;
        if let Some(ops) = &mut self.ops {
            ops.udelay((self.delay_us + 1) >> 1);
            ops.set_sda(1);
            ops.udelay(self.delay_us);
            return Ok(())
        }
        return Err(Error::Intr)
    }
    pub fn send_address(&mut self,msg:&I2cMsg) -> Result<(),Error>{
        let flags = msg.flags();
        let ignore_nack = msg.flags() & {I2cState::IgnoreNack as u16};
        let mut retries = 0;

        let mut addr1:u8 = 0;
        let mut addr2:u8 = 0;
        if ignore_nack == 0 {
            retries = self.retries;
        }
        if (flags & {I2cState::Addr10bit as u16}) != 0 {
            addr1 = 0xf0 | ((msg.addr() >> 7) as u8 & 0x06);
            addr2 = (msg.addr() & 0xff) as u8;
            if self.send_addr(addr1, retries).is_err() &&  ignore_nack != 0 {
                return Err(Error::IO)
            }
            if self.writeb(addr2).is_err() && ignore_nack != 0 {
                return Err(Error::IO)
            }
            if (flags & {I2cState::RD as u16}) != 0 {
                self.restart()?;
                addr1 |= 0x01;
                if self.send_addr(addr1, retries).is_err() && ignore_nack != 0 {
                    return Err(Error::IO)
                }
            }
        }else {
            addr1 = (msg.addr() << 1) as u8;
            if (flags & {I2cState::RD as u16}) != 0 {
                addr1 |= 1;
            }
            if self.send_addr(addr1, retries).is_err() && ignore_nack != 0 {
                return Err(Error::IO)
            }
        }
        Ok(())
    }
    fn send_addr(&mut self, addr:u8, retries:usize) -> Result<(),Error>{
        for i in 0..retries {
            self.writeb(addr)?;
            if i == retries {
                break;
            }
            self.stop()?;
            if let Some(ops) = &mut self.ops {
                ops.udelay(self.delay_us);
            }
            self.start()?;
        }
        Ok(())
    }
    fn writeb(&mut self, data:u8)-> Result<(),Error>{
        if self.ops.is_none(){
            return Err(Error::Intr)
        }
        for i in 0..8 {
            if let Some(ops) = &mut self.ops {
                ops.set_scl(0);
                ops.set_sda(((data >> i) & 1).into());
                ops.udelay((self.delay_us + 1) >> 1);
            }
            self.scl_h()?;
        }
        return Ok(());
    }
    fn readb(&mut self) -> Result<u8,Error>{
        let mut data:u8 = 0;
        if let Some(ops) = &mut self.ops {
            ops.set_sda(1);
            ops.udelay((self.delay_us + 1) >> 1);
        }
        for i in 0..8 {
            data <<= 1;
            self.scl_h()?;

            if let Some(ops) = &mut self.ops {
                if ops.get_sda() != 0 {
                    data |= 1;
                }
                ops.set_scl(0);
                ops.udelay(self.delay_us);
            }
        }
        Ok(data)
    }
    fn scl_h(&mut self) -> Result<(),Error>{
        if let Some(ops) = &mut self.ops {
            ops.set_scl(1);
            let start = tick!(get());
            while ops.get_scl() == 0 {
                if (tick!(get()) - start) > self.timeout{
                    return Err(Error::TimeOut)
                }
                ops.udelay((self.delay_us + 1) >> 1);
            }
            return Ok(())
        }
        return Err(Error::Intr)
    }
    fn send_ack_or_nack(&mut self, ack:usize) -> Result<(),Error>{
        if let Some(ops) = &mut self.ops {
            if ack > 0 {
                ops.set_sda(0);
            }
            ops.udelay((self.delay_us + 1) >> 1);
        }
        self.scl_h()?;
        if let Some(ops) = &mut self.ops {
            ops.set_scl(0);
        }
        Ok(())
    }
    fn recv_bytes(&mut self, msg:&I2cMsg) -> Result<usize,Error>{
        let mut count: usize = msg.len();
        let mut val:usize;
        let mut bytes = 0;
        while count > 0 {
            match self.readb() {
                Ok(val) => {msg.buf_offset(bytes, val); bytes += 1;},
                Err(_) => break
            } ;
            count -= 1;
            if msg.flags() & {I2cState::NoReadAck as u16} == 0{
                self.send_ack_or_nack(count)?;
            }
        }
        Ok(bytes as usize)
    }
}
use crate::drivers::i2c::dev::I2cBusOps;
use crate::drivers::i2c::dev::DeviceI2cBus;
impl I2cBusOps for I2cBit {
    fn master_xfer(&mut self, msgs:Vec<&super::core::I2cMsg>) -> usize {
        let num = msgs.len();
        if num == 0{
            return 0;
        }
        for (i,msg) in msgs.iter().enumerate() {
            let ignore_nack = msg.flags() & {I2cState::IgnoreNack as u16};
            if (msg.flags() & {I2cState::NoStart as u16}) == 0{
                let _ = match i {
                    0 => self.start(),
                    _ => self.restart(),
                };
                self.send_address(msg);
            }
            if (msg.flags() & {I2cState::RD as u16}) == 0 {
                self.recv_bytes(msg);
            }else {
                self.recv_bytes(msg);
            }
            if (msg.flags() & {I2cState::RD as u16}) == 0 {
                self.stop();
            }
        }
        0
    }

}
use crate::drivers::DeviceRegister;
use crate::Error;

use super::core::I2cMsg;
use super::core::I2cState;

impl<T: I2cBitOps + 'static> DeviceRegister<T> for I2cBit {
    fn register(&mut self, name:&str, ops:T)
    {
        let mut hw_i2c_bit = I2cBit::new();
        hw_i2c_bit.ops = Some(Box::new(ops));
        DeviceI2cBus::new().register(name, hw_i2c_bit);
    }
}
