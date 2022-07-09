use heuristic_graph_coloring::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut paths = vec![];

    for entry in std::fs::read_dir("instances")? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }
        let path_str = path.to_string_lossy();
        if !path_str.ends_with(".col") {
            continue;
        }

        paths.push(path);
    }

    paths.sort();

    println!("colors\tmicroseconds\tname\tpath");
    for path in paths {
        let graph = &match VecVecGraph::from_dimacs_file(&path) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error loading {:?}: {}, Skipping.", path, e);
                continue;
            }
        };
        let csr = &CsrGraph::from(graph);
        for i in 0..8 {
            use std::time::Instant;
            let now = Instant::now();
            let (name, coloring) = match i {
                0 => ("color_greedy_naive", color_greedy_naive(graph)),
                1 => ("color_greedy_naive csr", color_greedy_naive(csr)),
                2 => ("color_greedy_by_degree", color_greedy_by_degree(graph)),
                3 => ("color_greedy_by_degree csr", color_greedy_by_degree(csr)),
                4 => ("color_greedy_dsatur", color_greedy_dsatur(graph)),
                5 => ("color_greedy_dsatur csr", color_greedy_dsatur(csr)),
                6 => ("color_greedy_rlf", color_greedy_rlf(graph)),
                7 => ("color_greedy_rlf csr", color_greedy_rlf(csr)),
                _ => unreachable!(),
            };
            let elapsed = now.elapsed();
            println!(
                "{}\t{}\t{}\t{}",
                count_colors(&coloring),
                elapsed.as_micros(),
                name,
                path.file_name().unwrap().to_str().unwrap(),
            );
            validate_coloring(graph, &coloring);
        }
    }

    Ok(())
}
