use bevy::prelude::*;

use super::{
    choose_movement_mode, keyboard_and_mouse_input, map_camera_transform, map_input_movement,
    map_input_orientation,
};

/// A standard FPS controller system configuration.
pub struct FpsControllerPlugin;

impl Plugin for FpsControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                // Handle Player Inputs
                keyboard_and_mouse_input,
                choose_movement_mode,
                // Update the Controller
                map_input_orientation,
                map_input_movement,
                // Update the Camera
                map_camera_transform,
            )
                .chain(),
        );
    }
}
