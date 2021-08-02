fn steps_until_rep_on_axis(mut positions: Vec<i64>) -> i64 {
    let mut velocities: Vec<i64> = vec![0; positions.len()];
    let vel_target = velocities.clone();
    let mut steps: i64 = 0;
    loop {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                velocities[i] += (positions[j] - positions[i]).signum();
                velocities[j] += (positions[i] - positions[j]).signum();
            }
        }
        for i in 0..positions.len() {
            positions[i] += velocities[i];
        }

        steps += 1;

        if velocities == vel_target {
            break;
        }
    }
    steps * 2
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn lcm3(a: i64, b: i64, c: i64) -> i64 {
    lcm(a, lcm(b, c))
}

fn main() {
    let input = shared::read_string("day_12/data/input.txt").unwrap();
    let positions: Vec<Vec<_>> = input
        .split('\n')
        .map(|l| &l[1..l.len() - 1])
        .map(|l| {
            l.split(", ")
                .map(|v| &v[2..v.len()])
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let x_positions: Vec<i64> = positions.iter().map(|p| p[0]).collect();
    let y_positions: Vec<i64> = positions.iter().map(|p| p[1]).collect();
    let z_positions: Vec<i64> = positions.iter().map(|p| p[2]).collect();

    let x_steps = steps_until_rep_on_axis(x_positions);
    println!("{}", x_steps);
    let y_steps = steps_until_rep_on_axis(y_positions);
    println!("{}", y_steps);
    let z_steps = steps_until_rep_on_axis(z_positions);
    println!("{}", z_steps);

    println!("{}", lcm3(x_steps, y_steps, z_steps));
}
