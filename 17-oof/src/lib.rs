pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(value: Vec<i32>) -> AveragedCollection {
        let mut res = AveragedCollection {
            list: value,
            average: 0.0,
        };
        res.update_average();
        res
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&mut self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Using Trait Objects That Allow for Values of Different Types

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // holds a vector of components that have the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/*******************************************************************************
// Equivalent code, BUT each Screen will need to contain all identical components

pub struct Screen<T: Draw> {
    // holds a vector of components that have the Draw trait
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
*******************************************************************************/

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw a button
    }
}
