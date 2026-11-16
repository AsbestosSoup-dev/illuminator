//! M2: ray-sphere intersection.

use illuminator::ray::Ray;
use illuminator::sphere::hit_sphere;
use illuminator::vec3::Vec3;

fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn direct_hit_returns_near_surface() {
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0));
    let t = hit_sphere(v(0.0, 0.0, -1.0), 0.5, &r).expect("should hit");
    assert!(approx(t, 0.5), "nearest surface is at t = 0.5, got {t}");
}

#[test]
fn miss_returns_none() {
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 1.0, 0.0));
    assert!(hit_sphere(v(0.0, 0.0, -1.0), 0.5, &r).is_none());
}

#[test]
fn near_miss_returns_none() {
    // Ray passes 0.6 above the centre of a radius-0.5 sphere.
    let r = Ray::new(v(0.0, 0.6, 0.0), v(0.0, 0.0, -1.0));
    assert!(hit_sphere(v(0.0, 0.0, -1.0), 0.5, &r).is_none());
}

#[test]
fn sphere_behind_the_ray_is_none() {
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0));
    assert!(hit_sphere(v(0.0, 0.0, 1.0), 0.5, &r).is_none());
}

#[test]
fn ray_starting_inside_hits_the_far_side() {
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0));
    let t = hit_sphere(v(0.0, 0.0, 0.0), 1.0, &r).expect("should hit from inside");
    assert!(approx(t, 1.0), "got {t}");
}

#[test]
fn unnormalised_direction_scales_t() {
    // Direction has length 2, so the surface at distance 0.5 is at t = 0.25.
    let r = Ray::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -2.0));
    let t = hit_sphere(v(0.0, 0.0, -1.0), 0.5, &r).expect("should hit");
    assert!(approx(t, 0.25), "got {t}");
}

#[test]
fn hit_point_lies_on_the_sphere() {
    let center = v(1.0, -0.5, -3.0);
    let radius = 1.25;
    let r = Ray::new(v(0.2, 0.1, 0.0), v(0.3, -0.1, -1.0));
    let t = hit_sphere(center, radius, &r).expect("should hit");
    let p = r.at(t);
    let d = ((p.x - center.x).powi(2) + (p.y - center.y).powi(2) + (p.z - center.z).powi(2)).sqrt();
    assert!(
        (d - radius).abs() < 1e-9,
        "hit point is {d} from centre, expected {radius}"
    );
}
