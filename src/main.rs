// Rust UI - Iced

// modules ...
use ::iced::theme::Theme;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, container, text, Button, Text, TextInput};
use iced::{
    event, Background, Border, Element, Length, Padding, Sandbox, Settings, Shadow, Vector,
};

// main entry point ...
pub fn main() -> iced::Result {
    RustUI::run(Settings::default())
}

// define a new struct for RustUI
struct RustUI {
    // define the main variables => used when asking an instance ...
    theme: Theme,
    page: Page,              // use this to track the pages
    login_field: LoginField, // use this to set email and password
}

// define a seperate struct for login field
struct LoginField {
    email: String,
    password: String,
}

// define an enum for page => each var inside Page will create a new view/page
#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    Login,
    Register,
}

//define The message => these are similar to callback events/update triggers ...
#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,                       // use to change dark/light theme
    LoginSubmit,                       // use to submit email and password to console
    Router(String),                    // change the page depending on route
    LoginFieldChanged(String, String), // updates the input fields of email && password
}

// implement a sandbox for RustUI
impl Sandbox for RustUI {
    type Message = Message;

    // define the app constructor
    fn new() -> Self {
        Self {
            theme: Theme::Dark, // set default theme
            page: Page::Login,  // set default page
            login_field: LoginField {
                // create the login field for email and password
                email: String::new(),
                password: String::new(),
            },
        }
    }

    // define the app title
    fn title(&self) -> String {
        String::from("Rust UI - Iced")
    }

    // define the app theme function ...
    fn theme(&self) -> Theme {
        self.theme.clone() // return a copy of the theme
    }

    // define the update method ...
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {}
            Message::LoginFieldChanged(email, password) => {}
            Message::LoginSubmit => {}
            Message::Router(route) => {}
        }
    }

    // define the view method => this is where the UI goes ...
    fn view(&self) -> Element<Message> {
        let btn = submit_btn("Test Button", Message::ToggleTheme);

        container(btn)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

// setup the different UI components

// page footer ...

// login page / first page ...

// register page / second page ...

// input field ...
fn input_field(_placeholder: &str, _value: &str) -> TextInput<'static, Message> {
    TextInput::new(_placeholder, _value)
        .width(Length::Fixed(500.00))
        .padding(Padding::from(10.0))
        .line_height(text::LineHeight::Relative(1.75))
}

// submit button ...
fn submit_btn(name: &str, event: Message) -> Button<Message> {
    Button::new(
        text(name)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(21),
    )
    .on_press(event)
    .width(Length::Fixed(500.00))
    .height(Length::Fixed(45.00))
    // define the custom style
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}

// define a few structs for styling ...

// button styling
enum ButtonStyle {
    Standard,
    ThemeButton,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    // define the active trait => the default button
    fn active(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::Standard => iced::Color::from_rgb(0.059, 0.463, 0.702),
                Self::ThemeButton => iced::Color::default(),
            })),
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color: iced::Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                Self::ThemeButton => Shadow::default(),
            },
            text_color: {
                if theme == &Theme::Light {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::BLACK,
                    }
                } else {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::WHITE,
                    }
                }
            },
            ..Default::default()
        }
    }
}

// define the container style, similar to the button style ...
struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    //define the active trait as needed ...
    fn appearance(&self, _theme: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            border: Border::with_radius(5),
            background: None,
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
        }
    }
}
