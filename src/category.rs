use std::fmt;

use super::method::ObjcMethodList;
use super::ptr::Ptr;
use super::str_ptr::StrPtr;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcCategory {
    category_name: StrPtr,
    class_name: StrPtr,
    instance_methods: Option<Ptr<ObjcMethodList>>,
    class_methods: Option<Ptr<ObjcMethodList>>,
    protocols: Option<Ptr<()>>,
}

impl ObjcCategory {}

impl fmt::Display for ObjcCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Category @ {:p} [", self)?;
        writeln!(f, " category_name: {},", self.category_name)?;
        writeln!(f, " class_name: {},", self.class_name)?;
        writeln!(
            f,
            " instance_methods: {},",
            self.instance_methods
                .as_ref()
                .map_or("null".to_string(), |methods| format!(
                    "{}",
                    methods.as_ref()
                ))
        )?;
        writeln!(
            f,
            " class_methods: {},",
            self.class_methods
                .as_ref()
                .map_or("null".to_string(), |methods| format!(
                    "{}",
                    methods.as_ref()
                ))
        )?;
        write!(f, "]")
    }
}
