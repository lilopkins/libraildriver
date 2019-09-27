#![deny(missing_docs)]

//! # Lib Rail Driver
//!
//! Rust FFI bindings to the `RailDriver.dll` library.
//!
//! These allow you to read and write data to or from Train Simulator 2020. Note
//! that this doesn't work with Train Sim World.
//!
//! ## Quick example
//!
//! ```rust
//! extern crate libraildriver;
//!
//! fn main() {
//!     let context = libraildriver::Context::new();
//!     let speed = context.get_value(libraildriver::Value::Speedometer,
//!                   libraildriver::Kind::Current).expect("Failed to get value.");
//!     println!("The train's current speed is: {}", speed);
//! }
//! ```


extern crate libraildriver_sys as libraildriver;
extern crate libc;

#[derive(Debug)]
/// The value you wish to target for an operation.
pub enum Value {
    /// The current reverser setting (F/N/R).
    /// Usually:
    /// - `1`: F
    /// - `0`: N
    /// - `-1`: R
    Reverser,
    /// The current throttle setting, usually between `0` and `100`. For locomotives and units with
    /// a separate throttle and brake control.
    Throttle,
    /// The current combined throttle/brake setting, usually between `-100` and `100`. For
    /// locomotives and units with a combined throttle and brake.
    CombinedThrottle,
    /// The current gear, for trains fitted.
    GearLever,
    /// The train brake lever. Usually between `0` and `100`.
    TrainBrake,
    /// The locomotive brake lever. Usually between `0` and `100`.
    LocomotiveBrake,
    /// The dynamic brake lever. Usually between `0` and `100`.
    DynamicBrake,
    /// The emergency brake button. Usually operated by setting to `1`.
    EmergencyBrake,
    /// The hand brake. Usually operated by setting to `1`.
    HandBrake,
    /// The warning system reset button. Usually operated by setting to `1`.
    WarningSystemReset,
    /// The engine start/stop button. Usually operated by setting to `1`.
    StartStopEngine,
    /// The horn lever. Usually operated by setting to `1`.
    Horn,
    /// The wipers switch. Usually operated by setting to `1`.
    Wipers,
    /// The sander. Usually operated by setting to `1`.
    Sander,
    /// The headlights. Often operated by setting to `1`.
    Headlights,
    /// The pantograph switch. Usually raised by setting to `1`.
    Pantograph,
    /// The firebox door. Usually opened by setting to `1`.
    FireboxDoor,
    /// The exhaust-based steam injector. Usually between `0` and `100`.
    ExhaustInjectorSteam,
    /// The exhaust-based water injector. Usually between `0` and `100`.
    ExhaustInjectorWater,
    /// The live steam injector. Usually between `0` and `100`.
    LiveInjectorSteam,
    /// The live water injector. Usually between `0` and `100`.
    LiveInjectorWater,
    /// The damper. Usually actuated between `0` and `100`.
    Damper,
    /// The blower valve. Usually opened between `0` and `100`.
    Blower,
    /// Stoking. Usually stoking when set to `1`.
    Stoking,
    /// The cylinder cocks. Usually opened when set to `1`.
    CylinderCock,
    /// The waterscoop. Usually operated when set to `1`.
    Waterscoop,
    /// Currently undocumented.
    SmallCompressor,
    /// Get only: The state of the AWS.
    AWS,
    /// Set only: The AWS reset button, depressed when set to `1`.
    AWSReset,
    /// Whether the unit is in startup
    // TODO: Verify this
    Startup,
    /// Get only: The current speed of the unit.
    Speedometer,
    // Events
    /// The save event, usually triggered by `F2`, triggered when set to `1`.
    PromptSave,
    /// Toggle labels, triggered when set to `1`.
    // TODO: Check if this is toggled or enabled.
    ToggleLabels,
    /// The 2D Map, usually triggered by `9`, triggered when set to `1`.
    Toggle2DMap,
    /// Toggle the HUD visibilty, usually triggered by `F3` or `F4`, triggered when set to `1`.
    ToggleHud,
    /// Currently undocumented.
    ToggleQut,
    /// Pause the game, triggered when set to `1`.
    Pause,
    /// Currently undocumented.
    DriverGuide,
    /// Show the rail vehicle number, enabled when set to `1`.
    // TODO: Check whether this is enabled or toggled.
    ToggleRvNumber,
    /// Close the dialog given to you by an assignment in a scenario.
    DialogAssignment,
    /// Switch the set of points to the front of the train, triggered when set to `1`.
    SwitchJunctionAhead,
    /// Switch the set of points to the rear of the train, triggered when set to `1`.
    SwitchJunctionBehind,
    /// The load cargo event, triggered when set to `1`.
    LoadCargo,
    /// The unload cargo action, triggered when set to `1`.
    UnloadCargo,
    /// Request to pass a signal at danger to the front of the train, usually triggered by `Tab`,
    /// triggered when set to `1`.
    PassAtDangerAhead,
    /// Request to pass a signal at danger to the rear of the train, usually triggered by
    /// `Shift Tab`, triggered when set to `1`.
    PassAtDangerBehind,
    /// Manual coupling, triggered when set to `1`.
    ManualCouple,
    // Camera
    /// The cab camera, usually operated by pressing `1`, switched to by setting to `1`.
    CabCamera,
    /// The follow camera, usually operated by pressing `2`, switched to by setting to `1`.
    FollowCamera,
    /// The head-out-window camera, usually operated by pressing `Shift 2`, switched to by setting
    /// to `1`.
    HeadOutCamera,
    /// The rear camera, usually operated by pressing `3`, switched to by setting to `1`.
    RearCamera,
    /// The track-side camera, usually operated by pressing `4`, switched to by setting to `1`.
    TrackSideCamera,
    /// The passenger-view (carriage) camera, usually operated by pressing `5`, switched to by
    /// setting to `1`.
    CarriageCamera,
    /// The coupling camera, usually operated by pressing `6`, switched to by setting to `1`.
    CouplingCamera,
    /// The yard camera, usually operated by pressing `7`, switched to by setting to `1`.
    YardCamera,
    /// Cab camera switch, usually operated by pressing `Ctrl +`, switched to by setting to `1`.
    SwitchToNextFrontCab,
    /// Cab camera switch, usually operated by pressing `Ctrl -`, switched to by setting to `1`.
    SwitchToNextRearCab,
    /// The free camera, usually operated by pressing `8`, switched to by setting to `1`.
    FreeCamera,
}

