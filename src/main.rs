#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod patchstorage;
use patchstorage::{get_patch_list, get_patch_contents};

use std::collections::HashMap;
use prettytable::Table;

#[derive(Debug)]
pub struct ModuleStatistic {
    plugin: String,
    model: String,
    count: u32
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

#[derive(Serialize, Deserialize, Debug)]
struct VcvPatchModule {
  plugin: String,
  version: String,
  model: String
}

#[derive(Serialize, Deserialize, Debug)]
struct VcvPatch {
  version: String,
  modules: Vec<VcvPatchModule>
}


fn get_modules(s: String) -> Vec<VcvPatchModule> {
    let vcv_patch: VcvPatch = serde_json::from_str(&s).unwrap();
    return vcv_patch.modules;
}

fn process_module_statistics(modules: Vec<VcvPatchModule>, module_stats: &mut HashMap<String,ModuleStatistic>) {
    let mut current_module_stats = HashMap::new();

    for module in modules {
        let module_name = format!("{}{}", module.plugin, module.model);
        let module_plugin = module.plugin;
        let module_model = module.model;
        let module_statistic = current_module_stats.entry(module_name).or_insert(ModuleStatistic::new(module_plugin, module_model));
        module_statistic.count += 1;
    }

    for (_key, value) in current_module_stats {
        let module_name = format!("{}{}", value.plugin, value.model);
        let module_plugin = value.plugin;
        let module_model = value.model;
        let module_statistic = module_stats.entry(module_name).or_insert(ModuleStatistic::new(module_plugin, module_model));
        module_statistic.count += 1;
    }
}

fn print_statistics(statistics: &HashMap<String,ModuleStatistic>) {
    let mut table = Table::new();
    for (_key, value) in statistics {
        table.add_row(row![FdBybl->value.plugin, Fy->value.model, Fy->value.count]);
    }
    table.printstd();
}

fn main() {
    let urls = get_patch_list();
    let mut module_stats = HashMap::new();
    for url in urls {
        println!("Getting patch contents from: {:?}", url);
        match get_patch_contents(url) {
            Some(patch) => {
                let modules = get_modules(patch);
                process_module_statistics(modules, &mut module_stats);
            },
            None => println!("Could not retrieve VCVRack patch")
        }

    }
    print_statistics(&module_stats);
}
