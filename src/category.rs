use std::fmt;

use super::method::ObjcMethodList;
use super::str_ptr::StrPtr;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcCategory<'a> {
    category_name: StrPtr,
    class_name: StrPtr,
    instance_methods: Option<&'a ObjcMethodList>,
    class_methods: Option<&'a ObjcMethodList>,
    protocols: Option<&'a ()>,
}

impl<'a> ObjcCategory<'a> {}

impl<'a> fmt::Display for ObjcCategory<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Category @ {:p} [", self)?;
        writeln!(f, " category_name: {},", self.category_name)?;
        writeln!(f, " class_name: {},", self.class_name)?;
        writeln!(
            f,
            " instance_methods: {},",
            self.instance_methods
                .as_ref()
                .map_or("null".to_string(), |methods| format!("{}", methods))
        )?;
        writeln!(
            f,
            " class_methods: {},",
            self.class_methods
                .as_ref()
                .map_or("null".to_string(), |methods| format!("{}", methods))
        )?;
        write!(f, "]")
    }
}
