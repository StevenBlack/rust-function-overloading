#[derive(Debug)]
struct Dog {
    name: String,
}

impl From<String> for Dog {
    fn from(value: String) -> Dog {
        Dog {name: value}
    }
}

impl From<i32> for Dog {
    fn from(value: i32) -> Dog {
        Dog {name: String::from(value.to_string())}
    }
}

trait Bark<T> {
    fn bark(&self, s: T);
}

impl Bark<String> for Dog {
    fn bark(&self, s: String) {
        let _ = s;
        println!("{} bark on string type", self.name);
    }
}

impl Bark<i32> for Dog {
    fn bark(&self, s: i32) {
        let _ = s;
        println!("{} bark on int type", self.name);
    }
}

fn main() {
    let d = Dog::from("toby".to_string());
    d.bark("string".to_string());
    d.bark(4);
}
