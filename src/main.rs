mod wm;

use cosmic::app::Core;
use cosmic::iced::Length;
use cosmic::{Action, Application, Element, Task};
use wm::WindowManager;

fn main() -> cosmic::iced::Result {
    let mut single_shot = false;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-s" | "--single-shot" => single_shot = true,
            "-h" | "--help" => {
                println!("Usage: cosmic-ext-showdesktop [-s]");
                return Ok(());
            }
            _ => {
                eprintln!("Unknown flag: {arg}");
                eprintln!("Usage: cosmic-ext-showdesktop [-s]");
                return Ok(());
            }
        }
    }

    if single_shot {
        match WindowManager::new() {
            Ok(wm) => {
                if let Err(err) = wm.toggle_persistent() {
                    eprintln!("single-shot toggle failed: {err}");
                }
            }
            Err(err) => {
                eprintln!("single-shot startup failed: {err}");
            }
        }

        return Ok(());
    }

    cosmic::applet::run::<Applet>(())
}

#[derive(Debug, Clone)]
enum Message {
    TogglePressed,
}

struct Applet {
    core: Core,
    wm: WindowManager,
}

impl Application for Applet {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.example.CosmicShowDesktop";

    fn init(core: Core, _flags: ()) -> (Self, Task<Action<Self::Message>>) {
        let wm = WindowManager::new()
            .unwrap_or_else(|err| panic!("failed to init window manager backend: {err}"));

        (
            Self {
                core,
                wm,
            },
            Task::none(),
        )
    }

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::TogglePressed => {
                let _ = self.wm.toggle();
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let button = self
            .core
            .applet
            .icon_button("view-grid-symbolic")
            .on_press(Message::TogglePressed)
            .padding([4, 6])
            .width(Length::Shrink)
            .height(Length::Shrink);

        button.into()
    }
}
