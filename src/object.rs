use super::class::ObjcClass;
use super::ptr::Ptr;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcObject {
    class_pointer: Ptr<ObjcClass>,
}

impl ObjcObject {
    pub fn get_class_pointer(&self) -> &Ptr<ObjcClass> {
        &self.class_pointer
    }

    pub fn initialize(&mut self, class: Ptr<ObjcClass>) {
        self.class_pointer = class;
    }
}
