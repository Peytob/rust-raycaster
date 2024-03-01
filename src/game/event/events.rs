use std::cell::RefCell;
use std::rc::Rc;
use sdl2::EventPump;

pub struct Events {
    event_pump: Rc<RefCell<EventPump>>
}

impl Events {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump: Rc::new(RefCell::new(event_pump))
        }
    }

    pub fn event_pump(&self) -> &Rc<RefCell<EventPump>> {
        &self.event_pump
    }
}
