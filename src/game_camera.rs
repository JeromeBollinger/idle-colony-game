use bevy::prelude::*;

const ZOOM_SPEED: f32 = 10.0;
const ZOOM_MAX: f32 = -4.0; // Ensure that ZOOM_MAX < ZOOM_MIN
const ZOOM_MIN: f32 = 2.0;

pub fn default_camera(mut commands: Commands) {
    assert!(ZOOM_MAX < ZOOM_MIN);
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: (ZOOM_MAX - ((ZOOM_MAX - ZOOM_MIN) / 2.0)).exp(),
            ..default()
        },
        ..default()
    });
}

pub fn camera_zoom(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let dist = ZOOM_SPEED * time.delta().as_secs_f32();
    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();
        if kb.pressed(KeyCode::PageUp) {
            if log_scale - dist > ZOOM_MAX {
                log_scale -= dist;
            } else {
                log_scale = ZOOM_MAX;
            }
        }
        if kb.pressed(KeyCode::PageDown) {
            if log_scale + dist < ZOOM_MIN {
                log_scale += dist;
            } else {
                log_scale = ZOOM_MIN;
            }
        }
        projection.scale = log_scale.exp();
    }
}
