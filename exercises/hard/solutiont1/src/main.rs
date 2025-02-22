// I AM NOT DONE

mod conjecture;
mod perfect_square;
mod prime;
mod sum;

fn main() {
    let values = conjecture::goldbach_conjecture();
    println!("top 2 goldbach's conjecture on primes: {values}");
}
