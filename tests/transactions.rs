extern crate plusminus;
use plusminus::{Date, Expense, MemoryStore, Store};

#[test]
fn general() {
    let mut store = MemoryStore::empty();

    let expense1 = Expense {
        amount: 1250,
        date: Date::from_ymd(2020, 2, 1),
    };
    let id1 = store.add(expense1);
    assert_eq!(&expense1, store.read(id1));

    let expense2 = Expense {
        amount: 2000,
        date: Date::from_ymd(2020, 2, 2),
    };
    let id2 = store.add(expense2);
    assert_eq!(&expense2, store.read(id2));

    let all = store.index();

    // Chronological order expected.
    let expected = vec![&expense2, &expense1];
    itertools::assert_equal(expected, all.iter());

    assert_eq!(3250, all.sum())
}
