use std::fmt;
use std::slice;

use super::category::ObjcCategory;
use super::class::ObjcClass;
use super::selector::ObjcSelector;
use super::str_ptr::StrPtr;
use super::ULong;
use super::UShort;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcSymtab<'a> {
    _sel_ref_cnt: ULong,
    refs: Option<&'a ObjcSelector>,
    cls_def_cnt: UShort,
    cat_def_cnt: UShort,
    defs: [&'a (); 0],
}

impl<'a> ObjcSymtab<'a> {
    pub fn cls_def_cnt(&self) -> usize {
        self.cls_def_cnt as usize
    }

    pub fn cat_def_cnt(&self) -> usize {
        self.cat_def_cnt as usize
    }

    fn nth_def<T>(&self, i: usize) -> Option<&mut T> {
        let num_entries = self.cls_def_cnt() + self.cat_def_cnt();
        if i >= num_entries {
            return None;
        }
        let addr_defs = &self.defs as *const &() as *const *mut T;
        unsafe {
            let defs = slice::from_raw_parts(addr_defs, num_entries);
            Some(&mut (*defs[i]))
        }
    }

    pub fn nth_class_ptr(&self, i: usize) -> Option<&ObjcClass> {
        self.nth_def(i).map(|class| class as &ObjcClass)
    }

    pub fn nth_class_ptr_mut(&self, i: usize) -> Option<&mut ObjcClass> {
        self.nth_def(i)
    }

    pub fn nth_category_ptr(&self, i: usize) -> Option<&ObjcCategory> {
        self.nth_def(self.cls_def_cnt() + i)
            .map(|class| class as &ObjcCategory)
    }

    pub fn nth_category_ptr_mut(&self, i: usize) -> Option<&mut ObjcCategory> {
        self.nth_def(self.cls_def_cnt() + i)
    }
}

impl<'a> fmt::Display for ObjcSymtab<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Symtab @ {:p} [", self)?;
        write!(f, " cls_def_cnt: {},", self.cls_def_cnt)?;
        write!(f, " cat_def_cnt: {},", self.cat_def_cnt)?;
        writeln!(f, " refs:")?;
        {
            if let Some(mut selector) = self.refs.clone() {
                loop {
                    writeln!(f, "  * {},", selector)?;
                    if selector.get_id().as_ref().is_none() {
                        break;
                    }
                    selector = unsafe { &*(selector as *const ObjcSelector).offset(1) };
                }
            } else {
                writeln!(f, "  <no selectors>")?;
            }
        }
        write!(f, ",")?;
        {
            writeln!(f, " defs:")?;
            for i in 0..self.cls_def_cnt() {
                match self.nth_class_ptr(i) {
                    Some(cls) => {
                        writeln!(f, "  ({}) {},", i, cls)?;
                        writeln!(f, "  ({}/m) {},", i, cls.get_class_pointer())?;
                    }
                    None => unreachable!(),
                }
            }
        }
        write!(f, " ]")
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjcModule<'a> {
    version: ULong,
    size: ULong,
    name: StrPtr,
    symtab: &'a ObjcSymtab<'a>,
}

impl<'a> ObjcModule<'a> {
    pub fn get_symtab(&self) -> &ObjcSymtab<'a> {
        self.symtab
    }
}

impl<'a> fmt::Display for ObjcModule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Module @ {:p} [ name: {}, version: {}, size: {}, symtab: {:p} ]",
            self, self.name, self.version, self.size, self.symtab
        )
    }
}
