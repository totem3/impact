use std::result::Result;

pub struct Encoder<'a> {
    buffer: &'a mut Vec<u8>,
}

pub trait Encodable {
    fn encode(&self, s: &mut Encoder) -> EncodeResult<()>;
}

pub fn encode<T: Encodable>(object: &T) -> EncodeResult<Vec<u8>> {
    let mut buffer = Vec::new();
    {
        let mut encoder = Encoder::new(&mut buffer);
        try!(object.encode(&mut encoder));
    }
    Ok(buffer)
}

pub type EncodeResult<T> = Result<T, String>;

impl<'a> Encoder<'a> {
    fn new(buffer: &'a mut Vec<u8>) -> Encoder<'a> {
        Encoder{
            buffer: buffer,
        }
    }
    fn emit_u8(&mut self, v: u8) -> EncodeResult<()> {
        self.buffer.push(v);
        Ok(())
    }
    fn emit_u16(&mut self, v: u16) -> EncodeResult<()> {
        self.buffer.push((v >> 8) as u8);
        self.buffer.push((v & 0b11111111) as u8);
        Ok(())
    }
    fn emit_u32(&mut self, v: u32) -> EncodeResult<()> {
        self.buffer.push((v >> 24) as u8);
        self.buffer.push((v >> 16) as u8);
        self.buffer.push((v >>  8) as u8);
        self.buffer.push((v & 0b11111111) as u8);
        Ok(())
    }
    fn emit_str(&mut self, v: &str) -> EncodeResult<()> {
        let s = String::from(v);
        self.buffer.extend(s.as_bytes());
        Ok(())
    }
    fn emit_string(&mut self, v: String) -> EncodeResult<()> {
        self.buffer.extend(v.as_bytes());
        Ok(())
    }
}

mod test {
    use super::{Encoder, EncodeResult, Encodable};
    struct Person {
        name: &'static str,
        age: u32,
    }
    impl Encodable for Person {
        fn encode(&self, encoder: &mut Encoder) -> EncodeResult<()> {
            match encoder.emit_str(self.name) {
                Err(s) => return Err(s),
                _ => {},
            };
            encoder.emit_u32(self.age)
        }
    }
    #[test]
    fn test_write_u8() {
        let mut buf = Vec::new();
        let mut encoder = Encoder::new(&mut buf);
        let result = encoder.emit_u8(170u8);
        let result = encoder.emit_u8(140u8);
        assert_eq!(result, Ok(()));
        assert_eq!(encoder.buffer, &vec![170u8, 140u8]);
    }

    #[test]
    fn test_write_u16() {
        let mut buf = Vec::new();
        let mut encoder = Encoder::new(&mut buf);

        let result = encoder.emit_u16(43605u16);
        assert_eq!(result, Ok(()));
        assert_eq!(encoder.buffer, &vec![170u8, 85u8]);
    }

    #[test]
    fn test_write_u32() {
        let mut buf = Vec::new();
        let mut encoder = Encoder::new(&mut buf);

        let val: u32 = 0b11111111000011111111000010101010;
        let result = encoder.emit_u32(val);
        assert_eq!(result, Ok(()));
        assert_eq!(encoder.buffer, &vec![
            0b11111111,
            0b00001111,
            0b11110000,
            0b10101010,
        ]);
    }

    #[test]
    fn test_write_str() {
        let mut buf = Vec::new();
        let mut encoder = Encoder::new(&mut buf);

        let result = encoder.emit_str("hoge");
        assert_eq!(result, Ok(()));
        assert_eq!(encoder.buffer, &vec![104, 111, 103, 101]);
    }

    #[test]
    fn test_write_string() {
        let mut buf = Vec::new();
        let mut encoder = Encoder::new(&mut buf);

        let result = encoder.emit_string(String::from("hoge"));
        assert_eq!(result, Ok(()));
        assert_eq!(encoder.buffer, &vec![104, 111, 103, 101]);
    }

    #[test]
    fn test_encode() {
        let person = Person{
            name: "takafumi",
            age: 26,
        };
        let encoded = super::encode(&person);
        assert_eq!(encoded.unwrap(), vec![116, 97, 107, 97, 102, 117, 109, 105, 0, 0, 0, 26]);
    }
}
