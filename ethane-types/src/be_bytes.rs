pub trait BeBytes<const N: usize> {
    fn be_bytes(&self) -> [u8; N];
}

impl BeBytes<1> for u8 {
    #[inline]
    fn be_bytes(&self) -> [u8; 1] {
        self.to_be_bytes()
    }
}

impl BeBytes<2> for u16 {
    #[inline]
    fn be_bytes(&self) -> [u8; 2] {
        self.to_be_bytes()
    }
}

impl BeBytes<4> for u32 {
    #[inline]
    fn be_bytes(&self) -> [u8; 4] {
        self.to_be_bytes()
    }
}

impl BeBytes<8> for u64 {
    #[inline]
    fn be_bytes(&self) -> [u8; 8] {
        self.to_be_bytes()
    }
}

impl BeBytes<16> for u128 {
    #[inline]
    fn be_bytes(&self) -> [u8; 16] {
        self.to_be_bytes()
    }
}

impl BeBytes<1> for i8 {
    #[inline]
    fn be_bytes(&self) -> [u8; 1] {
        self.to_be_bytes()
    }
}

impl BeBytes<2> for i16 {
    #[inline]
    fn be_bytes(&self) -> [u8; 2] {
        self.to_be_bytes()
    }
}

impl BeBytes<4> for i32 {
    #[inline]
    fn be_bytes(&self) -> [u8; 4] {
        self.to_be_bytes()
    }
}

impl BeBytes<8> for i64 {
    #[inline]
    fn be_bytes(&self) -> [u8; 8] {
        self.to_be_bytes()
    }
}

impl BeBytes<16> for i128 {
    #[inline]
    fn be_bytes(&self) -> [u8; 16] {
        self.to_be_bytes()
    }
}
