use super::Parameter;

use ethane_types::{Address, U256};
use std::convert::TryFrom;
use std::fmt;
use std::str;

impl fmt::Display for Parameter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // data is stored in a H256 type on 32 bytes, so right padded 20
            // bytes have to be extracted
            Self::Address(data) => {
                // unwrap is fine because we know that data is H256
                let address = Address::try_from(&data.into_bytes()[12..]).unwrap();
                write!(formatter, "{}", address)
            }
            Self::Bool(data) => write!(formatter, "{}", data.as_bytes()[31] != 0),
            Self::Uint(data, len) => match len {
                8 => write!(formatter, "{}", data.as_bytes()[31]),
                16 => {
                    let mut bytes = [0u8; 2];
                    bytes.copy_from_slice(&data.as_bytes()[30..]);
                    write!(formatter, "{}", u16::from_be_bytes(bytes))
                }
                32 => {
                    let mut bytes = [0u8; 4];
                    bytes.copy_from_slice(&data.as_bytes()[28..]);
                    write!(formatter, "{}", u32::from_be_bytes(bytes))
                }
                64 => {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&data.as_bytes()[24..]);
                    write!(formatter, "{}", u64::from_be_bytes(bytes))
                }
                128 => {
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(&data.as_bytes()[16..]);
                    write!(formatter, "{}", u128::from_be_bytes(bytes))
                }
                256 => {
                    // unwrap is fine, because we know that data has length 256
                    //U256::try_from(data.as_bytes()).unwrap().to_string()
                }
                _ => panic!("Invalid number!"),
            },
            Self::Int(data, len) => match len {
                8 => write!(formatter, "{}", data.as_bytes()[31] as i8),
                16 => {
                    let mut bytes = [0u8; 2];
                    bytes.copy_from_slice(&data.as_bytes()[30..]);
                    write!(formatter, "{}", i16::from_be_bytes(bytes))
                }
                32 => {
                    let mut bytes = [0u8; 4];
                    bytes.copy_from_slice(&data.as_bytes()[28..]);
                    write!(formatter, "{}", i32::from_be_bytes(bytes))
                }
                64 => {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&data.as_bytes()[24..]);
                    write!(formatter, "{}", i64::from_be_bytes(bytes))
                }
                128 => {
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(&data.as_bytes()[16..]);
                    write!(formatter, "{}", i128::from_be_bytes(bytes))
                }
                // TODO do some conversion based on 2's complement?
                256 => write!(
                    formatter,
                    "0x{}",
                    data.as_bytes()
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<String>>()
                        .join("")
                ),

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
    use ethane_types::{Address, U256};
    use std::convert::TryFrom;
    #[test]
    fn display() {
        let expected = format!(
            "{}",
            Parameter::from(
                Address::try_from("0x99429f64cf4d5837620dcc293c1a537d58729b68").unwrap()
            )
        );
        assert_eq!(&expected, "0x99429f64cf4d5837620dcc293c1a537d58729b68");

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
            Parameter::from(U256::try_from("1234567890123456789012345678901234567890").unwrap())
        );
        assert_eq!(&expected, "0x1234567890123456789012345678901234567890");

        let expected = format!("{}", Parameter::from(-89i8));
        assert_eq!(&expected, "-89");

        let expected = format!("{}", Parameter::from(-1333_i16));
        assert_eq!(&expected, "-1333");

        let expected = format!("{}", Parameter::from(-1234567_i32));
        assert_eq!(&expected, "-1234567");

        let expected = format!("{}", Parameter::from(i64::MIN));
        assert_eq!(&expected, "-9223372036854775808");

        let expected = format!("{}", Parameter::new_int([1u8; 32], true));
        assert_eq!(
            &expected,
            "0x0101010101010101010101010101010101010101010101010101010101010101"
        );
    }
}
