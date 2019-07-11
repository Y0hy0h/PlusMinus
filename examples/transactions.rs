use chrono::prelude::*;

extern crate plusminus;
use plusminus::{Expense, MemoryStore, Store};

fn main() {
    let mut store = MemoryStore::empty();

    let expense1 = Expense {
        amount: 1250,
        time: Utc::now(),
    };
    let id1 = store.save(expense1);
    assert_eq!(Some(&expense1), store.read(&id1));

    let expense2 = Expense {
        amount: 2000,
        time: Utc::now(),
    };
    let id2 = store.save(expense2);
    assert_eq!(Some(&expense2), store.read(&id2));

    for (actual, expected) in store.all().iter().zip(vec![expense2, expense1].iter()) {
        assert_eq!(&expected, actual);
    }
}