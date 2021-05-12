use std::convert::TryFrom;

pub type Address = EthereumType<20_usize>;

pub struct EthereumType<const N: usize>([u8; N]);

impl<const N: usize> EthereumType<N> {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn into_string(self) -> String {
        format!(
            "0x{}",
            self.0
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join("")
        )
    }

    #[inline]
    pub fn into_bytes(self) -> [u8; N] {
        self.0
    }
}

impl<const N: usize> TryFrom<&str> for EthereumType<N> {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let stripped = if let Some(s) = value.strip_prefix("0x") {
            s
        } else {
            value
        };

        let length = stripped.len() / 2;
        // NOTE only works for even lengths! Should it support types like
        // EthereumType<25> with an odd const generic parameter?
        if length == N {
            let mut data = [0_u8; N];
            for i in 0..length {
                data[i] = u8::from_str_radix(&stripped[2 * i..2 * i + 2], 16).map_err(|e| e.to_string())?;
            }
            Ok(Self(data))
        } else {
            Err(format!("Expected input length was {}, found {}", N, length))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn try_from_str() {
        let test_str = "0x1234567890abcdeffedcba098765432100007777";
        let non_prefixed_address =
            Address::try_from(test_str.strip_prefix("0x").unwrap()).unwrap();
        let zerox_prefixed_address =
            Address::try_from(test_str).unwrap();

        let non_prefixed_string = non_prefixed_address.into_string();
        let zerox_prefixed_string = zerox_prefixed_address.into_string();

        assert_eq!(non_prefixed_string.as_str(), test_str);
        assert_eq!(zerox_prefixed_string.as_str(), test_str);
    }
}
