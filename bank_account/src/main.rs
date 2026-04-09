struct BankAccount {
    owner: String,
    balance: f32,
}

impl BankAccount {
    fn view_balance(&self) {
        println!("Your balance is, {}", self.balance)
    }

    fn deposit(&mut self, amount: f32) {
        self.balance += amount;
        println!("Your deposit of {} was made succesfully", amount)
    }
}
fn main() {
    let mut my_account = BankAccount {
        owner: String::from("Alpy"),
        balance: 100.0,
    };
    {
        let current_balance = &my_account.balance;
        println!("The old balance was: {}", current_balance);
    }
    my_account.deposit(50.0);
}
