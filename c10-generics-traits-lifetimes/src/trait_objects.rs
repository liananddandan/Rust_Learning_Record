trait Draw {
    fn draw(&self);
}

struct Button { pub label: String }
struct Text { pub value: String }

impl Draw for Button {
    fn draw(&self) { println!("[Button] {}", self.label); }
}

impl Draw for Text {
    fn draw(&self) { println!("[Text] {}", self.value); }
}

pub fn run() {
    let ui: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: "OK".into() }),
        Box::new(Text { value: "Hello".into() }),
    ];

    for widget in ui {
        widget.draw(); // dynamic dispatch via vtable
    }
}