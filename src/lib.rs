pub mod input;
pub mod state;

use std::{cell::Cell, fmt::Display, marker::PhantomData, os::raw::c_int, ptr::NonNull};

use pico_args::Arguments;
use rlbot_bm_sys::*;

// The PhantomData Cell is used to make the type !Sync
pub struct RlBotBm {
    handle: NonNull<RLBotBM_H>,
    index: usize,
    input: ControllerInput,
    not_sync: PhantomData<Cell<()>>,
}

impl RlBotBm {
    pub fn new() -> Result<Self, impl Display> {
        let mut args = Arguments::from_env();
        let player_index = args
            .value_from_str("--player-index")
            .map_err(|_| "Failed to read player-index from args")?;

        let unchecked_handle = unsafe { RLBotBM_create() };
        let handle =
            NonNull::new(unchecked_handle).ok_or("RLBot-BM handle could not be created")?;

        Ok::<Self, &str>(Self {
            handle,
            index: player_index,
            input: Default::default(),
            not_sync: PhantomData,
        })
    }

    /// returns whether it had to wait
    pub fn next_state(&mut self, state: &mut GameStateObj) -> bool {
        let handle = self.handle.as_ptr();
        unsafe { RLBotBM_pollNextTick(handle, state) }
    }

    pub fn set_bot_input(&mut self, input: &input::ControllerInput) {
        let handle = self.handle.as_ptr();

        self.input.throttle = input.throttle;
        self.input.steer = input.steer;
        self.input.pitch = input.pitch;
        self.input.yaw = input.yaw;
        self.input.roll = input.roll;
        self.input.set_handbrake(input.handbrake as u32);
        self.input.set_jump(input.jump as u32);
        self.input.set_boost(input.boost as u32);
        self.input.set_useItem(input.item_target.is_some() as u32);
        if let Some(item_target) = input.item_target {
            self.input.itemTarget = item_target
        }

        unsafe { RLBotBM_setBotInput(handle, &self.input, self.index as c_int) }
    }
}

impl Drop for RlBotBm {
    fn drop(&mut self) {
        let handle = self.handle.as_ptr();
        unsafe { RLBotBM_destroy(handle) };
    }
}

// fn as_dyn_display<T: Display + 'static>(val: T) -> Box<dyn Display> {
//     Box::new(val)
// }

// pub fn run_bot<B: rlbot::Bot>(bot: B) -> Result<(), impl Display> {
//     fn internal<B: rlbot::Bot>(mut bot: B) -> Result<(), Box<dyn Display>> {
//         let mut handle = RlBotBm::new().map_err(as_dyn_display)?;

//         let args = rlbot::parse_framework_args()
//             .map_err(|()| as_dyn_display("could not parse framework arguments"))?
//             .ok_or_else(|| as_dyn_display("not launched by framework"))?;

//         let player_index = args.player_index as usize;

//         bot.set_player_index(player_index);

//         let mut state = Default::default();
//         loop {
//             handle.next_state(&mut state);
//             let input = bot.tick(&state.convert());
//             handle.set_bot_input(&input.convert(), player_index);
//         }
//     }
//     internal(bot)
// }
