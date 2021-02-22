mod problem_122;


fn main() {
    use std::time::Instant;

    const TARGET: u32 = 200;

    let now = Instant::now();

    problem_122::main(TARGET);

    let elapsed = now.elapsed();

    println!("Time taken : {:?}", elapsed);
}
