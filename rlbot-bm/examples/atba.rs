use quaternion::{conj, rotate_vector};
use rlbot_bm::{input::ControllerInput, state::Physics};
use vecmath::vec3_normalized_sub;

fn main() {
    let mut rlbot_bm = rlbot_bm::RlBotBm::new().expect("could not configure RLBot-BM");
    let mut input = ControllerInput::default();
    loop {
        let car = &rlbot_bm.state.cars()[rlbot_bm.car_id];
        let ball = &rlbot_bm.state.balls()[0];

        let ball_direction = vec3_normalized_sub(ball.position(), car.position());
        let global_to_local = conj(car.orientation());
        let local_ball_direction = rotate_vector(global_to_local, ball_direction);

        input.steer = local_ball_direction[1].signum();
        input.throttle = 1.0;
        rlbot_bm.update(&input)
    }
}
