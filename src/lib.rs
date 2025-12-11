use hex::FromHexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Invalid hexadecimal string: {0}")]
    InvalidHex(#[from] FromHexError),
    #[error("Incorrect hexadecimal string length: expected {expected}, got {found}")]
    IncorrectLength { expected: usize, found: usize },
    #[error("Failed to convert bytes to target integer type")]
    ConversionFailed,
}

fn hex_to_bytes_vec(hex_str: &str, expected_len: usize) -> Result<Vec<u8>, ConversionError> {
    let bytes = hex::decode(hex_str)?;
    if bytes.len() != expected_len {
        return Err(ConversionError::IncorrectLength {
            expected: expected_len,
            found: bytes.len(),
        });
    }
    Ok(bytes)
}

// Helper to convert hex string to a fixed-size byte array
#[allow(clippy::needless_question_mark)]
fn hex_to_bytes<const N: usize>(hex_str: &str) -> Result<[u8; N], ConversionError> {
    let bytes_vec = hex_to_bytes_vec(hex_str, N)?;
    Ok(bytes_vec
        .try_into()
        .map_err(|_| ConversionError::ConversionFailed)?)
}

pub fn hex_to_i8(hex_str: &str) -> Result<i8, ConversionError> {
    let bytes: [u8; 1] = hex_to_bytes(hex_str)?;
    Ok(i8::from_be_bytes(bytes))
}

pub fn hex_to_u8(hex_str: &str) -> Result<u8, ConversionError> {
    let bytes: [u8; 1] = hex_to_bytes(hex_str)?;
    Ok(u8::from_be_bytes(bytes))
}

pub fn hex_to_i16_be(hex_str: &str) -> Result<i16, ConversionError> {
    let bytes: [u8; 2] = hex_to_bytes(hex_str)?;
    Ok(i16::from_be_bytes(bytes))
}

pub fn hex_to_i16_le(hex_str: &str) -> Result<i16, ConversionError> {
    let bytes: [u8; 2] = hex_to_bytes(hex_str)?;
    Ok(i16::from_le_bytes(bytes))
}

pub fn hex_to_u16_be(hex_str: &str) -> Result<u16, ConversionError> {
    let bytes: [u8; 2] = hex_to_bytes(hex_str)?;
    Ok(u16::from_be_bytes(bytes))
}

pub fn hex_to_u16_le(hex_str: &str) -> Result<u16, ConversionError> {
    let bytes: [u8; 2] = hex_to_bytes(hex_str)?;
    Ok(u16::from_le_bytes(bytes))
}

pub fn hex_to_i32_be(hex_str: &str) -> Result<i32, ConversionError> {
    let bytes: [u8; 4] = hex_to_bytes(hex_str)?;
    Ok(i32::from_be_bytes(bytes))
}

pub fn hex_to_i32_le(hex_str: &str) -> Result<i32, ConversionError> {
    let bytes: [u8; 4] = hex_to_bytes(hex_str)?;
    Ok(i32::from_le_bytes(bytes))
}

pub fn hex_to_u32_be(hex_str: &str) -> Result<u32, ConversionError> {
    let bytes: [u8; 4] = hex_to_bytes(hex_str)?;
    Ok(u32::from_be_bytes(bytes))
}

pub fn hex_to_u32_le(hex_str: &str) -> Result<u32, ConversionError> {
    let bytes: [u8; 4] = hex_to_bytes(hex_str)?;
    Ok(u32::from_le_bytes(bytes))
}

pub fn hex_to_i64_be(hex_str: &str) -> Result<i64, ConversionError> {
    let bytes: [u8; 8] = hex_to_bytes(hex_str)?;
    Ok(i64::from_be_bytes(bytes))
}

pub fn hex_to_i64_le(hex_str: &str) -> Result<i64, ConversionError> {
    let bytes: [u8; 8] = hex_to_bytes(hex_str)?;
    Ok(i64::from_le_bytes(bytes))
}

pub fn hex_to_u64_be(hex_str: &str) -> Result<u64, ConversionError> {
    let bytes: [u8; 8] = hex_to_bytes(hex_str)?;
    Ok(u64::from_be_bytes(bytes))
}

