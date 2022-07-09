pub trait ColorableGraph {
    fn num_vertices(&self) -> usize;
    fn neighbors(&self, vi: usize) -> &[usize];

    fn max_degree(&self) -> usize {
        (0..self.num_vertices())
            .map(|i| self.degree(i))
            .max()
            .unwrap_or(0)
    }

    fn degree(&self, i: usize) -> usize {
        self.neighbors(i).len()
    }
}

pub struct VecVecGraph {
    pub edges: Vec<Vec<usize>>,
}

impl ColorableGraph for &VecVecGraph {
    fn num_vertices(&self) -> usize {
        self.edges.len()
    }

    fn neighbors(&self, vi: usize) -> &[usize] {
        &self.edges[vi]
    }
}

impl VecVecGraph {
    pub fn new(num_vertices: usize) -> Self {
        VecVecGraph {
            edges: (0..num_vertices).map(|_| vec![]).collect(),
        }
    }

    pub fn add_edge(&mut self, w: usize, v: usize) {
        self.edges[w].push(v);
        self.edges[v].push(w);
    }

    pub fn from_dimacs_file(path: &dyn AsRef<std::path::Path>) -> Result<Self, Box<dyn std::error::Error>> {
        use std::io::BufRead;

        let f = std::fs::File::open(path)?;
        let r = std::io::BufReader::new(f);
        let mut graph = None;
        let rgraph = &mut graph;
        for (i, line) in r.lines().enumerate() {
            let line = line?;
            (|| {
                let mut it = line.split(' ');
                match it.next() {
                    Some("c" | "n" | "x" | "d" | "v") | None => {},
                    Some("p") => {
                        let format = it.next()?;
                        if format != "edge" { return None }
                        let num_vertices = it.next()?.parse::<usize>().ok()?;
                        let _num_edges = it.next()?.parse::<usize>().ok()?;
                        if !it.next().is_none() { return None; }
                        if ! rgraph.is_none() { return None }
                        *rgraph = Some(Self::new(num_vertices));
                    }
                    Some("e") => {
                        let w = it.next()?.parse::<usize>().ok()? - 1;
                        let v = it.next()?.parse::<usize>().ok()? - 1;
                        let max = rgraph.as_ref()?.num_vertices();
                        if w >= max || v >= max { return None; }
                        if w != v {
                           rgraph.as_mut()?.add_edge(w, v);
                        }
                    }
                    _ => { return None },
                }
                Some(())
            })().ok_or(format!("invalid line {}: {}", i+1, line))?;
        }
        Ok(graph.ok_or("no graph defined in file")?)
    }
}

pub struct CsrGraph {
    pub vertices: Vec<usize>,
    pub edges: Vec<usize>,
}

impl ColorableGraph for &CsrGraph {
    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn neighbors(&self, vi: usize) -> &[usize] {
        &self.edges[if vi == 0 { 0 } else { self.vertices[vi-1] } .. self.vertices[vi]]
    }
}

impl<T> From<T> for CsrGraph
where T : ColorableGraph {
    fn from(graph : T) -> Self {
        let mut vertices = Vec::with_capacity(graph.num_vertices());
        let mut edges = vec![];
        let mut offset = 0;
        for i in 0..graph.num_vertices() {
            let neighbors = graph.neighbors(i);
            edges.extend(neighbors);
            offset += neighbors.len();
            vertices.push(offset);
        }
        CsrGraph {
            vertices,
            edges,
        }
    }
}

fn color_greedy_by_order(
    graph: impl ColorableGraph,
    order: impl Iterator<Item = usize>,
) -> Vec<usize> {
    let mut coloring = vec![usize::MAX; graph.num_vertices()];

    for i in order {
        let mut ncs = vec![];
        for &n in graph.neighbors(i) {
            let nc = coloring[n];
            if nc != usize::MAX {
                if nc >= ncs.len() {
                    ncs.resize(nc + 1, false);
                }
                ncs[nc] = true;
            }
        }
        for c in 0.. {
            if c >= ncs.len() || !ncs[c] {
                coloring[i] = c;
                break;
            }
        }
    }

    coloring
}

pub fn color_greedy_naive(graph: impl ColorableGraph) -> Vec<usize> {
    let range = 0..graph.num_vertices();
    color_greedy_by_order(graph, range)
}

