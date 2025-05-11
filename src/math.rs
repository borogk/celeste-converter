pub fn make_divisible_by(value: usize, divisor: usize) -> usize {
    divisor * (value / divisor + (value % divisor > 0) as usize)
}
