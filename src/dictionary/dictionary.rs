use std::collections::HashMap;
use std::iter::Map;
use std::mem::type_info::Str;
use std::string;

struct Dictionary {
    map: HashMap<String, String>
}

impl Dictionary {
    fn set_item(&self, key: &str, value: &str) {
        self.map.insert(key, value);
    }

    fn get_item(&self, key: &str) -> Option<String> {
        return self.map.get(key);
    }

    fn del_item(&self, key: &str) {
        self.map.remove(key);
    }
}
