mod rec_mc;

fn main() {
    let amount = 93u32;
    let cash_num = rec_mc::dp_rec_mc(amount);
    println!("{cash_num}");
}
