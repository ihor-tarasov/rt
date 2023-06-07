use crate::{Vec3, vec3};

pub fn near_zero(v: Vec3) -> bool {
    const S: f32 = 1e-8;
    v.x().abs() < S && v.y().abs() < S && v.z().abs() < S
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn mul_per_comp(a: Vec3, b: Vec3) -> Vec3 {
    vec3(a.x() * b.x(), a.y() * b.y(), a.z() * b.z())
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn apply_sampling(color: Vec3, samples: usize) -> Vec3 {
    let v = color * (1.0 / samples as f32);
    vec3(v.x().sqrt(), v.y().sqrt(), v.z().sqrt())
}
