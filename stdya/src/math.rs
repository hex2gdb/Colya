pub trait CheckedBalance {
    fn safe_add(self, amount: u128) -> Option<u128>;
    fn safe_sub(self, amount: u128) -> Option<u128>;
}

impl CheckedBalance for u128 {
    fn safe_add(self, amount: u128) -> Option<u128> {
        self.checked_add(amount)
    }

    fn safe_sub(self, amount: u128) -> Option<u128> {
        self.checked_sub(amount)
    }
}
