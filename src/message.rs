use super::class::ObjcClass;
use super::method::CodePtr;
use super::object::ObjcObject;
use super::ptr::{NilablePtr, Ptr};
use super::{Id, Imp, Sel};

#[no_mangle]
pub extern "C" fn objc_msg_lookup(receiver: Id, selector: Sel) -> Imp {
    match receiver.0.as_ref() {
        Some(object) => {
            let class = object.get_class_pointer();
            let selector = match selector.0.as_ref() {
                Some(selector) => selector.clone(),
                None => {
                    return Imp(CodePtr::null_function());
                }
            };
            Imp(class
                .resolve_method(selector)
                .map_or(CodePtr::null_function(), |method| method.get_imp().clone()))
        }
        None => Imp(CodePtr::null_function()),
    }
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
        .map_or(CodePtr::null_function(), |method| method.get_imp().clone()))
}
