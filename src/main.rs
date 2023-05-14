use iced::widget::{Button, Column, Text, TextInput};
use iced::{Element, Sandbox, Settings};

fn main() {
    Gui::run(Settings::default()).unwrap();
}

struct Gui {
    user_input: String,
    password_input: String,
    error_message: String,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui {
            user_input: String::new(),
            password_input: String::new(),
            error_message: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Very secure authentication system")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            GuiMessage::UserInput(user_input) => self.user_input = user_input,
            GuiMessage::PasswordInput(password_input) => self.set_password_input(password_input),
            GuiMessage::Login => {
                println!("User: {}", self.user_input);
                println!("Password: {}", self.password_input);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("User:"))
            .push(
                TextInput::new("", &self.user_input)
                    .on_input(GuiMessage::UserInput)
                    .padding(10)
                    .size(20),
            )
            .push(Text::new("Password:"))
            .push(
                TextInput::new("", &self.password_input)
                    .on_input(GuiMessage::PasswordInput)
                    .padding(10)
                    .size(20),
            )
            .push(Button::new(Text::new("Login")).on_press(GuiMessage::Login))
            .push(Text::new(&self.error_message))
            .padding(30)
            .spacing(15)
            .into()
    }
}

impl Gui {
    fn set_password_input(&mut self, pw: String) {
        //check if password contains any non-digit characters
        if pw.chars().any(|c| !c.is_digit(10)) {
            self.error_message = String::from("Password must only contain digits");
        } else if pw.len() > 5 {
            self.error_message = String::from("Password can be at most 5 characters long");
        } else {
            self.error_message = String::new();
            self.password_input = pw;
        }
    }
}

#[derive(Debug, Clone)]
enum GuiMessage {
    UserInput(String),
    PasswordInput(String),
    Login,
}
