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
    pub fn add_flag<T: Flag + Default + Copy>(&mut self, flag: T) {
        let u_flag = flag.into_usize();
        let p_chunk = &mut self._data[u_flag / 8] as *mut u8;
        let bit = u_flag % 8;
        unsafe {
            *p_chunk = *p_chunk | (1u8 << bit as u8);
        }
    }

    pub fn has_flag<T: Flag + Default + Copy>(&self, flag: T) -> bool {
        let u_flag = flag.into_usize();
        let p_chunk = &self._data[u_flag / 8] as *const u8;
        let bit = u_flag % 8;
        unsafe { *p_chunk & (1u8 << bit as u8) == (1u8 << bit as u8) }
    }

    pub fn remove_flag<T: Flag + Default + Copy>(&mut self, flag: T) {
        let u_flag = flag.into_usize();
        let p_chunk = &mut self._data[u_flag / 8] as *mut u8;
        let bit = u_flag % 8;
        unsafe {
            *p_chunk = *p_chunk & !(1u8 << bit as u8);
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

mod test {
    use crate::Flags;

    use super::Flag;
    use derive::Flag;

    #[derive(Flag, Clone, Copy, Default, Debug)]
    enum SomeFlag {
        #[default]
        A = 0,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
    }

    #[test]
    fn test() {
        let mut flags = Flags::<{ SomeFlag::SIZE }>::default();
        flags.add_flag(SomeFlag::N);
        flags.add_flag(SomeFlag::A);
        flags.add_flag(SomeFlag::H);
        flags.remove_flag(SomeFlag::H);

        SomeFlag::debug_flags(&flags);
    }
}
