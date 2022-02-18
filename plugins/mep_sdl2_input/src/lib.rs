use std::rc::Rc;

use libmep::{
    graphics::Style,
    plugin::{GraphicPlugin, Plugin, PluginInfo, PluginType},
};

struct MepSdl2InputPlugin {}

impl Plugin for MepSdl2InputPlugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        return Self {};
    }

    fn init(&mut self) {}

    fn get_plugin_info(&self) -> PluginInfo {
        return PluginInfo {
            plugin_name: "SDL2 Input",
            plugin_version: option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
            plugin_type: PluginType::InputPlugin,
        };
    }
}

#[no_mangle]
pub fn mep_get_plugin() -> Rc<dyn Plugin> {
    return Rc::new(MepSdl2InputPlugin::new());
}
