use std::str::Utf8Error;

pub(crate) trait Utf8ErrorOffsetBy {
    fn offset_by(self, offset: usize) -> Self;
}

struct _Utf8Error {
    valid_up_to: usize,
    error_len: Option<u8>,
}

impl Utf8ErrorOffsetBy for Utf8Error {
    #[inline]
    fn offset_by(self, offset: usize) -> Self {
        // SAFETY: They have the same repr. This is not truly safe due to undefined struct layout,
        // but there is no other way
        let mut this = unsafe { std::mem::transmute::<Self, _Utf8Error>(self) };
        this.valid_up_to += offset;
        unsafe { std::mem::transmute::<_Utf8Error, Self>(this) }
    }
}