use iced::widget::{Button, Column, Text, TextInput};
use iced::{Element, Sandbox};

use crate::sql::check_user;

pub(crate) struct Gui {
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
            GuiMessage::Login => self.validate_login(),
            GuiMessage::AddUser => self.add_user(),
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
            .push(Button::new(Text::new("Add user")).on_press(GuiMessage::AddUser))
            .push(Text::new(&self.error_message))
            .padding(30)
            .spacing(15)
            .into()
    }
}

impl Gui {
    fn set_password_input(&mut self, pw: String) {
        if pw.chars().any(|c| !c.is_digit(10)) {
            self.error_message = String::from("Password must only contain digits");
        } else if pw.len() > 5 {
            self.error_message = String::from("Password can be at most 5 characters long");
        } else {
            self.error_message = String::new();
            self.password_input = pw;
        }
    }

    fn validate_login(&mut self) {
        match check_user(&self.user_input, &self.password_input) {
            Ok(true) => self.error_message = "Authentication successful.".to_string(),
            Ok(false) => self.error_message = "Invalid credentials.".to_string(),
            Err(e) => self.error_message = format!("Error: {}", e),
        }
    }

    fn add_user(&mut self) {
        if self.user_input.is_empty() || self.password_input.is_empty() {
            self.error_message = String::from("User and password must not be empty");
        } else {
            match crate::sql::add_user(&self.user_input, &self.password_input) {
                Ok(_) => {
                    self.error_message = String::from("User added successfully");
                    self.user_input = String::new();
                    self.password_input = String::new();
                }
                Err(e) => self.error_message = format!("Error: {}", e),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    UserInput(String),
    PasswordInput(String),
    Login,
    AddUser,
}
