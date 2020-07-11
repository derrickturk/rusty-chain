// see https://hackernoon.com/learn-blockchains-by-building-one-117428612f46

use std::{
    mem,
    time::SystemTime,
};

use uuid::Uuid;

pub type Hash = [u8; 32];

pub type Addr = String;

// required prefix zero bytes
pub const POW_DIFFICULTY: usize = 2;

// wooden nickels received as mining reward
pub const MINE_REWARD: u64 = 10;

// the "sender" for mining rewards
pub const NAME_OF_GOD: &str = "GOD";

pub const GENESIS: Block = Block {
    id: 0,
    time: SystemTime::UNIX_EPOCH,
    txs: Vec::new(),
    proof: 0,
    prev_hash: [0; 32],
};

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub time: SystemTime,
    pub txs: Vec<Tx>,
    pub proof: u64,
    pub prev_hash: Hash,
}

impl Block {
    // TODO: make this less terrible
    pub fn hash(&self) -> [u8; 32] {
        let as_string = format!("{:?}", self);
        hmac_sha256::Hash::hash(as_string.as_bytes())
    }
}

#[derive(Debug, Clone)]
pub struct Tx {
    pub from: Addr,
    pub to: Addr,
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_txs: Vec<Tx>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![GENESIS],
            pending_txs: Vec::new(),
        }
    }

    pub fn new_block(&mut self, proof: u64) -> &Block {
        self.blocks.push(Block {
            id: self.blocks.len() as u64,
            time: SystemTime::now(),
            txs: mem::take(&mut self.pending_txs),
            proof: proof,
            prev_hash: self.blocks.last().unwrap().hash(),
        });

        self.last_block()
    }

    pub fn new_tx(&mut self, tx: Tx) -> u64 {
        self.pending_txs.push(tx);
        self.blocks.len() as u64
    }

    pub fn last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    pub fn proof_of_work(&self) -> u64 {
        let last_proof = self.last_block().proof;
        let last_hash = self.last_block().hash();
        let mut proof = 0;
        while !valid_proof(last_proof, &last_hash, proof) {
            proof += 1
        }
        proof
    }
}

pub struct Node {
    pub id: Uuid,
    pub chain: Blockchain,
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: Uuid::new_v4(),
            chain: Blockchain::new(),
        }
    }

    pub fn mine(&mut self) -> &Block {
        let proof = self.chain.proof_of_work();
        self.chain.new_tx(Tx {
            from: NAME_OF_GOD.to_string(),
            to: self.id.to_string(),
            amount: MINE_REWARD,
        });
        self.chain.new_block(proof)
    }
}

pub fn valid_proof(last_proof: u64, last_hash: &Hash, proof: u64) -> bool {
    let mut to_hash = format!("{}{}", last_proof, proof).into_bytes();
    to_hash.extend_from_slice(last_hash);
    let hash = hmac_sha256::Hash::hash(&to_hash[..]);
    for byte in hash.iter().take(POW_DIFFICULTY) {
        if *byte != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut node = Node::new();

    node.chain.new_tx(Tx {
        from: "joe".to_string(),
        to: "sally".to_string(),
        amount: 100,
    });

    node.chain.new_tx(Tx {
        from: "sally".to_string(),
        to: "mallory".to_string(),
        amount: 50,
    });

    let block = node.mine();
    dbg!(block);

    node.chain.new_tx(Tx {
        from: "bob".to_string(),
        to: "jimmy".to_string(),
        amount: 1,
    });

    let block = node.mine();
    dbg!(block);
}
