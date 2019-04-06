use super::class::ObjcClass2;
use super::ptr::Ptr;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcObject {
    class_pointer: Ptr<ObjcClass2>,
}

impl ObjcObject {
    pub fn get_class_pointer(&self) -> &Ptr<ObjcClass2> {
        &self.class_pointer
    }

    pub fn initialize(&mut self, class: Ptr<ObjcClass2>) {
        self.class_pointer = class;
    }
}
