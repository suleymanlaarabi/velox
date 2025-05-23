use bevy::prelude::*;

use crate::components::*;

pub fn move_player(
    mut query: Query<
        (
            &mut Transform,
            &CharacterInputConfig,
            &CharacterMovement,
            Option<&CharacterAcceleration>,
        ),
        With<CharacterController>,
    >,
    camera_query: Query<
        (&ChildOf, &Transform),
        (
            With<CharacterControllerFpsCamera>,
            Without<CharacterController>,
        ),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (parent, camera_transform) in &camera_query {
        let (mut player_transform, config, movement, acceleration) =
            match query.get_mut(parent.parent) {
                Ok(transform) => transform,
                Err(_) => {
                    continue;
                }
            };
        let mut direction = Vec3::ZERO;
        if keys.pressed(config.move_front) {
            direction += camera_transform.forward().as_vec3();
        }
        if keys.pressed(config.move_back) {
            direction += camera_transform.back().as_vec3();
        }
        if keys.pressed(config.move_left) {
            direction += camera_transform.left().as_vec3();
        }
        if keys.pressed(config.move_right) {
            direction += camera_transform.right().as_vec3();
        }
        if direction != Vec3::ZERO {
            direction.y = 0.;
            let mut direction = (direction.normalize()) * movement.speed * delta_time;
            if let Some(acceleration) = acceleration {
                direction *= acceleration.0;
            }
            player_transform.translation.x += direction.x;
            player_transform.translation.z += direction.z;
        }
    }
}
