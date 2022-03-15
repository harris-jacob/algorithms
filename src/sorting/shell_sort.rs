pub fn shell_sort(arr: &mut [i32]) {
    let mut interval = arr.len() / 2;
    while interval != 0 {
        // h sort
        for i in (0..arr.len()).step_by(interval) {
            let mut j = i;
            let temp = arr[i];

            // shift earlier sorted elements until j finds correct place
            while j >= interval && arr[j - interval] > temp {
                arr[j] = arr[j - interval];
                j -= interval;
            }

            // put temp back in correct place
            arr[j] = temp;
        }

        // update gaps
        interval = interval / 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let mut arr = [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        shell_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
