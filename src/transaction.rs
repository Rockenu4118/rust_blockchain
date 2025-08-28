use std::fmt;

#[derive(Clone)]
pub struct Transaction {
    pub recipient: [u8; 33],
    pub sender: [u8; 33],
    pub amount: u64,
}

impl Transaction {
    pub fn new(recipient: [u8; 33], sender: [u8; 33], amount: u64) -> Self {
        Self {
            recipient,
            sender,
            amount,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Transaction")?;
        writeln!(f, "  Recipient: {}", hex::encode(self.recipient))?;
        writeln!(f, "  Sender:    {}", hex::encode(self.sender))?;
        writeln!(f, "  Amount:    {}", self.amount)
    }
}
