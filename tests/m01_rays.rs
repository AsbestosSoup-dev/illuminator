//! M1: rays and the sky gradient.

use illuminator::ray::Ray;
use illuminator::sky::sky_color;
use illuminator::vec3::Vec3;

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
fn ray_keeps_origin_and_dir() {
    let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, -2.0));
    assert_vec(r.origin, (1.0, 2.0, 3.0));
    assert_vec(r.dir, (0.0, 0.0, -2.0));
}

#[test]
fn at_walks_along_the_ray() {
    let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, -2.0));
    assert_vec(r.at(0.0), (1.0, 2.0, 3.0));
    assert_vec(r.at(0.5), (1.0, 2.0, 2.0));
    assert_vec(r.at(2.0), (1.0, 2.0, -1.0));
}

#[test]
fn sky_is_white_looking_down() {
    let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
    assert_vec(sky_color(&r), (1.0, 1.0, 1.0));
}

#[test]
fn sky_is_blue_looking_up() {
    let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert_vec(sky_color(&r), (0.5, 0.7, 1.0));
}

#[test]
fn sky_is_halfway_at_the_horizon() {
    let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
    assert_vec(sky_color(&r), (0.75, 0.85, 1.0));
}

#[test]
fn sky_ignores_ray_length_and_origin() {
    let short = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let long = Ray::new(Vec3::new(9.0, -4.0, 2.0), Vec3::new(0.0, 5.0, 0.0));
    let a = sky_color(&short);
    let b = sky_color(&long);
    assert_vec(b, (a.x, a.y, a.z));
}
