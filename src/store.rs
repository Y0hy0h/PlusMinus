use slotmap::SlotMap;

use crate::expense::{Currency, Expense};

pub trait Store<'a> {
    type Id: Copy;

    /// Returns an iterator over all stored `Expense`s
    /// in reverse chronological order.
    fn index(&'a self) -> Cursor<'a>;

    fn add(&mut self, expense: Expense) -> Self::Id;

    fn read(&self, id: Self::Id) -> &Expense;
}

#[derive(Clone)]
pub struct Cursor<'a>(Vec<&'a Expense>);

impl<'a> Cursor<'a> {
    pub fn iter(&'a self) -> impl Iterator<Item = &'a Expense> {
        self.0.iter().map(|expense| *expense)
    }

    pub fn sum(self) -> Currency {
        self.0.into_iter().map(|expense| expense.amount).sum()
    }
}

type Id = slotmap::DefaultKey;

pub struct MemoryStore {
    expenses: SlotMap<Id, Expense>,
    order: Vec<Id>,
}

impl MemoryStore {
    pub fn empty() -> MemoryStore {
        MemoryStore {
            expenses: SlotMap::new(),
            order: Vec::new(),
        }
    }
}

impl<'a> Store<'a> for MemoryStore {
    type Id = Id;

    fn index(&'a self) -> Cursor<'a> {
        Cursor(self.order.iter().map(|id| &self.expenses[*id]).collect())
    }

    fn add(&mut self, expense: Expense) -> Self::Id {
        let id = self.expenses.insert(expense);

        // Insertion sort
        let insertion_index = self
            .order
            .iter()
            .position(|id| self.expenses[*id].date < expense.date)
            .unwrap_or(0);
        self.order.insert(insertion_index, id);

        id
    }

    fn read(&self, id: Self::Id) -> &Expense {
        self.expenses.get(id).expect("Id was invalid.")
    }
}
