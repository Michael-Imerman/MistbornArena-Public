use crate::player::{
    enums::player_states::PlayerStates, player::Player, traits::player_state::PlayerState,
};

// NOTE: This and most of the other states have not had documentation added
// because Rust will automatically derive the documentation from the PlayerState trait.
// The functions in these traits work nearly identically in each state and just have slitghtly different conditions
// for swapping to a new state.
//

#[derive(Clone, Copy)]
pub struct CrouchEnd;

impl PlayerState for CrouchEnd {
    fn enter(_player: &mut Player) {}

    fn update(player: &mut Player) {
        if player.is_anim_finished() {
            player.set_state(PlayerStates::Idle);
        }
    }
}
