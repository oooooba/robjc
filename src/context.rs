use std::collections::HashMap;
use std::mem;
use std::sync;

use super::category::ObjcCategory;
use super::class::ObjcClass;
use super::module::ObjcModule;
use super::str_ptr::StrPtr;

pub struct ClassTableEntry<'a> {
    class: &'a ObjcClass<'a>,
    meta_class: &'a ObjcClass<'a>,
}

impl<'a> ClassTableEntry<'a> {
    fn new(class: &'a ObjcClass<'a>, meta_class: &'a ObjcClass<'a>) -> ClassTableEntry<'a> {
        ClassTableEntry { class, meta_class }
    }

    pub fn get_class<'b>(&self) -> &'b ObjcClass<'a> {
        self.class
    }

    pub fn get_meta_class<'b>(&self) -> &'b ObjcClass<'a> {
        self.meta_class
    }
}

pub struct Context<'a> {
    class_table: HashMap<StrPtr, ClassTableEntry<'a>>,
    orphan_classes: Vec<&'a mut ObjcClass<'a>>,
    _unresolved_categories: Vec<&'a mut ObjcCategory<'a>>,
}

impl<'a> Context<'a> {
    fn new() -> Context<'a> {
        Context {
            class_table: HashMap::new(),
            orphan_classes: Vec::new(),
            _unresolved_categories: Vec::new(),
        }
    }

    pub fn get_class_table(&self) -> &HashMap<StrPtr, ClassTableEntry<'a>> {
        &self.class_table
    }

    pub fn get_orphan_classes(&self) -> &Vec<&'a mut ObjcClass<'a>> {
        &self.orphan_classes
    }

    pub fn load_module(&mut self, module: &'a ObjcModule) {
        let symtab = module.get_symtab();
        for i in 0..symtab.cls_def_cnt() {
            let class = symtab.nth_class_ptr_mut(i).unwrap();
            class.initialize(self);
            unsafe { class.get_mut_class_pointer() }.initialize(self);
            let name = class.get_name().clone();
            let entry = ClassTableEntry::new(class, class.get_class_pointer());
            self.class_table.insert(name, entry);
        }

        // rewrite super pointer of class
        for i in 0..symtab.cls_def_cnt() {
            let class = symtab.nth_class_ptr_mut(i).unwrap();
            if !class.initialize_super_pointer(self) {
                self.orphan_classes.push(class);
            }
        }

        // rewrite super pointer of meta class
        for i in 0..symtab.cls_def_cnt() {
            let class = symtab.nth_class_ptr_mut(i).unwrap();
            let meta_class = unsafe { class.get_mut_class_pointer() };
            if !meta_class.initialize_super_pointer(self) {
                self.orphan_classes.push(meta_class);
            }
        }

        // registry categories
        for i in 0..symtab.cat_def_cnt() {
            let category = symtab.nth_category_ptr(i).unwrap();
            println!("{:?}", category);
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
