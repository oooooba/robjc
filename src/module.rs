use std::collections::HashMap;
use std::fmt;
use std::slice;

use super::category::ObjcCategory;
use super::class::ObjcClass;
use super::ptr::Ptr;
use super::selector::ObjcSelector;
use super::str_ptr::StrPtr;
use super::ULong;
use super::UShort;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcSymtab {
    _sel_ref_cnt: ULong,
    refs: Option<Ptr<ObjcSelector>>,
    cls_def_cnt: UShort,
    cat_def_cnt: UShort,
    defs: [Ptr<()>; 0],
}

pub type SelectorMap = HashMap<StrPtr, Ptr<ObjcSelector>>;
/*
 * // ToDo: currently, ignore types
pub type SelectorMap = HashMap<(StrPtr, StrPtr), Ptr<ObjcSelector>>;
 */

impl ObjcSymtab {
    pub fn cls_def_cnt(&self) -> usize {
        self.cls_def_cnt as usize
    }

    pub fn cat_def_cnt(&self) -> usize {
        self.cat_def_cnt as usize
    }

    fn nth_def<T>(&self, i: usize) -> Option<&mut Ptr<T>> {
        let num_entries = self.cls_def_cnt() + self.cat_def_cnt();
        if i >= num_entries {
            return None;
        }
        let addr_defs = &self.defs as *const Ptr<()> as *mut Ptr<T>;
        unsafe {
            let defs = slice::from_raw_parts_mut(addr_defs, num_entries);
            Some(&mut defs[i])
        }
    }

    pub fn nth_class_ptr(&self, i: usize) -> Option<&Ptr<ObjcClass>> {
        self.nth_def(i).map(|class| class as &Ptr<ObjcClass>)
    }

    pub fn nth_class_ptr_mut(&mut self, i: usize) -> Option<&mut Ptr<ObjcClass>> {
        self.nth_def(i)
    }

    pub fn nth_category_ptr(&self, i: usize) -> Option<&Ptr<ObjcCategory>> {
        self.nth_def(self.cls_def_cnt() + i)
            .map(|category| category as &Ptr<ObjcCategory>)
    }

    pub fn nth_category_ptr_mut(&self, i: usize) -> Option<&mut Ptr<ObjcCategory>> {
        self.nth_def(self.cls_def_cnt() + i)
    }

    pub fn iter_class(&self) -> ObjcClassIterator {
        ObjcClassIterator {
            symtab: unsafe { Ptr::new(self) },
            index: 0,
        }
    }

    pub fn iter_category(&self) -> ObjcCategoryIterator {
        ObjcCategoryIterator {
            symtab: unsafe { Ptr::new(self) },
            index: 0,
        }
    }

    pub fn iter_selector(&self) -> ObjcSelectorIterator {
        ObjcSelectorIterator(self.refs.clone())
    }

    pub fn create_selector_map(&self) -> SelectorMap {
        let mut map = HashMap::new();
        for selector in self.iter_selector() {
            let id = selector.as_ref().get_id().clone();
            assert_ne!(id, StrPtr::null());
            let _types = selector.as_ref().get_types().clone();
            /*
             * // ToDo: currently, ignore types
             * assert_ne!(_types, StrPtr::null());
             * map.insert((id, _types), selector);
             */
            map.insert(id, selector);
        }
        map
    }
}

pub struct ObjcClassIterator {
    symtab: Ptr<ObjcSymtab>,
    index: usize,
}

impl Iterator for ObjcClassIterator {
    type Item = Ptr<ObjcClass>;

    fn next(&mut self) -> Option<Self::Item> {
        let symtab = self.symtab.as_ref();
        let index = self.index;
        if index >= symtab.cls_def_cnt() {
            return None;
        }
        self.index += 1;
        symtab.nth_class_ptr(index).map(|p| p.clone())
    }
}

pub struct ObjcCategoryIterator {
    symtab: Ptr<ObjcSymtab>,
    index: usize,
}

impl Iterator for ObjcCategoryIterator {
    type Item = Ptr<ObjcCategory>;

    fn next(&mut self) -> Option<Self::Item> {
        let symtab = self.symtab.as_ref();
        let index = self.index;
        if index >= symtab.cat_def_cnt() {
            return None;
        }
        self.index += 1;
        symtab.nth_category_ptr(index).map(|p| p.clone())
    }
}

pub struct ObjcSelectorIterator(Option<Ptr<ObjcSelector>>);

impl Iterator for ObjcSelectorIterator {
    type Item = Ptr<ObjcSelector>;

    fn next(&mut self) -> Option<Self::Item> {
        let selector = self.0.clone()?;
        if selector.as_ref().get_id().is_null() {
            self.0 = None;
            return None;
        }
        self.0 = Some(unsafe { Ptr::new((selector.as_ptr()).offset(1)) });
        Some(selector)
    }
}

impl fmt::Display for ObjcSymtab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Symtab @ {:p} [", self)?;
        write!(f, " cls_def_cnt: {},", self.cls_def_cnt)?;
        write!(f, " cat_def_cnt: {},", self.cat_def_cnt)?;
        writeln!(f, " refs:")?;
        for selector in self.iter_selector() {
            writeln!(f, "  * {},", selector.as_ref())?;
        }
        write!(f, ",")?;
        {
            writeln!(f, " defs:")?;
            for i in 0..self.cls_def_cnt() {
                match self.nth_class_ptr(i) {
                    Some(cls) => {
                        writeln!(f, "  ({}) {},", i, cls.as_ref())?;
                        writeln!(f, "  ({}/m) {},", i, cls.class_pointer().as_ref())?;
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
pub struct ObjcModule {
    version: ULong,
    size: ULong,
    name: StrPtr,
    symtab: Ptr<ObjcSymtab>,
}

impl ObjcModule {
    pub fn symtab(&self) -> &Ptr<ObjcSymtab> {
        &self.symtab
    }

    pub fn symtab_mut(&mut self) -> &mut Ptr<ObjcSymtab> {
        &mut self.symtab
    }
}

impl fmt::Display for ObjcModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Module @ {:p} [ name: {}, version: {}, size: {}, symtab: {:p} ]",
            self,
            self.name,
            self.version,
            self.size,
            self.symtab.as_ptr()
        )
    }
}
