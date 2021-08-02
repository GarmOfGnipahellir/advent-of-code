// use nalgebra::Vector3;

// pub struct Moon {
//     pos: Vector3<i64>,
//     vel: Vector3<i64>,
// }

// impl Moon {
//     pub fn new(x: i64, y: i64, z: i64) -> Self {
//         Self {
//             pos: Vector3::new(x, y, z),
//             vel: Vector3::new(0, 0, 0),
//         }
//     }

//     pub fn apply_gravity(&mut self, other: &Moon) {
//         let delta: Vector3<i64> = other.pos - self.pos;
//         self.vel += Vector3::new(delta[0].signum(), delta[1].signum(), delta[2].signum());
//     }

//     pub fn get_energy(&self) -> i64 {
//         (self.pos[0].abs() + self.pos[1].abs() + self.pos[2].abs())
//             * (self.vel[0].abs() + self.vel[1].abs() + self.vel[2].abs())
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_moon() {
//         let mut moons = [
//             Moon::new(-1, 0, 2),
//             Moon::new(2, -10, -7),
//             Moon::new(4, -8, 8),
//             Moon::new(3, 5, -1),
//         ];

//         for moon in &mut moons {
//             for other in &moons {
//                 moon.apply_gravity(other);
//             }
//         }
//     }
// }