
pub fn quick_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    sort(arr, 0, (len-1) as isize);
}

fn sort(arr: &mut Vec<i32>, low: isize, high: isize) {
    if low < high {
        let partition_idx = partition(arr, low, high);

        sort(arr, low, partition_idx-1);
        sort(arr, partition_idx + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];
    // abstract ptr that keeps track of where our "smaller" elements end 
    // also represents the position at which pivot will go in the final array
    let mut smaller_than_idx = low;

    for i in low..high {
        if arr[i as usize] < pivot {
            //add ith to smaller side
            arr.swap(i as usize, smaller_than_idx as usize);
            smaller_than_idx+=1;
        }
    }

    arr.swap(high as usize, smaller_than_idx as usize);

    return smaller_than_idx;

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let mut arr = vec![10, 16, 5, 3, 8, 1];
        quick_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }

        let mut arr = vec![1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
