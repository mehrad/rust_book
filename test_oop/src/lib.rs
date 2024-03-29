pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct GenericTypedScreen< T: Draw > {
    pub components: Vec<T>,
}

impl<T> GenericTypedScreen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button { 
    pub width: u32, 
    pub height: u32, 
    pub label: String 
}

pub struct SelectBox { 
    pub width: u32, 
    pub height: u32, 
    pub options: Vec<String> 
}

pub struct TextField { 
    pub width: u32, 
    pub height: u32, 
    pub text: String 
}


impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a Button with width: {}, height: {}, label: {}", self.width, self.height, self.label);
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox with width: {}, height: {}, options: {:?}", self.width, self.height, self.options);
    }
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing a TextField with width: {}, height: {}, text: {}", self.width, self.height, self.text);
    }
}