use super::class::ObjcClass;

#[repr(C)]
#[derive(Debug)]
pub struct ObjcObject<'a> {
    class_pointer: &'a ObjcClass<'a>,
}

impl<'a> ObjcObject<'a> {
    pub fn get_class_pointer(&self) -> &ObjcClass<'a> {
        self.class_pointer
    }

    pub fn initialize(&mut self, class: &'a ObjcClass<'a>) {
        self.class_pointer = class;
    }
}
