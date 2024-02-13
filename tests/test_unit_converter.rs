use rustymath::unit_converter::*;


#[test]
fn test_unit_conversion() {
    let meters = Unit { name: "meters", conversion_factor: 1.0 };
    let feet = Unit { name: "feet", conversion_factor: 0.3048 };

    let value_in_meters = 10.0;
    let value_in_feet = convert_unit(value_in_meters, &meters, &feet);
    assert_eq!(value_in_feet, 32.80839895013123);

    let value_in_feet = 100.0;
    let value_in_meters = convert_unit(value_in_feet, &feet, &meters);
    assert_eq!(value_in_meters, 30.48);
}
