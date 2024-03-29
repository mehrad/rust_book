use test_oop::Draw;
use test_oop::{Button, SelectBox, TextField, Screen, GenericTypedScreen};

struct NinePatch {
    width: u32,
    height: u32,
    content: String,
}

impl Draw for NinePatch {
    fn draw(&self) {
        println!("Drawing a NinePatch with width: {}, height: {}, content: {}", self.width, self.height, self.content);
    }
}

fn main() {
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
            Box::new(TextField {
                width: 10,
                height: 10,
                text: String::from("Hello"),
            }),
        ],
    };

    screen.run();

    // uncomment this block to see the error
    // which is the reason why we need to use Box<dyn Draw> in Screen struct
    // let screen = GenericTypedScreen {
    //     components: vec![
    //         SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No"),
    //             ],
    //         },
    //         SelectBox {
    //             width: 50,
    //             height: 10,
    //             options: vec![
    //                 String::from("OK"),
    //             ],
    //         },
    //         TextField {
    //             width: 10,
    //             height: 10,
    //             text: String::from("Hello"),
    //         },
    //         NinePatch {
    //             width: 20,
    //             height: 20,
    //             content: String::from("This is a NinePatch"),
    //         },
    //     ],
    // };

    screen.run();
}

