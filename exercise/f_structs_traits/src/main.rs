use std::fmt;

// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...

trait Bite {
    fn bite(&mut self);
}

// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
// #[derive(Debug)] // include this line right before your struct definition
// struct Grapes...

// #[derive(Debug)]
struct Grapes {
    amount_left: u32,
}

impl std::fmt::Debug for Grapes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{ Grapes amount_left: {} }}", self.amount_left)
    }
}

// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
// impl Bite for...

impl Bite for Grapes {
    fn bite(&mut self) {
        self.amount_left -= 1;
    }
}

impl fmt::Display for Grapes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} grapes)", self.amount_left)
    }
}

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);
    println!("ToString:    {:?}", grapes.to_string());

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)

    fn bunny_nibbles<T: Bite>(yummy: &mut T) {
        yummy.bite();
    }

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
    println!("ToString: {:?}", carrot.to_string());
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl fmt::Display for Carrot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}% left)", self.percent_left)
    }
}

impl Bite for Carrot {
    fn bite(&mut self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}
