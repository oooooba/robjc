use std::collections::HashMap;
use std::mem;
use std::sync;

use super::category::ObjcCategory;
use super::class::ObjcClass;
use super::module::ObjcModule;
use super::str_ptr::StrPtr;

pub struct ClassTableEntry<'a> {
    class: &'a ObjcClass,
    meta_class: &'a ObjcClass,
}

impl<'a> ClassTableEntry<'a> {
    fn new(class: &'a ObjcClass, meta_class: &'a ObjcClass) -> ClassTableEntry<'a> {
        ClassTableEntry { class, meta_class }
    }

    pub fn get_class(&self) -> &ObjcClass {
        &self.class
    }

    pub fn get_meta_class(&self) -> &ObjcClass {
        &self.meta_class
    }
}

pub struct Context<'a> {
    class_table: HashMap<StrPtr, ClassTableEntry<'a>>,
    orphan_classes: Vec<&'a mut ObjcClass>,
    _unresolved_categories: Vec<&'a mut ObjcCategory>,
}

impl<'a> Context<'a> {
    fn new() -> Context<'a> {
        Context {
            class_table: HashMap::new(),
            orphan_classes: Vec::new(),
            _unresolved_categories: Vec::new(),
        }
    }

    pub fn get_class_entry(&self, name: &StrPtr) -> Option<&ClassTableEntry<'a>> {
        self.class_table.get(name)
    }

    fn register_class_pair(&mut self, class: &'a ObjcClass) {
        assert!(class.is_class());
        let meta_class = class.class_pointer();
        let name = class.get_name().clone();
        let entry = ClassTableEntry::new(class, meta_class);
        self.class_table.insert(name, entry);
    }

    pub fn load_module(&mut self, module: &'a mut ObjcModule) {
        let symtab = module.symtab_mut();
        for i in 0..symtab.cls_def_cnt() {
            let class = symtab.nth_class_ptr_mut(i).unwrap();
            class.initialize(self);
            if !class.initialize_super_pointer(self) {
                self.orphan_classes.push(class);
            }

            let meta_class = symtab
                .nth_class_ptr_mut(i)
                .unwrap()
                .class_pointer_mut()
                .as_mut();
            meta_class.initialize(self);
            if !meta_class.initialize_super_pointer(self) {
                self.orphan_classes.push(meta_class);
            }

            let class = symtab.nth_class_ptr_mut(i).unwrap();
            self.register_class_pair(class);
        }

        let mut num_orphan_classes = self.orphan_classes.len();
        loop {
            let mut orphan_classes = Vec::new();
            mem::swap(&mut self.orphan_classes, &mut orphan_classes);
            for class in orphan_classes {
                if !class.initialize_super_pointer(self) {
                    self.orphan_classes.push(class);
                }
            }
            let new_num_orphan_classes = self.orphan_classes.len();
            if new_num_orphan_classes == num_orphan_classes {
                break;
            }
            num_orphan_classes = new_num_orphan_classes;
        }
    }
}

unsafe impl<'a> Send for Context<'a> {}
unsafe impl<'a> Sync for Context<'a> {}

lazy_static! {
    pub static ref CONTEXT: sync::RwLock<Context<'static>> = sync::RwLock::new(Context::new());
}
