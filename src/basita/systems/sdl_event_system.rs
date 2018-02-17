use sdl2::event::Event;

use super::super::EngineState;
use super::System;

pub struct SdlEventSystem {}

impl System for SdlEventSystem {
	fn update(&self, state: &mut EngineState) {
		state.input.update();

		let mut event_pump = state.sdl_context.event_pump.borrow_mut();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => {
					state.running = false;
					break;
				}
				_ => state.input.handle_event(event),
			};
		}
	}
}