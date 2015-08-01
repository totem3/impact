use std::slice::Iter;
use std::char;
pub struct Decoder<'a> {
    iter: &'a mut Iter<'a, u8>,
}

pub trait Decodable {
    fn decode(d: &mut Decoder) -> Result<Self, String>;
}

pub type DecodeResult<T> = Result<T, String>;

impl<'a> Decoder<'a> {
    pub fn new(iter: &'a mut Iter<'a, u8>) -> Decoder {
        Decoder{
            iter: iter
        }
    }
    pub fn read_u8(&mut self) -> DecodeResult<u8> {
        let u = self.iter.next();
        match u {
            Some(&v) => { println!("{:08b}", v); Ok(v)},
            None => Err(String::from("End of input")),
        }
    }
    pub fn read_u16(&mut self) -> DecodeResult<u16> {
        let mut res = 0u16;
        match self.iter.next() {
            Some(&v) => { println!("{:08b}", v);res = res | ((v as u16) << 8) },
            None => { return Err(String::from("End of input")) },
        }
        match self.iter.next() {
            Some(&v) => { println!("{:08b}", v);res = res | (v as u16) },
            None => { return Err(String::from("End of input")) },
        }
        Ok(res)
    }
    pub fn read_u32(&mut self) -> DecodeResult<u32> {
        let mut res = 0u32;
        match self.iter.next() {
            Some(&v) => { println!("{:08b}", v); res = res | ((v as u32) << 24) },
            None => { return Err(String::from("End of input")) },
        }
        match self.iter.next() {
            Some(&v) => { println!("{:08b}", v); res = res | ((v as u32) << 16) },
            None => { return Err(String::from("End of input")) },
        }
        match self.iter.next() {
            Some(&v) => { println!("{:08b}", v); res = res | ((v as u32) << 8) },
            None => { return Err(String::from("End of input")) },
        }
        match self.iter.next() {
            Some(&v) => { println!("{:08b}", v); res = res | (v as u32) },
            None => { return Err(String::from("End of input")) },
        }
        Ok(res)
    }
    pub fn read_str(&mut self, len: usize) -> DecodeResult<String> {
        let mut s = String::new();
        for _ in 0..len {
            match self.iter.next() {
                Some(&v) => match char::from_u32(v as u32) { 
                    Some(vv) => { s.push(char::from(vv)) },
                    None => { return Err(String::from(format!("Conversion Error. {} to char", v))) },
                },
                None => { return Err(String::from("End of input")) },
            }
        }
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::{Decoder, Decodable, DecodeResult};

    #[test]
    fn test_decode_u() {
        let vec = vec![
            0b01010101, 0b01010101, 0b00001111, 0b11110000,
            0b11001100, 0b00110011,
            0b00000011,
        ];
        let mut iter = vec.iter();
        let mut decoder = Decoder{
            iter: &mut iter,
        };
        let n1 = decoder.read_u32();
        assert_eq!(n1.unwrap(), 0b01010101010101010000111111110000);
        let n2 = decoder.read_u16();
        assert_eq!(n2.unwrap(), 0b1100110000110011);
        let n3 = decoder.read_u8();
        assert_eq!(n3.unwrap(), 0b00000011);
    }
}
