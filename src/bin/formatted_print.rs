fn main() {
    // Line comments with slashes.
    // Everything after the slashes is ignored by the compiler
    //

    // a comment here to break away from the doc comment
    println!("Hello, world!");
    println!("I am now a rustacean 🦀");

    println!("{} days in February", 28);

    println!("{0}, this is {1}. {1}, this is {0}", "Sankar", "Apoorva");

    // named arguments example
    {
        // block scoped inner-attribute is idiomatic in rust
        // make clippy happy with inner-attribute. silences compiler suggestion that the params can
        // easily be inlined. fine for example code.

        #![allow(clippy::print_literal)]
        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );
    }

    // format characters example
    {
        let val: String = format!("Base 2 (binary):        {:b}", 69420);
        println!("Base 10:                {}", 69420);
        println!("{val}");
        println!("Base 8 (octal):         {:o}", 69420);
        println!("Base 16 (hexadecimal):  {:x}", 69420);
    }

    // format justified
    {
        println!("{number:>5}", number = 1);
    }

    // number padding
    {
        println!("{number:0>5}", number = 1);
        println!("{number:0<5}", number = 2);
        // can pad with symbols
        println!("{number:a<5}", number = 3);
        println!("{number:k>5}", number = 4);
        println!("{number:💯>4}", number = 5);
        println!("{number:🚡<3}", number = 6);
    }

    // named arguments for format specifier with `$` suffix
    {
        println!("{number:%>width$}", number = 7, width = 8);
    }

    // expected failure
    {
        #![allow(clippy::print_literal)]

        println!("The name is {1}, {0} {1}", "James", "Bond");
    }

    // only types that implement fmt::Display can be formatted with `{}`. User-defined types do not
    // implement fmt::Display by default.

    // example of fmt::Display only formatters
    {
        #[allow(dead_code)]
        struct Structure(i32);

        // this will not compile because `Structure` does not implement fmt::Display.
        // un-comment to try it out
        // println!("This struct `{}` won't print...", Structure(3));
        // 💣 ^ try uncommenting this line
    }
    // ---

    // Rust 1.58 and above will directly capture the argument from a surrounding variable.
    {
        let number: f64 = 1.0;
        let width: usize = 5;
        println!("{number:*>width$}");
    }

    // - summary - inject values into print/format macros with {}
    // - you can reference surrounding variables by name inside {}
    // - you can pass in parameterized varargs to macros using name = value pairs, comma delimited
    // - by default, varargs to macros are zero-indexed numbers and can be accessed as such
    // - you can pad formatted values using {[value]:[fill][align][count]} expressions in strings
    // - sending in format characters {:o} {:b} {:x} etc will format to common number formats
    // - print macros can only handle types that implement fmt::Display. custom types that don't
    //      will fail at compile time
    // - named arguments in macros can be used as format specifier value by suffixing with the `$`
    //      character
    // - format! returns a string while println! outputs to stdout directly
    println!("End of formatting");
}
