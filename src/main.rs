struct Trip {
    depart: f32,
    from: String,
    to: String,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_custom() {
        let custom_trip = Trip {
            depart: 2_f32,
            from: String::from("home location"),
            to: String::from("work location"),
        };

        assert_eq!(custom_trip.depart, 2_f32);
        assert_eq!(custom_trip.from, String::from("home location"));
        assert_eq!(custom_trip.to, String::from("work location"));
    }

    #[test]
    fn purposefully_failing() {
        // mimicing catching a bug in the code to see if the Github automated CI/CD can pick it up and fail the build
        let custom_trip = Trip {
            depart: 2_f32,
            from: String::from("home location"),
            to: String::from("work location"),
        };

        assert_eq!(custom_trip.depart, 2_f32);
        assert_eq!(custom_trip.from, String::from("home location"));
        assert_ne!(custom_trip.to, String::from("wrong location"));
    }

    #[test]
    fn test_read_input() {
        let input_file_path = Path::new("tests/input_file/test.txt");

        assert!(input_file_path.is_file());
    }
}
