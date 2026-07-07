//! The side-effect-free core (crux `App`). Counter behavior is a placeholder
//! smoke test proving the core/shell wiring; coaching logic lands later.

use crux_core::{
    App, Command,
    macros::effect,
    render::{RenderOperation, render},
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Model {
    count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Increment,
    Decrement,
    Reset,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct ViewModel {
    pub count: String,
}

#[effect]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
}

#[derive(Default)]
pub struct Engine;

impl App for Engine {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Self::Event, model: &mut Self::Model) -> Command<Effect, Event> {
        match event {
            Event::Increment => model.count += 1,
            Event::Decrement => model.count -= 1,
            Event::Reset => model.count = 0,
        }
        render()
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            count: format!("Count is: {}", model.count),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_and_renders() {
        let app = Engine;
        let mut model = Model::default();

        app.update(Event::Increment, &mut model).expect_only_render();

        assert_eq!(app.view(&model).count, "Count is: 1");
    }

    #[test]
    fn resets_count() {
        let app = Engine;
        let mut model = Model { count: 42 };

        app.update(Event::Reset, &mut model).expect_only_render();

        assert_eq!(app.view(&model).count, "Count is: 0");
    }
}
