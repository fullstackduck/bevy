mod drawable;
pub mod prelude;

pub use bevy_geometry;
pub use drawable::*;

use bevy_app::{App, CoreStage, Plugin};

mod system;

// TODO: using commands/ecs the best way? How about an array as resource?

// TODO: draw fn for line which has start and end point instead of origin like most shapes in immediate mode
// maybe not a problem in persistent mode as transform or helping component will be present?
#[derive(Default)]
pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            // TODO: PreUpdate better?
            .add_system_to_stage(CoreStage::Last, system::despawn);
    }
}
