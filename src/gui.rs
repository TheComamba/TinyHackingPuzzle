use iced::widget::{Button, Column, Text, TextInput};
use iced::{Element, Sandbox};

use crate::sql::check_user;

pub(crate) struct Gui {
    user: String,
    password: String,
    message: String,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui {
            user: String::new(),
            password: String::new(),
            message: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Very secure authentication system")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            GuiMessage::UserInput(user) => self.user = user,
            GuiMessage::PasswordInput(password) => self.set_password_input(password),
            GuiMessage::Login => self.validate_login(),
            GuiMessage::AddUser => self.add_user(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("User:"))
            .push(
                TextInput::new("", &self.user)
                    .on_input(GuiMessage::UserInput)
                    .on_submit(GuiMessage::Login)
                    .padding(10)
                    .size(20),
            )
            .push(Text::new("Password:"))
            .push(
                TextInput::new("", &self.password)
                    .on_input(GuiMessage::PasswordInput)
                    .on_submit(GuiMessage::Login)
                    .padding(10)
                    .size(20),
            )
            .push(Button::new(Text::new("Login")).on_press(GuiMessage::Login))
            .push(Button::new(Text::new("Add user")).on_press(GuiMessage::AddUser))
            .push(Text::new(&self.message))
            .padding(30)
            .spacing(15)
            .into()
    }
}

impl Gui {
    fn set_password_input(&mut self, pw: String) {
        if pw.chars().any(|c| !c.is_digit(10)) {
            self.message = String::from("Password must only contain digits");
        } else if pw.len() > 5 {
            self.message = String::from("Password can be at most 5 characters long");
        } else {
            self.message = String::new();
            self.password = pw;
        }
    }

    fn validate_login(&mut self) {
        match check_user(&self.user, &self.password) {
            Ok(true) => self.message = "Authentication successful.".to_string(),
            Ok(false) => self.message = "Invalid credentials.".to_string(),
            Err(e) => self.message = format!("Error: {}", e),
        }
    }

    fn add_user(&mut self) {
        if self.user.is_empty() || self.password.is_empty() {
            self.message = String::from("User and password must not be empty");
        } else {
            match crate::sql::add_user(&self.user, &self.password) {
                Ok(_) => {
                    self.message = String::from("User added successfully");
                    self.user = String::new();
                    self.password = String::new();
                }
                Err(e) => self.message = format!("Error: {}", e),
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
