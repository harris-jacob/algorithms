pub fn merge_sort_bottom_up(arr: &mut Vec<i32>) {
    let mut aux = vec![0; arr.len()];
    let mut size = 1;

    while size < arr.len() {
        let mut low = 0;
        while low < arr.len() - size {
            let mid = low + size - 1;
            let high = std::cmp::min(arr.len() - 1, low + 2 * size - 1);
            merge(arr, &mut aux, low, mid, high);
            low = low + 2 * size;
        }

        size = size + size;
    }
}

pub fn merge_sort_recursive(arr: &mut Vec<i32>) {
    let mut aux = vec![0; arr.len()];
    sort(arr, &mut aux, 0, arr.len() - 1)
}

// sort function for recursive solution
fn sort(arr: &mut Vec<i32>, aux: &mut Vec<i32>, low: usize, high: usize) {
    if high <= low {
        return;
    }

    let mid = low + (high - low) / 2;

    sort(arr, aux, low, mid);
    sort(arr, aux, mid + 1, high);
    merge(arr, aux, low, mid, high);
}

fn merge(arr: &mut Vec<i32>, aux: &mut Vec<i32>, low: usize, mid: usize, high: usize) {
    // put in aux arr
    for i in low..(high + 1) {
        aux[i] = arr[i]
    }

    // pointers to each elem
    let mut p1 = low;
    let mut p2 = mid + 1;

    // perform merge
    for i in low..(high + 1) {
        if p1 > mid {
            arr[i] = aux[p2];
            p2 += 1;
        } else if p2 > high {
            arr[i] = aux[p1];
            p1 += 1;
        } else if aux[p1] > aux[p2] {
            arr[i] = aux[p2];
            p2 += 1;
        } else {
            arr[i] = aux[p1];
            p1 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_recursive() {
        let mut arr = vec![10, 16, 5, 3, 8, 1];
        merge_sort_recursive(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }

        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort_recursive(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
    #[test]
    fn sort_bottom_up() {
        let mut arr = vec![10, 16, 5, 3, 8, 1];
        merge_sort_bottom_up(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }

        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort_bottom_up(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
