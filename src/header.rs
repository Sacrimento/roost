use std::io::{BufReader, Read};

struct HeaderParser<'a> {
    buffer: &'a [u8],

    pos: usize,
    is_big: bool,
}

impl<'a> HeaderParser<'a> {
    fn new(is_big: bool, buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            pos: 0,
            is_big,
        }
    }

    fn read_u32(&mut self) -> Result<u32, HeaderParsingError> {
        let val = u32::from_be_bytes(
            self.buffer[self.pos..self.pos + 4]
                .try_into()
                .map_err(|_| HeaderParsingError::InvalidI32)?,
        );
        self.pos += 4;
        Ok(val)
    }

    fn read_u64_auto(&mut self) -> Result<u64, HeaderParsingError> {
        if !self.is_big {
            let val = u32::from_be_bytes(
                self.buffer[self.pos..self.pos + 4]
                    .try_into()
                    .map_err(|_| HeaderParsingError::InvalidI32)?,
            );
            self.pos += 4;
            Ok(val as u64)
        } else {
            let val = u64::from_be_bytes(
                self.buffer[self.pos..self.pos + 8]
                    .try_into()
                    .map_err(|_| HeaderParsingError::InvalidI64)?,
            );
            self.pos += 8;
            Ok(val)
        }
    }

    fn read_u8(&mut self) -> Result<u8, HeaderParsingError> {
        let val = u8::from_be_bytes(
            self.buffer[self.pos..self.pos + 1]
                .try_into()
                .map_err(|_| HeaderParsingError::InvalidI8)?,
        );
        self.pos += 1;
        Ok(val)
    }

    fn read_u16(&mut self) -> Result<u16, HeaderParsingError> {
        let val = u16::from_be_bytes(
            self.buffer[self.pos..self.pos + 2]
                .try_into()
                .map_err(|_| HeaderParsingError::InvalidI16)?,
        );
        self.pos += 2;
        Ok(val)
    }

    fn read(&mut self, size: usize) -> &[u8] {
        let val = &self.buffer[self.pos..self.pos + size];
        self.pos += size;
        val
    }

    fn skip(&mut self, n: usize) -> &mut Self {
        self.pos += n;
        self
    }

    fn uuid_hex(buf: &[u8]) -> String {
        let formatted = buf.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        format!(
            "{}-{}-{}-{}-{}",
            &formatted[0..8],
            &formatted[8..12],
            &formatted[12..16],
            &formatted[16..20],
            &formatted[20..32]
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum HeaderParsingError {
    InvalidMagic,
    UnexpectedFileSize,
    InvalidI8,
    InvalidI16,
    InvalidI32,
    InvalidI64,
}

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub struct FileHeader {
    version: u32,
    begin: u32,
    end: u64,
    seekfree: u64,
    nbytesfree: u32,
    nbytesname: u32,
    units: u8,
    compress: u32,
    seekinfo: u64,
    nbytesinfo: u32,
    uuidversion: u16,
    uuid: String,
}

impl FileHeader {
    pub fn new<R: Read>(reader: &mut BufReader<R>) -> Result<Self, HeaderParsingError> {
        let mut buf = [0; 4];

        // magic
        reader
            .read_exact(&mut buf[..])
            .map_err(|_| HeaderParsingError::UnexpectedFileSize)?;
        if std::str::from_utf8(&buf[..]).map_err(|_| HeaderParsingError::InvalidMagic)? != "root" {
            return Err(HeaderParsingError::InvalidMagic);
        }

        reader
            .read_exact(&mut buf[..])
            .map_err(|_| HeaderParsingError::UnexpectedFileSize)?;
        let version = u32::from_be_bytes(buf);

        let is_big = version > 1_000_000;
        let mut buf: Vec<u8> = vec![0; if is_big { 67 } else { 55 }];
        reader
            .read_exact(&mut buf[..])
            .map_err(|_| HeaderParsingError::UnexpectedFileSize)?;

        let mut parser = HeaderParser::new(is_big, &buf);

        Ok(Self {
            version,
            begin: parser.read_u32()?,
            end: parser.read_u64_auto()?,
            seekfree: parser.read_u64_auto()?,
            nbytesfree: parser.read_u32()?,
            nbytesname: parser.skip(4).read_u32()?,
            units: parser.read_u8()?,
            compress: parser.read_u32()?,
            seekinfo: parser.read_u64_auto()?,
            nbytesinfo: parser.read_u32()?,
            uuidversion: parser.read_u16()?,
            uuid: HeaderParser::uuid_hex(parser.read(16)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn mock_reader(data: &[u8]) -> BufReader<Cursor<&[u8]>> {
        BufReader::new(Cursor::new(data))
    }

    #[test]
    fn test_empty_file() {
        let mut reader = mock_reader(&[]);
        let result = FileHeader::new(&mut reader);
        assert_eq!(result, Err(HeaderParsingError::UnexpectedFileSize));
    }

    #[test]
    fn test_invalid_magic() {
        let mut reader = mock_reader(&b"abcd"[..]);
        let result = FileHeader::new(&mut reader);
        assert_eq!(result, Err(HeaderParsingError::InvalidMagic));
    }

    #[test]
    fn test_header_too_short() {
        let mut data = b"root".to_vec();
        data.extend_from_slice(&1_000_001u32.to_be_bytes());
        data.extend(std::iter::repeat_n(0, 10));
        let mut reader = mock_reader(&data);
        let result = FileHeader::new(&mut reader);
        assert_eq!(result, Err(HeaderParsingError::UnexpectedFileSize));
    }
}
