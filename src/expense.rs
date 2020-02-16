pub use chrono::NaiveDate as Date;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Expense {
    pub amount: Currency,
    pub date: Date,
}

pub type Currency = u32;
