pub(crate) fn shift_left(bytes: &mut Vec<u8>,
                         mut count: usize,
                         mut trailing_zeros: usize)
                         -> (u8, usize)
{
    debug_assert!(trailing_zeros < 8);
    debug_assert!(bytes.len() * 8 >= count);

    // Early out if count is zero
    if count == 0 || bytes.is_empty()
    {
        return (0, 0);
    }

    let mut removed_bytes: Vec<u8>;

    // remove whole bytes
    if count >= 8
    {
        removed_bytes = bytes.split_off(count / 8);
        std::mem::swap(&mut removed_bytes, bytes);
    }
    else
    {
        removed_bytes = vec![]
    }

    count %= 8;

    // Early out if no further shifting is required
    if count == 0
    {
        return (removed_bytes.pop().unwrap(), trailing_zeros);
    }

    trailing_zeros += count; // 17 bits, with 7 tz, shifted left 5 = 12 tz

    // var to store the trimmed bits
    let mut trimmed = 0;

    // The actual shifting, done in reverse
    for item in bytes.iter_mut().rev()
    {
        // Create a new item by shifting item left by count
        // bitwise OR the previous trimmed value in
        let new_item = (*item << count) | trimmed;
        // store the trimmed value for next iteration
        trimmed = *item >> (8 - count);

        *item = new_item;
    }

    // Remove trailing bytes that have been emptied
    if trailing_zeros >= 8
    {
        bytes.pop();
    }

    // Return the last trimmed value for use
    (trimmed, trailing_zeros % 8)
}
