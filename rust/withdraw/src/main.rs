#[no_mangle]
pub extern "C" fn withdraw(acct_balance: f32, withdraw_amt: f32) -> f32 {
    match withdraw_amt < acct_balance {
        true => acct_balance - withdraw_amt,
        false => acct_balance,
    }
}

fn main() {

}