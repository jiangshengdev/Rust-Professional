// I AM NOT DONE

mod calculate;
mod compute;
mod format;
mod params;
mod parse;
mod retirement;
mod update;

fn main() {
    let res = retirement::retire_time("1971-04", "原法定退休年龄55周岁女职工");
    println!("{res}");
}
