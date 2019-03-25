#[macro_use]
extern crate lazy_static;

mod category;
mod class;
mod context;
mod ivar;
mod message;
mod method;
mod module;
mod object;
pub mod runtime;
mod selector;
mod str_ptr;

use std::convert;
use std::os::raw;
use std::ptr;

use class::ObjcClass;
use context::{ClassHandle, ObjectHandle, CONTEXT};
use ivar::ObjcIvar;
use method::{CodePtr, ObjcMethod};
use module::ObjcModule;
use object::ObjcObject;
use selector::ObjcSelector;

type UShort = i16;
type Int = i32;
type Long = i64;
type ULong = u64;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Bool(raw::c_uchar);

impl convert::From<bool> for Bool {
    fn from(b: bool) -> Self {
        if b {
            YES
        } else {
            NO
        }
    }
}
pub const YES: Bool = Bool(1u8);
pub const NO: Bool = Bool(0u8);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Ivar<'a>(Option<&'a ObjcIvar>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Class<'a>(Option<&'a ObjcClass<'a>>);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Class2(Option<ClassHandle>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Id<'a>(Option<&'a ObjcObject<'a>>);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Id2(Option<ObjectHandle>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Sel<'a>(Option<&'a ObjcSelector>);

#[repr(transparent)]
#[derive(Clone)]
pub struct Imp<'a>(CodePtr<'a>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Method<'a>(Option<&'a ObjcMethod<'a>>);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Module<'a>(ptr::NonNull<ObjcModule<'a>>);

#[no_mangle]
pub extern "C" fn __objc_exec_class(module: &'static ObjcModule) {
    let mut ctx = CONTEXT.write().unwrap();
    ctx.load_module(module);
}

#[cfg(test)]
mod tests {
    use super::str_ptr::StrPtr;
    use super::{Class, Id, Imp, Ivar, Method, Module, Sel};
    use std::mem;

    #[test]
    fn object_size() {
        assert_eq!(mem::size_of::<Ivar>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Id>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Class>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Sel>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Imp>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Method>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<Module>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<StrPtr>(), mem::size_of::<usize>());
    }
}
