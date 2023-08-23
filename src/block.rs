use crate::maths::{find_point, Complex};

type BlockHash = Vec<u8>;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}

#[derive(Clone)]
pub struct Difficulty {
    pub set: Complex,
    pub points: Vec<Complex>,
    pub target_iterations: u32,
}

pub struct Block {
    pub hash: BlockHash,
    pub prev_hash: BlockHash,
    pub timestamp: u128,
    pub nonce: Complex,
    pub payload: String,
    pub difficulty: Difficulty,
}

impl Block {
    pub fn new(
        payload: String,
        difficulty: Difficulty,
        prev_hash: BlockHash,
        timestamp: u128,
    ) -> Self {
        Self {
            hash: vec![0u8; 32],
            prev_hash,
            timestamp,
            nonce: Complex::new(0., 0.),
            payload,
            difficulty,
        }
    }

    pub fn mine(&mut self) -> Result<Complex, String> {
        let nonce = find_point(
            self.difficulty.set,
            &self.difficulty.points,
            self.difficulty.target_iterations,
        );

        match nonce {
            Some(n) => {
                self.nonce = n;
                self.hash = self.hash();
                Ok(n)
            }
            None => Err("Couldn't mine block :(".to_string()),
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.nonce.bytes().iter());
        bytes.extend(self.payload.bytes());
        bytes.extend(self.timestamp.to_ne_bytes().iter());

        bytes
    }
}
