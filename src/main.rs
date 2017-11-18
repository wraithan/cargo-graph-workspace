extern crate cargo_metadata;
extern crate clap;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use clap::{App, Arg};
use cargo_metadata::{DependencyKind};

fn main() {
    let matches = App::new("cargo-graph-workspace")
        .arg(
            Arg::with_name("manifest-path")
                .long("manifest-path")
                .value_name("PATH")
                .takes_value(true),
        )
        .get_matches();

    let manifest_path = matches.value_of("manifest-path").map(Path::new);
    let workspace_metadata =
        cargo_metadata::metadata(manifest_path).expect("Couldn't load workspace metadata");
    let deps_metadata =
        cargo_metadata::metadata_deps(manifest_path, true).expect("Couldn't load deps metadata");
    let workspace_packages: Vec<String> = workspace_metadata
        .packages
        .into_iter()
        .map(|p| p.name)
        .collect();

    let mut graph = Graph::new();
    let mut workload = workspace_packages.clone();
    while let Some(package_name) = workload.pop() {
        if graph.contains_package(&package_name) {
            continue;
        }
        graph.add(package_name.clone(), Node::Dependency);
        if let Some(package) = deps_metadata.packages.iter().find(|p| p.name == package_name) {
            for dep in &package.dependencies {
                match dep.kind {
                    DependencyKind::Build => continue,
                    DependencyKind::Development => continue,
                    DependencyKind::Normal => {
                        graph.add_relationship(package.name.clone(), dep.name.clone(), 1);
                        workload.push(dep.name.clone());
                    },
                };
            }
        }
    }

    for package in workspace_packages.into_iter() {
        graph.add(package, Node::Core);
    }
    // println!("{:?}", graph);
    graph.draw();
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<String, Node>,
    relationships: HashMap<(String, String), u32>,
}

#[derive(Debug)]
enum Node {
    Core,
    Dependency,
}

impl Graph {
    fn new() -> Self {
        Graph {
            ..Default::default()
        }
    }

    fn add(&mut self, name: String, node_type: Node) {
        match node_type {
            Node::Core => {
                self.nodes.insert(name, node_type);
            }
            Node::Dependency => if !self.nodes.contains_key(&name) {
                self.nodes.insert(name, node_type);
            },
        };
    }

    fn add_relationship(&mut self, left: String, right: String, weight: u32) {
        self.relationships.insert((left, right), weight);
    }

    fn contains_package(&self, name: &String) -> bool {
        self.nodes.contains_key(name)
    }

    fn draw(&self) {
        let mut file = File::create("output.dot").unwrap();
        file.write(b"digraph DepGraph {\n\tmincross = 2.0;\n\tratio = \"auto\";\n").unwrap();
        for (name, node_type) in &self.nodes {
            let shape = match node_type {
                &Node::Core => "box",
                &Node::Dependency => "circle",
            };
            write!(file, "\t\"{}\" [shape={}];\n", name, shape).unwrap();
        }
        for (&(ref left, ref right), weight) in &self.relationships {
            write!(file, "\t\"{}\" -> \"{}\" [weight={}];\n", left, right, weight).unwrap();
        }
        file.write(b"}\n").unwrap();
    }
}
