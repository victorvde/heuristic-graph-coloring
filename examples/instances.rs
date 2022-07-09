use heuristic_graph_coloring::*;
use poloto::prelude::*;

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

    println!("colors\tmicros\tname\tpath");
    let names = [
        "naive",
        "by_degree",
        "dsatur",
        "rlf",
    ];
    let mut results = std::collections::HashMap::new();
    for path in paths {
        results.insert(path.clone(), std::collections::HashMap::new());
        let graph = &match VecVecGraph::from_dimacs_file(&path) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error loading {:?}: {}, Skipping.", path, e);
                continue;
            }
        };
        let graph = &CsrGraph::from(graph);
        for i in 0..4 {
            use std::time::Instant;
            let now = Instant::now();
            let name = names[i];
            let coloring = match i {
                0 => color_greedy_naive(graph),
                1 => color_greedy_by_degree(graph),
                2 => color_greedy_dsatur(graph),
                3 => color_rlf(graph),
                _ => unreachable!(),
            };
            let elapsed = now.elapsed();
            validate_coloring(graph, &coloring);
            let count = count_colors(&coloring);
            println!(
                "{}\t{}\t{}\t{}",
                count,
                elapsed.as_micros(),
                name,
                path.file_name().unwrap().to_str().unwrap(),
            );
            results
                .get_mut(&path)
                .unwrap()
                .insert(name, vec![count as  f64, elapsed.as_nanos() as f64 / 1000000000.]);
        }
    }
    let titles = ["colors vs naive", "time vs naive"];
    let files = ["colors.svg", "time.svg"];
    let x_axis = ["naive colors", "naive (s)"];
    let y_axis = ["colors", "time (s)"];
    for i in 0..2 {
        let plotter = poloto::quick_fmt!(
            titles[i],
            x_axis[i],
            y_axis[i],
            poloto::build::plots_dyn(
                names[1..]
                    .iter()
                    .map(|n| {
                        let mut data : Vec<_> = results
                            .values()
                            .map(|v| (v["naive"][i], v[n][i])).collect();
                        data.sort_by(|a, b| a.0.total_cmp(&b.0));
                        data.iter().buffered_plot().scatter(n)
                    })
                    .collect()
            )
        );
        let k = format!("{}", poloto::disp(|w| plotter.simple_theme(w)));
        std::fs::write(files[i], k.as_bytes())?;
    }
    Ok(())
}
