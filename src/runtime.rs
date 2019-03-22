use std::mem;

use super::context::CONTEXT;
use super::object::ObjcObject;
use super::str_ptr::StrPtr;
use super::{Bool, Class, Id, Method, Sel};

unsafe fn alloc(len: usize) -> *mut u8 {
    let word_size = mem::size_of::<usize>();
    let len = (len + word_size - 1) / word_size;
    let mut vec = Vec::<u8>::with_capacity(len);
    vec.set_len(len);
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}

/*
* Maybe, not called because the function is inlined.
*/
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn object_getClass(object: Id) -> Class {
    Class(object.0.map(|object| object.get_class_pointer()))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn sel_getName(_selector: Sel) -> StrPtr {
    unimplemented!()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn sel_getTypeEncoding(_selector: Sel) -> StrPtr {
    unimplemented!()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn sel_getUid<'a>(_name: StrPtr) -> Sel<'a> {
    unimplemented!()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_createInstance(class: Class, extra_bytes: usize) -> Id {
    Id(class.0.map(|class| {
        let p: &mut ObjcObject =
            unsafe { mem::transmute(alloc(class.get_instance_size() + extra_bytes)) };
        p.initialize(class);
        p as &ObjcObject
    }))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_getInstanceMethod<'a>(class: Class<'a>, selector: Sel) -> Method<'a> {
    let class = match class.0 {
        Some(class) => class,
        None => return Method(None),
    };
    let selector = match selector.0 {
        Some(selector) => selector,
        None => return Method(None),
    };
    Method(class.resolve_method(selector))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_getClassMethod<'a>(class: Class<'a>, selector: Sel) -> Method<'a> {
    let class = match class.0 {
        Some(class) => class,
        None => return Method(None),
    };
    let selector = match selector.0 {
        Some(selector) => selector,
        None => return Method(None),
    };
    Method(class.get_class_pointer().resolve_method(selector))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_getSuperclass(class: Class) -> Class {
    Class(class.0.and_then(|class| class.get_super_pointer()))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn object_dispose(_object: Id) -> Id {
    // ToDo: free the object
    Id(None)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_getName(class: Class) -> StrPtr {
    class
        .0
        .map(|class| class.get_name().clone())
        .unwrap_or(StrPtr::null())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_isMetaClass(class: Class) -> Bool {
    Bool::from(class.0.map_or(false, |class| class.is_meta()))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn objc_getClass(name: StrPtr) -> Class<'static> {
    objc_get_class(name)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn objc_get_class(name: StrPtr) -> Class<'static> {
    let ctx = CONTEXT.read().unwrap();
    match ctx.get_class_table().get(&name) {
        Some(entry) => Class(Some(entry.get_class())),
        None => Class(None),
    }
}
