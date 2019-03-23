use std::cell;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::hash;
use std::mem;

use super::context::Context;
use super::ivar::ObjcIvarList;
use super::method::ObjcMethod;
use super::method::ObjcMethodList;
use super::selector::ObjcSelector;
use super::str_ptr::StrPtr;
use super::Long;
use super::Sel;
use super::ULong;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcClass<'a> {
    class_pointer: cell::UnsafeCell<&'a ObjcClass<'a>>,
    super_pointer: Option<&'a ObjcClass<'a>>,
    name: StrPtr,
    version: Long,
    info: ULong,
    instance_size: Long,
    ivars: Option<&'a ObjcIvarList>,
    methods: Option<&'a ObjcMethodList<'a>>,
    dtable: Option<Box<HashMap<StrPtr, &'a ObjcMethod<'a>>>>,
    subclass_list: Option<&'a ()>,
    sibling_list: Option<&'a ()>,
    protocols: Option<&'a ()>,
    gc_object_type: Option<&'a ()>,
}

impl<'a> ObjcClass<'a> {
    pub unsafe fn get_mut_class_pointer(&self) -> &mut ObjcClass {
        *(self.class_pointer.get() as *mut &mut ObjcClass)
    }

    pub fn get_class_pointer(&self) -> &ObjcClass<'a> {
        unsafe { *self.class_pointer.get() }
    }

    pub fn get_super_pointer(&self) -> Option<&ObjcClass<'a>> {
        self.super_pointer
    }

    pub fn get_name(&self) -> &StrPtr {
        &self.name
    }

    pub fn get_instance_size(&self) -> usize {
        self.instance_size as usize
    }

    pub fn is_class(&self) -> bool {
        self.info & 0b1 != 0
    }

    pub fn is_meta(&self) -> bool {
        self.info & 0b10 != 0
    }

    pub fn resolve_method(&self, selector: &ObjcSelector) -> Option<&ObjcMethod<'a>> {
        let method_name = selector.get_id().clone();
        let table = self.dtable.as_ref().expect("dtable is not initialized");
        table.get(&method_name).map(|method| *method).or_else(|| {
            self.super_pointer
                .and_then(|super_class| super_class.resolve_method(selector))
        })
    }

    pub fn initialize(&mut self, ctx: &mut Context) {
        self.initialize_dtable(ctx);
    }

    pub fn initialize_super_pointer(&mut self, ctx: &mut Context<'a>) -> bool {
        if self.super_pointer.is_none() {
            return true;
        }
        let super_class_name: StrPtr = unsafe { mem::transmute(self.super_pointer.unwrap()) };
        if let Some(entry) = ctx.get_class_table().get(&super_class_name) {
            self.super_pointer = Some(if self.is_meta() {
                entry.get_meta_class()
            } else {
                entry.get_class()
            });
            true
        } else {
            false
        }
    }

    pub fn registry_method(
        &mut self,
        name: StrPtr,
        method: &'a ObjcMethod<'a>,
    ) -> Option<&ObjcMethod<'a>> {
        self.dtable
            .as_mut()
            .expect("dtable is not initialized")
            .insert(name, method)
    }

    fn initialize_dtable(&mut self, _ctx: &mut Context) {
        self.dtable = Some(Box::new(HashMap::new()));
        let mut method_list_ptr = self.methods.clone();
        while let Some(method_list) = method_list_ptr {
            for i in 0..method_list.method_count() {
                let method = method_list.nth_method(i).unwrap();
                let method_name =
                    unsafe { mem::transmute::<Sel, StrPtr>(method.get_name().clone()) };
                self.registry_method(method_name, method);
            }
            method_list_ptr = method_list.get_next().clone();
        }
    }
}

impl<'a> fmt::Display for ObjcClass<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        assert!(self.is_class() || self.is_meta());
        writeln!(
            f,
            "Class{} @ {:p} [",
            if self.is_meta() { "(meta)" } else { "" },
            self
        )?;
        writeln!(f, " name: {},", self.name)?;
        if self.is_class() {
            writeln!(
                f,
                " class: {} ({:p}),",
                self.get_class_pointer().name,
                self.get_class_pointer()
            )?;
        } else if self.is_meta() {
            let s = unsafe { mem::transmute::<&ObjcClass, StrPtr>(self.get_class_pointer()) };
            writeln!(f, " class: {} ({:p}),", s, self.get_class_pointer())?;
        } else {
            unreachable!()
        }
        {
            match self.super_pointer {
                Some(p) => {
                    let s = unsafe { mem::transmute::<&ObjcClass, StrPtr>(p) };
                    writeln!(f, " super: {} ({:p}),", s, p)?
                }
                None => writeln!(f, " super: null,")?,
            }
        }
        writeln!(
            f,
            " version: {}, info: {}, instance_size: {} ",
            self.version, self.info, self.instance_size
        )?;
        writeln!(
            f,
            " ivars: {},",
            self.ivars
                .as_ref()
                .map_or("null".to_string(), |ivars| format!("{}", ivars))
        )?;
        writeln!(
            f,
            " methods: {},",
            self.methods
                .as_ref()
                .map_or("null".to_string(), |methods| format!("{}", methods))
        )?;
        writeln!(f, " dtable: disabled,")?;
        writeln!(
            f,
            " subclass_list: {},",
            self.subclass_list
                .as_ref()
                .map_or("null".to_string(), |subclass_list| format!(
                    "{:p}",
                    subclass_list
                ))
        )?;
        writeln!(
            f,
            " sibling_list: {},",
            self.sibling_list
                .as_ref()
                .map_or("null".to_string(), |sibling_list| format!(
                    "{:p}",
                    sibling_list
                ))
        )?;
        writeln!(
            f,
            " gc_object_type: {},",
            self.gc_object_type
                .as_ref()
                .map_or("null".to_string(), |gc_object_type| format!(
                    "{:p}",
                    gc_object_type
                ))
        )?;
        write!(f, "]")
    }
}

impl<'a> cmp::PartialEq for &ObjcClass<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self as *const ObjcClass == *other as *const ObjcClass
    }
}

impl<'a> cmp::Eq for &ObjcClass<'a> {}

impl<'a> hash::Hash for &ObjcClass<'a> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (*self as *const ObjcClass).hash(state);
    }
}
