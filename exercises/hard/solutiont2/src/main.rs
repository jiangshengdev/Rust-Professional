// I AM NOT DONE

mod factorize;
mod gcd;
mod is_prime;
mod mod_mul;
mod mod_pow;
mod pollard_rho;
mod prime_factor;

fn main() {
    let number = 100;
    let res = prime_factor::find_max_prime_factor(number);
    println!("{number}'s max prime factor: {res}");
}
