struct Car{
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = Car{
        color,
        transmission,
        convertible,
        mileage: 0
    };

    // Factory's Quality Control Department says that new cars must always have zero mileage!
    assert_eq!(car.mileage, 0);
    return car;
}

fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = numbers.1;

    assert_eq!(
        2, second,
        "This is not the 2nd number in the tuple: {}",
        second
    )
}

fn indexing_array() {
    let characters = ['a', 'b', 'c', 'd', 'e'];
    let letter_d = characters[3];

    assert_eq!(
        'd', letter_d,
        "This is not the character for the letter d: {}",
        letter_d
    )
}

fn main() {
    let client_request_1 = car_factory(String::from("Red"), Transmission::Manual, false);
    assert_eq!(client_request_1.color, "Red");
    assert_eq!(client_request_1.transmission, Transmission::Manual);
    assert_eq!(client_request_1.convertible, false);

    let client_request_2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    assert_eq!(client_request_2.color, "Silver");
    assert_eq!(client_request_2.transmission, Transmission::Automatic);
    assert_eq!(client_request_2.convertible, true);

    let client_request_2 = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    assert_eq!(client_request_2.color, "Yellow");
    assert_eq!(client_request_2.transmission, Transmission::SemiAuto);
    assert_eq!(client_request_2.convertible, false);

    indexing_tuple();
    indexing_array();
}