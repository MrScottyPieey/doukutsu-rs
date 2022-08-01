use crate::bitfield;
use crate::framework::context::Context;
use crate::framework::error::GameResult;
use crate::framework::gamepad::{self, Button, PlayerControllerInputType};
use crate::input::player_controller::{KeyState, PlayerController};
use crate::player::TargetPlayer;
use crate::shared_game_state::SharedGameState;

#[derive(Clone)]
pub struct GamepadController {
    gamepad_id: u32,
    target: TargetPlayer,
    state: KeyState,
    old_state: KeyState,
    trigger: KeyState,
}

impl GamepadController {
    pub fn new(gamepad_id: u32, target: TargetPlayer) -> GamepadController {
        GamepadController { gamepad_id, target, state: KeyState(0), old_state: KeyState(0), trigger: KeyState(0) }
    }
}

impl PlayerController for GamepadController {
    fn update(&mut self, state: &mut SharedGameState, ctx: &mut Context) -> GameResult {
        let button_map = match self.target {
            TargetPlayer::Player1 => &state.settings.player1_controller_button_map,
            TargetPlayer::Player2 => &state.settings.player2_controller_button_map,
        };

        self.state.set_up(gamepad::is_active(ctx, self.gamepad_id, &button_map.up));
        self.state.set_down(gamepad::is_active(ctx, self.gamepad_id, &button_map.down));
        self.state.set_left(gamepad::is_active(ctx, self.gamepad_id, &button_map.left));
        self.state.set_right(gamepad::is_active(ctx, self.gamepad_id, &button_map.right));
        self.state.set_map(gamepad::is_active(ctx, self.gamepad_id, &button_map.map));
        self.state.set_inventory(gamepad::is_active(ctx, self.gamepad_id, &button_map.inventory));
        self.state.set_jump(gamepad::is_active(ctx, self.gamepad_id, &button_map.jump));
        self.state.set_shoot(gamepad::is_active(ctx, self.gamepad_id, &button_map.shoot));
        self.state.set_next_weapon(gamepad::is_active(ctx, self.gamepad_id, &button_map.next_weapon));
        self.state.set_prev_weapon(gamepad::is_active(ctx, self.gamepad_id, &button_map.prev_weapon));
        self.state.set_escape(gamepad::is_active(
            ctx,
            self.gamepad_id,
            &PlayerControllerInputType::ButtonInput(Button::Start),
        ));
        self.state.set_enter(gamepad::is_active(ctx, self.gamepad_id, &button_map.jump));
        self.state.set_skip(gamepad::is_active(ctx, self.gamepad_id, &button_map.skip));
        self.state.set_strafe(gamepad::is_active(ctx, self.gamepad_id, &button_map.strafe));

        Ok(())
    }

    fn update_trigger(&mut self) {
        let mut trigger = self.state.0 ^ self.old_state.0;
        trigger &= self.state.0;
        self.old_state = self.state;
        self.trigger = KeyState(trigger);
    }

    fn move_up(&self) -> bool {
        self.state.up()
    }

    fn move_left(&self) -> bool {
        self.state.left()
    }

    fn move_down(&self) -> bool {
        self.state.down()
    }

    fn move_right(&self) -> bool {
        self.state.right()
    }

    fn prev_weapon(&self) -> bool {
        self.state.prev_weapon()
    }

    fn next_weapon(&self) -> bool {
        self.state.next_weapon()
    }

    fn map(&self) -> bool {
        self.state.map()
    }

    fn inventory(&self) -> bool {
        self.state.inventory()
    }

    fn jump(&self) -> bool {
        self.state.jump()
    }

    fn shoot(&self) -> bool {
        self.state.shoot()
    }

    fn skip(&self) -> bool {
        self.state.skip()
    }

    fn strafe(&self) -> bool {
        self.state.strafe()
    }

    fn trigger_up(&self) -> bool {
        self.trigger.up()
    }

    fn trigger_left(&self) -> bool {
        self.trigger.left()
    }

    fn trigger_down(&self) -> bool {
        self.trigger.down()
    }

    fn trigger_right(&self) -> bool {
        self.trigger.right()
    }

    fn trigger_prev_weapon(&self) -> bool {
        self.trigger.prev_weapon()
    }

    fn trigger_next_weapon(&self) -> bool {
        self.trigger.next_weapon()
    }

    fn trigger_map(&self) -> bool {
        self.trigger.map()
    }

    fn trigger_inventory(&self) -> bool {
        self.trigger.inventory()
    }

    fn trigger_jump(&self) -> bool {
        self.trigger.jump()
    }

    fn trigger_shoot(&self) -> bool {
        self.trigger.shoot()
    }

    fn trigger_skip(&self) -> bool {
        self.trigger.skip()
    }

    fn trigger_strafe(&self) -> bool {
        self.trigger.strafe()
    }

    fn trigger_menu_ok(&self) -> bool {
        self.trigger.jump() || self.trigger.enter()
    }

    fn trigger_menu_back(&self) -> bool {
        self.trigger.shoot() || self.trigger.escape()
    }

    fn trigger_menu_pause(&self) -> bool {
        self.trigger.escape()
    }

    fn look_up(&self) -> bool {
        self.state.up()
    }

    fn look_left(&self) -> bool {
        self.state.left()
    }

    fn look_down(&self) -> bool {
        self.state.down()
    }

    fn look_right(&self) -> bool {
        self.state.right()
    }

    fn move_analog_x(&self) -> f64 {
        if self.state.left() && self.state.right() {
            0.0
        } else if self.state.left() {
            -1.0
        } else if self.state.right() {
            1.0
        } else {
            0.0
        }
    }

    fn move_analog_y(&self) -> f64 {
        if self.state.up() && self.state.down() {
            0.0
        } else if self.state.up() {
            -1.0
        } else if self.state.down() {
            1.0
        } else {
            0.0
        }
    }

    fn dump_state(&self) -> (u16, u16, u16) {
        (self.state.0, self.old_state.0, self.trigger.0)
    }

    fn set_state(&mut self, state: (u16, u16, u16)) {
        self.state = KeyState(state.0);
        self.old_state = KeyState(state.1);
        self.trigger = KeyState(state.2);
    }
}
