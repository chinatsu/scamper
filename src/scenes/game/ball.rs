use agb::{display::object::Object, input::Tri};

pub struct Ball {
    pub x: i32,
    pub y: i32,
    x_velocity: i32,
    y_velocity: i32,
    boost: i32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            x: 50,
            y: 50,
            x_velocity: 0,
            y_velocity: 0,
            boost: 1
        }
    }

    pub fn boost(&mut self, boost: bool) {
        if boost {
            self.boost = 4;
        } else {
            self.boost = 1;
        }
    }

    pub fn update_x(&mut self, tri: Tri) {
        match tri {
            Tri::Positive => {
                self.x_velocity = 1;
            }
            Tri::Negative => {
                self.x_velocity = -1;
            }
            Tri::Zero => {
                self.x_velocity = 0;
            }
        }
    }

    pub fn update_y(&mut self, tri: Tri) {
        match tri {
            Tri::Positive => {
                self.y_velocity = 1;
            }
            Tri::Negative => {
                self.y_velocity = -1;
            }
            Tri::Zero => {
                self.y_velocity = 0;
            }
        }
    }

    pub fn run_logic(&mut self) {
        self.x = (self.x + self.x_velocity * self.boost).clamp(0, agb::display::WIDTH - 16);
        self.y = (self.y + self.y_velocity * self.boost).clamp(0, agb::display::HEIGHT - 16);
    }

    pub fn render(&mut self, ball: &mut Object) {
        ball.set_x(self.x as u16).set_y(self.y as u16).show();
    }
}
