float withdraw(float acct_balance, float withdraw_amt) {
    if( withdraw_amt < acct_balance ) {
        return acct_balance - withdraw_amt;
    } else {
        return acct_balance;
    }
}

int main() {
    return 0;
}