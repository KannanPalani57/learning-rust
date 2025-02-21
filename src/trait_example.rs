struct SavingsAccount {
    holder_name: String,
    balance: i32,
}

trait BankAccount {
    fn account_details(&self) -> String;
}

impl SavingsAccount {
    fn new(holder_name: String, balance: i32) -> SavingsAccount {
        SavingsAccount {
            holder_name,
            balance,
        }
    }
}

impl BankAccount for SavingsAccount {
    fn account_details(&self) -> String {
        return format!(
            "The Account Type is {}, account holder name is {}, account balance is {}",
            "Savings Account", self.holder_name, self.balance
        );
    }
}

struct CurrentAccount {
    holder_name: String,
    balance: i32,
    overdraft_limit: i32,
}

impl CurrentAccount {
    fn new(holder_name: String, balance: i32, overdraft_limit: i32) -> CurrentAccount {
        CurrentAccount {
            holder_name,
            balance,
            overdraft_limit,
        }
    }
}

impl BankAccount for CurrentAccount {
    fn account_details(&self) -> String {
        return format!("The Account Type is {}, account holder name is {}, account balance is {}, overdraft limit is {}", "Current Account", self.holder_name, self.balance, self.overdraft_limit);
    }
}

fn main() {
    let john_savings_account = SavingsAccount::new (
         String::from("John"),
     50,
    );

    let john_acc_details = john_savings_account.account_details();

    println!("{}", john_acc_details);

    let dan_savings_account = CurrentAccount::new (
         String::from("Dan"),
         500,
         100,
    );

    let dan_acc_details = dan_savings_account.account_details();

    println!("{}", dan_acc_details);
}
