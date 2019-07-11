#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)] // Many of the traits need to be derived, because they are not automatically derived correctly for PhantomData
pub struct Expense {
    pub amount: Currency,
}

pub type Currency = u32;
