impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let mut ans = 0;
        let mut vowel = 0;
        for (i, &c) in s.iter().enumerate() {
            // 枚举窗口右端点 i
            // 1. 右端点进入窗口
            if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
                vowel += 1;
            }

            if i + 1 < k {
                // 窗口大小不足 k，尚未形成第一个窗口
                continue;
            }

            // 2. 更新答案
            ans = ans.max(vowel);
            if ans == k {
                // 答案已经等于理论最大值
                break; // 无需再循环
            }

            // 3. 左端点离开窗口，为下一个循环做准备
            let out = s[i + 1 - k];
            if out == b'a' || out == b'e' || out == b'i' || out == b'o' || out == b'u' {
                vowel -= 1;
            }
        }
        ans as _
    }
}
