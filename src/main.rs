use julia_blockchain::block::{Block, Difficulty};
use julia_blockchain::maths::{sample_disk, verify, Complex};
use julia_blockchain::now;

fn main() {
    let difficulty = Difficulty {
        set: Complex::new(0.285, 0.),
        points: sample_disk(2., 500),
        target_iterations: 31,
    };

    let mut block = Block::new(
        String::from("Genesis Block"),
        difficulty.clone(),
        vec![0; 32],
        now(),
    );

    match block.mine() {
        Ok(nonce) => {
            if verify(difficulty.set, nonce, difficulty.target_iterations) {
                println!("Found point {nonce} correctly!")
            } else {
                println!("Point {nonce} was incorrect")
            }
        }
        Err(e) => println!("{}", e),
    }
}

#[cfg(test)]
mod tests {}
