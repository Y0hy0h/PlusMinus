use std::collections::HashMap;

use crate::expense::{Currency, Expense};

pub trait Store<'a, Item> {
    type Id;
    type Filter;

    fn index(&'a self, filter: Self::Filter) -> Cursor<'a, Self::Id, Item>;

    fn add(&mut self, item: Item) -> Self::Id;

    fn read(&self, id: &Self::Id) -> &Item;
}

#[derive(Clone)]
pub struct Cursor<'a, Id, Item>(Vec<(&'a Id, &'a Item)>);

impl<'a, Id, Item> Cursor<'a, Id, Item> {
    pub fn new(items: Vec<(&'a Id, &'a Item)>) -> Self {
        Cursor(items)
    }

    pub fn iter(&'a self) -> impl Iterator<Item = (&Id, &'a Item)> {
        self.0.iter().map(|tuple| *tuple)
    }
}

impl<'a, Id> Cursor<'a, Id, Expense<Id>> {
    pub fn sum(&self) -> Currency {
        self.0.iter().map(|(_, expense)| expense.amount).sum()
    }
}

pub type Id = usize;

pub struct MemoryStore<Item> {
    next_id: Id,
    items: HashMap<Id, Item>,
}

impl<Item> MemoryStore<Item> {
    pub fn new() -> Self {
        MemoryStore {
            next_id: 0,
            items: HashMap::new(),
        }
    }
}

impl<'a, Item> Store<'a, Item> for MemoryStore<Item> {
    type Id = Id;
    type Filter = Filter<Self::Id, Item>;

    fn index(&'a self, filter: Self::Filter) -> Cursor<'a, Self::Id, Item> {
        Cursor::new(
            self.items
                .iter()
                .filter(|(id, item)| filter((id, item)))
                .collect(),
        )
    }

    fn add(&mut self, item: Item) -> Self::Id {
        let id = self.next_id;
        self.next_id += 1;

        self.items.insert(id, item);

        id
    }

    fn read(&self, id: &Self::Id) -> &Item {
        self.items.get(id).expect("Id was invalid")
    }
}

type Filter<Id, Item> = Box<dyn Fn((&Id, &Item)) -> bool>;

pub struct OrderedStore<'a, Item, S>
where
    S: Store<'a, Item>,
    Item: StoreOrder,
{
    store: S,
    order: Vec<S::Id>,
}

pub trait StoreOrder {
    fn cmp(left: &Self, right: &Self) -> std::cmp::Ordering;
}

impl<T> StoreOrder for T
where
    T: Ord,
{
    fn cmp(left: &Self, right: &Self) -> std::cmp::Ordering {
        left.cmp(right)
    }
}

impl<Id> StoreOrder for Expense<Id> {
    fn cmp(left: &Self, right: &Self) -> std::cmp::Ordering {
        // Reverse chronological order
        right.date.cmp(&left.date)
    }
}

impl<'a, Item> OrderedStore<'a, Item, MemoryStore<Item>>
where
    Item: StoreOrder,
{
    pub fn new() -> Self {
        OrderedStore {
            store: MemoryStore::new(),
            order: Vec::new(),
        }
    }
}

impl<'a, Item, S> Store<'a, Item> for OrderedStore<'a, Item, S>
where
    S: Store<'a, Item>,
    S::Id: Copy,
    Item: StoreOrder,
{
    type Id = S::Id;
    type Filter = Filter<Self::Id, Item>;

    fn index(&'a self, filter: Self::Filter) -> Cursor<'a, Self::Id, Item> {
        Cursor::new(
            self.order
                .iter()
                .filter_map(|id| {
                    let item = self.store.read(id);
                    if filter((&id, &item)) {
                        Some((id, item))
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }

    fn add(&mut self, item: Item) -> Self::Id {
        let index = self
            .order
            .iter()
            .position(|id| {
                StoreOrder::cmp(self.store.read(id), &item) == std::cmp::Ordering::Greater
            })
            .unwrap_or(0);

        let id = self.store.add(item);
        self.order.insert(index, id);

        id
    }

    fn read(&self, id: &Self::Id) -> &Item {
        self.store.read(id)
    }
}
