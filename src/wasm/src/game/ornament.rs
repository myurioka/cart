pub mod ornament {
    //! ornament related functions.
    use crate::engine::{Point, Velocity};
    use crate::game::{Piece, Renderer, STAGE_GOAL, State, StateMachine};

    /* <-- CONSTANT VALUE */
    const TREE: [&str; 4] = [" ▯ ", "$$$", " $$ ", " $ "];
    const TREE_DISTANCE: f32 = 12.0;
    const FONT_COLOR: &str = "green";
    const GOAL_X: f32 = 100.0;
    const GOAL_Y: f32 = -6700.0;
    const GOAL: [&str; 2] = [
        "□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■",
        "□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■□□■",
    ];
    const GOAL_DISTANCE: f32 = 25.0;

    pub struct Ornament {
        pub state_machine: StateMachine,
    }
    impl Piece for Ornament {
        fn new(p: Point, q: Point, velocity: Velocity) -> Self {
            Ornament {
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
            // GOAL
            let mut _distance: f32 = 0.0;
            for i in 0..GOAL.len() {
                renderer.text(
                    &Point {
                        x: GOAL_X + self.state_machine.context().p.x,
                        y: GOAL_Y + self.state_machine.context().p.y + _distance,
                    },
                    GOAL[i],
                    FONT_COLOR,
                    "32 myfont",
                    "center",
                );
                _distance += GOAL_DISTANCE;
            }
            for i in 0..TREE.len() {
                renderer.text(
                    &Point {
                        x: 270.0 + self.state_machine.context().p.x,
                        y: 800.0 + self.state_machine.context().p.y - _distance,
                    },
                    TREE[i],
                    FONT_COLOR,
                    "32 myfont",
                    "center",
                );
                _distance += TREE_DISTANCE;
            }
        }
    }
}
