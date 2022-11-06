pub mod input;
pub mod state;

use std::{
    ffi::{c_int, c_ulong},
    fmt::Debug,
    ptr::NonNull,
};

use pico_args::Arguments;

use crate::state::{CarId, GameState};

/// This is the wrapper for RLBot-BM.
/// All of the methods are blocking, so if you want to do computation while waiting,
/// you should call these methods on a separate thread.
pub struct RlBotBm {
    handle: NonNull<rlbot_bm_sys::RLBotBM_H>,
    pub index: CarId,
    input: rlbot_bm_sys::ControllerInput,
    pub state: GameState,
}

impl RlBotBm {
    /// This creates a connectionto the RLBot-BM plugin and waits for a valid state
    pub fn new() -> Result<Self, impl Debug> {
        let mut args = Arguments::from_env();
        let player_index = args
            .value_from_str("--player-index")
            .map_err(|_| "Failed to read --player-index from args")?;

        let unchecked_handle = unsafe { rlbot_bm_sys::RLBotBM_create() };
        let handle =
            NonNull::new(unchecked_handle).ok_or("RLBot-BM handle could not be created")?;

        let mut framework = Self {
            handle,
            index: CarId(player_index),
            input: Default::default(),
            state: GameState::default(),
        };

        while !framework.state.is_round_active() {
            framework.wait_next_tick();
        }

        Ok::<Self, &str>(framework)
    }

    /// returns whether it had to wait
    fn wait_next_tick(&mut self) -> bool {
        let handle = self.handle.as_ptr();
        unsafe { rlbot_bm_sys::RLBotBM_waitNextTick(handle, &mut self.state.0) }
    }

    /// Use this function inside a loop to set inputs for each tick
    pub fn update(&mut self, input: &input::ControllerInput) {
        let handle = self.handle.as_ptr();

        self.input.throttle = input.throttle;
        self.input.steer = input.steer;
        self.input.pitch = input.pitch;
        self.input.yaw = input.yaw;
        self.input.roll = input.roll;
        self.input.set_handbrake(input.handbrake as c_ulong);
        self.input.set_jump(input.jump as c_ulong);
        self.input.set_boost(input.boost as c_ulong);
        self.input
            .set_useItem(input.item_target.is_some() as c_ulong);
        if let Some(item_target) = input.item_target {
            self.input.itemTarget = item_target as c_ulong
        }

        unsafe { rlbot_bm_sys::RLBotBM_setBotInput(handle, &self.input, self.index.0 as c_int) }

        self.wait_next_tick();
    }
}

impl Drop for RlBotBm {
    fn drop(&mut self) {
        let handle = self.handle.as_ptr();
        unsafe { rlbot_bm_sys::RLBotBM_destroy(handle) };
    }
}
