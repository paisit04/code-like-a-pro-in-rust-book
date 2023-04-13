macro_rules! print_what_it_is {
    () => {
        println!("A macro with no arguments")
    };
    ($e:expr) => {
        println!("A macro with an expression")
    };
    ($s:stmt) => {
        println!("A macro with a statement")
    };
    ($e:expr, $s:stmt) => {
        println!("An expression followed by a statement")
    };
}

macro_rules! special_println {
  ($($arg:tt)*) => {
    println!("Printed specially: {}", $($arg)*);
  };
}

macro_rules! var_print {
  ($($v:ident),*) => {
      println!(concat!($(concat!(stringify!($v),"={:?} ")),*), $($v),*)
  };
}

fn main() {
    print_what_it_is!();
    print_what_it_is!({});
    print_what_it_is!(;);
    print_what_it_is!({}, ;);

    special_println!("hello world!");

    let counter = 7;
    let gauge = 3.14;
    let name = "Peter";
    var_print!(counter, gauge, name);
}
