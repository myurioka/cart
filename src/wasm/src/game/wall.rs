pub mod wall {
    use crate::game::{Piece, Point, Renderer, State, StateMachine};

    pub struct Wall {
        pub state_machine: StateMachine,
    }
    impl Piece for Wall {
        fn new(p: Point, q: Point, velocity: Point) -> Self {
            Wall {
                state_machine: StateMachine::Running(State::new(p, q, velocity)),
            }
        }
        fn get_state_machine(&self) -> StateMachine {
            self.state_machine
        }
        fn set_state_machine(&mut self, _state_machine: StateMachine) {
            self.state_machine = _state_machine.update();
        }

        fn draw(&self, renderer: &Renderer) {
            renderer.line(
                &Point {
                    x: self.state_machine.context().p.x,
                    y: self.state_machine.context().p.y
                },
                &Point {
                    x: self.state_machine.context().q.x,
                    y: self.state_machine.context().q.y
                },
            );
        }
    }

    /* <-- CONSTANT VALUE */
    pub const WALLS_DATA : [(f32, f32, f32, f32); 26] = 
        [
            (     30.0,   0.0,    30.0,  600.0),
            (     30.0,  600.0,   140.0, 1000.0),
            (    140.0, 1000.0,    30.0, 1600.0),
            (     30.0, 1600.0,   200.0, 2000.0),
            (    200.0, 2000.0,   200.0, 2400.0),
            (    200.0, 2400.0,    30.0, 2400.0),
            (     30.0, 2400.0,    30.0, 5500.0),
            (    100.0, 4000.0,   300.0, 3400.0),
            (    100.0, 4000.0,   250.0, 4300.0),
            (    120.0, 2700.0,   120.0, 3000.0),
            (    120.0, 2700.0,   190.0, 2700.0),
            (    120.0, 3000.0,   190.0, 3000.0),
            (    190.0, 2700.0,   190.0, 3000.0),
            (    280.0, 2700.0,   280.0, 3000.0),
            (    280.0, 2700.0,   350.0, 2700.0),
            (    280.0, 3000.0,   350.0, 3000.0),
            (    350.0, 2700.0,   350.0, 3000.0),
            (    430.0,   0.0,    430.0, 600.0),
            (    430.0,  600.0,   320.0, 1000.0),
            (    320.0, 1000.0,   160.0, 1600.0),
            (    160.0, 1600.0,   330.0, 2000.0),
            (    330.0, 2000.0,   330.0, 2400.0),
            (    330.0, 2400.0,   430.0, 2400.0),
            (    430.0, 2400.0,   430.0, 5500.0),
            (    300.0, 3400.0,   360.0, 4000.0),
            (    250.0, 4300.0,   360.0, 4000.0),
        ];
}