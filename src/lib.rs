use std::fmt::Debug;

pub trait Flag {
    const SIZE: usize;
    fn into_usize(&self) -> usize;
}

#[derive(Clone, Copy)]
pub struct Flags<const SIZE: usize> {
    _data: [u8; SIZE],
}

impl<const SIZE: usize> Default for Flags<SIZE> {
    fn default() -> Self {
        let data = [0u8; SIZE];
        Self { _data: data }
    }
}

impl<const SIZE: usize> Flags<SIZE> {
    pub fn add_flag<T: Flag + Copy>(&mut self, flag: T) {
        let u_flag = flag.into_usize();
        if let Some(byte) = self._data.get_mut(u_flag / 8) {
            *byte |= 1u8 << (u_flag % 8);
        }
    }

    pub fn has_flag<T: Flag + Copy>(&self, flag: T) -> bool {
        let u_flag = flag.into_usize();
        match self._data.get(u_flag / 8) {
            Some(byte) => (byte & (1u8 << (u_flag % 8))) != 0,
            None => false,
        }
    }

    pub fn remove_flag<T: Flag + Copy>(&mut self, flag: T) {
        let u_flag = flag.into_usize();
        if let Some(byte) = self._data.get_mut(u_flag / 8) {
            *byte &= !(1u8 << (u_flag % 8));
        }
    }
}

impl<const SIZE: usize> Debug for Flags<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for i in 0..SIZE * 8 {
            res += &format!("{:b}", self._data[i]);
        }
        write!(f, "{res}")
    }
}
