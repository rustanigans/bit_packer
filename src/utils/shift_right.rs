pub(crate) fn shift_right(bytes: &mut Vec<u8>, mut count: usize, mut trailing_zeros: usize) -> usize
{
    if count == 0
    {
        return trailing_zeros;
    }

    let pad_whole_bytes = count / 8;
    count -= pad_whole_bytes * 8;
    {
        let mut new_bytes = vec![0u8; pad_whole_bytes];
        new_bytes.append(bytes);
        std::mem::swap(&mut new_bytes, bytes);
    }

    if count == 0
    {
        return trailing_zeros;
    }

    if count > trailing_zeros
    {
        bytes.push(0);
    }

    trailing_zeros = 8 - count + trailing_zeros;   //  (count as isize - trailing_zeros as isize).abs() as usize;

    let mut trimmed = 0;

    // The actual shifting
    for item in bytes[pad_whole_bytes..].iter_mut()
    {
        // Create a new item by shifting item right by count
        // bitwise OR the previous trimmed value in
        let new_item = (*item >> count) | trimmed;
        // store the trimmed value for next iteration
        trimmed = *item << (8 - count);

        *item = new_item;
    }
    trailing_zeros % 8
}