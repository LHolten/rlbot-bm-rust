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
        let ball = state
            .balls()
            .iter()
            .next()
            .expect("there should be at least one ball");

        let ball_direction = vec3_normalized_sub(ball.position(), car.position());
        let local_ball_direction = rotate_vector(conj(car.orientation()), ball_direction);
        input.steer = local_ball_direction[1].signum();
        input.throttle = 1.0;

        framework.set_bot_input(&input)
    }
}
