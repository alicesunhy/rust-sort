pub fn sum(arr: &[u32]) -> Option<u32> {
    let mut total: u32 = 0;
    for v in arr.into_iter() {
        total += v;
    }
    Some(total)
}
