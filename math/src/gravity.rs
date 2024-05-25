/// Calculate the time it takes to fall from a height with gravity
pub fn fall_time(height: f32, g: f32) -> f32 {
    (2.0 * height / g).sqrt()
}

/// Calculate the height at certain progress of falling
pub fn height_at_fall_progress(initial_height: f32, g: f32, progress: f32) -> f32 {
    let time = fall_time(initial_height, g);
    initial_height - g * libm::powf(progress * time, 2.0) / 2.0
}

/// Calculate the gravity from the height and the time it takes to fall
pub fn gravity_from_fall_time(height: f32, time: f32) -> f32 {
    height * 2.0 / time
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    pub fn should_calculate_fall_time() {
        assert_relative_eq!(fall_time(1.0, 18.0), 1.0 / 3.0);
        assert_relative_eq!(fall_time(2.0, 18.0), 0.47140452);
        assert_relative_eq!(fall_time(3.0, 18.0), 0.57735026);
        assert_relative_eq!(fall_time(4.0, 18.0), 2.0 / 3.0);
    }

    #[test]
    pub fn should_calculate_height_at_fall_progress() {
        assert_relative_eq!(height_at_fall_progress(1.0, 18.0, 0.0), 1.0);
        assert_relative_eq!(height_at_fall_progress(1.0, 18.0, 0.25), 0.9375);
        assert_relative_eq!(height_at_fall_progress(1.0, 18.0, 0.5), 0.75);
        assert_relative_eq!(height_at_fall_progress(1.0, 18.0, 0.75), 0.4375);
        assert_relative_eq!(height_at_fall_progress(1.0, 18.0, 1.0), 0.0);
    }
}
