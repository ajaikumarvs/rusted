use iced::widget::text_editor;
use iced::{Element, Sandbox, Settings};


fn main() -> iced::Result{
   Editor::run(Settings::default())
}

struct Editor
{
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self
        {
            content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Rusted")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        text_editor(&self.content).on_action(Message::Edit).into()
    }
}