use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate(length: usize) -> String {
    if length == 0 {
        panic!("Length must be greater than zero");
    }

    thread_rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
