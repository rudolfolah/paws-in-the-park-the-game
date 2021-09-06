use cosmwasm_std::Timestamp;
use oorandom::Rand32;

const ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate_id(seed_timestamp: Timestamp) -> String {
    let mut rng = Rand32::new(seed_timestamp.nanos());
    let mut result = String::new();
    for _i in 0..32 {
        let index = rng.rand_range(0u32..ALPHANUMERIC.len() as u32);
        result.push(ALPHANUMERIC.chars().nth(index as usize).unwrap());
    }
    println!("generate_id: #{}", result);
    return result;
}

pub fn rand_int_between(seed_timestamp: Timestamp, a: u8, b: u8) -> u8 {
    let (min, max) = if a > b {
        (u32::from(b), u32::from(a) + 1u32)
    } else {
        (u32::from(a), u32::from(b) + 1u32)
    };
    let mut rng = Rand32::new(seed_timestamp.nanos());
    return rng.rand_range(min..max) as u8;
}

// use rand::{thread_rng, Rng};
// use rand::distributions::{Alphanumeric, DistString};
//
// #[allow(dead_code)]
// pub(crate) fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
//
// pub fn generate_id() -> String {
//     return Alphanumeric.sample_string(&mut thread_rng(), 32);
// }
//
// pub fn rand_int_between(a: u8, b: u8) -> u8 {
//     let (min, max) = if a > b {
//         (b, a)
//     } else {
//         (a, b)
//     };
//     let mut rng = thread_rng();
//     return rng.gen_range(min..=max);
// }

#[test]
fn test_rand_int_between() {
    rand_int_between(Timestamp::from_nanos(123456), 1u8, 10u8);
}
