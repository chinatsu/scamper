use core::fmt::Write;

use super::{Scene, Scenes};

use agb::display::{font::TextWriter, object::OamManaged};
use agb::input::{Button, ButtonController, Tri};

#[derive(PartialEq)]
enum Menu {
    Start,
    Options,
}

pub struct MainMenuScene {
    menu_selection: Menu,
    desired_scene: Scenes,
}

impl MainMenuScene {
    fn handle_input(&mut self, controller: &ButtonController) {
        self.change_item(controller.just_pressed_x_tri());
        if controller.is_just_pressed(Button::A) {
            match self.menu_selection {
                Menu::Options => {
                    self.desired_scene = Scenes::OptionsScene;
                }
                _ => {
                    self.desired_scene = Scenes::MainMenuScene;
                }
            }
        }
    }

    fn change_item(&mut self, direction: Tri) {
        if self.menu_selection == Menu::Start && direction == Tri::Positive {
            self.menu_selection = Menu::Options;
            return;
        }
        if self.menu_selection == Menu::Options && direction == Tri::Negative {
            self.menu_selection = Menu::Start;
            return;
        }
    }

    fn render(&self, _object: &OamManaged, mut writer: TextWriter) {
        match self.menu_selection {
            Menu::Start => writeln!(&mut writer, "Start").unwrap(),
            Menu::Options => writeln!(&mut writer, "Options").unwrap(),
        }
        writer.commit();
    }
}

impl Scene for MainMenuScene {
    fn new() -> MainMenuScene {
        MainMenuScene {
            menu_selection: Menu::Start,
            desired_scene: Scenes::MainMenuScene,
        }
    }

    fn process(
        &mut self,
        controller: &ButtonController,
        object: &OamManaged,
        writer: TextWriter,
    ) -> Option<Scenes> {
        self.handle_input(&controller);
        self.render(object, writer);

        Some(self.desired_scene)
    }
}
