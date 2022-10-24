fn print_x(x: u32) {
    println!("Value of x in lexical scope: {x}");
}

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;

        print_x(x);
    }

    print_x(x);

    let mut spaces = "   ";
    spaces = spaces.len();
}
