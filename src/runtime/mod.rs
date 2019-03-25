use std::collections::HashMap;
use runtime::class::RuntimeClass;

pub mod class;
pub mod gc;
pub mod jit;
pub mod interpreter;

//pub struct RuntimeContext {
//    loaded_class_table: HashMap<String, RuntimeClass>
//}
//
//impl RuntimeContext {
//
//    pub fn load_class(&mut self, class: RuntimeClass) {
//        self.loaded_class_table.insert(class.class_name, class);
//
//        ()
//    }
//
//}
