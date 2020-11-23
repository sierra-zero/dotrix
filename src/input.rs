use std::collections::HashMap;

// Input Service
pub struct Input {
    mapper: Box<dyn std::any::Any + Send + Sync>,
    states: HashMap<u32, State>,
}

impl Input {
    pub fn new(mapper: Box<dyn std::any::Any + Send + Sync>) -> Self {
        Self {
            mapper,
            states: HashMap::new(),
        }
    }

    /// This is probably temporarary method, all the magic will be in handle_event
    pub fn set_state(&mut self, keycode: u32, state: State) {
        self.states.insert(keycode, state);
    }

    /// This checks if user defined action is active or not according to the input
    pub fn is_action<T>(&self, action: T) -> bool
    where
        Self: ActionMapper<T>,
        T: Eq + std::hash::Hash
    {
        if let Some(key_code) = self.get_action(action) {
            if let Some(state) = self.states.get(key_code) {
                return *state == State::Pressed
            }
        }
        false
    }

    /// just a mapper getter
    pub fn mapper<T: 'static + Send + Sync>(&self) -> &T {
        self.mapper.downcast_ref::<T>().unwrap()
    }


    // add match case here and all the routines to control input states
    // this method should be called in application.rs before the main match case,
    // so systems will get actual Input data
    pub fn handle_event(&mut self, event: &winit::event::Event<'_, ()>) {
        /*
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput{device_id, input, is_synthetic},
                ..
            } => services.get_mut::<InputManager>().handle_keyboard_event(device_id, input, is_synthetic),
            Event::WindowEvent {
                event: WindowEvent::MouseInput{device_id, state, button, ..},
                ..
            } => services.get_mut::<InputManager>().handle_mouse_event(device_id, state, button),
            Event::WindowEvent {
                event: WindowEvent::MouseWheel{device_id, delta, phase, ..},
                ..
            } => services.get_mut::<InputManager>().handle_mouse_wheel_event(device_id, delta, phase),
            */
    }
}

pub trait ActionMapper<T>
where
    T: Eq + std::hash::Hash 
{
    fn get_action(&self, action: T) -> Option<&u32>;
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum State {
    Released,
    Pressed,
    Hold,
}

pub struct Mapper<T> {
    map: HashMap<T, u32>,
}

impl<T> Mapper<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_action(&mut self, action: T, keycode: u32)
    where
        T: Eq + std::hash::Hash
    {
        self.map.insert(action, keycode);
    }

    pub fn get_keycode(&self, action: T) -> Option<&u32>
    where
        T: Eq + std::hash::Hash
    {
        self.map.get(&action)
    }
}


#[cfg(test)]
mod tests {

    use super::{Input, ActionMapper, State, Mapper};

    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    enum Action {
        Jump,
        Run,
    }

    const KEY_CODE: u32 = 10;

    impl ActionMapper<Action> for Input {
        fn get_action(&self, action: Action) -> Option<&u32>
        {
            let mapper = self.mapper::<Mapper<Action>>();
            mapper.get_keycode(action)
        }
    }

    #[test]
    fn actions_mapping() {
        let mut mapper: Mapper<Action> = Mapper::new();
        mapper.add_action(Action::Jump, KEY_CODE);

        let mut input = Input::new(Box::new(mapper));
        assert_eq!(input.is_action(Action::Jump), false);

        input.set_state(KEY_CODE, State::Pressed);
        assert_eq!(input.is_action(Action::Jump), true);
    }
}
