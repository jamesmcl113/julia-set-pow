# Julia Set POW
The source code for this task can be found at https://github.com/jamesmcl113/julia-set-pow.

## The Julia Set
We first need to add functionality to our library so that either Bob or Alice can find how many iterations it takes for a point `P` from a set of points `Q` to escape a given Julia set (defined by the complex parameter `c`). This is accomplished with the `maths::check` function:

```rust
/// The number of iterations for `p` to escape `julia_set` - returns `None` if it didn't escape
fn check(julia_set: Complex, p: Complex, show_iterates: bool) -> Option<u32> {
    let mut i: u32 = 0;
    let mut p = p;

    loop {
        if show_iterates {
            println!("{i}th iterate: {}", p);
        }

        if p.sq() > 4. || i == MAX_ITERATIONS {
            break;
        } else {
            let tmp = p.x * p.x - p.y * p.y + julia_set.x;
            p.y = 2.0 * p.x * p.y + julia_set.y;
            p.x = tmp;
            i += 1;
        }
    }

    if i == MAX_ITERATIONS {
        None
    } else {
        Some(i)
    }
}
```

We simply iterate the point until one of two things happen:
1. The point doesn't escape in `MAX_ITERATIONS` => return `None`
2. The point escapes => return `Some(n_iterations)`

In this example code, `MAX_ITERATIONS` is set to some arbitrarily large constant, but in working code it should be adjusted based on the average iterations it takes for a given point to escape.

Next, we created `maths::find_point` which takes a Julia set, a set of complex points and the target number of iterations and returns a point `P` which escapes in `target` iterations if such a point is found. Otherwise, it returns `None`:
```rust
pub fn find_point(julia_set: Complex, points: &[Complex], target: u32) -> Option<Complex> {
    for &p in points {
        match check(julia_set, p, false) {
            Some(i) => {
                if i == target {
                    return Some(p);
                }
            }
            None => {}
        }
    }

    None
}
```

We use our `check` function to test each point in `Q` and return the first one that matches the `target`. Obviously there could exist zero or more points in `Q` that escape in `target` iterations - working code could account for this by instead returning a collection of the matching points that Alice can check against.

## The Blockchain (mostly)
Next, we created a `Block` struct to test our Proof-of-Work functionality:
```rust
type BlockHash = Vec<u8>;
pub struct Block {
    pub hash: BlockHash,
    pub prev_hash: BlockHash,
    pub timestamp: u128,
    pub nonce: Complex,
    pub payload: String,
    pub difficulty: Difficulty,
}
```
Some assumptions were made about the POW puzzle:
1. Alice guarantess that there will exist a point `P` in `Q` that escapes in exactly `R` iterations - if this were not the case we could represent the nonce with an `Option<Complex` instead.
2. The `difficulty` field - which represents the puzzle - is not encrypted. This is obviously insecure but keeping it unencrypted makes testing easier. In working code, we would want to obfuscate this field using some cryptographic primitive.

The aformentioned `Difficulty` struct looks like this:
```rust
#[derive(Clone)]
pub struct Difficulty {
    pub set: Complex,
    pub points: Vec<Complex>,
    pub target_iterations: u32,
}
```
The size of `points` can be modified to change the difficulty - a larger set will mean that the miner will probably need to go through more points before they find one that matches the target. `set` can be modified to change the given Julia set.

We also created a `Hashable` trait to allow for easy hashing of our user types:
```rust
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
```
`hash` will be the same for all the implementers - it only depends on the `Hashable::bytes` method which we must define when implementing the trait. So for the `Block` type, the hash includes the nonce, payload and timestamp:
```rust
impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.nonce.bytes().iter());
        bytes.extend(self.payload.bytes());
        bytes.extend(self.timestamp.to_ne_bytes().iter());

        bytes
    }
}
```
We can call `nonce.bytes()` since `Complex` also implements `Hashable`.

### Mining
To mine a new block, we call the `Block::mine` method. It attempts to find a point from its `Difficulty` puzzle and sets its nonce to this point if one was found - otherwise it fails:
```rust
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
```
It returns the matching point if it was successful so Alice can easily verify it (or she could look at the nonce).
We're still working on the assumption that the point `P` must exist, so if `find_point` fails, something else must've gone wrong.

See `src/main.rs` for a block mining example. This code doesn't contain any kind of Blockchain struct but this would be trivial with the working `Block` implementation we have shown here.
