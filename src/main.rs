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
}
