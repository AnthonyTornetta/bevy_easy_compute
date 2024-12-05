#![doc = include_str!("../README.md")]

use bevy::{
    prelude::{AssetEvent, Assets, EventReader, Res, ResMut, Shader},
    render::render_resource::PipelineCache,
};

mod error;
mod gpu_readback;
mod plugin;
mod traits;
mod worker;
mod worker_builder;

/// Helper module to import most used elements.
pub mod prelude {
    pub use crate::{
        plugin::{
            AppComputePlugin, AppComputeWorkerPlugin, BevyEasyComputePostUpdateSet,
            BevyEasyComputeSet,
        },
        traits::{ComputeShader, ComputeWorker},
        worker::AppComputeWorker,
        worker_builder::AppComputeWorkerBuilder,
    };

    // Since these are always used when using this crate
    pub use bevy::render::render_resource::{ShaderRef, ShaderType};
}

// https://github.com/bevyengine/bevy/blob/d2a07f9f72ea2b2d57383d91e63c37d0b7952324/crates/bevy_render/src/render_resource/pipeline_cache.rs#L918
// pub(crate) fn process_pipeline_queue_system(mut pipeline_cache: ResMut<AppPipelineCache>) {
//     pipeline_cache.process_queue();
// }
//
// pub(crate) fn extract_shaders(
//     mut pipeline_cache: ResMut<PipelineCache>,
//     shaders: Res<Assets<Shader>>,
//     mut events: EventReader<AssetEvent<Shader>>,
// ) {
//     for event in events.read() {
//         match event {
//             AssetEvent::Added { id: shader_id } | AssetEvent::Modified { id: shader_id } => {
//                 if let Some(shader) = shaders.get(*shader_id) {
//                     pipeline_cache.set_shader(shader_id, shader);
//                 }
//             }
//             AssetEvent::Removed { id: shader_id } => pipeline_cache.remove_shader(shader_id),
//             AssetEvent::LoadedWithDependencies { id: shader_id } => {
//                 if let Some(shader) = shaders.get(*shader_id) {
//                     pipeline_cache.set_shader(shader_id, shader);
//                 }
//             }
//             AssetEvent::Unused { id: _ } => (),
//         }
//     }
// }
