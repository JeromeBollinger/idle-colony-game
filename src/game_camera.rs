use bevy::prelude::*;

pub fn default_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_zoom(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>) {

    let dist = ZOOM_SPEED * time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        if kb.pressed(KeyCode::PageUp) {
            log_scale -= dist;
        }
        if kb.pressed(KeyCode::PageDown) {
            log_scale += dist;
        }

        projection.scale = log_scale.exp();
    }
}
