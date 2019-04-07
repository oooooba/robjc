use std::mem;
use std::ptr;

use super::context::CONTEXT;
use super::object::ObjcObject;
use super::ptr::{NilablePtr, Ptr};
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
    Class(NilablePtr::from(
        object
            .0
            .as_ref()
            .map(|object| object.get_class_pointer().clone()),
    ))
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
    Id(NilablePtr::from(class.0.as_ref().map(|class| {
        let p: &mut ObjcObject = unsafe {
            let (p, num_words) = alloc(class.get_instance_size() + extra_bytes);
            ptr::write_bytes(p, 0, mem::size_of::<usize>() * num_words);
            mem::transmute(p)
        };
        p.initialize(class.clone());
        unsafe { Ptr::new(p) }
    })))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_getInstanceMethod(class: Class, selector: Sel) -> Method {
    let class = match class.0.as_ref() {
        Some(class) => class,
        None => return Method(NilablePtr::nil()),
    };
    let selector = match selector.0.as_ref() {
        Some(selector) => selector.clone(),
        None => return Method(NilablePtr::nil()),
    };
    Method(NilablePtr::from(class.resolve_method(selector)))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_getClassMethod(class: Class, selector: Sel) -> Method {
    let class = match class.0.as_ref() {
        Some(class) => class,
        None => return Method(NilablePtr::nil()),
    };
    let selector = match selector.0.as_ref() {
        Some(selector) => selector.clone(),
        None => return Method(NilablePtr::nil()),
    };
    Method(NilablePtr::from(
        class.class_pointer().resolve_method(selector),
    ))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_getSuperclass(class: Class) -> Class {
    Class(NilablePtr::from(
        class
            .0
            .as_ref()
            .and_then(|class| class.super_pointer().clone()),
    ))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn object_dispose(_object: Id) -> Id {
    // ToDo: free the object
    Id(NilablePtr::nil())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_getName(class: Class) -> StrPtr {
    class
        .0
        .as_ref()
        .map(|class| class.get_name().clone())
        .unwrap_or(StrPtr::null())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn class_isMetaClass(class: Class) -> Bool {
    Bool::from(class.0.as_ref().map_or(false, |class| class.is_meta()))
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn objc_getClass(name: StrPtr) -> Class {
    objc_get_class(name)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn objc_get_class(name: StrPtr) -> Class {
    let ctx = CONTEXT.read().unwrap();
    Class(NilablePtr::from(
        ctx.get_class_entry(&name)
            .map(|entry| entry.class().clone()),
    ))
}
