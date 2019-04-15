use std::collections::HashMap;
use std::mem;
use std::sync;

use super::category::ObjcCategory;
use super::class::ObjcClass;
use super::method::ObjcMethod;
use super::module::ObjcModule;
use super::ptr::Ptr;
use super::selector::ObjcSelector;
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
    unresolved_methods: Vec<(Ptr<ObjcClass>, Ptr<ObjcMethod>)>,
    _unresolved_categories: Vec<Ptr<ObjcCategory>>,
}

impl Context {
    fn new() -> Context {
        Context {
            class_table: HashMap::new(),
            orphan_classes: Vec::new(),
            unresolved_methods: Vec::new(),
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

    pub fn append_unresolved_methods(&mut self, class: Ptr<ObjcClass>, method: Ptr<ObjcMethod>) {
        self.unresolved_methods.push((class, method));
    }

    fn resolve_orphan_classes(&mut self) {
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

    fn link_selectors_to_methods(&mut self, module: &mut ObjcModule) {
        let selector_map = module.symtab().create_selector_map();
        let mut num_unresolved_methods = self.unresolved_methods.len();
        loop {
            let mut unresolved_methods = Vec::new();
            mem::swap(&mut self.unresolved_methods, &mut unresolved_methods);

            for (mut class, mut method) in unresolved_methods {
                let name =
                    unsafe { mem::transmute::<Ptr<ObjcSelector>, StrPtr>(method.name().clone()) };
                let _types = method.types().clone();
                /*
                 * // ToDo: currently, ignore types
                 * if let Some(selector) = selector_map.get(&(name, _types)) {
                 */
                if let Some(selector) = selector_map.get(&name) {
                    unsafe {
                        method.link_to_selector(selector.clone());
                    }
                    class.register_method(selector.clone(), method);
                } else {
                    self.unresolved_methods.push((class, method))
                }
            }

            let new_num_unresolved_methods = self.unresolved_methods.len();
            if new_num_unresolved_methods == num_unresolved_methods {
                break;
            }
            num_unresolved_methods = new_num_unresolved_methods;
        }
    }

    pub fn load_module(&mut self, module: &mut ObjcModule) {
        let symtab = module.symtab_mut();
        for mut class in symtab.iter_class() {
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

        for mut category in symtab.iter_category() {
            category.as_mut().initialize(self);
            category.as_mut().defer_resolving_methods(self);
        }

        self.resolve_orphan_classes();
        self.link_selectors_to_methods(module);
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

lazy_static! {
    pub static ref CONTEXT: sync::RwLock<Context> = sync::RwLock::new(Context::new());
}
