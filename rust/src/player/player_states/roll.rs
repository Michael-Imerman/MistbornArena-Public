use crate::player::{player::Player, traits::player_state::PlayerState};

#[derive(Clone, Copy)]
pub struct Roll;

impl PlayerState for Roll {
    fn enter(player: &mut Player) {
        player.set_animation_speed(1.0);
    }

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            let previous_state = player.get_previous_state();
            player.set_state(previous_state);
        }
    }
}
