#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::engine::{KeyState, Point, Line};

    #[test]
    fn test_context_creation() {
        let p = Point::new(10.0, 20.0);
        let q = Point::new(30.0, 40.0);
        let velocity = Point::new(1.0, 2.0);
        
        let context = Context { p, q, velocity };
        
        assert_eq!(context.p.x, 10.0);
        assert_eq!(context.p.y, 20.0);
        assert_eq!(context.q.x, 30.0);
        assert_eq!(context.q.y, 40.0);
        assert_eq!(context.velocity.x, 1.0);
        assert_eq!(context.velocity.y, 2.0);
    }

    #[test]
    fn test_context_run() {
        let mut context = Context {
            p: Point::new(0.0, 0.0),
            q: Point::new(0.0, 0.0),
            velocity: Point::new(0.0, 0.0),
        };
        
        let new_velocity = Point::new(5.0, 3.0);
        context = context.run(new_velocity);
        
        assert_eq!(context.velocity.x, 5.0);
        assert_eq!(context.velocity.y, 3.0);
    }

    #[test]
    fn test_state_running_creation() {
        let p = Point::new(100.0, 200.0);
        let q = Point::new(150.0, 250.0);
        let velocity = Point::new(2.0, 3.0);
        
        let state = State::<Running>::new(p, q, velocity);
        
        assert_eq!(state.context.p.x, 100.0);
        assert_eq!(state.context.p.y, 200.0);
        assert_eq!(state.context.q.x, 150.0);
        assert_eq!(state.context.q.y, 250.0);
        assert_eq!(state.context.velocity.x, 2.0);
        assert_eq!(state.context.velocity.y, 3.0);
    }

    #[test]
    fn test_state_running_update() {
        let p = Point::new(100.0, 200.0);
        let q = Point::new(150.0, 250.0);
        let velocity = Point::new(2.0, 3.0);
        
        let state = State::<Running>::new(p, q, velocity);
        let updated_state = state.update();
        
        assert_eq!(updated_state.context.p.x, 102.0);
        assert_eq!(updated_state.context.p.y, 203.0);
        assert_eq!(updated_state.context.q.x, 152.0);
        assert_eq!(updated_state.context.q.y, 253.0);
    }

    #[test]
    fn test_state_running_run() {
        let p = Point::new(100.0, 200.0);
        let q = Point::new(150.0, 250.0);
        let velocity = Point::new(2.0, 3.0);
        
        let state = State::<Running>::new(p, q, velocity);
        let new_velocity = Point::new(5.0, 7.0);
        let updated_state = state.run(new_velocity);
        
        assert_eq!(updated_state.context.velocity.x, 5.0);
        assert_eq!(updated_state.context.velocity.y, 7.0);
    }

    #[test]
    fn test_state_machine_context() {
        let p = Point::new(10.0, 20.0);
        let q = Point::new(30.0, 40.0);
        let velocity = Point::new(1.0, 2.0);
        
        let state = State::<Running>::new(p, q, velocity);
        let machine = StateMachine::Running(state);
        
        let context = machine.context();
        assert_eq!(context.p.x, 10.0);
        assert_eq!(context.p.y, 20.0);
    }

    #[test]
    fn test_state_machine_transition_run() {
        let p = Point::new(10.0, 20.0);
        let q = Point::new(30.0, 40.0);
        let velocity = Point::new(1.0, 2.0);
        
        let state = State::<Running>::new(p, q, velocity);
        let machine = StateMachine::Running(state);
        
        let new_velocity = Point::new(5.0, 7.0);
        let updated_machine = machine.transition(Event::Run(new_velocity));
        
        assert_eq!(updated_machine.context().velocity.x, 5.0);
        assert_eq!(updated_machine.context().velocity.y, 7.0);
    }

    #[test]
    fn test_state_machine_transition_update() {
        let p = Point::new(100.0, 200.0);
        let q = Point::new(150.0, 250.0);
        let velocity = Point::new(2.0, 3.0);
        
        let state = State::<Running>::new(p, q, velocity);
        let machine = StateMachine::Running(state);
        
        let updated_machine = machine.transition(Event::Update);
        
        assert_eq!(updated_machine.context().p.x, 102.0);
        assert_eq!(updated_machine.context().p.y, 203.0);
        assert_eq!(updated_machine.context().q.x, 152.0);
        assert_eq!(updated_machine.context().q.y, 253.0);
    }

    #[test]
    fn test_constants() {
        assert_eq!(STAGE_LEFT, 100.0);
        assert_eq!(STAGE_GOAL, 2300.0);
        assert_eq!(CART_START_X, 220.0);
        assert_eq!(CART_START_Y, 70.0);
        assert_eq!(VELOCITY_X, 0.8);
        assert_eq!(VELOCITY_STEP, 0.03);
        assert_eq!(VELOCITY_BRAKE_STEP, 0.06);
        assert_eq!(VELOCITY_LIMIT, 5.0);
        assert_eq!(VELOCITY_ZERO, 0.0);
        assert_eq!(ORNAMENT_X, 120.0);
        assert_eq!(ORNAMENT_Y, 50.0);
        assert_eq!(ORNAMENT_WIDTH, 10.0);
        assert_eq!(ORNAMENT_HEIGHT, 10.0);
    }

    #[test]
    fn test_gamestage_creation() {
        let game_stage = GameStage::new();
        assert!(game_stage.machine.is_none());
    }

    #[test]
    fn test_ready_state_creation() {
        let p = Point::new(100.0, 100.0);
        let q = Point::new(200.0, 200.0);
        let velocity = Point::new(0.0, 0.0);
        
        let state_machine = StateMachine::Running(State::<Running>::new(p, q, velocity));
        
        assert_eq!(state_machine.context().p.x, 100.0);
        assert_eq!(state_machine.context().p.y, 100.0);
    }

    #[test]
    fn test_piece_trait_functions() {
        let p = Point::new(10.0, 20.0);
        let q = Point::new(30.0, 40.0);
        let velocity = Point::new(1.0, 2.0);
        
        let state = State::<Running>::new(p, q, velocity);
        let machine = StateMachine::Running(state);
        
        assert_eq!(machine.context().p.x, 10.0);
        assert_eq!(machine.context().p.y, 20.0);
        assert_eq!(machine.context().q.x, 30.0);
        assert_eq!(machine.context().q.y, 40.0);
    }
}