use libmep::plugin::{PluginManager, PluginType};

fn main() {
    colog::init();
    let mut manager = PluginManager::new();
    manager.load_all_plugins();
    if manager.plugins[0].get_plugin_info().plugin_type == PluginType::GraphicsPlugin {}
}
