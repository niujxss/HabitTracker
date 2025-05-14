use iced::{Sandbox, Settings};
use iced::widget::{button, Button, Column, Text};  // 从 widget 模块导入组件

struct Counter {
    value : i32,
}

#[derive(Debug, Clone, Copy)]
enum CounterMessage {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Self {
        Counter {
            value:0,
        }
    }

    fn title(&self) -> String {
        String::from("计数器测试")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Decrement => self.value -= 1,
            CounterMessage::Increment => self.value += 1,
        }
    }

    fn view(& self) -> iced::Element<'_, Self::Message> {
        Column::new()  //垂直画布
                    .push(Button::new("+").on_press(CounterMessage::Increment))
                    .push(Button::new("-").on_press(CounterMessage::Decrement))
                    .push(Text::new(self.value.to_string()))
                    .into()
    }


}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
