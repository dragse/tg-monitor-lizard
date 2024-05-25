use rand::{Rng};

pub fn generate_calculation(correct: bool) -> String {
    let first_num = rand::thread_rng().gen_range(0..100);
    let second_num = rand::thread_rng().gen_range(0..100);

    let mut solution = first_num + second_num;

    while !correct && solution == first_num + second_num {
        solution = rand::thread_rng().gen_range(0..150)
    }

    return format!("{} + {} = {}", first_num, second_num, solution)
}