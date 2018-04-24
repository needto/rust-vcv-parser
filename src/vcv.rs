extern crate serde_json;

use std::collections::HashMap;

#[derive(Debug)]
pub struct ModuleStatistic {
    pub plugin: String,
    pub model: String,
    pub count: u32
}

impl ModuleStatistic {
    pub fn new(plugin: String, model: String) -> ModuleStatistic {
        ModuleStatistic {
            plugin: plugin,
            model: model,
            count: 0
        }
    }
}

type ModuleStats = HashMap<String,ModuleStatistic>;

trait CountModule {
    fn count_module(&mut self, plugin: String, model: String);
}

impl CountModule for ModuleStats{
    fn count_module(&mut self, plugin: String, model: String) {
        let module_name = format!("{}{}", plugin, model);
        let module_plugin = plugin;
        let module_model = model;
        let module_statistic = self.entry(module_name).or_insert(ModuleStatistic::new(module_plugin, module_model));
        module_statistic.count += 1;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VcvPatchModule {
  plugin: String,
  #[serde(default)]
  version: Option<String>,
  model: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VcvPatch {
  version: String,
  modules: Vec<VcvPatchModule>
}


pub fn get_modules(json: String) -> Vec<VcvPatchModule> {
    let vcv_patch: VcvPatch = serde_json::from_str(&json).unwrap();
    return vcv_patch.modules;
}

pub fn process_module_statistics(modules: Vec<VcvPatchModule>, module_stats: &mut ModuleStats) {
    let mut current_module_stats: ModuleStats = HashMap::new();

    for module in modules {
        current_module_stats.count_module(module.plugin, module.model);
    }

    for (_key, module) in current_module_stats {
        module_stats.count_module(module.plugin, module.model);
    }
}
