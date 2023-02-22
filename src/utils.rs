use std::convert::TryInto;

use crate::error::{Error, Result};

pub fn cut_slice(bytes: &[u8], offset: usize, len: usize) -> Result<&[u8]> {
    if bytes.len() >= offset + len {
        Ok(&bytes[offset..offset + len])
    } else {
        Err(Error::DataBytesShortage("Out of range".to_owned()))
    }
}

pub fn bytes_to_u16(bytes: &[u8]) -> Result<u16> {
    if bytes.len() > 1 {
        Ok(u16::from_be_bytes(bytes.try_into().unwrap()))
    } else {
        Err(Error::DataBytesShortage("Too short for u16".to_owned()))
    }
}

pub fn bytes_to_i16(bytes: &[u8]) -> Result<i16> {
    if bytes.len() > 1 {
        Ok(i16::from_be_bytes(bytes.try_into().unwrap()))
    } else {
        Err(Error::DataBytesShortage("Too short for i16".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_slice() {
        let bytes = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a];
        assert_eq!(cut_slice(bytes, 0, 0).unwrap(), &[][..] as &[u8]);
        assert_eq!(cut_slice(bytes, 0, 1).unwrap(), &[0x01][..]);
        assert_eq!(cut_slice(bytes, 0, 2).unwrap(), &[0x01, 0x02][..]);
        assert_eq!(cut_slice(bytes, 3, 2).unwrap(), &[0x04, 0x05][..]);
        assert_eq!(
            cut_slice(bytes, 2, 6).unwrap(),
            &[0x03, 0x04, 0x05, 0x06, 0x07, 0x08][..]
        );
        assert_eq!(
            cut_slice(bytes, 0, 10).unwrap(),
            &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a][..]
        );
        assert_eq!(cut_slice(bytes, 8, 2).unwrap(), &[0x09, 0x0a][..]);
        assert_eq!(cut_slice(bytes, 9, 1).unwrap(), &[0x0a][..]);
        assert_eq!(cut_slice(bytes, 10, 0).unwrap(), &[][..] as &[u8]);
        assert!(cut_slice(bytes, 0, 11).is_err());
        assert!(cut_slice(bytes, 1, 10).is_err());
        assert!(cut_slice(bytes, 10, 1).is_err());
        assert!(cut_slice(bytes, 9, 2).is_err());
        assert!(cut_slice(bytes, 5, 6).is_err());
        assert!(cut_slice(bytes, 100, 2).is_err());
    }

    #[test]
    fn test_bytes_to_u16() {
        assert_eq!(bytes_to_u16(&[0x00, 0x00]).unwrap(), 0);
        assert_eq!(bytes_to_u16(&[0x01, 0x01]).unwrap(), 257);
        assert_eq!(bytes_to_u16(&[0x00, 0x79]).unwrap(), 121);
        assert_eq!(bytes_to_u16(&[0x65, 0x00]).unwrap(), 25856);
        assert_eq!(bytes_to_u16(&[0x2b, 0x35]).unwrap(), 11061);
        assert_eq!(bytes_to_u16(&[0x13, 0xf8]).unwrap(), 5112);
        assert_eq!(bytes_to_u16(&[0x87, 0x16]).unwrap(), 34582);
        assert_eq!(bytes_to_u16(&[0xcb, 0x51]).unwrap(), 52049);
        assert_eq!(bytes_to_u16(&[0xa1, 0xb7]).unwrap(), 41399);
        assert_eq!(bytes_to_u16(&[0xff, 0xff]).unwrap(), 65535);
        assert!(bytes_to_u16(&[0xe2]).is_err());
    }

    #[test]
    fn test_bytes_to_i16() {
        assert_eq!(bytes_to_i16(&[0x00, 0x00]).unwrap(), 0);
        assert_eq!(bytes_to_i16(&[0x01, 0x01]).unwrap(), 257);
        assert_eq!(bytes_to_i16(&[0x00, 0x79]).unwrap(), 121);
        assert_eq!(bytes_to_i16(&[0x65, 0x00]).unwrap(), 25856);
        assert_eq!(bytes_to_i16(&[0x2b, 0x35]).unwrap(), 11061);
        assert_eq!(bytes_to_i16(&[0x13, 0xf8]).unwrap(), 5112);
        assert_eq!(bytes_to_i16(&[0x87, 0x16]).unwrap(), -30954);
        assert_eq!(bytes_to_i16(&[0xcb, 0x51]).unwrap(), -13487);
        assert_eq!(bytes_to_i16(&[0xa1, 0xb7]).unwrap(), -24137);
        assert_eq!(bytes_to_i16(&[0xff, 0xff]).unwrap(), -1);
        assert!(bytes_to_u16(&[0xe2]).is_err());
    }
}
