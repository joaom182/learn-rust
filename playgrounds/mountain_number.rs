fn is_mountain_number(arr: Vec<i32>) -> bool {
    let mut res: bool = true;
    let mut highest: i32 = 0;

    if arr.len() < 3 {
        return false;
    }

    for index in 0..arr.len() -1 {
        if index == 0 {
            if arr[index] > arr[index + 1] || arr[index] == arr[index + 1] {
                res = false;
                break;
            }
            continue;
        };

        let is_last = index == (arr.len() - 1);
        let current = &arr[index];
        let prev = &arr[index - 1];
        let current_is_equal_than_highest = current == &highest;
        let current_is_greater_than_highest = current > &highest;
        let current_is_lower_than_highest = current < &highest;
        let current_is_greather_than_prev = current > prev;
        let highest_is_great_than_prev = &highest > prev;

        if is_last && current_is_greater_than_highest || (
            current_is_greater_than_highest
                && current_is_greather_than_prev
                && highest_is_great_than_prev
        ) || (
            current_is_greather_than_prev && (current_is_lower_than_highest || current_is_equal_than_highest)
        ) {
            res = false;
        }

        if is_last {
            break;
        }

        let next = &arr[index + 1];
        let is_negative_or_big_number = current < &0 || current > &i32::pow(10, 4);
        let first_number_is_greather_than_next = index == 0 && current > next;
        let current_is_equal_than_next = current == next;

        if is_negative_or_big_number
            || current_is_equal_than_highest
            || first_number_is_greather_than_next
            || current_is_equal_than_next
            || (current_is_greather_than_prev && current_is_lower_than_highest)
        {
            res = false;
            break;
        }

        if current_is_greater_than_highest {
            highest = *current;
        }
    }
    res
}

fn main() {
    println!("[1, 2, 3, 2] {}", is_mountain_number((&[1, 2, 3, 2]).to_vec()));
    println!("[0, 3, 2, 1] {}", is_mountain_number((&[0, 3, 2, 1]).to_vec()));
    println!("[1, 2, 2, 3] {}", is_mountain_number((&[1, 2, 2, 3]).to_vec()));
    println!("[2, 1] {}", is_mountain_number((&[2, 1]).to_vec()));
    println!("[0, 1, 2, 2, 1, 0] {}", is_mountain_number((&[0, 1, 2, 2, 1, 0]).to_vec()));
    println!("[0, 1, 4, 2, 3, 1] {}", is_mountain_number((&[0, 1, 4, 2, 3, 1]).to_vec()));
    println!("[0, 1, 4, 2, 4, 1] {}", is_mountain_number((&[0, 1, 4, 2, 4, 1]).to_vec()));
    println!("[0, 1, 4, 2, 3] {}", is_mountain_number((&[0, 1, 4, 2, 3]).to_vec()));
    println!("[-1, 0, 2, 0] {}", is_mountain_number((&[-1, 0, 2, 0]).to_vec()));
    println!("[0, 1, 3, 2, 4, 1] {}", is_mountain_number((&[0, 1, 3, 2, 4, 1]).to_vec()));
    println!("[0, 1, 100000, 1, 0]  {}", is_mountain_number((&[0, 1, 100000, 1, 0]).to_vec()));
    println!("[1, 0, 1, 2, 3, 2, 1, 0] {}", is_mountain_number((&[1, 0, 1, 2, 3, 2, 1, 0]).to_vec()));
    println!("[0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {}", is_mountain_number((&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).to_vec()));
    println!("[9,8,7,6,5,4,3,2,1,0] {}", is_mountain_number((&[9,8,7,6,5,4,3,2,1,0]).to_vec()));
    println!("[0,1,2,1,2] {}", is_mountain_number((&[0,1,2,1,2]).to_vec()));
    println!("[4,4,3,2,1] {}", is_mountain_number((&[4,4,3,2,1]).to_vec()));
}