#[derive(Debug)]
/// The kind of value that is required
pub enum Kind {
    /// The current value
    Current,
    /// The minimum value
    Min,
    /// The maximum value
    Max,
}

/// A custom `Result` type to simplify dealing with errors.
pub type Result<T> = std::result::Result<T, RailDriverError>;

#[derive(Debug)]
/// Errors returned by the operation being performed.
pub enum RailDriverError {
    /// The simulator doens't think this context is connected, therefore the operation cannot
    /// continue.
    NotConnected,
}

/// A controller context with the simulator.
pub struct Context {
    connected: bool,
}

impl Drop for Context {
    fn drop(&mut self) {
        self.disconnect();
    }
}

impl Context {

    /// Creates a new controller context with the simulator.
    pub fn new() -> Self {
        let mut val = Self {
            connected: false,
        };
        val.connect();
        val
    }

    fn connect(&mut self) {
        unsafe { libraildriver::SetRailDriverConnected(true); }
        self.connected = true;
    }

    fn disconnect(&mut self) {
        unsafe { libraildriver::SetRailDriverConnected(false); }
        self.connected = false;
    }

    /// Return the value of `of`. Depending on `kind`, this may return the current value, a
    /// minimum, or a maximum.
    pub fn get_value(&self, of: Value, kind: Kind) -> Result<f32> {
        if !self.connected {
            return Err(RailDriverError::NotConnected);
        }
        let of = of as libc::c_int;
        let kind = kind as libc::c_int;
        unsafe { Ok(libraildriver::GetRailSimValue(of, kind)) }
    }

    /// Set the value of `of` within the simulator to `value`.
    pub fn set_value(&self, of: Value, value: i32) -> Result<()> {
        if !self.connected {
            return Err(RailDriverError::NotConnected);
        }
        let of = of as libc::c_int;
        let value = value as libc::c_int;
        unsafe { libraildriver::SetRailSimValue(of, value); }
        Ok(())
    }
}
