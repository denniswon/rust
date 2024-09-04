/*

You are given two arrays of integers nums1 and nums2, possibly of different lengths.
The values in the arrays are between 1 and 6, inclusive.

In one operation, you can change any integer's value in any of the arrays to any value between 1 and 6, inclusive.

Return the minimum number of operations required to make the sum of values in nums1
equal to the sum of values in nums2. Return -1​​​​​ if it is not possible to make the sum of the two arrays equal.

Example 1:

Input: nums1 = [1,2,3,4,5,6], nums2 = [1,1,2,2,2,2]
Output: 3
Explanation: You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
- Change nums2[0] to 6. nums1 = [1,2,3,4,5,6], nums2 = [6,1,2,2,2,2].
- Change nums1[5] to 1. nums1 = [1,2,3,4,5,1], nums2 = [6,1,2,2,2,2].
- Change nums1[2] to 2. nums1 = [1,2,2,4,5,1], nums2 = [6,1,2,2,2,2].

*/

use log::info;
use std::mem::swap;

pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut sum1 = nums1.iter().fold(0_i32, |acc, x| acc.wrapping_add(*x));
    let mut sum2 = nums2.iter().fold(0_i32, |acc, x| acc.wrapping_add(*x));

    let mut nums1 = nums1.clone();
    nums1.sort();
    let mut nums2 = nums2.clone();
    nums2.sort();
    if sum1 <= sum2 {
        nums2.reverse();
    } else {
        nums1.reverse();
        swap(&mut nums1, &mut nums2);
        swap(&mut sum1, &mut sum2);
    }
    // nums1 with sum1 (asc) < nums2 with sum2 (desc)

    let mut diff = sum2 - sum1;
    info!(
        target: "leetcode::min_operations",
        "nums1: {:?} (sum1: {}), nums2: {:?} (sum2: {}), diff: {}",
        nums1, sum1, nums2, sum2, diff
    );

    let mut i = 0;
    let mut j = 0;
    let mut ret = 0;
    while i < nums1.len() || j < nums2.len() {
        if diff == 0 {
            return ret;
        } else {
            let n1 = if i < nums1.len() { nums1[i] } else { 6 };
            let n2 = if j < nums2.len() { nums2[j] } else { 1 };
            let op = (n2 - 1) + (6 - n1);
            if op > diff {
                if diff <= (n2 - 1) || diff <= (6 - n1) {
                    ret += 1;
                } else {
                    ret += 2
                }
                diff = 0;
                break;
            }

            diff -= op;
            ret += (n2 > 1) as i32 + (n1 < 6) as i32;
            i += 1;
            j += 1;
        }
    }

    info!(
        target: "leetcode::min_operations",
        "ret: {:?}, diff: {}",
        ret, diff
    );

    if diff == 0 {
        ret
    } else {
        -1
    }
}
