use agb::{
    display::{font::TextWriter, object::OamManaged},
    input::{Button, ButtonController},
};
use core::fmt::Write;

use super::{Scene, Scenes};

pub struct OptionsScene {
    desired_scene: Scenes,
}

impl OptionsScene {
    fn handle_input(&mut self, controller: &ButtonController) {
        if controller.is_just_pressed(Button::B) {
            self.desired_scene = Scenes::MainMenuScene;
        }
    }

    fn render_inner(&mut self, mut writer: TextWriter) {
        writeln!(&mut writer, "Hello I am").unwrap();
        writeln!(&mut writer, "Options!").unwrap();
        writer.commit()
    }
}

impl Scene for OptionsScene {
    fn new() -> OptionsScene {
        OptionsScene {
            desired_scene: Scenes::OptionsScene,
        }
    }

    fn process(
        &mut self,
        controller: &ButtonController
    ) -> Option<Scenes> {
        self.handle_input(controller);

        Some(self.desired_scene)
    }

    fn render(&mut self, _object: &OamManaged, _ball: &mut agb::display::object::Object, writer: TextWriter) {
        self.render_inner(writer);
    }
}
