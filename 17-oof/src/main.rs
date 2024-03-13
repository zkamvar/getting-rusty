use oof::AveragedCollection;
use oof::{Button, Draw, Screen};
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("eyyy!");
    }
}
fn main() {
    println!("Hello, world!");
    let mut x = AveragedCollection::new(vec![69, 11]);
    x.add(420);
    x.add(666);
    x.remove();
    println!("Average {}", x.average());

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
