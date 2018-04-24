#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod patchstorage;
use patchstorage::{get_patch_list, get_patch_contents};

mod vcv;
use vcv::{get_modules, process_module_statistics, ModuleStatistic};

use prettytable::Table;
use std::collections::HashMap;

    for (_key, module) in current_module_stats {
        module_stats.count_module(module.plugin, module.model);
    }
}

fn print_statistics(statistics: &HashMap<String,ModuleStatistic>) {
    let mut table = Table::new();
    let mut statistics_vec: Vec<&ModuleStatistic> = Vec::new();

    for (_key, value) in statistics {
        statistics_vec.push(value);
    }

    statistics_vec.sort_by(|a,b| b.count.cmp(&a.count));

    for value in statistics_vec {
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
