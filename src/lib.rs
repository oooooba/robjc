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
mod ptr;
pub mod runtime;
mod selector;
mod str_ptr;

use std::convert;
use std::os::raw;

use class::ObjcClass;
use context::CONTEXT;
use ivar::ObjcIvar;
use method::{ObjcMethod, Procedure};
use module::ObjcModule;
use object::ObjcObject;
use ptr::{NilablePtr, Ptr};
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
#[derive(Debug)]
pub struct Class(NilablePtr<ObjcClass>);

#[repr(transparent)]
#[derive(Debug)]
pub struct Id(NilablePtr<ObjcObject>);

#[repr(transparent)]
#[derive(Debug)]
pub struct Sel(NilablePtr<ObjcSelector>);

#[repr(transparent)]
#[derive(Debug)]
pub struct Imp(NilablePtr<Procedure>);

#[repr(transparent)]
#[derive(Debug)]
pub struct Method(NilablePtr<ObjcMethod>);

#[repr(transparent)]
#[derive(Debug)]
pub struct Module(Ptr<ObjcModule>);

#[no_mangle]
pub extern "C" fn __objc_exec_class(module: &'static mut ObjcModule) {
    let mut ctx = CONTEXT.write().unwrap();
    ctx.load_module(module);
}

#[cfg(test)]
mod tests {
    use super::ptr::NilablePtr;
    use super::str_ptr::StrPtr;
    use super::{Class, Id, Imp, Ivar, Method, Module, Ptr, Sel};
    use std::mem;

    #[test]
    fn object_size() {
        assert_eq!(mem::size_of::<Ptr<()>>(), mem::size_of::<usize>());
        assert_eq!(mem::size_of::<NilablePtr<()>>(), mem::size_of::<usize>());

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
