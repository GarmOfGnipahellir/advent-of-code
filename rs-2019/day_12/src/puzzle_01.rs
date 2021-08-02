use nalgebra::{Matrix3x4, Vector3};

fn main() {
    let mut pos = Matrix3x4::from_columns(&[
        Vector3::new(13, -13, -2),
        Vector3::new(16, 2, -15),
        Vector3::new(7, -18, -12),
        Vector3::new(-3, -8, -8),
    ]);
    let mut vel = Matrix3x4::zeros();

    for _ in 0..1000 {
        for i in 0..4 {
            for j in i + 1..4 {
                let delta: Vector3<i32> = Vector3::from_iterator(
                    (pos.column(j) - pos.column(i))
                        .iter()
                        .map(|x| i32::signum(*x)),
                );
                vel.set_column(i, &(vel.column(i) + delta));
                vel.set_column(j, &(vel.column(j) - delta));
            }
        }
        pos += vel;
    }

    let pos = pos.column_iter()
        .map(|c| c.iter().map(|v| v.abs()).sum::<i32>())
        .collect::<Vec<i32>>();
    let vel = vel.column_iter()
        .map(|c| c.iter().map(|v| v.abs()).sum::<i32>())
        .collect::<Vec<i32>>();

    let energy = pos.iter().zip(vel.iter()).map(|(p, v)| p * v).sum::<i32>();

    println!("{}", energy);
}
