pub fn insertion_sort(arr: &mut [i32]) {
    // points to first unsorted elem
    for i in 1..arr.len() {
        // point to last sorted
        let mut j = i - 1;
        while j != 0 && arr[j] > arr[i] {
            arr[j + 1] = arr[j];
            if j != 0 {
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let mut arr = [10, 16, 5, 3, 8, 1];
        insertion_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }

        let mut arr = [1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
