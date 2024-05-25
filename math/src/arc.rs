use glam::{vec2, Vec2};

// https://math.stackexchange.com/questions/482751/how-do-i-move-through-an-arc-between-two-specific-points
// https://www.desmos.com/geometry/aofv2koj0k
/// Returns the center and radius of an arc that goes through `from` and `to` with a given `bulge`
pub fn arc_center_radius(from: Vec2, to: Vec2, bulge: f32, flip: bool) -> (Vec2, f32) {
    let distance = from.distance(to);

    let s = bulge * distance / 2.0;

    let radius = (libm::powf(distance / 2.0, 2.0) + libm::powf(s, 2.0)) / (2.0 * s);

    let arc = 4.0 * libm::atanf(bulge);

    let c_x = radius * libm::cosf(arc / 2.0 - std::f32::consts::FRAC_PI_2);
    let mut c_y = -radius * libm::sinf(arc / 2.0 - std::f32::consts::FRAC_PI_2);
    if flip {
        c_y = -c_y;
    }

    let center = normalize_or_right(to - from).rotate(vec2(c_x, c_y)) + from;
    (center, radius)
}

/// Returns the starting and ending angles of an arc that starts from `from` with a given `bulge` and radius
pub fn arc_angles(
    center: Vec2,
    radius: f32,
    bulge: f32,
    from: Vec2,
    to: Vec2,
    flip: bool,
) -> (f32, f32) {
    let arc = 4.0 * libm::atanf(bulge);
    let anchor = if flip { to } else { from };

    let starting_angle = if center.y < anchor.y {
        libm::acosf(((anchor.x - center.x) / radius).clamp(-1.0, 1.0))
    } else {
        libm::asinf(((anchor.x - center.x) / radius).clamp(-1.0, 1.0)) - std::f32::consts::FRAC_PI_2
    };
    let end_angle = starting_angle + arc;
    (starting_angle, end_angle)
}

fn normalize_or_right(vec: Vec2) -> Vec2 {
    let rcp = vec.length_recip();

    if rcp.is_finite() && rcp > 0.0 {
        vec * rcp
    } else {
        Vec2::X
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use proptest::{prop_assume, proptest};

    // https://www.desmos.com/geometry/d0kujz75r0
    #[test]
    fn arc_center_tests() {
        // Bulge 1
        let (center, radius) = arc_center_radius(vec2(0.0, 0.0), vec2(2.0, 0.0), 1.0, false);
        assert_relative_eq!(center.x, 1.0);
        assert_relative_eq!(center.y, 0.0);
        assert_relative_eq!(radius, 1.0);

        // Bulge 0.5
        let (center, radius) = arc_center_radius(vec2(0.0, 0.0), vec2(2.0, 0.0), 0.5, false);
        assert_relative_eq!(center.x, 1.0);
        assert_relative_eq!(center.y, 0.75);
        assert_relative_eq!(radius, 1.25);

        // Weird position
        let (center, radius) = arc_center_radius(vec2(0.61, 1.23), vec2(2.283, 1.67), 0.673, false);
        assert_relative_eq!(center.x, 1.3570827);
        assert_relative_eq!(center.y, 1.7899889);
        assert_relative_eq!(radius, 0.93365955);
    }

    proptest! {
        #[test]
        fn radius_tests(
            x1 in -1000f32..=1000f32,
            y1 in -1000f32..=1000f32,
            x2 in -1000f32..=1000f32,
            y2 in -1000f32..=1000f32,
            bulge in 0.01f32..=1f32,
            flip: bool,
        ) {
            let from = vec2(x1, y1);
            let to = vec2(x2, y2);
            prop_assume!(from.distance(to) > 0.01);
            let (center, radius) = arc_center_radius(from, to, bulge, flip);

            let epsilon = radius / 1e5;

            let perp_center = (from + to) / 2.0;

            // Check that radi are correct
            assert_relative_eq!(from.distance(center), radius, epsilon = epsilon);
            assert_relative_eq!(to.distance(center), radius, epsilon = epsilon);
            // Check that bulge is properly applied
            assert_relative_eq!(perp_center.distance(center), radius - (bulge * from.distance(to) / 2.0), epsilon = epsilon);
        }
    }
}
