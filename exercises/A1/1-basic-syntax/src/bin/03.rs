fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut largest = input[0];
    let mut minimum = input[0];
    for num in input {
        if num > largest {
            largest = num;
        } else if num < minimum {
            minimum = num;
        }
    }

    println!("{} is largest and {} is smallest", largest, minimum);
}
