use bevy::prelude::*;

use super::{
    choose_movement_mode, keyboard_and_mouse_input, map_camera_transform, map_input_movement,
    map_input_orientation, FpsControllerSet,
};

/// A standard FPS controller system configuration.
pub struct FpsControllerPlugin;

impl Plugin for FpsControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (keyboard_and_mouse_input, choose_movement_mode)
                .chain()
                .in_set(FpsControllerSet::Input),
        )
        .add_systems(
            (
                map_input_orientation,
                map_input_movement,
                map_camera_transform,
            )
                .chain()
                .in_set(FpsControllerSet::Update),
        )
        .configure_set(
            FpsControllerSet::Input.before(FpsControllerSet::Update),
        );
    }
}
