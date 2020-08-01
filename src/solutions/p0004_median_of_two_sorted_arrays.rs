struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (n, m) = (nums1.len(), nums2.len());
        let (shorter, longer) = if n <= m {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        let (mut left, mut right, mid) = (0, m, (m + n + 1) / 2);
        while left <= right {
            let i = (left + right) / 2;
            let j = mid - i;
            if i < m && longer[j - 1] > shorter[i] {
                left = i + 1;
            } else if i > 0 && shorter[i - 1] > longer[j] {
                right = i - 1;
            } else {
                let max_left = if i == 0 {
                    longer[j - 1]
                } else if j == 0 {
                    shorter[i - 1]
                } else {
                    std::cmp::max(shorter[i - 1], longer[j - 1])
                };

                if (m + n) % 2 == 1 {
                    return max_left as f64;
                }

                let min_right = if i == m {
                    longer[j]
                } else if j == n {
                    shorter[i]
                } else {
                    std::cmp::min(shorter[i], longer[j])
                };

                return (max_left + min_right) as f64 / 2.0;
            }
        }

        unreachable!()
    }
}
