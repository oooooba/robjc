use std::fmt;

use super::context::Context;
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

impl ObjcCategory {
    pub fn initialize(&mut self, _ctx: &mut Context) {}

    pub fn defer_resolving_methods(&self, ctx: &mut Context) -> bool {
        let (class, meta_class) = match ctx.get_class_entry(&self.class_name) {
            Some(entry) => (entry.class().clone(), entry.meta_class().clone()),
            None => return false,
        };
        if let Some(methods) = self.instance_methods.as_ref() {
            for method in methods.iter() {
                ctx.append_unresolved_methods(class.clone(), method);
            }
        }
        if let Some(methods) = self.class_methods.as_ref() {
            for method in methods.iter() {
                ctx.append_unresolved_methods(meta_class.clone(), method);
            }
        }
        true
    }
}

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
