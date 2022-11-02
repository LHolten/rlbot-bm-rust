#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn take_bot_control() {
        unsafe {
            let handle = RLBotBM_create();
            if handle.is_null() {
                return;
            }

            let controls = ControllerInput {
                throttle: -1.,
                ..Default::default()
            };

            let mut state = GameStateObj::default();

            loop {
                RLBotBM_waitNextTick(handle, &mut state);
                RLBotBM_setBotInput(handle, &controls, 0);
            }
        }
    }
}
