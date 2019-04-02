use std::mem;
use std::ptr;

use super::context::CONTEXT;
use super::object::ObjcObject;
use super::str_ptr::StrPtr;
use super::{Bool, Class, Id, Method, Sel};

unsafe fn alloc(len: usize) -> (*mut u8, usize) {
    let word_size = mem::size_of::<usize>();
    let num_words = (len + word_size - 1) / word_size;
    let mut vec = Vec::<usize>::with_capacity(num_words);
    vec.set_len(num_words);
    (Box::into_raw(vec.into_boxed_slice()) as *mut u8, num_words)
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
pub extern "C" fn sel_getUid(_name: StrPtr) -> Sel {
    unimplemented!()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_createInstance(class: Class, extra_bytes: usize) -> Id {
    Id(class.0.map(|class| {
        let p: &mut ObjcObject = unsafe {
            let (p, num_words) = alloc(class.get_instance_size() + extra_bytes);
            ptr::write_bytes(p, 0, mem::size_of::<usize>() * num_words);
            mem::transmute(p)
        };
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
    let selector = match (selector.0).0 {
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
    let selector = match (selector.0).0 {
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
    Class(ctx.get_class_entry(&name).map(|entry| entry.get_class()))
}
