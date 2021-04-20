use std::time::SystemTime;
use chrono::DateTime;
use chrono::offset::Utc;
use sha256::digest;
#[derive(Clone, Debug)]
struct Block {
    time_stamp: SystemTime,
    data: String,
    hash: String,
    prev_hash: String
}
#[derive(Clone, Debug)]
struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    fn calculate_hash(time_stamp: String, data_to_hash: String, prev_hash: String) -> String {
        digest(time_stamp + &data_to_hash + &prev_hash)
    }

    fn add_block(&mut self, my_data: String) {
        let system_time = SystemTime::now();
        let time_stamp: DateTime<Utc> = system_time.into();
        let time_stamp = time_stamp.format("%d/%m/%Y %T").to_string();
        let data_to_hash = my_data.clone();
        let hash = Blockchain::calculate_hash(time_stamp, data_to_hash, self.last_hash());
        let prev_hash = self.last_hash();

        let new_block = Block { time_stamp: system_time , data: my_data, hash, prev_hash };
        self.chain.push(new_block);
    }

    fn last_hash(&self) -> String {
        match self.chain.last() {
            Some(block) => block.hash.clone(),
            None => String::from("000")
        }
    }

    fn new() -> Self {
        let mut blockchain = Self {
            chain: Vec::new()
        };

        blockchain.add_block(String::from("Genresis Block"));
        blockchain
    }

}

fn main() {
    let mut chain = Blockchain::new();
    chain.add_block(String::from("new data"));

    println!("{:?}", chain);
}
