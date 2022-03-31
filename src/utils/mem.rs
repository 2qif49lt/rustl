
macro_rules! field_offset {
    ($t:ident :: $field:ident) => {{
        use memoffset::offset_of;
        offset_of!($t, $field)
    }};
}
pub(crate) use field_offset;

#[allow(unused_macros)]
macro_rules! field_size {
    ($t:ident :: $field:ident) => {{
        let m = core::mem::MaybeUninit::<$t>::uninit();
        // According to https://doc.rust-lang.org/stable/std/ptr/macro.addr_of_mut.html#examples,
        // you can dereference an uninitialized MaybeUninit pointer in addr_of!
        // Raw pointer deref in const contexts is stabilized in 1.58:
        // https://github.com/rust-lang/rust/pull/89551
        let p = unsafe { core::ptr::addr_of!((*(&m as *const _ as *const $t)).$field) };

        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        size_of_raw(p)
    }};
}

#[allow(unused_imports)]
pub(crate) use field_size;

macro_rules! field_len {
    ($t:ident :: $member:ident) => {{
        fn field_len_helper<F, S, M>(_: F) -> usize
        where
            F: Fn(S) -> M,
        {
            std::mem::size_of::<M>()
        }
        field_len_helper(|s: $t| s.$member)
    }};
}

pub(crate) use field_len;

#[cfg(test)]
mod tests {
    use std::mem::size_of;
    use super::*;
    
    #[repr(C, packed)]
    struct Tf {
        a: u32,
        b: String,
    }
    #[test]
    fn check_field_offset() {
        assert_eq!(field_offset!(Tf::a), 0);
        assert_eq!(field_offset!(Tf::b), size_of::<u32>());
    }
    #[test]
    fn check_field_size() {
        assert_eq!(field_size!(Tf::a), size_of::<u32>());
        assert_eq!(field_size!(Tf::b), size_of::<String>());
    }

    #[test]
    fn check_field_len() {
        assert_eq!(field_len!(Tf::a), 4);
        assert_eq!(field_len!(Tf::b), size_of::<String>());
    }
}
