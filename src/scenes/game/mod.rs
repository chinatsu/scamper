use agb::{
    display::{
        font::TextWriter,
        object::{OamManaged, Object},
    },
    input::{Button, ButtonController},
};
// use core::fmt::Write;

mod ball;
use ball::Ball;

use super::{Scene, Scenes};

struct GameState {
    ball: Ball,
}

pub struct GameScene {
    desired_scene: Scenes,
    game_state: GameState,
}

impl GameScene {
    fn handle_input(&mut self, controller: &ButtonController) {
        if controller.is_just_pressed(Button::START) {
            self.desired_scene = Scenes::MainMenuScene;
        }
        self.game_state.ball.update_x(controller.x_tri());
        self.game_state.ball.update_y(controller.y_tri());
        self.game_state.ball.boost(controller.is_pressed(Button::B))
    }

    fn update_logic(&mut self) {
        self.game_state.ball.run_logic();
    }

    fn render(&mut self, mut _writer: TextWriter, ball: &mut Object, _object: &OamManaged) {
        // writeln!(&mut writer, "Hello I am").unwrap();
        // writeln!(&mut writer, "Game!").unwrap();
        // writer.commit();
        self.game_state.ball.render(ball);
    }
}

impl Scene for GameScene {
    fn new() -> GameScene {
        GameScene {
            desired_scene: Scenes::GameScene,
            game_state: GameState { ball: Ball::new() },
        }
    }

    fn process(&mut self, controller: &ButtonController) -> Option<Scenes> {
        self.handle_input(controller);
        self.update_logic();

        Some(self.desired_scene)
    }

    fn render(&mut self, object: &OamManaged, ball: &mut Object, writer: TextWriter) {
        self.render(writer, ball, object);
    }
}
