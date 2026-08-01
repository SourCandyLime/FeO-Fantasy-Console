mod physics;
mod error;

pub use physics::Physics;
pub use error::PhysicsError;

pub fn init() -> Result<Physics, PhysicsError> {
    Physics::new().map_err(|e| PhysicsError::InitializationFailed)
}