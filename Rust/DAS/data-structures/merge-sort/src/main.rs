fn main() {
    let mut arr = [6, 5, 2, 1, 4, 8, 7, 3];

    println!("{:#?}", merge_sort(&mut arr));
}

fn merge_sort(list: &mut [i32]) -> &mut [i32] {
    if list.len() == 1 {
        return list;
    }

    // devide the list in half and pass each again to the merge_sort
    let left_list = &list[0..list.len() / 2];
    let right_list = &list[list.len() / 2..];
    // is there a better way to do this??

    println!("Left wing: {:#?}", left_list);
    println!("Right wing: {:#?}", right_list);

    return list;
}
