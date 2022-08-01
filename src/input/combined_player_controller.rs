use crate::{
    framework::{context::Context, error::GameResult},
    shared_game_state::SharedGameState,
};

use super::player_controller::PlayerController;

#[derive(Clone)]
pub struct CombinedPlayerController {
    controllers: Vec<Box<dyn PlayerController>>,
}

impl CombinedPlayerController {
    pub fn new() -> CombinedPlayerController {
        CombinedPlayerController { controllers: Vec::new() }
    }

    pub fn add(&mut self, controller: Box<dyn PlayerController>) {
        self.controllers.push(controller);
    }
}

impl PlayerController for CombinedPlayerController {
    fn update(&mut self, state: &mut SharedGameState, ctx: &mut Context) -> GameResult {
        for cont in &mut self.controllers {
            cont.update(state, ctx)?;
        }

        Ok(())
    }

    fn move_up(&self) -> bool {
        self.controllers.iter().any(|cont| cont.move_up())
    }

    fn move_down(&self) -> bool {
        self.controllers.iter().any(|cont| cont.move_down())
    }

    fn move_left(&self) -> bool {
        self.controllers.iter().any(|cont| cont.move_left())
    }

    fn move_right(&self) -> bool {
        self.controllers.iter().any(|cont| cont.move_right())
    }

    fn prev_weapon(&self) -> bool {
        self.controllers.iter().any(|cont| cont.prev_weapon())
    }

    fn next_weapon(&self) -> bool {
        self.controllers.iter().any(|cont| cont.next_weapon())
    }

    fn shoot(&self) -> bool {
        self.controllers.iter().any(|cont| cont.shoot())
    }

    fn jump(&self) -> bool {
        self.controllers.iter().any(|cont| cont.jump())
    }

    fn map(&self) -> bool {
        self.controllers.iter().any(|cont| cont.map())
    }

    fn inventory(&self) -> bool {
        self.controllers.iter().any(|cont| cont.inventory())
    }

    fn skip(&self) -> bool {
        self.controllers.iter().any(|cont| cont.skip())
    }

    fn strafe(&self) -> bool {
        self.controllers.iter().any(|cont| cont.strafe())
    }

    fn trigger_up(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_up())
    }

    fn trigger_down(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_down())
    }

    fn trigger_left(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_left())
    }

    fn trigger_right(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_right())
    }

    fn trigger_prev_weapon(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_prev_weapon())
    }

    fn trigger_next_weapon(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_next_weapon())
    }

    fn trigger_shoot(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_shoot())
    }

    fn trigger_jump(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_jump())
    }

    fn trigger_map(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_map())
    }

    fn trigger_inventory(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_inventory())
    }

    fn trigger_skip(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_skip())
    }

    fn trigger_strafe(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_strafe())
    }

    fn trigger_menu_ok(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_menu_ok())
    }

    fn trigger_menu_back(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_menu_back())
    }

    fn trigger_menu_pause(&self) -> bool {
        self.controllers.iter().any(|cont| cont.trigger_menu_pause())
    }

    fn look_up(&self) -> bool {
        self.controllers.iter().any(|cont| cont.look_up())
    }

    fn look_down(&self) -> bool {
        self.controllers.iter().any(|cont| cont.look_down())
    }

    fn look_left(&self) -> bool {
        self.controllers.iter().any(|cont| cont.look_left())
    }

    fn look_right(&self) -> bool {
        self.controllers.iter().any(|cont| cont.look_right())
    }

    fn update_trigger(&mut self) {
        for cont in &mut self.controllers {
            cont.update_trigger();
        }
    }

    fn move_analog_x(&self) -> f64 {
        self.controllers.iter().fold(0.0, |acc, cont| acc + cont.move_analog_x()).clamp(-1.0, 1.0)
    }

    fn move_analog_y(&self) -> f64 {
        self.controllers.iter().fold(0.0, |acc, cont| acc + cont.move_analog_y()).clamp(-1.0, 1.0)
    }

    fn dump_state(&self) -> (u16, u16, u16) {
        let mut state = (0, 0, 0);

        for c in self.controllers.iter() {
            let s = c.dump_state();
            state.0 |= s.0;
            state.1 |= s.1;
            state.2 |= s.2;
        }

        state
    }

    fn set_state(&mut self, state: (u16, u16, u16)) {
        for c in self.controllers.iter_mut() {
            c.set_state(state);
        }
    }
}
