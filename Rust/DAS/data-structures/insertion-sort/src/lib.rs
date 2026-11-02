pub fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
    for i in (0..list.len()).step_by(2) {
        if i == 0 {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                continue;
            }
        }

        let temp_list = &list[..i];

        for j in 0..temp_list.len() {
            if list[i] < list[j] {
                list.swap(i, j);
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![3, 5, 1, 6, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 5, 6]);
    }
}
