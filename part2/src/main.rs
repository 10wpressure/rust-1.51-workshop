struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
    coastalness: String,
}

fn new_city(residents: u64, is_coastal: bool, coastalness: String) -> City {
    City {
        description: format!(
            "a *{}* city of approximately {} residents",
            coastalness, residents
        ),
        residents,
        is_coastal,
        coastalness,
    }
}

fn main() {
    let rustville: City = new_city(123, false, "coastal".to_string());

    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
