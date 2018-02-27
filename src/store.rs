
use std::collections::HashMap;

pub struct Store {
    packages: HashMap<String, bool>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            packages: HashMap::new(),
        }
    }

    /// Add a package to the store
    /// # Panics
    /// if the package is already inserted
    pub fn add_package(&mut self, name: &str) {
        match self.packages.get(name) {
            Some(_p) => panic!("Cannot add same package twice to the store"),
            None => (),
        };
        self.packages.insert(String::from(name), true);
    }

    pub fn get_package(&mut self, name: &str) -> bool {
        match self.packages.get(name) {
            Some(_p) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn cannot_add_package_twice() {
        let mut store = Store::new();

        store.add_package("a");
        store.add_package("a");
    }

    #[test]
    fn get_package() {
        let mut store = Store::new();
        store.add_package("a");
        assert_eq!(store.get_package("a"), true);
        assert_eq!(store.get_package("b"), false);
    }
}
