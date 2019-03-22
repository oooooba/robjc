use std::cmp;
use std::ffi;
use std::fmt;
use std::hash;
use std::os::raw;
use std::ptr;

#[repr(transparent)]
#[derive(Clone, Debug, Eq)]
pub struct StrPtr(Option<ptr::NonNull<raw::c_char>>);

impl StrPtr {
    pub fn null() -> StrPtr {
        StrPtr(None)
    }

    pub fn as_ref(&self) -> Option<&str> {
        match self.0 {
            None => None,
            Some(ref p) => Some(unsafe { ffi::CStr::from_ptr(p.as_ptr()).to_str().unwrap() }),
        }
    }
}

impl fmt::Display for StrPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            Some(s) => write!(f, r#""{}" @ {:p}"#, s, s),
            None => write!(f, "null"),
        }
    }
}

impl cmp::PartialEq for StrPtr {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl hash::Hash for StrPtr {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}
