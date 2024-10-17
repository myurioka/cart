pub mod ornament {
    use crate::game::{Piece, Point, Renderer, State, StateMachine};
//

/* <-- CONSTANT VALUE */
    const GOAL_X: f32 = 110.0;
    const GOAL_Y: f32 = 4500.0;
    const GOAL: [&str; 2] = ["□□■□□■□□■□□■□□■□□■□□■□","■□□■□□■□□■□□■□□■□□■□□■"];
    const GOAL_DISTANCE: f32 = 20.0;
    const TREE: [&str; 4]  = [" $ ", " $$ ", "$$$"," ▯ "];
    const TREE_DISTANCE: f32 = 12.0;

    pub struct Ornament {
        pub state_machine: StateMachine,
    }
    impl Piece for Ornament {
        fn new(p: Point, q: Point,velocity: Point) -> Self {
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
            let mut _distance:f32 = 0.0;
            for i in 0..GOAL.len() {
                renderer.text(
                    &Point{
                        x: GOAL_X + self.state_machine.context().p.x,
                        y: GOAL_Y + self.state_machine.context().p.y + _distance,
                    },
                    GOAL[i],
                );
                _distance += GOAL_DISTANCE;
            }
            _distance = 0.0;
            for i in 0..TREE.len() {
                renderer.text(
                    &Point{
                        x: 270.0 + self.state_machine.context().p.x,
                        y: 800.0 + self.state_machine.context().p.y - _distance,
                    },
                    TREE[i],
                );
                _distance += TREE_DISTANCE;
            }
            for i in 0..TREE.len() {
                renderer.text(
                    &Point{
                        x: -40.0 + self.state_machine.context().p.x,
                        y: 1800.0 + self.state_machine.context().p.y - _distance,
                    },
                    TREE[i],
                );
                _distance += TREE_DISTANCE;
            }
            for i in 0..TREE.len() {
                renderer.text(
                    &Point{
                        x: 200.0 + self.state_machine.context().p.x,
                        y: 2800.0 + self.state_machine.context().p.y - _distance,
                    },
                    TREE[i],
                );
                _distance += TREE_DISTANCE;
            }
            for i in 0..TREE.len() {
                renderer.text(
                    &Point{
                        x: 120.0 + self.state_machine.context().p.x,
                        y: 3800.0 + self.state_machine.context().p.y - _distance,
                    },
                    TREE[i],
                );
                _distance += TREE_DISTANCE;
            }
        }
    }
}