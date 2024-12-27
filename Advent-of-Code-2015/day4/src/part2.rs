use md5;
use std::fmt::Write;

fn md5_hash(input: &str) -> md5::Digest {
    md5::compute(input)
}

fn has_6_leading_0(digest: md5::Digest) -> bool {
    // Check if the first 3 bytes are zero -> correspond to the six first 0 in the hex representation
    digest[0] == 0 && digest[1] == 0 && digest[2] == 0
}

pub fn part2(input: &Vec<String>) -> i64 {
    let base = &input[0];
    let mut res: i64 = 0;
    let mut buffer = String::with_capacity(base.len() + 20); // Allocate enough capacity
    buffer.push_str(base);

    loop {
        res += 1;
        buffer.truncate(base.len()); // Reset buffer to base
        write!(&mut buffer, "{}", res).unwrap(); // Append res

        let digest = md5_hash(&buffer);
        if has_6_leading_0(digest) {
            break;
        }
    }

    res
}