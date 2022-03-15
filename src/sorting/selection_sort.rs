pub fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut smallest = i;
        for j in i..arr.len() {
            // find smallest element swap for
            if arr[j] <= arr[smallest] {
                smallest = j;
            }
        }
        // swap the smallest element with ith
        arr[i] = arr[smallest]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let mut arr = [10, 16, 5, 3, 8, 1];
        selection_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }

        let mut arr = [1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
