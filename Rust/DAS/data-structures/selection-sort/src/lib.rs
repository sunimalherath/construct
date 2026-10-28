pub fn selection_sort<T: PartialOrd>(v: &mut [T]) {
    for i in 0..v.len() {
        let mut min_val = &v[i];
        let mut min_idx = i;

        for j in i..v.len() {
            if v[j] < *min_val {
                min_val = &v[j];
                min_idx = j;
            }
        }

        // swap the values
        v.swap(min_idx, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![3, 5, 1, 6, 2];
        selection_sort(&mut v);

        assert_eq!(v, vec![1, 2, 3, 5, 6])
    }
}
