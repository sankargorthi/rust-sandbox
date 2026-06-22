#[allow(dead_code)]
// This struct cannot be printed either with `fmt:Display` or with `fmt::Debug`.
struct Unprintable(i32);

#[allow(dead_code)]
// The `derive` attribute automatically creates the implementation required to make this `struct`
// printable with `fmt:Debug`
#[derive(Debug)]
struct DebugPrintable(i32);

#[allow(dead_code)]
// Derive the `fmt::Debug` implementation for `Structure`. `Structure` is a structure which
// contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

#[allow(dead_code)]
// Put a `Structure` inside of the structure `Deep`. Make it printable also.
#[derive(Debug)]
struct Deep(Structure);

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // the problem with `derive` is there is no control over how the results look. What if I want
    // this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // pretty printing with rust
    {
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };

        println!("{:#?}", peter);
    }

    // Summary
    // ---
    // - adding debug nature to structures is done with #[derive(Debug)] attributes
    // - pretty printing debug objects is possible with {:#?} formatters
    // - there is no control over how the results look with derive(Debug)
}
