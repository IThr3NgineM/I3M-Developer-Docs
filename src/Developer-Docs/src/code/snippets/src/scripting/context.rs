use i3m::asset::manager::ResourceManager;
use i3m::core::pool::Handle;
use i3m::engine::task::TaskPoolHandler;
use i3m::engine::{
    AsyncSceneLoader, GraphicsContext, PerformanceStatistics, ScriptMessageDispatcher,
    ScriptProcessor, SerializationContext,
};
use i3m::event_loop::EventLoopWindowTarget;
use i3m::gui::constructor::WidgetConstructorContainer;
use i3m::gui::UiContainer;
use i3m::scene::node::Node;
use i3m::scene::{Scene, SceneContainer};
use i3m::script::{PluginsRefMut, ScriptMessageSender};
use std::sync::Arc;

// ANCHOR: context
pub struct ScriptContext<'a, 'b, 'c> {
    pub dt: f32,
    pub elapsed_time: f32,
    pub plugins: PluginsRefMut<'a>,
    pub handle: Handle<Node>,
    pub scene: &'b mut Scene,
    pub scene_handle: Handle<Scene>,
    pub resource_manager: &'a ResourceManager,
    pub message_sender: &'c ScriptMessageSender,
    pub message_dispatcher: &'c mut ScriptMessageDispatcher,
    pub task_pool: &'a mut TaskPoolHandler,
    pub graphics_context: &'a mut GraphicsContext,
    pub user_interfaces: &'a mut UiContainer,
    pub script_index: usize,
}
// ANCHOR_END: context

// ANCHOR: plugin_context
pub struct PluginContext<'a, 'b> {
    pub scenes: &'a mut SceneContainer,
    pub resource_manager: &'a ResourceManager,
    pub user_interfaces: &'a mut UiContainer,
    pub graphics_context: &'a mut GraphicsContext,
    pub dt: f32,
    pub lag: &'b mut f32,
    pub serialization_context: &'a Arc<SerializationContext>,
    pub widget_constructors: &'a Arc<WidgetConstructorContainer>,
    pub performance_statistics: &'a PerformanceStatistics,
    pub elapsed_time: f32,
    pub script_processor: &'a ScriptProcessor,
    pub async_scene_loader: &'a mut AsyncSceneLoader,
    pub window_target: Option<&'b EventLoopWindowTarget<()>>,
    pub task_pool: &'a mut TaskPoolHandler,
}
// ANCHOR_END: plugin_context
