use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Generate a random string of a specified length.
///
/// # Arguments
///
/// * `length` - The desired length of the random string.
///
/// # Panics
///
/// This function will panic if `length` is set to 0 because a random string of zero length is meaningless.
///
/// # Examples
///
/// ```
/// use generate::generate;
///
/// // Generate random string with length 10
/// let random_string = generate(10);
/// println!("Random String: {}", random_string);
/// ```
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
