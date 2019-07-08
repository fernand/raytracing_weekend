extern "C" {
    fn drand48() -> f64;
}

#[inline]
pub fn drand() -> f64 {
    unsafe { drand48() }
}
