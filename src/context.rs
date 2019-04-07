use std::collections::HashMap;
use std::mem;
use std::sync;

use super::category::ObjcCategory;
use super::class::ObjcClass;
use super::module::ObjcModule;
use super::ptr::Ptr;
use super::str_ptr::StrPtr;

pub struct ClassTableEntry {
    class: Ptr<ObjcClass>,
    meta_class: Ptr<ObjcClass>,
}

impl ClassTableEntry {
    fn new(class: Ptr<ObjcClass>, meta_class: Ptr<ObjcClass>) -> ClassTableEntry {
        ClassTableEntry { class, meta_class }
    }

    pub fn class(&self) -> &Ptr<ObjcClass> {
        &self.class
    }

    pub fn meta_class(&self) -> &Ptr<ObjcClass> {
        &self.meta_class
    }
}

pub struct Context {
    class_table: HashMap<StrPtr, ClassTableEntry>,
    orphan_classes: Vec<Ptr<ObjcClass>>,
    _unresolved_categories: Vec<Ptr<ObjcCategory>>,
}

impl Context {
    fn new() -> Context {
        Context {
            class_table: HashMap::new(),
            orphan_classes: Vec::new(),
            _unresolved_categories: Vec::new(),
        }
    }

    pub fn get_class_entry(&self, name: &StrPtr) -> Option<&ClassTableEntry> {
        self.class_table.get(name)
    }

    fn register_class_pair(&mut self, class: Ptr<ObjcClass>) {
        assert!(class.is_class());
        let meta_class = class.class_pointer().clone();
        let name = class.get_name().clone();
        let entry = ClassTableEntry::new(class, meta_class);
        self.class_table.insert(name, entry);
    }

    pub fn load_module(&mut self, module: &mut ObjcModule) {
        let symtab = module.symtab_mut();
        for i in 0..symtab.cls_def_cnt() {
            let class = symtab.nth_class_ptr_mut(i).unwrap();
            class.initialize(self);
            if !class.initialize_super_pointer(self) {
                self.orphan_classes.push(class.clone());
            }

            let meta_class = class.class_pointer_mut();
            meta_class.initialize(self);
            if !meta_class.initialize_super_pointer(self) {
                self.orphan_classes.push(meta_class.clone());
            }

            self.register_class_pair(class.clone());
        }

        let mut num_orphan_classes = self.orphan_classes.len();
        loop {
            let mut orphan_classes = Vec::new();
            mem::swap(&mut self.orphan_classes, &mut orphan_classes);
            for mut class in orphan_classes {
                if !class.as_mut().initialize_super_pointer(self) {
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

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

lazy_static! {
    pub static ref CONTEXT: sync::RwLock<Context> = sync::RwLock::new(Context::new());
}
