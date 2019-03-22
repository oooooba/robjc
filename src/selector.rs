use std::fmt;

use super::str_ptr::StrPtr;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcSelector {
    sel_id: StrPtr, // ToDo: fix to represent void*
    sel_types: StrPtr,
}

impl ObjcSelector {
    pub fn get_id(&self) -> &StrPtr {
        &self.sel_id
    }

    pub fn get_types(&self) -> &StrPtr {
        &self.sel_types
    }
}

impl fmt::Display for ObjcSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Selector @ {:p} [ id: {}, types: {} ]",
            self, self.sel_id, self.sel_types
        )
    }
}