pub fn color_greedy_by_degree(graph: impl ColorableGraph) -> Vec<usize> {
    let mut vertices: Vec<_> = (0..graph.num_vertices()).collect();
    vertices.sort_by_key(|&i| std::cmp::Reverse(graph.neighbors(i).len()));
    color_greedy_by_order(graph, vertices.iter().cloned())
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct DSatur {
    saturation: usize,
    degree_uncolored: usize,
    index: std::cmp::Reverse<usize>,
}

pub fn color_greedy_dsatur(graph: impl ColorableGraph) -> Vec<usize> {
    let mut coloring = vec![usize::MAX; graph.num_vertices()];
    let mut neighbor_coloring: Vec<Vec<bool>> = vec![];
    neighbor_coloring.resize_with(graph.num_vertices(), Vec::new);

    let mut dsaturs = vec![];
    for i in 0..graph.num_vertices() {
        dsaturs.push(DSatur {
            saturation: 0,
            degree_uncolored: graph.neighbors(i).len(),
            index: std::cmp::Reverse(i),
        });
    }

    let mut heap = std::collections::BTreeSet::from_iter(dsaturs.iter().cloned());

    loop {
        // TODO: use pop_last when stabilized
        let dsatur = match heap.iter().last() {
            None => break,
            Some(x) => x.clone(),
        };
        let i = dsatur.index.0;
        let removed = heap.remove(&dsatur);
        assert!(removed);

        let ncs = &neighbor_coloring[i];
        let mut c = usize::MAX;
        for ci in 0.. {
            if ci >= ncs.len() || !ncs[ci] {
                c = ci;
                break;
            }
        }
        assert!(coloring[i] == usize::MAX);
        coloring[i] = c;

        for &n in graph.neighbors(i) {
            if coloring[n] != usize::MAX {
                continue;
            }
            let dsatur = &mut dsaturs[n];
            let removed = heap.remove(dsatur);
            assert!(removed);

            let ncs = &mut neighbor_coloring[n];
            if c >= ncs.len() {
                ncs.resize(c + 1, false);
            }
            let has_color = ncs[c];
            if !has_color {
                ncs[c] = true;
                dsatur.saturation += 1;
            }
            dsatur.degree_uncolored -= 1;

            heap.insert(dsatur.clone());
        }
    }

    coloring
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Rlf {
    next_neighbors: usize,
    other_neighbors: std::cmp::Reverse<usize>,
    index: std::cmp::Reverse<usize>,
}

pub fn color_greedy_rlf(graph: impl ColorableGraph) -> Vec<usize> {
    let mut coloring = vec![usize::MAX; graph.num_vertices()];
    let mut degree_next: Vec<usize> = vec![0; graph.num_vertices()];
    let mut degree_other: Vec<usize> = vec![0; graph.num_vertices()];

    let mut nocolor = usize::MAX;
    let mut nextcolor = usize::MAX - 1;

    for c in 0.. {
        for i in 0..graph.num_vertices() {
            if coloring[i] != nocolor {
                continue;
            }

            degree_next[i] = 0;
            degree_other[i] = graph
                .neighbors(i)
                .iter()
                .filter(|&&n| coloring[n] == nocolor)
                .count();
        }

        let mut current_vertex = (0..graph.num_vertices())
            .filter(|&i| coloring[i] == nocolor)
            .max_by_key(|&i| (degree_other[i], std::cmp::Reverse(i)));
        if current_vertex.is_none() {
            break;
        }

        while let Some(i) = current_vertex {
            debug_assert!(coloring[i] == nocolor);
            coloring[i] = c;

            for &n in graph.neighbors(i) {
                if coloring[n] != nocolor {
                    continue;
                }
                coloring[n] = nextcolor;

                for &n2 in graph.neighbors(n) {
                    if coloring[n2] != nocolor {
                        continue;
                    }

                    degree_next[n2] += 1;
                    degree_other[n2] -= 1;
                }
            }

            // TODO: use pop_last when stabilized?
            current_vertex = (0..graph.num_vertices())
                .filter(|&i| coloring[i] == nocolor)
                .max_by_key(|&i| {
                    (
                        degree_next[i],
                        std::cmp::Reverse(degree_other[i]),
                        std::cmp::Reverse(i),
                    )
                });
        }
        std::mem::swap(&mut nextcolor, &mut nocolor);
    }

    coloring
}

pub fn count_colors(coloring: &[usize]) -> usize {
    match coloring.iter().max() {
        None => 0,
        Some(x) => x + 1,
    }
}

pub fn validate_coloring(graph: impl ColorableGraph, coloring: &[usize]) {
    for i in 0..graph.num_vertices() {
        let c = coloring[i];
        assert!(c != usize::MAX, "no color for vertex {i}");
        for &n in graph.neighbors(i) {
            assert!(
                c != coloring[n],
                "vertex {} and neighbor {} both have color {}",
                i + 1,
                n + 1,
                c
            );
        }
    }
}
