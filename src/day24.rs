use std::str::FromStr;

use itertools::Itertools;
use nalgebra::{Matrix, SMatrix, Vector3, Vector6};
use num::Zero;

use crate::common::Solution;

pub enum Day24 {}

impl Solution for Day24 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let hailstones = Hailstones::from_lines(lines);
        hailstones
            .forward_collisions_within_xy_range(200000000000000.0, 400000000000000.0)
            .to_string()
    }
}

pub enum Day24P2 {}

macro_rules! set_view {
    ($view:expr, $other:expr) => {
        for row in 0..$other.nrows() {
            $view.set_row(row, &$other.row(row));
        }
    };
}

impl Solution for Day24P2 {
    // this is basically totally copied from https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepu26z
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let hailstones = Hailstones::from_lines(lines).hailstones;
        let mut matrix: SMatrix<f64, 6, 6> = Matrix::zero();
        let mut rhs: Vector6<f64> = Matrix::zero();
        *rhs.fixed_rows_mut::<3>(0) = *(-hailstones[0].position.cross(&hailstones[0].velocity)
            + hailstones[1].position.cross(&hailstones[1].velocity));

        *rhs.fixed_rows_mut::<3>(3) = *(-hailstones[0].position.cross(&hailstones[0].velocity)
            + hailstones[2].position.cross(&hailstones[2].velocity));
        {
            let mut view = matrix.fixed_view_mut::<3, 3>(0, 0);
            let values: SMatrix<f64, 3, 3> =
                hailstones[0].velocity.cross_matrix() - hailstones[1].velocity.cross_matrix();
            set_view!(view, values);
        }
        {
            let mut view = matrix.fixed_view_mut::<3, 3>(3, 0);
            let values: SMatrix<f64, 3, 3> =
                hailstones[0].velocity.cross_matrix() - hailstones[2].velocity.cross_matrix();
            set_view!(view, values);
        }
        {
            let mut view = matrix.fixed_view_mut::<3, 3>(0, 3);
            let values: SMatrix<f64, 3, 3> =
                -hailstones[0].position.cross_matrix() + hailstones[1].position.cross_matrix();
            set_view!(view, values);
        }

        {
            let mut view = matrix.fixed_view_mut::<3, 3>(3, 3);
            let values: SMatrix<f64, 3, 3> =
                -hailstones[0].position.cross_matrix() + hailstones[2].position.cross_matrix();
            set_view!(view, values);
        }

        let result = matrix.try_inverse().unwrap() * rhs;
        let position: Vector3<f64> = Vector3::new(result.x, result.y, result.z);

        (position.x + position.y + position.z).round().to_string()
    }
}

struct Hailstones {
    hailstones: Vec<PositionVelocity>,
}

impl Hailstones {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        Hailstones {
            hailstones: lines.map(|line| line.as_ref().parse().unwrap()).collect(),
        }
    }

    pub fn forward_collisions_within_xy_range(&self, min_val: f64, max_val: f64) -> usize {
        self.hailstones
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                let time = a.get_xy_intersection_times(b);
                if let Some((a_time, b_time)) = time {
                    let a_pos = a.position_at_time(a_time);
                    let b_pos = b.position_at_time(b_time);
                    a_time >= 0.0
                        && b_time >= 0.0
                        && a_pos.x >= min_val
                        && a_pos.x <= max_val
                        && a_pos.y >= min_val
                        && a_pos.y <= max_val
                        && b_pos.x >= min_val
                        && b_pos.x <= max_val
                        && b_pos.y >= min_val
                        && b_pos.y <= max_val
                } else {
                    false
                }
            })
            .count()
    }
}

struct PositionVelocity {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl PositionVelocity {
    pub fn get_xy_intersection_times(&self, other: &Self) -> Option<(f64, f64)> {
        let self_slope = self.xy_slope();
        let other_slope = other.xy_slope();

        if approx_equal(self_slope, other_slope) {
            return None;
        }

        let self_intercept = self.y_intercept();
        let other_intercept = other.y_intercept();

        let intersection_x_val =
            (other_intercept.y - self_intercept.y) / (self_slope - other_slope);

        Some((
            self.time_until_x_value(intersection_x_val),
            other.time_until_x_value(intersection_x_val),
        ))
    }

    pub fn y_intercept(&self) -> Vector3<f64> {
        let time = -self.position.x / self.velocity.x;
        self.position_at_time(time)
    }

    pub fn xy_slope(&self) -> f64 {
        self.velocity.y / self.velocity.x
    }

    pub fn position_at_time(&self, time: f64) -> Vector3<f64> {
        Vector3::new(
            self.position.x + self.velocity.x * time,
            self.position.y + self.velocity.y * time,
            self.position.z + self.velocity.z * time,
        )
    }

    fn time_until_x_value(&self, x_position: f64) -> f64 {
        (x_position - self.position.x) / self.velocity.x
    }
}

impl FromStr for PositionVelocity {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (positions, velocities) = line.split_once(" @ ").ok_or_else(|| {
            anyhow::Error::msg("Could not split into positions and velocities")
                .context(line.to_string())
        })?;
        let position_parts = positions.split(", ").collect::<Vec<_>>();

        let [px, py, pz] = position_parts.as_slice() else {
            return Err(
                anyhow::Error::msg("Could not split positions").context(positions.to_string())
            );
        };
        let velocity_parts = velocities.split(", ").collect::<Vec<_>>();

        let [vx, vy, vz] = velocity_parts.as_slice() else {
            return Err(
                anyhow::Error::msg("Could not split velocities").context(velocities.to_string())
            );
        };

        Ok(PositionVelocity {
            position: Vector3::<f64>::new(
                px.trim().parse()?,
                py.trim().parse()?,
                pz.trim().parse()?,
            ),
            velocity: Vector3::<f64>::new(
                vx.trim().parse()?,
                vy.trim().parse()?,
                vz.trim().parse()?,
            ),
        })
    }
}

fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.000000001
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day24::{Day24P2, Hailstones};

    const EXAMPLE_INPUT: &str = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    #[test]
    fn test_example() {
        let hailstones = Hailstones::from_lines(EXAMPLE_INPUT.lines());

        assert_eq!(hailstones.forward_collisions_within_xy_range(7.0, 27.0), 2)
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day24P2::solve(EXAMPLE_INPUT.lines()), "47")
    }
}
