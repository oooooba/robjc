use std::fmt;
use std::mem;
use std::slice;

use super::object::ObjcObject;
use super::ptr::{NilablePtr, Ptr};
use super::selector::ObjcSelector;
use super::str_ptr::StrPtr;
use super::Int;

#[repr(transparent)]
#[derive(Clone)]
pub struct CodePtr(fn(NilablePtr<ObjcObject>, Ptr<ObjcSelector>) -> NilablePtr<ObjcObject>);

impl CodePtr {
    pub fn null_function() -> CodePtr {
        CodePtr(
            |_id: NilablePtr<ObjcObject>, _sel: Ptr<ObjcSelector>| -> NilablePtr<ObjcObject> {
                NilablePtr::nil()
            },
        )
    }
}

impl fmt::Debug for CodePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CodePtr [ {:p} ]", self.0 as *const ())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjcMethod {
    method_name: Ptr<ObjcSelector>,
    method_type: StrPtr,
    method_imp: CodePtr,
}

impl ObjcMethod {
    pub fn get_name(&self) -> &Ptr<ObjcSelector> {
        &self.method_name
    }

    pub fn get_imp(&self) -> &CodePtr {
        &self.method_imp
    }
}

impl fmt::Display for ObjcMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Method @ {:p} [ name: {}, type: {}, imp: {:?}",
            self,
            unsafe { mem::transmute::<Ptr<ObjcSelector>, StrPtr>(self.method_name.clone()) },
            self.method_type,
            self.method_imp
        )
    }
}

// ToDo: fix to use clear algorithm and data structures
pub struct ObjcMethodIterator {
    current_list: Ptr<ObjcMethodList>,
    index: Option<usize>,
}

impl Iterator for ObjcMethodIterator {
    type Item = Ptr<ObjcMethod>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let count = self.current_list.method_count();
        if index < count {
            self.index = Some(index + 1);
            unsafe {
                let list = (self.current_list.as_ptr()).offset(1) as *const ObjcMethod;
                let list = slice::from_raw_parts(list, count);
                Some(Ptr::new(&list[index]))
            }
        } else {
            match self.current_list.get_next() {
                Some(next_list) => {
                    self.current_list = next_list;
                    self.index = Some(0);
                    self.next()
                }
                None => {
                    self.index = None;
                    None
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjcMethodList {
    method_next: Option<Ptr<ObjcMethodList>>,
    method_count: Int,
    method_list: [ObjcMethod; 0],
}

impl ObjcMethodList {
    fn get_next(&self) -> Option<Ptr<ObjcMethodList>> {
        self.method_next.clone()
    }

    // ToDo: consider whether rename to get_count and its signature
    fn method_count(&self) -> usize {
        self.method_count as usize
    }

    fn nth_method(&self, i: usize) -> Option<Ptr<ObjcMethod>> {
        let count = self.method_count();
        if i >= count {
            return None;
        }
        unsafe {
            let list = (self as *const ObjcMethodList).offset(1) as *const ObjcMethod;
            let list = slice::from_raw_parts(list, count);
            Some(Ptr::new(&list[i]))
        }
    }

    pub fn iter(&self) -> ObjcMethodIterator {
        ObjcMethodIterator {
            current_list: unsafe { Ptr::new(self) },
            index: Some(0),
        }
    }
}

impl fmt::Display for ObjcMethodList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "MethodList @ {:p} [ next: disabled, count: {}, list:",
            self, /*self.method_next,*/ self.method_count
        )?;
        for i in 0..self.method_count() {
            if let Some(method) = self.nth_method(i) {
                writeln!(f, "  ({}) {},", i, method.as_ref())?;
            } else {
                unreachable!()
            }
        }
        write!(f, "]")
    }
}
