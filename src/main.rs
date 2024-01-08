// Definition for singly-linked list.
//
//

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // for _ in 0..n {
    //     nums1.pop();
    // }
    nums1.truncate(m.try_into().unwrap());
    nums1.append(nums2);
    nums1.sort();
    dbg!(nums1);
    // for (idx, val) in nums1.iter().enumerate() {
    //     if idx < m.try_into().unwrap() {
    //         dbg!(nums1);
    //     } else {
    //         println!("{val}")
    //         // nums1.push(val);
    //     }
    // }
}

fn main() {
    let mut vectard: Vec<i32> = vec![1, 2, 3, 4, 0, 0, 0, 0];
    let m = 4;
    let n = 4;
    let mut nums2: Vec<i32> = vec![1, 5, 6, 7];
    merge(&mut vectard, m, &mut nums2, n);
}
