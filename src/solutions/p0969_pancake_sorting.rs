struct Solution;

impl Solution {
    pub fn pancake_sort(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut output = vec![];
        let mut k = a.len() as i32;

        while k > 0 {
            let idx = a.iter().position(|&x| x == k).unwrap() as i32;

            if idx != k - 1 {
                if idx != 0 {
                    output.push(idx + 1);
                    flip(&mut a, idx + 1);
                }
                output.push(k);
                flip(&mut a, k);
            }

            k -= 1;
        }

        output
    }
}

fn flip(a: &mut [i32], k: i32) {
    let mut i = 0;
    while i < k / 2 {
        a.swap(i as usize, (k - i - 1) as usize);
        i += 1;
    }
}
