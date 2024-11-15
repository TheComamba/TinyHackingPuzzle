use gui::Gui;

mod gui;
mod sql;

fn main() -> Result<(), iced::Error> {
    iced::application(
        "VSAS - Very secure authentication system",
        Gui::update,
        Gui::view,
    )
    .run()
}
