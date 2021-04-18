use super::Parameter;

use ethereum_types::U256;
use std::fmt;
use std::str;

impl fmt::Display for Parameter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(data) => write!(
                formatter,
                "Address(0x{})",
                data.as_bytes()[12..]
                    .iter()
                    .map(|c| format!("{:02x}", c))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            Self::Bool(data) => write!(formatter, "{}", data.as_bytes()[31] != 0),
            Self::Uint(data, len) => match len {
                8 => write!(formatter, "{}", data[31]),
                16 => {
                    let mut bytes = [0u8; 2];
                    bytes.copy_from_slice(&data[30..]);
                    write!(formatter, "{}", u16::from_be_bytes(bytes))
                }
                32 => {
                    let mut bytes = [0u8; 4];
                    bytes.copy_from_slice(&data[28..]);
                    write!(formatter, "{}", u32::from_be_bytes(bytes))
                }
                64 => {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&data[24..]);
                    write!(formatter, "{}", u64::from_be_bytes(bytes))
                }
                128 => {
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(&data[16..]);
                    write!(formatter, "{}", u128::from_be_bytes(bytes))
                }
                256 => write!(formatter, "{}", U256::from(data.as_bytes())),
                _ => panic!("Invalid number!"),
            },
            Self::Int(data, len) => match len {
                8 => write!(formatter, "{}", data[31] as i8),
                16 => {
                    let mut bytes = [0u8; 2];
                    bytes.copy_from_slice(&data[30..]);
                    write!(formatter, "{}", i16::from_be_bytes(bytes))
                }
                32 => {
                    let mut bytes = [0u8; 4];
                    bytes.copy_from_slice(&data[28..]);
                    write!(formatter, "{}", i32::from_be_bytes(bytes))
                }
                64 => {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&data[24..]);
                    write!(formatter, "{}", i64::from_be_bytes(bytes))
                }
                128 => {
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(&data[16..]);
                    write!(formatter, "{}", i128::from_be_bytes(bytes))
                }
                _ => panic!("Invalid number!"),
            },
            Self::Bytes(data) | Self::FixedBytes(data) => {
                if let Ok(string) = str::from_utf8(&data) {
                    write!(formatter, "{}", string)
                } else {
                    write!(
                        formatter,
                        "[{}]",
                        data.iter()
                            .map(|c| format!("0x{:02x}", c))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
            Self::String(data) => {
                write!(formatter, "{}", String::from_utf8(data.to_vec()).unwrap())
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parameter;
    use ethereum_types::{Address, U256};
    use std::str::FromStr;
    #[test]
    fn display() {
        let expected = format!(
            "{}",
            Parameter::from(
                Address::from_str("0x99429f64cf4d5837620dcc293c1a537d58729b68").unwrap()
            )
        );
        assert_eq!(
            &expected,
            "Address(0x99429f64cf4d5837620dcc293c1a537d58729b68)"
        );

        let expected = format!("{}", Parameter::from(true));
        assert_eq!(&expected, "true");

        let expected = format!("{}", Parameter::from(false));
        assert_eq!(&expected, "false");

        let expected = format!("{}", Parameter::new_bytes("abc".as_bytes()));
        assert_eq!(&expected, "abc");

        let expected = format!("{}", Parameter::new_fixed_bytes("hello there".as_bytes()));
        assert_eq!(&expected, "hello there");

        let expected = format!("{}", Parameter::new_bytes(&[240, 12, 13]));
        assert_eq!(&expected, "[0xf0, 0x0c, 0x0d]");

        let expected = format!("{}", Parameter::new_fixed_bytes(&[240, 12, 170]));
        assert_eq!(&expected, "[0xf0, 0x0c, 0xaa]");

        let expected = format!("{}", Parameter::from("hello world!"));
        assert_eq!(&expected, "hello world!");

        let expected = format!("{}", Parameter::from(133_u8));
        assert_eq!(&expected, "133");

        let expected = format!("{}", Parameter::from(1333_u16));
        assert_eq!(&expected, "1333");

        let expected = format!("{}", Parameter::from(1234567_u32));
        assert_eq!(&expected, "1234567");

        let expected = format!("{}", Parameter::from(u64::MAX));
        assert_eq!(&expected, "18446744073709551615");

        let expected = format!("{}", Parameter::from(200000000000000000000_u128));
        assert_eq!(&expected, "200000000000000000000");

        let expected = format!(
            "{}",
            Parameter::from(
                U256::from_str_radix("1234567890123456789012345678901234567890", 10).unwrap()
            )
        );
        assert_eq!(&expected, "1234567890123456789012345678901234567890");

        let expected = format!("{}", Parameter::from(-89i8));
        assert_eq!(&expected, "-89");

        let expected = format!("{}", Parameter::from(-1333_i16));
        assert_eq!(&expected, "-1333");

        let expected = format!("{}", Parameter::from(-1234567_i32));
        assert_eq!(&expected, "-1234567");

        let expected = format!("{}", Parameter::from(i64::MIN));
        assert_eq!(&expected, "-9223372036854775808");

        let expected = format!("{}", Parameter::new_int(&[1u8; 32], true));
        assert_eq!(&expected, "-1");
    }
}
