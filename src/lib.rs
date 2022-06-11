mod generator;
pub fn pring_random_number() {
    let n = generator::gen_ran();
    println!("Uandom u8: {}", n)
}
