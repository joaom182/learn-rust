fn print_out_items(items: &Vec<u32>) {
    for i in items {
        println!("{}", i);
    }
}

fn main () {
    let items = vec![1, 2, 3, 4];
    print_out_items(&items);
    print_out_items(&items);
}

