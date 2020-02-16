extern crate plusminus;
use plusminus::expense::{Category, Date, Expense};
use plusminus::store::{Cursor, Id, MemoryStore, OrderedStore, Store};

#[test]
fn general() {
    // Categories

    let mut categories: MemoryStore<Category> = MemoryStore::new();

    let rent = Category {
        name: "Rent".into(),
    };
    let rent_id = categories.add(rent.clone());

    let restaurant = Category {
        name: "Restaurant".into(),
    };
    let restaurant_id = categories.add(restaurant.clone());

    // Expenses

    let mut expenses: OrderedStore<'_, Expense<Id>, MemoryStore<Expense<Id>>> = OrderedStore::new();

    let rent_expense = Expense {
        amount: 1250,
        date: Date::from_ymd(2020, 2, 1),
        category: rent_id,
    };
    let _rent_expense_id = expenses.add(rent_expense.clone());

    let restaurant_expense = Expense {
        amount: 2000,
        date: Date::from_ymd(2020, 2, 2),
        category: restaurant_id,
    };
    let restaurant_expense_id = expenses.add(restaurant_expense.clone());

    // Assertions
    assert_eq!(&rent, categories.read(&rent_id));
    let fetched_restaurant_expense = expenses.read(&restaurant_expense_id);
    assert_eq!(&restaurant_expense, fetched_restaurant_expense);
    assert_eq!(
        &restaurant,
        categories.read(&fetched_restaurant_expense.category)
    );

    let all_expenses: Cursor<Id, Expense<Id>> = expenses.index();
    // Chronological order expected.
    let expected = vec![&restaurant_expense, &rent_expense];
    itertools::assert_equal(
        expected.iter().as_ref(),
        all_expenses
            .iter()
            .map(|(_, expense)| expense)
            .collect::<Vec<&Expense<Id>>>()
            .iter(),
    );

    assert_eq!(3250, all_expenses.sum())
}
