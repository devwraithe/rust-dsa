
// Performs a binary search on a sorted slice to find the target value
pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    // Initialize the searching index of the search range to 0
    let mut low = 0;
    // Initialize the searching index of the search range to the last index in the array
    let mut high = arr.len() as i32 - 1;

    // Continue searching as long as the search range is valid
    while low <= high {
        // Calculate the middle index of the current search range
        let mid = ((low + high) / 2) as usize;

        // Compare the middle value with the target value
        if arr[mid] == target {
            // If the middle value matches the target
            return Some(mid);
        }
        else if arr[mid] < target {
            // If the middle value is less than the target, adjust the search range to the right half
            low = mid as i32 + 1;
        }
        else {
            // If the middle value is greater than the target, adjust the search range to the left half
            high = mid as i32 -1;
        }
    }

    // If the target value is not found, return None
    None
}

// Insert a value into a sorted vector, maintaining a sorted order
pub fn insert(arr: &mut Vec<i32>, value: i32) {
    match arr.binary_search(&value) {
        Ok(pos) => arr.insert(pos,value),
        Err(pos) => arr.insert(pos, value),
    }
}

// Deletes a value from a sorted vector, maintaining a sorted order
pub fn delete(arr: &mut Vec<i32>, value: i32) -> bool {
    match arr.binary_search(&value) {
        Ok(pos) => {
            arr.remove(pos);
            true
        },
        Err(_) => false
    }
}