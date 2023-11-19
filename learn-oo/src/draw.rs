pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components {
            component.draw();
        }
    }
}

pub struct Button {
    pub weight: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "weight: {}, height: {}, label: {}",
            &self.weight, &self.height, &self.label
        );
    }
}
