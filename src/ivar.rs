use std::fmt;
use std::slice;

use super::str_ptr::StrPtr;
use super::Int;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcIvar {
    ivar_name: StrPtr,
    ivar_type: StrPtr,
    ivar_offset: Int,
}

impl fmt::Display for ObjcIvar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ivar @ {:p} [ name: {}, type: {}, offset: {} ]",
            self, self.ivar_name, self.ivar_type, self.ivar_offset
        )
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjcIvarList {
    ivar_count: Int,
    ivar_list: [ObjcIvar; 0],
}

impl ObjcIvarList {
    fn ivar_count(&self) -> usize {
        self.ivar_count as usize
    }

    fn nth_ivar(&self, i: usize) -> Option<&ObjcIvar> {
        let count = self.ivar_count();
        if i >= count {
            return None;
        }
        unsafe {
            let list = (self as *const ObjcIvarList).offset(1) as *const ObjcIvar;
            let list = slice::from_raw_parts(list, count);
            Some(&list[i])
        }
    }
}

impl fmt::Display for ObjcIvarList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "IvarList @ {:p} [ count: {}, list:",
            self, self.ivar_count
        )?;
        for i in 0..self.ivar_count() {
            if let Some(ivar) = self.nth_ivar(i) {
                writeln!(f, "  ({}) {},", i, ivar)?;
            } else {
                unreachable!()
            }
        }
        write!(f, "]")
    }
}
