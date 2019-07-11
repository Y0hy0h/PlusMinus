extern crate plusminus;
use plusminus::{Expense, MemoryStore, Store};

fn main() {
    let simple_expense = Expense { amount: 1250 };

    let mut store = MemoryStore::empty();
    let id = store.save(simple_expense);

    assert_eq!(Some(&simple_expense), store.read(&id));
}
