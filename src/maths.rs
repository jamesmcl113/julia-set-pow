use rand::{distributions::Distribution, distributions::Uniform};
use std::fmt::{Debug, Display};

const MAX_ITERATIONS: u32 = 200;

#[derive(Clone, Copy)]
pub struct Complex {
    pub x: f32,
    pub y: f32,
}

impl Complex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&self.x.to_ne_bytes());
        bytes.extend_from_slice(&self.y.to_ne_bytes());

        bytes
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.x, self.y)
    }
}

impl Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

pub fn sample_disk(radius: f32, n: usize) -> Vec<Complex> {
    let mut rng = rand::thread_rng();
    let disk = Uniform::from(-radius..radius);
    let mut ns: Vec<Complex> = Vec::with_capacity(n);

    for _ in 0..n {
        ns.push(Complex::new(disk.sample(&mut rng), disk.sample(&mut rng)));
    }

    ns
}

/// Verify that a point `p` escapes `julia_set` in exactly `n_iterations`
pub fn verify(julia_set: Complex, p: Complex, n_iterations: u32) -> bool {
    assert!(n_iterations < MAX_ITERATIONS);

    match check(julia_set, p, false) {
        Some(i) => i == n_iterations,
        None => false,
    }
}

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
