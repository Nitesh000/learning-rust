// impl for enums.

pub enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}

// Bunch of dummy payment handlers.

pub fn pay_by_credit(amt: u64) {
    println!("Paid {} by credit card.", amt);
}

pub fn pay_by_debit(amt: u64) {
    println!("Paid {} by debit card.", amt);
}

pub fn pay_by_paypal(amt: u64) {
    println!("Paid {} by paypal.", amt);
}

pub impl PaymentMode {
    fn pay(&self, amt: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amt),
            PaymentMode::Credit => pay_by_credit(amt),
            PaymentMode::Paypal => pay_by_paypal(amt),
        }
    }
}

fn main() {
    let payment_mode = PaymentMode::Credit;
    payment_mode.pay(100); // Paid 100 by credit card.
}
