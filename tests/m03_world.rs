//! M3: hit records, normals, and a world of spheres.
//! Only the public API from ROADMAP.md is used, so enum vs trait object is your call.

use illuminator::ray::Ray;
use illuminator::vec3::Vec3;
use illuminator::world::World;

fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

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
fn empty_world_hits_nothing() {
    let world = World::new();
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0));
    assert!(world.hit(&r, 0.0, f64::INFINITY).is_none());
}

#[test]
fn front_hit_fills_the_record() {
    let mut world = World::new();
    world.add_sphere(v(0.0, 0.0, -1.0), 0.5);
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0));
    let rec = world.hit(&r, 0.0, f64::INFINITY).expect("should hit");
    assert!(approx(rec.t, 0.5));
    assert_vec(rec.p, (0.0, 0.0, -0.5));
    assert_vec(rec.normal, (0.0, 0.0, 1.0));
    assert!(rec.front_face);
}

#[test]
fn normal_always_opposes_the_ray_even_from_inside() {
    let mut world = World::new();
    world.add_sphere(v(0.0, 0.0, 0.0), 1.0);
    let dir = v(0.0, 0.0, -1.0);
    let r = Ray::new(v(0.0, 0.0, 0.0), dir);
    let rec = world
        .hit(&r, 0.0, f64::INFINITY)
        .expect("should hit from inside");
    assert!(approx(rec.t, 1.0));
    assert_vec(rec.p, (0.0, 0.0, -1.0));
    assert!(
        !rec.front_face,
        "ray started inside, so this is not a front-face hit"
    );
    assert!(
        rec.normal.dot(dir) < 0.0,
        "stored normal must point against the ray"
    );
    assert_vec(rec.normal, (0.0, 0.0, 1.0));
}

#[test]
fn closest_sphere_wins_regardless_of_insertion_order() {
    let mut world = World::new();
    world.add_sphere(v(0.0, 0.0, -3.0), 0.5); // far one first
    world.add_sphere(v(0.0, 0.0, -1.0), 0.5);
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0));
    let rec = world.hit(&r, 0.0, f64::INFINITY).expect("should hit");
    assert!(
        approx(rec.t, 0.5),
        "expected the near sphere (t = 0.5), got {}",
        rec.t
    );
}

#[test]
fn t_max_excludes_far_hits() {
    let mut world = World::new();
    world.add_sphere(v(0.0, 0.0, -1.0), 0.5); // roots at t = 0.5 and t = 1.5
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0));
    assert!(world.hit(&r, 0.0, 0.4).is_none());
}

#[test]
fn t_min_skips_the_near_root_and_takes_the_far_one() {
    let mut world = World::new();
    world.add_sphere(v(0.0, 0.0, -1.0), 0.5); // roots at t = 0.5 and t = 1.5
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0));
    let rec = world
        .hit(&r, 0.6, f64::INFINITY)
        .expect("far root is in range");
    assert!(approx(rec.t, 1.5), "got {}", rec.t);
}

#[test]
fn normals_are_unit_length() {
    let mut world = World::new();
    world.add_sphere(v(0.5, 0.2, -2.0), 1.7);
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.2, 0.05, -1.0));
    let rec = world.hit(&r, 0.0, f64::INFINITY).expect("should hit");
    assert!(approx(rec.normal.length(), 1.0));
}
