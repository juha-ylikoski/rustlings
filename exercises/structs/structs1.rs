// structs1.rs
// Address all the TODOs to make the tests pass!

// Not really sure how this was intended to be done.
// Tests refer for struct(s) to store str but storing plain
// str in struct either requires defining its length somehow or lifetimes which
// I think are out of scope

struct ColorClassicStruct {
    name: String,
    hex: String
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            name: "green".to_string(),
            hex: "#00FF00".to_string()
        };

        assert_eq!(green.name, "green".to_string());
        assert_eq!(green.hex, "#00FF00".to_string());
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct("green".to_string(), "#00FF00".to_string());

        assert_eq!(green.0, "green".to_string());
        assert_eq!(green.1, "#00FF00".to_string());
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct {};
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