pub fn hex_to_u64_le(hex_str: &str) -> Result<u64, ConversionError> {
    let bytes: [u8; 8] = hex_to_bytes(hex_str)?;
    Ok(u64::from_le_bytes(bytes))
}

pub fn hex_to_i128_be(hex_str: &str) -> Result<i128, ConversionError> {
    let bytes: [u8; 16] = hex_to_bytes(hex_str)?;
    Ok(i128::from_be_bytes(bytes))
}

pub fn hex_to_i128_le(hex_str: &str) -> Result<i128, ConversionError> {
    let bytes: [u8; 16] = hex_to_bytes(hex_str)?;
    Ok(i128::from_le_bytes(bytes))
}

pub fn hex_to_u128_be(hex_str: &str) -> Result<u128, ConversionError> {
    let bytes: [u8; 16] = hex_to_bytes(hex_str)?;
    Ok(u128::from_be_bytes(bytes))
}

pub fn hex_to_u128_le(hex_str: &str) -> Result<u128, ConversionError> {
    let bytes: [u8; 16] = hex_to_bytes(hex_str)?;
    Ok(u128::from_le_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_i8_valid() {
        assert_eq!(hex_to_i8("00").unwrap(), 0);
        assert_eq!(hex_to_i8("7f").unwrap(), 127);
        assert_eq!(hex_to_i8("80").unwrap(), -128);
        assert_eq!(hex_to_i8("ff").unwrap(), -1);
    }

    #[test]
    fn test_hex_to_u8_valid() {
        assert_eq!(hex_to_u8("00").unwrap(), 0);
        assert_eq!(hex_to_u8("7f").unwrap(), 127);
        assert_eq!(hex_to_u8("80").unwrap(), 128);
        assert_eq!(hex_to_u8("ff").unwrap(), 255);
    }

    #[test]
    fn test_hex_to_i16_be_valid() {
        assert_eq!(hex_to_i16_be("0000").unwrap(), 0);
        assert_eq!(hex_to_i16_be("7fff").unwrap(), 32767);
        assert_eq!(hex_to_i16_be("8000").unwrap(), -32768);
        assert_eq!(hex_to_i16_be("ffff").unwrap(), -1);
    }

    #[test]
    fn test_hex_to_i16_le_valid() {
        assert_eq!(hex_to_i16_le("0000").unwrap(), 0);
        assert_eq!(hex_to_i16_le("ff7f").unwrap(), 32767); // 0x7fff
        assert_eq!(hex_to_i16_le("0080").unwrap(), -32768); // 0x8000
        assert_eq!(hex_to_i16_le("ffff").unwrap(), -1);
    }

    #[test]
    fn test_hex_to_u16_be_valid() {
        assert_eq!(hex_to_u16_be("0000").unwrap(), 0);
        assert_eq!(hex_to_u16_be("7fff").unwrap(), 32767);
        assert_eq!(hex_to_u16_be("8000").unwrap(), 32768);
        assert_eq!(hex_to_u16_be("ffff").unwrap(), 65535);
    }

    #[test]
    fn test_hex_to_u16_le_valid() {
        assert_eq!(hex_to_u16_le("0000").unwrap(), 0);
        assert_eq!(hex_to_u16_le("ff7f").unwrap(), 32767); // 0x7fff
        assert_eq!(hex_to_u16_le("0080").unwrap(), 32768); // 0x8000
        assert_eq!(hex_to_u16_le("ffff").unwrap(), 65535);
    }

    #[test]
    fn test_hex_to_i32_be_valid() {
        assert_eq!(hex_to_i32_be("00000000").unwrap(), 0);
        assert_eq!(hex_to_i32_be("7fffffff").unwrap(), 2147483647);
        assert_eq!(hex_to_i32_be("80000000").unwrap(), -2147483648);
        assert_eq!(hex_to_i32_be("ffffffff").unwrap(), -1);
    }

    #[test]
    fn test_hex_to_i32_le_valid() {
        assert_eq!(hex_to_i32_le("00000000").unwrap(), 0);
        assert_eq!(hex_to_i32_le("ffffff7f").unwrap(), 2147483647); // 0x7fffffff
        assert_eq!(hex_to_i32_le("00000080").unwrap(), -2147483648); // 0x80000000
        assert_eq!(hex_to_i32_le("ffffffff").unwrap(), -1);
    }

    #[test]
    fn test_hex_to_u32_be_valid() {
        assert_eq!(hex_to_u32_be("00000000").unwrap(), 0);
        assert_eq!(hex_to_u32_be("7fffffff").unwrap(), 2147483647);
        assert_eq!(hex_to_u32_be("80000000").unwrap(), 2147483648);
        assert_eq!(hex_to_u32_be("ffffffff").unwrap(), 4294967295);
    }

    #[test]
    fn test_hex_to_u32_le_valid() {
        assert_eq!(hex_to_u32_le("00000000").unwrap(), 0);
        assert_eq!(hex_to_u32_le("ffffff7f").unwrap(), 2147483647); // 0x7fffffff
        assert_eq!(hex_to_u32_le("00000080").unwrap(), 2147483648); // 0x80000000
        assert_eq!(hex_to_u32_le("ffffffff").unwrap(), 4294967295);
    }

    #[test]
    fn test_hex_to_i64_be_valid() {
        assert_eq!(hex_to_i64_be("0000000000000000").unwrap(), 0);
        assert_eq!(
            hex_to_i64_be("7fffffffffffffff").unwrap(),
            9223372036854775807
        );
        assert_eq!(
            hex_to_i64_be("8000000000000000").unwrap(),
            -9223372036854775808
        );
        assert_eq!(hex_to_i64_be("ffffffffffffffff").unwrap(), -1);
    }

    #[test]
    fn test_hex_to_i64_le_valid() {
        assert_eq!(hex_to_i64_le("0000000000000000").unwrap(), 0);
        assert_eq!(
            hex_to_i64_le("ffffffffffffff7f").unwrap(),
            9223372036854775807
        ); // 0x7fffffffffffffff
        assert_eq!(
            hex_to_i64_le("0000000000000080").unwrap(),
            -9223372036854775808
        ); // 0x8000000000000000
        assert_eq!(hex_to_i64_le("ffffffffffffffff").unwrap(), -1);
    }

    #[test]
    fn test_hex_to_u64_be_valid() {
        assert_eq!(hex_to_u64_be("0000000000000000").unwrap(), 0);
        assert_eq!(
            hex_to_u64_be("7fffffffffffffff").unwrap(),
            9223372036854775807
        );
        assert_eq!(
            hex_to_u64_be("8000000000000000").unwrap(),
            9223372036854775808
        );
        assert_eq!(
            hex_to_u64_be("ffffffffffffffff").unwrap(),
            18446744073709551615
        );
    }

    #[test]
    fn test_hex_to_u64_le_valid() {
        assert_eq!(hex_to_u64_le("0000000000000000").unwrap(), 0);
        assert_eq!(
            hex_to_u64_le("ffffffffffffff7f").unwrap(),
            9223372036854775807
        ); // 0x7fffffffffffffff
        assert_eq!(
            hex_to_u64_le("0000000000000080").unwrap(),
            9223372036854775808
        ); // 0x8000000000000000
        assert_eq!(
            hex_to_u64_le("ffffffffffffffff").unwrap(),
            18446744073709551615
        );
    }

    #[test]
    fn test_invalid_hex_string() {
        assert!(matches!(
            hex_to_i8("gg").unwrap_err(),
            ConversionError::InvalidHex(_)
        ));
        assert!(matches!(
            hex_to_u64_be("nothex").unwrap_err(),
            ConversionError::InvalidHex(_)
        ));
    }

    #[test]
    fn test_incorrect_length() {
        assert!(matches!(
            hex_to_i8("0000").unwrap_err(),
            ConversionError::IncorrectLength {
                expected: 1,
                found: 2
            }
        ));
        assert!(matches!(
            hex_to_u16_be("00").unwrap_err(),
            ConversionError::IncorrectLength {
                expected: 2,
                found: 1
            }
        ));
        assert!(matches!(
            hex_to_u64_be("0000").unwrap_err(),
            ConversionError::IncorrectLength {
                expected: 8,
                found: 2
            }
        ));
    }
}
