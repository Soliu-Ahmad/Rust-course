// STRUCT: structs are custom data types that let you name and package together multiple related values that make up a meaningful group. They are similar to classes in other programming languages but do not have methods associated with them.
// Structs are defined using the struct keyword, followed by the struct name and a block containing the fields of the struct.
// Structs can have named fields, tuple-like fields, or be unit-like (without any fields).
// Structs are useful for creating complex data types that group related data together, making code more organized and easier to understand.
// Example of defining and using a struct:


fn main() {
    let mut account = BankAccount {
        owner: "Focus".to_string(),
        balance: 1000.0,
    };

    // imutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraw(150.0);

    // imutable borrow to check the balance after withdrawal
    account.check_balance();

    println!("Account owner: {}", account.owner);
    println!("Account balance: {}", account.balance);

    account.withdraw(200.0);
    println!("New balance after withdrawal: {}", account.balance);
}
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64) {
      println!("Withdrawing {} from  account owned by {}.", amount, self.owner);
      self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("The balance for account owned by {} is {}.", self.owner, self.balance);
    }
}