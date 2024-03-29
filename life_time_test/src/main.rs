fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    {

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    println!("string1 is {}", string1);
    println!("string2 is {}", string2);

}

fn longest<'a>(x:  &str, y: &'a str) -> &'a str {
    let z:&'a str  = y;

    z
}