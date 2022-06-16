use rand::Rng;

pub fn d_20() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..21)
}
