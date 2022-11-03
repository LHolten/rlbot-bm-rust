use quaternion::{conj, rotate_vector};
use rlbot_bm::{
    input::ControllerInput,
    state::{GameState, Physics},
};
use vecmath::vec3_normalized_sub;

fn main() {
    let mut framework = rlbot_bm::RlBotBm::new().expect("could not configure RLBot-BM");
    let mut state = GameState::default();
    let mut input = ControllerInput::default();
    loop {
        framework.next_state(&mut state);

        if !state.is_round_active() {
            continue;
        }

        let car = &state.cars()[framework.index];
        let ball = &state.balls()[0];

        let ball_direction = vec3_normalized_sub(ball.position(), car.position());
        let global_to_local = conj(car.orientation());
        let local_ball_direction = rotate_vector(global_to_local, ball_direction);

        input.steer = local_ball_direction[1].signum();
        input.throttle = 1.0;
        framework.set_bot_input(&input)
    }
}
