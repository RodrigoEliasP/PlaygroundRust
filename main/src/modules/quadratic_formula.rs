pub fn quadratic_formula(a: f32, b:f32, c: f32) -> (f32, f32) {
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (x1, x2)
}