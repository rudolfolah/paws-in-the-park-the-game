use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric, DistString};

pub(crate) fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn generate_id() -> String {
    return Alphanumeric.sample_string(&mut thread_rng(), 32);
}

pub fn rand_int_between(a: u8, b: u8) -> u8 {
    let (min, max) = if a > b {
        (b, a)
    } else {
        (a, b)
    };
    let mut rng = thread_rng();
    return rng.gen_range(min..=max);
}

#[test]
fn test_rand_int_between() {
    rand_int_between(1u8, 10u8);
}
