use agb::{
    display::{font::TextWriter, object::OamManaged},
    input::ButtonController,
};

mod main_menu;
mod options;

use main_menu::MainMenuScene;
use options::OptionsScene;

pub trait Scene {
    fn new() -> Self;
    fn process(
        &mut self,
        controller: &ButtonController,
        object: &OamManaged,
        writer: TextWriter,
    ) -> Option<Scenes>;
}

#[derive(Clone, Copy, PartialEq)]
pub enum Scenes {
    MainMenuScene,
    OptionsScene,
}

pub struct SceneManager {
    current_scene: Scenes,
    main_menu: Option<MainMenuScene>,
    options: Option<OptionsScene>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            current_scene: Scenes::MainMenuScene,
            main_menu: Some(MainMenuScene::new()),
            options: None,
        }
    }

    pub fn switch(&mut self, scene: Scenes) {
        match scene {
            Scenes::MainMenuScene => {
                self.options = None;
                self.main_menu = Some(MainMenuScene::new());
            }
            Scenes::OptionsScene => {
                self.options = Some(OptionsScene::new());
                self.main_menu = None;
            }
        }
        self.current_scene = scene;
    }

    pub fn process(
        &mut self,
        controller: &ButtonController,
        object: &OamManaged,
        writer: TextWriter,
    ) {
        let mut scene: Option<Scenes> = None;
        match self.current_scene {
            Scenes::MainMenuScene => {
                if self.main_menu.is_some() {
                    scene = self
                        .main_menu
                        .as_mut()
                        .unwrap()
                        .process(controller, object, writer);
                }
            }
            Scenes::OptionsScene => {
                if self.options.is_some() {
                    scene = self
                        .options
                        .as_mut()
                        .unwrap()
                        .process(controller, object, writer);
                }
            }
        }
        if scene.is_some_and(|s| s != self.current_scene) {
            self.switch(scene.unwrap());
        }
    }
}
