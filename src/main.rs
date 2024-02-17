use iced::executor;
use iced::widget::{button, column, container};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use tokio_tungstenite::connect_async;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    let addr = "ws://0.0.0.0:4000".to_string();
    // let url = url::Url::parse(&addr).unwrap();

    let (ws_stream, _) = connect_async(addr).await.expect("Failed to connect");
    info!("WebSocket handshake has been successfully completed");

    AcuityDEX::run(Settings::default())
}

#[derive(Default)]
struct AcuityDEX {
    show_confirm: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Confirm,
    Exit,
}

impl Application for AcuityDEX {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Acuity DEX")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Confirm => window::close(window::Id::MAIN),
            Message::Exit => {
                self.show_confirm = true;

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if self.show_confirm {
            column![
                "Are you sure you want to exit?",
                button("Yes, exit now")
                    .padding([10, 20])
                    .on_press(Message::Confirm),
            ]
        } else {
            column![
                "Click the button to exit",
                button("Exit").padding([10, 20]).on_press(Message::Exit),
            ]
        }
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
