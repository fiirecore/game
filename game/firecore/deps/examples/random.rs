use firecore_dependencies::Random;

static RANDOM: Random = Random::new();

fn main() {

    // seed random
    RANDOM.seed(12345);

    // get random number from 0 to u32::MAX
    let x = RANDOM.rand();
    println!("x={}", x);

    // get random float between 0 and 1
    let x = RANDOM.gen_float();
    assert!(x >= 0. && x < 1.);
    println!("x={}", x);

    // gen_range works for most of standard number types
    let x: u8 = RANDOM.gen_range(64, 128);
    assert!(x >= 64 && x < 128);
    println!("x={}", x);
}
