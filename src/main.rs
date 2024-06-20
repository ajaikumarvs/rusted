
use iced::widget::{column, container, text_editor, Text};

use iced::{Theme, Element, Sandbox, Settings};


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
        let input = text_editor(&self.content).on_action(Message::Edit);

        let position = {
            let (line, column) = self.content.cursor_position();
            //text(format:("{}:{}", line + 1, column + 1))
            Text::new(format!("{}:{}", line + 1, column + 1))
            
        };

        container(column![input, position]).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }
}