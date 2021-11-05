use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::Rng;

// pub fn get_random_age() -> i8 {
//   let vs: Vec<i8> = vec![5, 10, 15];
//   *vs.choose(&mut rand::thread_rng()).unwrap()
// }

// pub fn get_random_active() -> i8 {
//   if rand::random() {
//     return 1;
//   }
//   0
// }

// pub fn get_random_bool() -> bool {
//   rand::random()
// }

// pub fn get_random_area_code() -> String {
//   let mut rng = rand::thread_rng();
//   format!("{:06}", rng.gen_range(0..999999))
// }

pub fn get_random_amount() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(11111111..99999999)
}

pub fn get_random_posted() -> i8 {
    if rand::random() {
        return 1;
    }
    0
}

pub fn get_random_address() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

pub fn get_random_account_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(11111111..99999999)
}

pub fn get_random_account_name() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

pub fn get_random_status() -> i8 {
    let vs: Vec<i8> = vec![1, 2, 3];
    *vs.choose(&mut rand::thread_rng()).unwrap()
}

pub fn get_randome_phone_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(11111111..99999999)
}
