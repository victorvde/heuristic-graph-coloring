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
        for i in 0..4 {
            let graph = VecVecGraph::from_dimacs_file(&path);
            use std::time::Instant;
            let now = Instant::now();
            let (name, coloring) = match i {
                0 => ("color_greedy_naive", color_greedy_naive(&graph)),
                1 => ("color_greedy_by_degree", color_greedy_by_degree(&graph)),
                2 => ("color_greedy_dsatur", color_greedy_dsatur(&graph)),
                3 => ("color_greedy_rlf", color_greedy_rlf(&graph)),
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
            validate_coloring(&graph, &coloring);
        }
    }

    Ok(())
}
