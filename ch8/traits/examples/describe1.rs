pub trait SelfDescribing {
    fn describe(&self) -> String;
}

fn describe_type<T: SelfDescribing>(t: &T) -> String {
    t.describe()
}

struct Dog();
struct Cat();

impl SelfDescribing for Dog {
    fn describe(&self) -> String {
        "happy little dog".into()
    }
}
impl SelfDescribing for Cat {
    fn describe(&self) -> String {
        "curious cat".into()
    }
}

fn main() {
    let dog = Dog();
    let cat = Cat();
    println!("I am a {}", describe_type(&dog));
    println!("I am a {}", describe_type(&cat));
}
