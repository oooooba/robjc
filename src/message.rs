use super::class::ObjcClass;
use super::method::CodePtr;
use super::object::ObjcObject;
use super::{Id, Imp, Sel2};

#[no_mangle]
pub extern "C" fn objc_msg_lookup<'a>(receiver: Id<'a>, selector: Sel2) -> Imp<'a> {
    match receiver.0 {
        Some(object) => {
            let class = object.get_class_pointer();
            let selector = match (selector.0).0 {
                Some(selector) => selector,
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
pub struct ObjcSuper<'a> {
    self_obj: Option<&'a ObjcObject<'a>>,
    super_class: &'a ObjcClass<'a>,
}

#[no_mangle]
pub extern "C" fn objc_msg_lookup_super<'a>(super_data: &ObjcSuper<'a>, selector: Sel2) -> Imp<'a> {
    let selector = match (selector.0).0 {
        Some(selector) => selector,
        None => {
            return Imp(CodePtr::null_function());
        }
    };
    Imp(super_data
        .super_class
        .resolve_method(selector)
        .map_or(CodePtr::null_function(), |method| method.get_imp().clone()))
}
