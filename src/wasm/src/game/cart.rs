pub mod cart {
    use crate::engine::{Line, Point, Renderer, Velocity};
    use crate::game::{CART_HEIGHT, CART_START_Y, CART_WIDTH};

    /* <-- CONSTANT VALUE */
    const FONT_COLOR: &str = "green";
    const CART: [&str; 2] = ["O❚O", "O❚O"];
    const CART_KNOCKED: [&str; 2] = ["O❚ O", "O❚"];
    const CART_DISTANCE: f32 = 18.0;
    /* CONSTANT VALUE --> */

    pub struct Cart {
        pub state_machine: CartStateMachine,
    }
    impl Cart {
        pub fn new(position: Point, velocity: Velocity, width: f32, height: f32) -> Self {
            Cart {
                state_machine: CartStateMachine::Idle(CartState::new(
                    position, velocity, width, height,
                )),
            }
        }
        fn get_state_machine(&self) -> CartStateMachine {
            self.state_machine.clone()
        }
        fn set_state_machine(&mut self, _state_machine: CartStateMachine) {
            self.state_machine = _state_machine.update();
        }
        pub fn update(&mut self) {
            let _state_machine = self.get_state_machine();
            self.set_state_machine(_state_machine);
        }
        pub fn run(&mut self, velocity: Velocity) {
            let _from_state_machine = self.get_state_machine();
            let _to_state_machine = _from_state_machine.transition(Event::Run(velocity));
            self.set_state_machine(_to_state_machine);
        }
        pub fn knocked(&mut self) {
            self.state_machine = self.state_machine.clone().transition(Event::Knocked);
        }
        // intersect() check cart crossing line
        // return true: crossing, falth: not crossing
        pub fn intersect(&self, _wall_line: Line) -> bool {
            let mut _cart_lines = vec![];

            // left side
            _cart_lines.push(Line::new(
                Point::new(
                    self.state_machine.context().position.x,
                    self.state_machine.context().position.y + CART_HEIGHT / 2.0,
                ),
                Point::new(
                    self.state_machine.context().position.x,
                    self.state_machine.context().position.y - CART_HEIGHT / 2.0,
                ),
            ));
            // right line
            _cart_lines.push(Line::new(
                Point::new(
                    self.state_machine.context().position.x + CART_WIDTH,
                    self.state_machine.context().position.y + CART_HEIGHT / 2.0,
                ),
                Point::new(
                    self.state_machine.context().position.x + CART_WIDTH,
                    self.state_machine.context().position.y - CART_HEIGHT / 2.0,
                ),
            ));
            // upper line
            _cart_lines.push(Line::new(
                Point::new(
                    self.state_machine.context().position.x,
                    self.state_machine.context().position.y + CART_HEIGHT / 2.0,
                ),
                Point::new(
                    self.state_machine.context().position.x + CART_WIDTH,
                    self.state_machine.context().position.y + CART_HEIGHT / 2.0,
                ),
            ));

            let mut _ans: bool = false;

            for i in 0.._cart_lines.len() {
                if ((_cart_lines[i].p.x - _cart_lines[i].q.x)
                    * (_wall_line.p.y - _cart_lines[i].p.y)
                    + (_cart_lines[i].p.y - _cart_lines[i].q.y)
                        * (_cart_lines[i].p.x - _wall_line.p.x))
                    * ((_cart_lines[i].p.x - _cart_lines[i].q.x)
                        * (_wall_line.q.y - _cart_lines[i].p.y)
                        + (_cart_lines[i].p.y - _cart_lines[i].q.y)
                            * (_cart_lines[i].p.x - _wall_line.q.x))
                    < 0.0
                {
                    if ((_wall_line.p.x - _wall_line.q.x) * (_cart_lines[i].p.y - _wall_line.p.y)
                        + (_wall_line.p.y - _wall_line.q.y) * (_wall_line.p.x - _cart_lines[i].p.x))
                        * ((_wall_line.p.x - _wall_line.q.x)
                            * (_cart_lines[i].q.y - _wall_line.p.y)
                            + (_wall_line.p.y - _wall_line.q.y)
                                * (_wall_line.p.x - _cart_lines[i].q.x))
                        < 0.0
                    {
                        _ans = true;
                        break;
                    }
                }
            }
            return _ans;
        }
        pub fn get_position(&self) -> Point {
            self.state_machine.context().position
        }
        pub fn get_velocity(&self) -> Velocity {
            self.state_machine.context().velocity
        }
        pub fn draw(&self, renderer: &Renderer) {
            let mut _distance: f32 = 0.0;
            match &self.state_machine {
                CartStateMachine::Knocked(_state) => {
                    for i in 0..CART_KNOCKED.len() {
                        renderer.text(
                            &Point {
                                x: self.state_machine.context().position.x,
                                y: CART_START_Y - _distance,
                            },
                            CART_KNOCKED[i],
                            FONT_COLOR,
                            "24px myfont",
                            "left",
                        );
                        _distance += CART_DISTANCE;
                    }
                }
                _ => {
                    for i in 0..CART.len() {
                        renderer.text(
                            &Point {
                                x: self.state_machine.context().position.x - CART_WIDTH / 2.0,
                                y: CART_START_Y - _distance,
                            },
                            CART[i],
                            FONT_COLOR,
                            "24px sans-serif",
                            "left",
                        );
                        _distance += CART_DISTANCE;
                    }
                }
            }
        }
    }

    #[derive(Clone)]
    pub enum CartStateMachine {
        Idle(CartState<Idle>),
        Running(CartState<Running>),
        Knocked(CartState<Knocked>),
    }

    pub enum Event {
        Run(Velocity),
        Update,
        Knocked,
    }

    impl CartStateMachine {
        fn transition(self, event: Event) -> Self {
            match (self.clone(), event) {
                (CartStateMachine::Idle(state), Event::Update) => state.update().into(),
                (CartStateMachine::Idle(state), Event::Run(velocity)) => state.run(velocity).into(),
                (CartStateMachine::Running(state), Event::Run(velocity)) => {
                    state.run(velocity).into()
                }
                (CartStateMachine::Running(state), Event::Update) => state.update().into(),
                (CartStateMachine::Running(state), Event::Knocked) => state.knocked().into(),
                _ => self,
            }
        }
        pub fn context(&self) -> &CartContext {
            match self {
                CartStateMachine::Idle(state) => state.context(),
                CartStateMachine::Running(state) => state.context(),
                CartStateMachine::Knocked(state) => state.context(),
            }
        }
        fn update(self) -> Self {
            self.transition(Event::Update)
        }
    }
    impl From<CartState<Idle>> for CartStateMachine {
        fn from(state: CartState<Idle>) -> Self {
            CartStateMachine::Idle(state)
        }
    }
    impl From<CartState<Running>> for CartStateMachine {
        fn from(state: CartState<Running>) -> Self {
            CartStateMachine::Running(state)
        }
    }
    impl From<CartState<Knocked>> for CartStateMachine {
        fn from(state: CartState<Knocked>) -> Self {
            CartStateMachine::Knocked(state)
        }
    }
    #[derive(Clone)]
    pub struct CartState<S> {
        context: CartContext,
        _state: S,
    }
    impl<S> CartState<S> {
        pub fn context(&self) -> &CartContext {
            &self.context
        }
        fn update_context(&mut self) {
            self.context = self.context.clone().update();
        }
    }

    #[derive(Copy, Clone)]
    pub struct Idle;
    impl CartState<Idle> {
        pub fn new(position: Point, velocity: Velocity, width: f32, height: f32) -> Self {
            CartState {
                context: CartContext {
                    position,
                    velocity,
                    width,
                    height,
                },
                _state: Idle {},
            }
        }
        pub fn update(mut self) -> CartState<Idle> {
            self.update_context();
            self
        }
        pub fn run(self, velocity: Velocity) -> CartState<Running> {
            CartState {
                context: self.context.run(velocity),
                _state: Running {},
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct Running;
    impl CartState<Running> {
        pub fn update(mut self) -> CartState<Running> {
            self.context.position.x = self.context.position.x + self.context.velocity.x;
            self.update_context();
            self
        }
        pub fn run(self, velocity: Velocity) -> CartState<Running> {
            CartState {
                context: self.context.run(velocity),
                _state: Running {},
            }
        }
        pub fn knocked(self) -> CartState<Knocked> {
            CartState {
                context: self.context.knocked(),
                _state: Knocked {},
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct Knocked;
    impl CartState<Knocked> {}

    #[derive(Clone)]
    pub struct CartContext {
        position: Point,
        velocity: Velocity,
        width: f32,
        height: f32,
    }
    impl CartContext {
        pub fn update(self) -> Self {
            self
        }
        fn run(mut self, velocity: Velocity) -> Self {
            self.velocity = velocity;
            self
        }
        fn knocked(self) -> Self {
            self
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::engine::{Point, Velocity};
    use crate::game::Cart;
    use crate::game::{CART_HEIGHT, CART_START_Y, CART_WIDTH, Line};

    #[test]
    fn intersect_wall() {
        let mut cart = Cart::new(
            Point { x: 245.0, y: 665.0 },
            Velocity { x: 245.0, y: 695.0 },
            CART_WIDTH,
            CART_HEIGHT,
        );

        let mut line = Line {
            p: Point { x: 320.0, y: 405.0 },
            q: Point {
                x: 160.0,
                y: 1005.0,
            },
        };
        assert_eq!(cart.intersect(line), true); // true: crossing

        cart = Cart::new(
            Point {
                x: 10.0,
                y: CART_START_Y,
            },
            Velocity {
                x: 10.0,
                y: CART_START_Y + CART_HEIGHT / 2.0,
            },
            CART_WIDTH,
            CART_HEIGHT,
        );

        line = Line {
            p: Point { x: 30.0, y: 0.0 },
            q: Point { x: 30.0, y: 600.0 },
        };
        assert_eq!(cart.intersect(line), false); // true: crossing
    }
}
