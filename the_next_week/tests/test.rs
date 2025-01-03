use the_next_week::aabb::AABB;
use utils::interval::Interval;

#[test]
fn test_aabb_longest_axis() {
    let aabb = AABB {
        x: Interval::new(10.5, 14.2),
        y: Interval::new(1.5, 3.),
        z: Interval::new(4., 13.4),
    };

    assert_eq!(aabb.longest_axis(), 2);

    let aabb = AABB {
        x: Interval::new(10.5, 14.2),
        y: Interval::new(11.5, 32.),
        z: Interval::new(4., 13.4),
    };

    assert_eq!(aabb.longest_axis(), 1);
}