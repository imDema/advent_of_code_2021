use std::{collections::HashMap, str::FromStr, io::Read};

use anyhow::Context;
use itertools::Itertools;

const START: &'static str = "start";
const END: &'static str = "end";


#[derive(Default)]
struct Node {
    edges: Vec<String>,
}


struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn count_paths(&self) -> usize {
        self.count_paths_inner(&START.to_string(),&mut vec![], false)
    }

    fn count_paths_inner(&self, from: &String, stack: &mut Vec<String>, doubled: bool) -> usize {
        let mut paths = 0;
        let node = &self.nodes[from];
        stack.push(from.to_string());
        for connected in node.edges.iter() {
            if connected == END {
                paths += 1;
            } else if connected != START {
                paths += match (count_element(stack, connected), is_upper(connected)) {
                    (_, true)               => self.count_paths_inner(connected, stack, doubled),
                    (0, false)              => self.count_paths_inner(connected, stack, doubled),
                    (1, false) if !doubled  => self.count_paths_inner(connected, stack, true),
                    _ => 0
                }
            }
        }
        stack.pop();
        paths
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let edges = s.lines()
            .map(|l| l.split('-').to_owned().collect_tuple().context("Wrong format"));
        for r in edges {
            let (a, b) = r?;
            let node = nodes.entry(a.to_owned()).or_default();
            node.edges.push(b.to_owned());
            let node = nodes.entry(b.to_owned()).or_default();
            node.edges.push(a.to_owned());
        }
        Ok(Graph{
            nodes,
        })
    }
}

fn is_upper(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn count_element<T: PartialEq>(v: &Vec<T>, e: &T) -> usize {
    v.iter().filter(|&x| x == e).count()
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let graph: Graph = buf.parse()?;
    let paths = graph.count_paths();

    println!("paths: {}", paths);

    Ok(())
}
