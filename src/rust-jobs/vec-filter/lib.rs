#[no_mangle]
pub extern "C" fn vec_filter(x: i32, y: i32, z: i32) -> i32 {
    let values: Vec<i32> = vec![x, y, z];

    values.iter().filter(|&x| (x & 1) == 0).sum()
}