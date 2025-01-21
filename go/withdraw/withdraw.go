package main

//export withdraw
func withdraw(acct_balance float32, withdraw_amt float32) float32 {
    if( withdraw_amt < acct_balance ) {
        return acct_balance - withdraw_amt;
    } else {
        return acct_balance;
    }
}

func main() {
}