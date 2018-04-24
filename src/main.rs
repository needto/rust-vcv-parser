#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate threadpool;
extern crate csv;
use csv::Writer;

mod patchstorage;
use patchstorage::{get_patch_list, get_patch_contents};

mod vcv;
use vcv::{get_modules, process_module_statistics, ModuleStatistic};

use prettytable::Table;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::collections::HashMap;

static NUM_THREADS:usize = 16;

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

fn export_statistics(statistics: &HashMap<String,ModuleStatistic>) {
    let mut statistics_vec: Vec<&ModuleStatistic> = Vec::new();

    for (_key, value) in statistics {
        statistics_vec.push(value);
    }

    statistics_vec.sort_by(|a,b| b.count.cmp(&a.count));

    let path = "modules.csv";
    let mut writer = Writer::from_file(path).unwrap();

    writer.encode(("Plugin", "Model", "Count")).expect("CSV writer error");

    for value in statistics_vec {
        writer.encode((&value.plugin, &value.model, value.count)).expect("CSV writer error");
    }
    writer.flush().expect("Flush error");
}

fn main() {
    let urls = get_patch_list();
    let mut module_stats = HashMap::new();

    let pool = ThreadPool::new(NUM_THREADS);
    let (sender, receiver) = channel();

    for url in urls.clone() {
        let sender = sender.clone();
        pool.execute(move || {
            println!("Getting patch contents from: {:?}", url);
            match get_patch_contents(url) {
                Some(patch) => {
                    let modules = get_modules(patch);
                    sender.send(Some(modules)).unwrap();
                },
                None => {
                    println!("Could not retrieve VCVRack patch");
                    sender.send(None).unwrap();
                }
            }
        });
    }

    pool.join();
    for _url in urls {
        match receiver.recv().unwrap() {
            Some(modules) => process_module_statistics(modules, &mut module_stats),
            None => ()
        }
    }
    print_statistics(&module_stats);
    export_statistics(&module_stats);
}
