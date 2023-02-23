fn main() {
    let mut data = [22, 12, 13, 17, 18];
    for i in 0..5 {
        data[i] = floored_half(data[i]);
    }

    print!("The floored half of the array is: {:?}\n", data);
}

fn floored_half(data: i32) -> i32 {
    data / 2
}
