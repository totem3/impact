use std::char;
pub struct Decoder {
    stack: Vec<u8>,
}

pub trait Decodable {
    fn decode(d: &mut Decoder) -> Result<Self, String>;
}

pub type DecodeResult<T> = Result<T, String>;

impl Decoder {
    fn pop(&mut self) -> Option<u8> {
        self.stack.pop()
    }
    pub fn read_u8(&mut self) -> DecodeResult<u8> {
        let u = self.pop();
        match u {
            Some(n) => Ok(n),
            None => Err(String::from("End of input")),
        }
    }
    pub fn read_u16(&mut self) -> DecodeResult<u16> {
        let mut res = 0u16;
        match self.pop() {
            Some(v) => { res = res | ((v as u16) << 8) },
            None => { return Err(String::from("End of input")) },
        }
        match self.pop() {
            Some(v) => { res = res | (v as u16) },
            None => { return Err(String::from("End of input")) },
        }
        Ok(res)
    }
    pub fn read_u32(&mut self) -> DecodeResult<u32> {
        let mut res = 0u32;
        match self.pop() {
            Some(v) => { res = res | ((v as u32) << 24) },
            None => { return Err(String::from("End of input")) },
        }
        match self.pop() {
            Some(v) => { res = res | ((v as u32) << 16) },
            None => { return Err(String::from("End of input")) },
        }
        match self.pop() {
            Some(v) => { res = res | ((v as u32) << 8) },
            None => { return Err(String::from("End of input")) },
        }
        match self.pop() {
            Some(v) => { res = res | (v as u32) },
            None => { return Err(String::from("End of input")) },
        }
        Ok(res)
    }
    pub fn read_str(&mut self, len: usize) -> DecodeResult<String> {
        let mut s = String::new();
        for _ in 0..len {
            match self.pop() {
                Some(v) => match char::from_u32(v as u32) { 
                    Some(vv) => { s.push(char::from(vv)) },
                    None => { return Err(String::from(format!("Conversion Error. {} to char", v))) },
                },
                None => { return Err(String::from("End of input")) },
            }
        }
        Ok(s)
    }
}
