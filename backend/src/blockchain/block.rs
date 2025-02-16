use crate::blockchain::hash::Hash;

#[derive(Debug)]
pub struct Block {
    pub nonce: usize,
    pub prev: Hash,
    pub index: usize,
    pub data: String,
}

impl Block {
    pub fn new(data: &str, index: usize) -> Self {
        Block {
            nonce: 0,
            prev: Hash::blank(),
            index,
            data: data.to_string(),
        }
    }

    pub fn genesis() -> Self {
        Block {
            nonce: 0,
            prev: Hash::blank(),
            index: 0,
            data: "".to_string(),
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::new(format!("{}{}{}{}", self.prev.0, self.nonce, self.data, self.index).as_str())
            .unwrap()
    }
}
