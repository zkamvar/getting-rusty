use std::thread;
// Closures:: capturing environment and scope

// Capturing the environment
//
// - T-shirt company gives away a special shirt to rando
// - people can add their favorite color to profile
// - if rando has no favorite color, they get the most stocked color
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Hey! This uses a closure without any arguments that returns
        // a valute T (the Some() variant of Option<T>).
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // the closures take the first type that use them.
    let untyped_1 = |x| x; // unknown now, but will take a string due to
                           // the closure later on
    let untyped_2 = |y| y; // will take a number later on

    println!(
        "{:?} and {:?} are from untyped closures",
        untyped_1(String::from("hello")),
        untyped_2(5)
    );

    println!("\n## Capturing References or Moving Ownership ------\n");
    {
        println!("---| Borrowing Immutably\n");
        let list = vec![1, 2, 3];
        println!("Before closure is defined: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    {
        println!("\n---| Borrowing mutably\n");
        let mut list = vec![1, 2, 3];
        println!("Before closure is defined: {:?}", list);

        let mut borrows_mutably = || list.push(7);

        // The print macro cannot borrow list because it was mutably
        // borrowed when the closure was defined.
        // println!("Before calling closure: {:?}", list);
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    {
        println!("\n---| Taking ownership with `move`\n");
        let list = vec![1, 2, 3];
        println!("Before closure is defined: {:?}", list);

        let move_it = move || println!("From thread: {:?}", list);

        thread::spawn(move_it).join().unwrap();
    }

    println!("## Moving Captured Values Out of Closures and the `Fn` traits");
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];
        let mut num_sort_operations = 0;
        // The signature is FnMut
        // sort_by_key(self mut f: ())
        list.sort_by_key(|r| {
            // This works because both the lest and the counter are mutable
            num_sort_operations += 1;
            r.width
        });
        println!("{:#?}", list);
        println!("Sort called {} times", num_sort_operations);
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.height
        });
        println!("{:#?}", list);
        println!("Sort called {} times", num_sort_operations);
    }
    println!("\n# Iterators ============\n");
    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }
    println!("\n## The `Iterator` Trait and the `next` Method ------\n");
    {
        println!("See Tests");

        println!("|--- Methods that Produce Other Iterators\n");

        let v1: Vec<i32> = vec![1, 2, 3];
        // map() itself produces an iterator over the modified values
        let _ = v1.iter().map(|x| x + 1);
        // we must collect it if we want to use the results
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
        println!("v1: {:?}\nv2: {:?}", v1, v2);
    }
}
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // NOTE:
    //   use v1.into_iter() to returned owned values
    //   use v1.iter_mut() to returned mutable references
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    println!("Methods that consume the iterator: .sum()");
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // total takes ownership of v1_iter
    let total: i32 = v1_iter.sum();
    // NOTE: the iterator creates immutable references to the original vector.
    // because these are refrences, this allows the iterator to not take
    // ownership
    println!("v1: {:?}", v1);

    assert_eq!(total, 6);
}
