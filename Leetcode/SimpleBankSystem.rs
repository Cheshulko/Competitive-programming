// https://leetcode.com/problems/simple-bank-system

struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = account1 as usize - 1;
        let account2 = account2 as usize - 1;

        if account1 >= self.balance.len() || account2 >= self.balance.len() {
            return false;
        }

        if self.balance[account1] < money {
            return false;
        }

        self.balance[account1] -= money;
        self.balance[account2] += money;

        return true;
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize - 1;

        if account >= self.balance.len() {
            return false;
        }

        self.balance[account] += money;

        return true;
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize - 1;

        if account >= self.balance.len() || self.balance[account] < money {
            return false;
        }

        self.balance[account] -= money;

        return true;
    }
}
