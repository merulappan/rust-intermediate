struct BankAccount {
    balance: i32,
    verified: bool
}

fn print_balance(my_account : &BankAccount) {
    println!("{:?}", my_account.balance);
}

fn print_verified(my_account : &BankAccount) {
    println!("{:?}", my_account.verified);
}

fn is_verified(my_account : &BankAccount) -> Result<bool, bool> {
   return match my_account.verified {
        true => Ok(true),
        false => Err(false)
    };
}

fn main() {
    let my_account = BankAccount {
        balance :20,
        verified: true
    };
    let verification_status = is_verified(&my_account).unwrap();
    print_balance(&my_account);
    print_verified(&my_account);
    println!("{:?}",verification_status)
}
