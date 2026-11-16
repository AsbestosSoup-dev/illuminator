//! M0: Vec3 maths and PPM output.

use illuminator::ppm::write_ppm;
use illuminator::vec3::{Color, Point3, Vec3};

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

fn assert_vec(actual: Vec3, expected: (f64, f64, f64)) {
    assert!(
        approx(actual.x, expected.0)
            && approx(actual.y, expected.1)
            && approx(actual.z, expected.2),
        "expected {:?}, got ({}, {}, {})",
        expected,
        actual.x,
        actual.y,
        actual.z
    );
}

#[test]
fn new_sets_public_fields() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!((v.x, v.y, v.z), (1.0, 2.0, 3.0));
}

#[test]
fn aliases_are_vec3() {
    let p: Point3 = Vec3::new(0.0, 0.0, 0.0);
    let c: Color = p;
    assert_eq!(c.x, 0.0);
}

#[test]
fn add_sub_neg() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    assert_vec(a + b, (5.0, 7.0, 9.0));
    assert_vec(b - a, (3.0, 3.0, 3.0));
    assert_vec(-a, (-1.0, -2.0, -3.0));
}

#[test]
fn scalar_mul_on_both_sides_and_div() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_vec(v * 2.0, (2.0, 4.0, 6.0));
    assert_vec(2.0 * v, (2.0, 4.0, 6.0));
    assert_vec(v / 2.0, (0.5, 1.0, 1.5));
}

#[test]
fn componentwise_mul() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    assert_vec(a * b, (4.0, 10.0, 18.0));
}

#[test]
fn dot_product() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    assert!(approx(a.dot(b), 32.0));
    assert!(approx(
        Vec3::new(1.0, 0.0, 0.0).dot(Vec3::new(0.0, 1.0, 0.0)),
        0.0
    ));
}

#[test]
fn cross_product() {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    assert_vec(x.cross(y), (0.0, 0.0, 1.0));
    assert_vec(y.cross(x), (0.0, 0.0, -1.0));
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    assert_vec(a.cross(b), (-3.0, 6.0, -3.0));
}

#[test]
fn length_and_unit() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert!(approx(v.length_squared(), 25.0));
    assert!(approx(v.length(), 5.0));
    assert_vec(v.unit(), (0.6, 0.8, 0.0));
    assert!(approx(Vec3::new(-2.0, 7.0, 1.5).unit().length(), 1.0));
}

#[test]
fn ppm_header_and_pixels() {
    let pixels = [Color::new(1.0, 0.0, 0.0), Color::new(0.0, 0.0, 1.0)];
    let mut buf: Vec<u8> = Vec::new();
    write_ppm(&mut buf, 2, 1, &pixels).unwrap();
    assert_eq!(
        String::from_utf8(buf).unwrap(),
        "P3\n2 1\n255\n255 0 0\n0 0 255\n"
    );
}

#[test]
fn ppm_is_row_major_top_row_first() {
    // 2x2: row 0 = red, green; row 1 = blue, white.
    let pixels = [
        Color::new(1.0, 0.0, 0.0),
        Color::new(0.0, 1.0, 0.0),
        Color::new(0.0, 0.0, 1.0),
        Color::new(1.0, 1.0, 1.0),
    ];
    let mut buf: Vec<u8> = Vec::new();
    write_ppm(&mut buf, 2, 2, &pixels).unwrap();
    assert_eq!(
        String::from_utf8(buf).unwrap(),
        "P3\n2 2\n255\n255 0 0\n0 255 0\n0 0 255\n255 255 255\n"
    );
}

#[test]
fn ppm_clamps_and_scales() {
    let pixels = [Color::new(2.0, -1.0, 0.5)];
    let mut buf: Vec<u8> = Vec::new();
    write_ppm(&mut buf, 1, 1, &pixels).unwrap();
    let text = String::from_utf8(buf).unwrap();
    let vals: Vec<u32> = text
        .lines()
        .nth(3)
        .expect("a pixel line after the header")
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    assert_eq!(vals[0], 255, "values above 1.0 clamp to 255");
    assert_eq!(vals[1], 0, "values below 0.0 clamp to 0");
    assert!(
        (127..=128).contains(&vals[2]),
        "0.5 should map to about half scale, got {}",
        vals[2]
    );
}
