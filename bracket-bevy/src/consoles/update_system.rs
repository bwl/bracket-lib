use crate::{BracketCamera, BracketContext, TerminalScalingMode};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::mesh::Mesh2d,
    window::WindowResized,
};

use super::{BracketMesh, ScreenScaler};

pub(crate) fn update_consoles(
    mut ctx: ResMut<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    find_mesh: Query<(&BracketMesh, &Mesh2d)>,
    scaler: Res<ScreenScaler>,
) {
    let mut new_meshes: Vec<(Mesh2d, Mesh2d, bool)> = Vec::new();
    {
        let mut terms = ctx.terminals.lock();
        for (id, handle) in find_mesh.iter() {
            let terminal_id = id.0;
            let new_mesh = terms[terminal_id].new_mesh(&ctx, &mut meshes, &scaler);
            if let Some(new_mesh) = new_mesh {
                let old_mesh = handle.clone();
                new_meshes.push((old_mesh, new_mesh.into(), false));
            }
        }
    }

    new_meshes
        .drain(0..)
        .for_each(|m| ctx.mesh_replacement.push(m));
}

pub(crate) fn replace_meshes(
    mut ctx: ResMut<BracketContext>,
    mut ev_asset: EventReader<AssetEvent<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut update_mesh: Query<&mut Mesh2d, With<BracketMesh>>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::Added { id } = ev {
            for (old, new, done) in ctx.mesh_replacement.iter_mut() {
                if *id == new.0.id() {
                    for mut m in &mut update_mesh {
                        if old.0.id() == m.0.id() {
                            *m = new.clone();
                        }
                    }
                    *done = true;
                }
            }
        }
    }

    for (old, _, _) in ctx.mesh_replacement.iter().filter(|(_, _, done)| *done) {
        meshes.remove(old.0.id());
    }
    ctx.mesh_replacement.retain(|(_, _, done)| !done);
}

pub(crate) fn update_timing(mut ctx: ResMut<BracketContext>, diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.measurement() {
            ctx.fps = fps_avg.value.round();
        }
    }

    if let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_avg) = frame_time.measurement() {
            ctx.frame_time_ms = (frame_time_avg.value * 1000.0).round();
        }
    }
}

pub(crate) fn window_resize(
    mut context: ResMut<BracketContext>,
    mut resize_event: EventReader<WindowResized>,
    mut scaler: ResMut<ScreenScaler>,
) {
    for e in resize_event.read() {
        scaler.set_screen_size(e.width, e.height);
        if let TerminalScalingMode::ResizeTerminals = context.scaling_mode {
            context.resize_terminals(&scaler);
        }
        scaler.recalculate(
            context.get_pixel_size(),
            context.largest_font(),
            context.scaling_mode,
        );
    }
}

pub(crate) fn apply_all_batches(mut context: ResMut<BracketContext>) {
    context.render_all_batches();
}

pub(crate) fn update_mouse_position(
    wnds: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<BracketCamera>>,
    mut context: ResMut<BracketContext>,
    scaler: Res<ScreenScaler>,
) {
    // Modified from: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    // Bevy really needs a nicer way to do this
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    let wnd = wnds.single().ok();

    let wnd = if let Some(wnd) = wnd {
        wnd
    } else {
        return;
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width(), wnd.height());
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.clip_from_view().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        let result = (world_pos.x, world_pos.y);

        context.set_mouse_pixel_position(result, &scaler);
    }
}
