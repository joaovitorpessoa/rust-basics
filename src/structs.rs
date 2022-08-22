struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct TupleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn the_constructor_member(_first_name: &str, _last_name: &str) -> Person {
        return Person {
            first_name: _first_name.to_string(),
            last_name: _last_name.to_string(),
        };
    }

    fn get_full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }
}

pub fn execute() {
    let c: Color = Color {
        red: 255,
        blue: 255,
        green: 255,
    };
    println!(
        "Color {{ red: {red}, green: {green}, blue: {blue} }}",
        blue = c.blue,
        red = c.red,
        green = c.green
    );

    let tuple_c: TupleColor = TupleColor(255, 255, 255);
    println!(
        "TupleColor {{ red: {red}, green: {green}, blue: {blue} }}",
        blue = tuple_c.0,
        red = tuple_c.1,
        green = tuple_c.2
    );

    let p: Person = Person::the_constructor_member("João Vitor", "Pessoa");
    let full_name: String = p.get_full_name();
    println!("Full name: {:?}", full_name);
}
