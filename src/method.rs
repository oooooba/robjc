use std::fmt;
use std::mem;
use std::slice;

use super::object::ObjcObject;
use super::selector::ObjcSelector;
use super::str_ptr::StrPtr;
use super::Int;
use super::Ptr;

#[repr(transparent)]
#[derive(Clone)]
pub struct CodePtr<'a>(fn(&'a ObjcObject<'a>, Ptr<ObjcSelector>) -> Option<&'a ObjcObject<'a>>);

impl<'a> CodePtr<'a> {
    pub fn null_function() -> CodePtr<'a> {
        CodePtr(
            |_id: &'a ObjcObject<'a>, _sel: Ptr<ObjcSelector>| -> Option<&'a ObjcObject<'a>> {
                None
            },
        )
    }
}

impl<'a> fmt::Debug for CodePtr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CodePtr [ {:p} ]", self.0 as *const ())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjcMethod<'a> {
    method_name: Ptr<ObjcSelector>,
    method_type: StrPtr,
    method_imp: CodePtr<'a>,
}

impl<'a> ObjcMethod<'a> {
    pub fn get_name(&self) -> &Ptr<ObjcSelector> {
        &self.method_name
    }

    pub fn get_imp(&self) -> &CodePtr<'a> {
        &self.method_imp
    }
}

impl<'a> fmt::Display for ObjcMethod<'a> {
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

pub struct ObjcMethodIterator<'a> {
    current_list: Option<&'a ObjcMethodList<'a>>,
    index: usize,
}

impl<'a> Iterator for ObjcMethodIterator<'a> {
    type Item = &'a ObjcMethod<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let list = self.current_list?;
        let index = self.index;
        if index >= list.method_count() {
            self.current_list = list.get_next();
            self.index = 0;
            return self.next();
        }
        self.index += 1;
        list.nth_method(index)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjcMethodList<'a> {
    method_next: Option<&'a ObjcMethodList<'a>>,
    method_count: Int,
    method_list: [ObjcMethod<'a>; 0],
}

impl<'a> ObjcMethodList<'a> {
    fn get_next(&self) -> Option<&'a ObjcMethodList<'a>> {
        self.method_next
    }

    // ToDo: consider whether rename to get_count and its signature
    fn method_count(&self) -> usize {
        self.method_count as usize
    }

    fn nth_method(&self, i: usize) -> Option<&ObjcMethod> {
        let count = self.method_count();
        if i >= count {
            return None;
        }
        unsafe {
            let list = (self as *const ObjcMethodList).offset(1) as *const ObjcMethod;
            let list = slice::from_raw_parts(list, count);
            Some(&list[i])
        }
    }

    pub fn iter(&'a self) -> ObjcMethodIterator<'a> {
        ObjcMethodIterator {
            current_list: Some(self),
            index: 0,
        }
    }
}

impl<'a> fmt::Display for ObjcMethodList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "MethodList @ {:p} [ next: disabled, count: {}, list:",
            self, /*self.method_next,*/ self.method_count
        )?;
        for i in 0..self.method_count() {
            if let Some(method) = self.nth_method(i) {
                writeln!(f, "  ({}) {},", i, method)?;
            } else {
                unreachable!()
            }
        }
        write!(f, "]")
    }
}
