use rand::rngs::OsRng;
use secp256k1::Secp256k1;
use std::fmt;

pub struct Account {
    pub name: String,
    pub private_key: [u8; 32],
    pub public_key: [u8; 33],
}

impl Account {
    pub fn new(name: String) -> Self {
        let secp = Secp256k1::new();

        let (private, public) = secp.generate_keypair(&mut OsRng);

        Self {
            name,
            private_key: private.secret_bytes(),
            public_key: public.serialize(),
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Account: {}", self.name)?;
        writeln!(f, "  Private: {}", hex::encode(self.private_key))?;
        writeln!(f, "  Public:  {}", hex::encode(self.public_key))
    }
}
