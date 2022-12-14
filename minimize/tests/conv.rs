#[allow(unused)]
union Conv<T> {
    p: *const T,
    i: usize,
}

#[allow(unused)]
unsafe fn p_to_i<T>(p: *const T) -> usize {
    let c = Conv { p };
    unsafe { c.i }
}

#[allow(unused)]
unsafe fn ref_to_i<T>(p: &T) -> usize {
    let p = p as *const _;
    let c = Conv { p };
    unsafe { c.i }
}

#[allow(unused)]
unsafe fn i_to_p<T>(i: usize) -> *const T {
    let c = Conv { i };
    unsafe { c.p }
}
