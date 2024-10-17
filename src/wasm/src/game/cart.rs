pub mod cart {
    use crate::game::{Line, Point, Renderer, CART_START_Y};

/* <-- CONSTANT VALUE */
    const CART: [&str; 2]  = ["O❚O", "O❚O"];
    const CART_KNOCKED: [&str; 2] = ["O❚ O", "O❚",];
    const CART_DISTANCE:f32 = 18.0;
    pub const CART_WIDTH: f32 = 20.0;
    pub const CART_HEIGHT: f32 = 14.0;
/* CONSTANT VALUE --> */

    pub struct Cart {
        pub state_machine: CartStateMachine,
    }
    impl Cart {
        pub fn new(position: Point, velocity: Point) -> Self {
            Cart {
                state_machine: CartStateMachine::Idle(CartState::new( position, velocity)),
            }
        }
        fn get_state_machine(&self) -> CartStateMachine {
            self.state_machine.clone()
        }
        fn set_state_machine(&mut self, _state_machine: CartStateMachine) {
            self.state_machine = _state_machine.update();
        }
        pub fn update(&mut self){
            let _state_machine = self.get_state_machine();
            self.set_state_machine(_state_machine);
        }
        pub fn run(&mut self, velocity:Point) {
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
                    Point::new(self.state_machine.context().position.x,
                                self.state_machine.context().position.y + CART_HEIGHT / 2.0),
                    Point::new(self.state_machine.context().position.x,
                                self.state_machine.context().position.y - CART_HEIGHT / 2.0)
            ));
            // right line
            _cart_lines.push(Line::new(
                    Point::new(self.state_machine.context().position.x + CART_WIDTH,
                                self.state_machine.context().position.y + CART_HEIGHT / 2.0),
                    Point::new(self.state_machine.context().position.x + CART_WIDTH,
                                self.state_machine.context().position.y - CART_HEIGHT / 2.0)
            ));
            // upper line
            _cart_lines.push(Line::new(
                    Point::new(self.state_machine.context().position.x,
                                self.state_machine.context().position.y + CART_HEIGHT / 2.0),
                    Point::new(self.state_machine.context().position.x + CART_WIDTH,
                                self.state_machine.context().position.y + CART_HEIGHT / 2.0)
            ));

            let mut _ans:bool = false;

            for i in 0.._cart_lines.len() {
                if ((_cart_lines[i].p.x - _cart_lines[i].q.x) * ( _wall_line.p.y - _cart_lines[i].p.y)
                    + (_cart_lines[i].p.y - _cart_lines[i].q.y) * (_cart_lines[i].p.x - _wall_line.p.x ))
                    * ((_cart_lines[i].p.x - _cart_lines[i].q.x ) * (_wall_line.q.y - _cart_lines[i].p.y )
                    + (_cart_lines[i].p.y - _cart_lines[i].q.y) * (_cart_lines[i].p.x - _wall_line.q.x )) < 0.0 {
                    if ((_wall_line.p.x - _wall_line.q.x) * ( _cart_lines[i].p.y - _wall_line.p.y)
                        + (_wall_line.p.y - _wall_line.q.y) * (_wall_line.p.x - _cart_lines[i].p.x ))
                        * ((_wall_line.p.x - _wall_line.q.x ) * (_cart_lines[i].q.y - _wall_line.p.y )
                        + (_wall_line.p.y - _wall_line.q.y) * (_wall_line.p.x - _cart_lines[i].q.x )) < 0.0 {
                        _ans = true;
                        break;
                    }
                }
            }
            return _ans;
        }
        pub fn get_position(&self) -> Point{
            self.state_machine.context().position
        }
        pub fn get_velocity(&self) -> Point{
            self.state_machine.context().velocity
        }
        pub fn draw(&self, renderer: &Renderer) {
            let mut _distance:f32 = 0.0;
            match &self.state_machine {
                CartStateMachine::Knocked(_state) => {
                    for i in 0..CART_KNOCKED.len(){
                        renderer.text_left(
                            &Point {
                                x: self.state_machine.context().position.x,
                                y: CART_START_Y - _distance,
                            },
                            CART_KNOCKED[i],
                        );
                        _distance += CART_DISTANCE;
                    };
                }
                _ => {
                    for i in 0..CART.len(){
                        renderer.text_left(
                            &Point {
                                x: self.state_machine.context().position.x,
                                y: CART_START_Y - _distance,
                            },
                            CART[i],
                        );
                        _distance += CART_DISTANCE;
                    }
                }
            }
        }
    }

    #[derive(Clone)]
    pub enum CartStateMachine{
        Idle(CartState<Idle>),
        Running(CartState<Running>),
        Knocked(CartState<Knocked>),
    }

    pub enum Event {
        Run(Point),
        Update,
        Knocked,
    }

    impl CartStateMachine {
        fn transition(self, event: Event) -> Self {
            match (self.clone(), event) {
                (CartStateMachine::Idle(state), Event::Update) => state.update().into(),
                (CartStateMachine::Idle(state), Event::Run(velocity)) => state.run(velocity).into(),
                (CartStateMachine::Running(state), Event::Run(velocity)) => state.run(velocity).into(),
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
    impl From<CartState<Idle>> for CartStateMachine{
        fn from(state: CartState<Idle>) -> Self {
            CartStateMachine::Idle(state)
        }
    }
    impl From<CartState<Running>> for CartStateMachine{
        fn from(state: CartState<Running>) -> Self {
            CartStateMachine::Running(state)
        }
    }
    impl From<CartState<Knocked>> for CartStateMachine{
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
        fn update_context(&mut self){
            self.context = self.context.clone().update();
        }
    }

    #[derive(Copy, Clone)]
    pub struct Idle;
    impl CartState<Idle> {
        pub fn new(position: Point, velocity: Point) -> Self {
             CartState {
                context: CartContext {
                    position: position,
                    velocity: velocity,
                },
                _state: Idle {},
             }
        }
        pub fn update(mut self) -> CartState<Idle> {
            self.update_context();
            self
        }
        pub fn run(self, velocity:Point) -> CartState<Running> {
            CartState {
                context: self.context.run(velocity),
                _state: Running{},
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct Running;
    impl CartState<Running> {
        pub fn update(mut self)  -> CartState<Running> {
            self.context.position.x = self.context.position.x+ self.context.velocity.x;
            self.update_context();
            self
        }
        pub fn run(self, velocity: Point) -> CartState<Running> {
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
    impl CartState<Knocked> {
    }

    #[derive(Clone)]
    pub struct CartContext {
        pub position: Point,
        pub velocity: Point,
    }
    impl CartContext {
        pub fn update(self) -> Self {
            self
        }
        fn run(mut self, velocity: Point) -> Self {
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
    use crate::game::{Line, Point, CART_START_Y};
    use crate::game::Cart;
    use crate::game::cart::cart::CART_HEIGHT;

    #[test]
    fn intersect_wall() {

        let mut cart = Cart::new (
            Point {x: 245.0, y: 665.0},
            Point {x: 245.0, y: 695.0},
        );

        let mut line = Line {
            p: Point { x: 320.0, y: 405.0 },
            q: Point { x: 160.0, y: 1005.0 },
        };
        assert_eq!(cart.intersect(line), true); // true: crossing

        cart = Cart::new (
            Point {x: 10.0, y: CART_START_Y},
            Point {x: 10.0, y: CART_START_Y + CART_HEIGHT / 2.0},
        );

        line = Line {
            p: Point { x: 30.0, y: 0.0 },
            q: Point { x: 30.0, y: 600.0 },
        };
        assert_eq!(cart.intersect(line), false); // true: crossing
    }
}