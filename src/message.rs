use super::class::ObjcClass;
use super::method::CodePtr;
use super::object::ObjcObject;
use super::ptr::{NilablePtr, Ptr};
use super::{Id, Imp, Sel};

#[no_mangle]
pub extern "C" fn objc_msg_lookup(receiver: Id, selector: Sel) -> Imp {
    let codeptr = match (receiver.0.as_ref(), selector.0.as_ref()) {
        (Some(object), Some(selector)) => {
            let class = object.get_class_pointer();
            class
                .resolve_method(selector.clone())
                .map_or(CodePtr::null_function(), |method| method.imp().clone())
        }
        _ => CodePtr::null_function(),
    };
    Imp(codeptr)
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjcSuper {
    self_obj: NilablePtr<ObjcObject>,
    super_class: Ptr<ObjcClass>,
}

#[no_mangle]
pub extern "C" fn objc_msg_lookup_super(super_data: Ptr<ObjcSuper>, selector: Sel) -> Imp {
    let selector = match selector.0.as_ref() {
        Some(selector) => selector.clone(),
        None => {
            return Imp(CodePtr::null_function());
        }
    };
    Imp(super_data
        .super_class
        .resolve_method(selector)
        .map_or(CodePtr::null_function(), |method| method.imp().clone()))
}
