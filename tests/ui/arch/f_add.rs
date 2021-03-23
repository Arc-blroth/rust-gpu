// build-pass

#[spirv(fragment)]
pub fn main() {
    let x = 5.0;
    let y = 2.0;
    let vx = glam::Vec2::new(2.0, 5.0);
    let vy = glam::Vec2::new(5.0, 2.0);
    assert!(spirv_std::arch::f_add_vector(vx, vy) == glam::Vec2::new(7.0, 7.0));
}
