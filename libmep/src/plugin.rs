use crate::graphics::Style;
use log::{error, info};
use std::{path::Path, rc::Rc};

const PLUGIN_ENTRY_SYMBOL: &[u8] = b"mep_get_plugin";
type PluginFunctionType = unsafe fn() -> Rc<dyn Plugin>;

#[derive(PartialEq)]
pub enum PluginType {
    GraphicsPlugin,
    InputPlugin,
}

pub struct PluginInfo {
    pub plugin_name: &'static str,
    pub plugin_version: &'static str,
    pub plugin_type: PluginType,
}

pub trait Plugin {
    fn new() -> Self
    where
        Self: Sized;
    fn init(&mut self);
    fn get_plugin_info(&self) -> PluginInfo;
}

pub trait GraphicPlugin: Plugin {
    fn commit_buffer(&mut self);
    fn draw_line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16);
    fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16);
    fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16);
    fn set_style(&mut self, style: Rc<Style>);
}

pub trait InputPlugin: Plugin {}

pub struct PluginManager {
    pub plugins: Vec<Rc<dyn Plugin>>,

    /// Libloading unloads libraries once they go out of context, so we have to force them to live as long as the plugin manager
    lib_cache: Vec<libloading::Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        return Self {
            plugins: Vec::new(),
            lib_cache: Vec::new(),
        };
    }

    pub fn load_all_plugins(&mut self) {
        #[cfg(target_os = "linux")]
        let regexp = "libmep*.so";

        #[cfg(target_os = "windows")]
        let regexp = "mep*.dll";

        // Glob all potential libraries
        let paths = match glob::glob(regexp) {
            Ok(tmp) => tmp,
            Err(err) => {
                error!("Looking for plugins resulted in: {}", err);
                std::process::exit(1);
            }
        };

        // Interate over the path results
        for path_result in paths {
            // Attempt the get the pathbuf
            let plugin_path = match path_result {
                Ok(tmp) => tmp,
                Err(err) => {
                    error!("Loading potential plugin path resulted in: {}", err);
                    std::process::exit(1);
                }
            };

            // Report that a module potentially has been found
            info!("Found potential plugin {}", plugin_path.display());

            let lib = match unsafe { libloading::Library::new(Path::new(".").join(plugin_path)) } {
                // Library instance is fine
                Ok(tmp) => tmp,
                // Could not load the module for some reason
                Err(err) => {
                    error!("Could not load plugin because: {}", err);
                    std::process::exit(1);
                }
            };

            // Get the function to extract the plugin
            let func: libloading::Symbol<PluginFunctionType> =

        // Look for the init symbol
        match unsafe { lib.get(PLUGIN_ENTRY_SYMBOL) } {
            // Symbol is correct
            Ok(tmp) => tmp,

            // Problem getting symbol
            Err(err) => {
                error!("Could not load symbol because: {}", err);
                std::process::exit(1);
            }
        };

            // Add the plugins to the collection
            self.plugins.push(unsafe { func() });

            // Save the library so libloading doesnt destroy it
            self.lib_cache.push(lib);
        }

        // Report how many plugins were loaded
        info!("Loaded {} plugins", self.plugins.len());

        info!("Loaded plugins infomation:");

        for plg in &self.plugins {
            let i = plg.get_plugin_info();
            info!("Plugin Name {}", i.plugin_name);
            info!("Plugin Version {}", i.plugin_version);
        }
    }
}
