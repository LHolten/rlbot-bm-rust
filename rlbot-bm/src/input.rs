#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy)]
pub struct ControllerInput {
    pub throttle: f32,
    pub steer: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub handbrake: bool,
    pub jump: bool,
    pub boost: bool,
    /// this is probably a car index
    pub(crate) item_target: Option<u32>,
}
