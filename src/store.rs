use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

use crate::expense::Expense;

pub trait Store {
    type Id;
    fn all(&self) -> Vec<&Expense>;
    fn save(&mut self, expense: Expense) -> Self::Id;
    fn read(&self, id: &Self::Id) -> Option<&Expense>;
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Id<Item>(Uuid, PhantomData<Item>);

impl<Item> Id<Item> {
    fn new() -> Id<Item> {
        Id(Uuid::new_v4(), PhantomData)
    }
}

pub struct MemoryStore {
    events: HashMap<Id<Expense>, Expense>,
}

impl MemoryStore {
    pub fn empty() -> MemoryStore {
        MemoryStore {
            events: HashMap::new(),
        }
    }
}

impl Store for MemoryStore {
    type Id = Id<Expense>;
    fn all(&self) -> Vec<&Expense> {
        self.events.values().collect()
    }
    fn save(&mut self, expense: Expense) -> Self::Id {
        let id = Id::new();
        self.events.insert(id, expense);

        id
    }
    fn read(&self, id: &Self::Id) -> Option<&Expense> {
        self.events.get(id)
    }
}
