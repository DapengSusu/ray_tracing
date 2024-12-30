use nalgebra::Vector3;

fn main() {
    // let v1 = Vector3::new(1., 2., 3.);
    // let v2 = Vector3::new(4., 5., 6.);
    // println!("v: {}", v1 + v2);
    // println!("v: {}", v1 - v2);

    // // 向量点积
    // println!("v: {}", v1.dot(&v2));
    // // 向量叉积
    // println!("v: {}", v1.cross(&v2));
    // // 向量归一化
    // println!("v: {}", v1.scale(1. / v2.magnitude()));

    let v1 = Vector3::new(3., 4., 5.);

    // println!("v: {}", v1.norm_squared());
    // println!("v: {}", v1.magnitude());
    // println!("v: {}", v1.norm());
    // println!("v: {}", v1.normalize());

    println!("x={},y={},z={}", v1.x, v1.y, v1.z);
}