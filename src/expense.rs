pub use chrono::NaiveDate as Date;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Category {
    pub name: String,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Expense<Id> {
    pub amount: Currency,
    pub date: Date,
    pub category: Id,
}

pub type Currency = u32;
