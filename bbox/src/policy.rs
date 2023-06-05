use std::any::Any;
use std::collections::HashMap;

use crate::context::Context;

pub trait AbstractPolicy<T, U, D> {
    fn check(&self, data: T, context: Context<U>, db: D) -> bool;
}

pub struct PolicyStorage<'a, U, D> {
    pub storage: HashMap<(String, String), Vec<&'a dyn AbstractPolicy<dyn Any, U, D>>>,
}

impl<'a, U, D> PolicyStorage<'a, U, D> {
    pub fn get_policies<T>(&self, table: String, column: String) -> &Vec<&'a dyn AbstractPolicy<T, U, D>> {
        self.storage.get(&(table, column)).unwrap()
    }

    pub fn add_policy<T, P: AbstractPolicy<T, U, D>>(&mut self, table: String, column: String, policy: P) {
        self.storage.insert((table, column), policy);
    }
}
