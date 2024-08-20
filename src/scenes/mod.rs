use agb::{
    display::{
        font::TextWriter,
        object::{OamManaged, Object},
    },
    input::ButtonController,
};

mod game;
mod main_menu;
mod options;

use game::GameScene;
use main_menu::MainMenuScene;
use options::OptionsScene;

pub trait Scene {
    fn new() -> Self;
    fn process(&mut self, controller: &ButtonController) -> Option<Scenes>;
    fn render(&mut self, object: &OamManaged, ball: &mut Object, writer: TextWriter);
}

#[derive(Clone, Copy, PartialEq)]
pub enum Scenes {
    MainMenuScene,
    OptionsScene,
    GameScene,
}

pub struct SceneManager {
    current_scene: Scenes,
    main_menu: Option<MainMenuScene>,
    options: Option<OptionsScene>,
    game: Option<GameScene>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            current_scene: Scenes::MainMenuScene,
            main_menu: Some(MainMenuScene::new()),
            options: None,
            game: None,
        }
    }

    pub fn switch(&mut self, scene: Scenes) {
        match scene {
            Scenes::MainMenuScene => {
                self.options = None;
                self.main_menu = Some(MainMenuScene::new());
                self.game = None;
            }
            Scenes::OptionsScene => {
                self.options = Some(OptionsScene::new());
                self.main_menu = None;
                self.game = None;
            }
            Scenes::GameScene => {
                self.game = Some(GameScene::new());
                self.main_menu = None;
                self.options = None;
            }
        }
        self.current_scene = scene;
    }

    pub fn render(&mut self, ball: &mut Object, object: &OamManaged, writer: TextWriter) {
        match self.current_scene {
            Scenes::MainMenuScene => {
                if self.main_menu.is_some() {
                    self.main_menu
                        .as_mut()
                        .unwrap()
                        .render(object, ball, writer);
                }
            }
            Scenes::GameScene => {
                if self.game.is_some() {
                    self.game.as_mut().unwrap().render(object, ball, writer);
                }
            }
            Scenes::OptionsScene => {
                if self.options.is_some() {
                    self.options.as_mut().unwrap().render(object, ball, writer);
                }
            }
        }
    }

    pub fn clear(&mut self, ball: &mut Object) {
        ball.hide();
    }

    pub fn process(&mut self, controller: &ButtonController) {
        let mut scene: Option<Scenes> = None;
        match self.current_scene {
            Scenes::MainMenuScene => {
                if self.main_menu.is_some() {
                    scene = self.main_menu.as_mut().unwrap().process(controller);
                }
            }
            Scenes::OptionsScene => {
                if self.options.is_some() {
                    scene = self.options.as_mut().unwrap().process(controller);
                }
            }
            Scenes::GameScene => {
                if self.game.is_some() {
                    scene = self.game.as_mut().unwrap().process(controller)
                }
            }
        }
        if scene.is_some_and(|s| s != self.current_scene) {
            self.switch(scene.unwrap());
        }
    }
}
