pub enum PluginError{
}

pub fn handle_plugin(plugin_name: &str, req: Request) -> Result<impl Serialize, PluginError> {
    plugin_handler::handle_request(plugin_name, req)
}
