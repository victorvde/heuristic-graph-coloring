[GitHub](https://github.com/victorvde/heuristic-graph-coloring) | [Crates.io](https://crates.io/crates/heuristic-graph-coloring) | [Docs.rs](https://docs.rs/heuristic-graph-coloring/latest/heuristic-graph-coloring/)

This crate provides algorithms for solving the [graph vertex coloring problem](https://en.wikipedia.org/wiki/Graph_coloring).
These algorithms return a "coloring", i.e. an assignment of each vertex to a "color" (index) such that no two vertices of the same color are connected by an edge.
The algorithms use heuristics to minimize the number of different colors used.

Current status: basic functionality is working, but not very optimized and not extensively tested.

Example:
```
use heuristic_graph_coloring::*;

let mut graph = VecVecGraph::new(4); // Create a new graph with 4 vertices
graph.add_edge(0, 1);
graph.add_edge(1, 2);
graph.add_edge(0, 2);
graph.add_edge(0, 3);
let coloring = color_greedy_by_degree(&graph);
println!("{:?}", coloring); // [0, 1, 2, 1]
```

# Algorithms

Name | Function | Colors used | Time used | Comment
---|---|---|---|---
[Greedy (naive)](https://en.wikipedia.org/wiki/Greedy_coloring) | [color_greedy_naive] | Most | Least | Only use as a baseline.
[Greedy (by degree)](https://en.wikipedia.org/wiki/Greedy_coloring) | [color_greedy_by_degree] | A bit less | Least | Fast decent results.
[DSatur](https://en.wikipedia.org/wiki/DSatur) | [color_greedy_dsatur] | Less | More | Good results but quite slow.
[Recursive largest first](https://en.wikipedia.org/wiki/Recursive_largest_first_algorithm) | [color_rlf] | Even less | Even more | Slowest and worst time complexity, but best results.

If you really want the least number of colors there is are slower algorightms like backtracking, SAT-solvers or HEA evolutionary approaches. The above algorithms are still useful to determine an upper bound in advance.

On the other hand, if you want to go faster then there exist parallel and distributed graph coloring algorithms.

# Tests

Using a data set of instances (see `instances/` folder) and the number of colors found by the naive algorithm as a measure of difficulty, we get the following results:

<svg class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg"><style>.poloto{stroke-linecap:round;stroke-linejoin:round;font-family:Roboto,sans-serif;font-size:16px;}.poloto_background{fill:AliceBlue;}.poloto_scatter{stroke-width:7}.poloto_tick_line{stroke:gray;stroke-width:0.5}.poloto_line{stroke-width:2}.poloto_text{fill: black;}.poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none}.poloto_title{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_xname{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_yname{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_legend_text{font-size:20px;dominant-baseline:middle;text-anchor:start;}.poloto0stroke{stroke:blue;}.poloto1stroke{stroke:red;}.poloto2stroke{stroke:green;}.poloto3stroke{stroke:gold;}.poloto4stroke{stroke:aqua;}.poloto5stroke{stroke:lime;}.poloto6stroke{stroke:orange;}.poloto7stroke{stroke:chocolate;}.poloto0fill{fill:blue;}.poloto1fill{fill:red;}.poloto2fill{fill:green;}.poloto3fill{fill:gold;}.poloto4fill{fill:aqua;}.poloto5fill{fill:lime;}.poloto6fill{fill:orange;}.poloto7fill{fill:chocolate;}</style><circle  r="1e5" class="poloto_background" /><text  class="poloto_text poloto_legend_text" x="675" y="100" >naive</text><line  class="poloto_line poloto_legend_icon poloto0stroke poloto0legend" stroke="black" x1="680" x2="730" y1="81.25" y2="81.25" /><path  class="poloto_line poloto0stroke" fill="none" stroke="black" d=" M 150.00 400.00 L 151.58 399.05 L 153.15 398.11 L 154.73 397.16 L 156.31 396.21 L 156.31 396.21 L 156.31 396.21 L 157.89 395.27 L 157.89 395.27 L 159.46 394.32 L 159.46 394.32 L 161.04 393.38 L 161.04 393.38 L 162.62 392.43 L 162.62 392.43 L 164.20 391.48 L 164.20 391.48 L 164.20 391.48 L 165.77 390.54 L 167.35 389.59 L 167.35 389.59 L 167.35 389.59 L 168.93 388.64 L 168.93 388.64 L 170.50 387.70 L 170.50 387.70 L 172.08 386.75 L 175.24 384.86 L 175.24 384.86 L 176.81 383.91 L 178.39 382.97 L 178.39 382.97 L 178.39 382.97 L 179.97 382.02 L 183.12 380.13 L 183.12 380.13 L 184.70 379.18 L 186.28 378.23 L 187.85 377.29 L 191.01 375.39 L 191.01 375.39 L 191.01 375.39 L 191.01 375.39 L 191.01 375.39 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 197.32 371.61 L 198.90 370.66 L 202.05 368.77 L 205.21 366.88 L 209.94 364.04 L 211.51 363.09 L 213.09 362.15 L 214.67 361.20 L 216.25 360.25 L 217.82 359.31 L 220.98 357.41 L 220.98 357.41 L 228.86 352.68 L 232.02 350.79 L 246.21 342.27 L 257.26 335.65 L 263.56 331.86 L 299.84 310.09 L 315.62 300.63 L 336.12 288.33 L 340.85 285.49 L 342.43 284.54 L 344.01 283.60 L 369.24 268.45 L 419.72 238.17 L 479.65 202.21 L 650.00 100.00" /><text  class="poloto_text poloto_legend_text" x="675" y="150" >by_degree</text><line  class="poloto_line poloto_legend_icon poloto1stroke poloto1legend" stroke="black" x1="680" x2="730" y1="131.25" y2="131.25" /><path  class="poloto_line poloto1stroke" fill="none" stroke="black" d=" M 150.00 400.00 L 151.58 399.05 L 153.15 398.11 L 154.73 397.16 L 156.31 397.16 L 156.31 397.16 L 156.31 396.21 L 157.89 396.21 L 157.89 395.27 L 159.46 392.43 L 159.46 394.32 L 161.04 393.38 L 161.04 395.27 L 162.62 393.38 L 162.62 393.38 L 164.20 391.48 L 164.20 392.43 L 164.20 393.38 L 165.77 393.38 L 167.35 389.59 L 167.35 391.48 L 167.35 391.48 L 168.93 387.70 L 168.93 389.59 L 170.50 392.43 L 170.50 387.70 L 172.08 390.54 L 175.24 385.80 L 175.24 386.75 L 176.81 382.02 L 178.39 386.75 L 178.39 386.75 L 178.39 384.86 L 179.97 380.13 L 183.12 380.13 L 183.12 378.23 L 184.70 382.02 L 186.28 380.13 L 187.85 379.18 L 191.01 375.39 L 191.01 375.39 L 191.01 379.18 L 191.01 375.39 L 191.01 375.39 L 192.59 374.45 L 192.59 379.18 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 376.34 L 197.32 373.50 L 198.90 375.39 L 202.05 376.34 L 205.21 371.61 L 209.94 373.50 L 211.51 364.98 L 213.09 363.09 L 214.67 361.20 L 216.25 361.20 L 217.82 362.15 L 220.98 357.41 L 220.98 357.41 L 228.86 352.68 L 232.02 353.63 L 246.21 342.27 L 257.26 336.59 L 263.56 334.70 L 299.84 315.77 L 315.62 309.15 L 336.12 287.38 L 340.85 289.27 L 342.43 287.38 L 344.01 289.27 L 369.24 276.97 L 419.72 243.85 L 479.65 202.21 L 650.00 107.57" /><text  class="poloto_text poloto_legend_text" x="675" y="200" >dsatur</text><line  class="poloto_line poloto_legend_icon poloto2stroke poloto2legend" stroke="black" x1="680" x2="730" y1="181.25" y2="181.25" /><path  class="poloto_line poloto2stroke" fill="none" stroke="black" d=" M 150.00 400.00 L 151.58 399.05 L 153.15 398.11 L 154.73 397.16 L 156.31 399.05 L 156.31 398.11 L 156.31 396.21 L 157.89 396.21 L 157.89 395.27 L 159.46 394.32 L 159.46 394.32 L 161.04 393.38 L 161.04 395.27 L 162.62 393.38 L 162.62 393.38 L 164.20 391.48 L 164.20 397.16 L 164.20 394.32 L 165.77 394.32 L 167.35 390.54 L 167.35 390.54 L 167.35 391.48 L 168.93 390.54 L 168.93 392.43 L 170.50 393.38 L 170.50 390.54 L 172.08 393.38 L 175.24 388.64 L 175.24 388.64 L 176.81 387.70 L 178.39 387.70 L 178.39 387.70 L 178.39 384.86 L 179.97 385.80 L 183.12 384.86 L 183.12 383.91 L 184.70 382.02 L 186.28 380.13 L 187.85 380.13 L 191.01 375.39 L 191.01 375.39 L 191.01 381.07 L 191.01 375.39 L 191.01 375.39 L 192.59 374.45 L 192.59 381.07 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 380.13 L 197.32 374.45 L 198.90 376.34 L 202.05 377.29 L 205.21 379.18 L 209.94 384.86 L 211.51 368.77 L 213.09 364.04 L 214.67 364.04 L 216.25 364.98 L 217.82 364.04 L 220.98 357.41 L 220.98 357.41 L 228.86 352.68 L 232.02 356.47 L 246.21 342.27 L 257.26 342.27 L 263.56 334.70 L 299.84 318.61 L 315.62 320.50 L 336.12 294.95 L 340.85 297.79 L 342.43 296.85 L 344.01 295.90 L 369.24 278.86 L 419.72 249.53 L 479.65 281.70 L 650.00 118.93" /><text  class="poloto_text poloto_legend_text" x="675" y="250" >rlf</text><line  class="poloto_line poloto_legend_icon poloto3stroke poloto3legend" stroke="black" x1="680" x2="730" y1="231.25" y2="231.25" /><path  class="poloto_line poloto3stroke" fill="none" stroke="black" d=" M 150.00 400.00 L 151.58 399.05 L 153.15 398.11 L 154.73 397.16 L 156.31 399.05 L 156.31 398.11 L 156.31 396.21 L 157.89 396.21 L 157.89 395.27 L 159.46 395.27 L 159.46 394.32 L 161.04 393.38 L 161.04 396.21 L 162.62 393.38 L 162.62 393.38 L 164.20 394.32 L 164.20 396.21 L 164.20 395.27 L 165.77 396.21 L 167.35 391.48 L 167.35 391.48 L 167.35 391.48 L 168.93 391.48 L 168.93 393.38 L 170.50 399.05 L 170.50 390.54 L 172.08 399.05 L 175.24 390.54 L 175.24 389.59 L 176.81 388.64 L 178.39 388.64 L 178.39 387.70 L 178.39 384.86 L 179.97 387.70 L 183.12 386.75 L 183.12 385.80 L 184.70 384.86 L 186.28 380.13 L 187.85 380.13 L 191.01 375.39 L 191.01 375.39 L 191.01 382.02 L 191.01 375.39 L 191.01 375.39 L 192.59 374.45 L 192.59 382.02 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 374.45 L 192.59 381.07 L 197.32 374.45 L 198.90 377.29 L 202.05 377.29 L 205.21 381.07 L 209.94 377.29 L 211.51 371.61 L 213.09 364.04 L 214.67 367.82 L 216.25 368.77 L 217.82 368.77 L 220.98 357.41 L 220.98 357.41 L 228.86 352.68 L 232.02 356.47 L 246.21 342.27 L 257.26 347.95 L 263.56 334.70 L 299.84 325.24 L 315.62 319.56 L 336.12 304.42 L 340.85 304.42 L 342.43 303.47 L 344.01 303.47 L 369.24 278.86 L 419.72 258.04 L 479.65 288.33 L 650.00 141.64" /><text  class="poloto_labels poloto_text poloto_title" x="400" y="37.5" >colors vs naive</text><text  class="poloto_labels poloto_text poloto_xname" x="400" y="481.25" >naive colors</text><text  class="poloto_labels poloto_text poloto_yname" transform="rotate(-90,37.5,250)" x="37.5" y="250" >colors</text><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="start" x="150" y="70" ></text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="356.4668769716088" y2="356.4668769716088" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="356.4668769716088" >50</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="309.1482649842271" y2="309.1482649842271" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="309.1482649842271" >100</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="261.8296529968454" y2="261.8296529968454" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="261.8296529968454" >150</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="214.51104100946372" y2="214.51104100946372" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="214.51104100946372" >200</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="167.19242902208202" y2="167.19242902208202" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="167.19242902208202" >250</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="119.87381703470032" y2="119.87381703470032" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="119.87381703470032" >300</text><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="start" x="440.00000000000006" y="70" ></text><line  class="poloto_axis_lines" stroke="black" x1="222.55520504731862" x2="222.55520504731862" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="222.55520504731862" y="430" >50</text><line  class="poloto_axis_lines" stroke="black" x1="301.4195583596214" x2="301.4195583596214" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="301.4195583596214" y="430" >100</text><line  class="poloto_axis_lines" stroke="black" x1="380.2839116719243" x2="380.2839116719243" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="380.2839116719243" y="430" >150</text><line  class="poloto_axis_lines" stroke="black" x1="459.1482649842271" x2="459.1482649842271" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="459.1482649842271" y="430" >200</text><line  class="poloto_axis_lines" stroke="black" x1="538.01261829653" x2="538.01261829653" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="538.01261829653" y="430" >250</text><line  class="poloto_axis_lines" stroke="black" x1="616.8769716088327" x2="616.8769716088327" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="616.8769716088327" y="430" >300</text><path  stroke="black" fill="none" class="poloto_axis_lines" style="stroke-dasharray:7.886435331230284;stroke-dashoffset:-72.55520504731861;" d=" M 150 400 L 650 400" /><path  stroke="black" fill="none" class="poloto_axis_lines" style="stroke-dasharray:4.73186119873817;stroke-dashoffset:-43.53312302839117;" d=" M 150 400 L 150 100" /></svg>

<svg class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg"><style>.poloto{stroke-linecap:round;stroke-linejoin:round;font-family:Roboto,sans-serif;font-size:16px;}.poloto_background{fill:AliceBlue;}.poloto_scatter{stroke-width:7}.poloto_tick_line{stroke:gray;stroke-width:0.5}.poloto_line{stroke-width:2}.poloto_text{fill: black;}.poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none}.poloto_title{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_xname{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_yname{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_legend_text{font-size:20px;dominant-baseline:middle;text-anchor:start;}.poloto0stroke{stroke:blue;}.poloto1stroke{stroke:red;}.poloto2stroke{stroke:green;}.poloto3stroke{stroke:gold;}.poloto4stroke{stroke:aqua;}.poloto5stroke{stroke:lime;}.poloto6stroke{stroke:orange;}.poloto7stroke{stroke:chocolate;}.poloto0fill{fill:blue;}.poloto1fill{fill:red;}.poloto2fill{fill:green;}.poloto3fill{fill:gold;}.poloto4fill{fill:aqua;}.poloto5fill{fill:lime;}.poloto6fill{fill:orange;}.poloto7fill{fill:chocolate;}</style><circle  r="1e5" class="poloto_background" /><text  class="poloto_text poloto_legend_text" x="675" y="100" >naive</text><line  class="poloto_line poloto_legend_icon poloto0stroke poloto0legend" stroke="black" x1="680" x2="730" y1="81.25" y2="81.25" /><path  class="poloto_line poloto0stroke" fill="none" stroke="black" d=" M 150.00 400.00 L 151.58 400.00 L 153.15 400.00 L 154.73 399.99 L 156.31 400.00 L 156.31 399.99 L 156.31 399.99 L 157.89 399.99 L 157.89 399.99 L 159.46 399.99 L 159.46 399.99 L 161.04 399.99 L 161.04 400.00 L 162.62 399.99 L 162.62 399.99 L 164.20 399.99 L 164.20 399.95 L 164.20 399.97 L 165.77 399.95 L 167.35 399.99 L 167.35 399.96 L 167.35 399.97 L 168.93 399.99 L 168.93 399.99 L 170.50 399.93 L 170.50 399.98 L 172.08 399.93 L 175.24 399.98 L 175.24 399.92 L 176.81 399.97 L 178.39 399.93 L 178.39 399.93 L 178.39 399.98 L 179.97 399.97 L 183.12 399.96 L 183.12 399.95 L 184.70 399.97 L 186.28 399.93 L 187.85 399.92 L 191.01 399.95 L 191.01 399.95 L 191.01 399.89 L 191.01 399.98 L 191.01 399.98 L 192.59 399.98 L 192.59 399.89 L 192.59 399.98 L 192.59 399.92 L 192.59 399.93 L 192.59 399.98 L 192.59 399.98 L 192.59 399.72 L 197.32 399.98 L 198.90 399.87 L 202.05 399.88 L 205.21 399.91 L 209.94 399.90 L 211.51 399.91 L 213.09 399.97 L 214.67 399.88 L 216.25 399.88 L 217.82 399.88 L 220.98 399.98 L 220.98 399.98 L 228.86 399.91 L 232.02 399.96 L 246.21 399.95 L 257.26 399.69 L 263.56 399.96 L 299.84 399.85 L 315.62 399.69 L 336.12 398.87 L 340.85 398.89 L 342.43 398.89 L 344.01 398.85 L 369.24 399.62 L 419.72 399.46 L 479.65 398.94 L 650.00 397.74" /><text  class="poloto_text poloto_legend_text" x="675" y="150" >by_degree</text><line  class="poloto_line poloto_legend_icon poloto1stroke poloto1legend" stroke="black" x1="680" x2="730" y1="131.25" y2="131.25" /><path  class="poloto_line poloto1stroke" fill="none" stroke="black" d=" M 150.00 400.00 L 151.58 400.00 L 153.15 400.00 L 154.73 399.99 L 156.31 400.00 L 156.31 399.99 L 156.31 399.98 L 157.89 399.99 L 157.89 399.98 L 159.46 399.99 L 159.46 399.99 L 161.04 399.99 L 161.04 399.99 L 162.62 399.98 L 162.62 399.99 L 164.20 399.99 L 164.20 399.92 L 164.20 399.96 L 165.77 399.92 L 167.35 399.98 L 167.35 399.93 L 167.35 399.94 L 168.93 399.98 L 168.93 399.98 L 170.50 399.90 L 170.50 399.97 L 172.08 399.90 L 175.24 399.96 L 175.24 399.87 L 176.81 399.96 L 178.39 399.90 L 178.39 399.90 L 178.39 399.97 L 179.97 399.95 L 183.12 399.94 L 183.12 399.93 L 184.70 399.96 L 186.28 399.89 L 187.85 399.89 L 191.01 399.91 L 191.01 399.93 L 191.01 399.85 L 191.01 399.97 L 191.01 399.97 L 192.59 399.97 L 192.59 399.84 L 192.59 399.97 L 192.59 399.90 L 192.59 399.90 L 192.59 399.97 L 192.59 399.97 L 192.59 399.58 L 197.32 399.96 L 198.90 399.83 L 202.05 399.84 L 205.21 399.88 L 209.94 399.86 L 211.51 399.88 L 213.09 399.95 L 214.67 399.83 L 216.25 399.84 L 217.82 399.84 L 220.98 399.97 L 220.98 399.97 L 228.86 399.88 L 232.02 399.95 L 246.21 399.93 L 257.26 399.55 L 263.56 399.93 L 299.84 399.82 L 315.62 399.52 L 336.12 398.35 L 340.85 398.38 L 342.43 398.43 L 344.01 398.35 L 369.24 399.59 L 419.72 399.33 L 479.65 399.07 L 650.00 397.28" /><text  class="poloto_text poloto_legend_text" x="675" y="200" >dsatur</text><line  class="poloto_line poloto_legend_icon poloto2stroke poloto2legend" stroke="black" x1="680" x2="730" y1="181.25" y2="181.25" /><path  class="poloto_line poloto2stroke" fill="none" stroke="black" d=" M 150.00 400.00 L 151.58 399.99 L 153.15 399.96 L 154.73 399.89 L 156.31 399.96 L 156.31 399.85 L 156.31 399.66 L 157.89 399.87 L 157.89 399.82 L 159.46 399.88 L 159.46 399.93 L 161.04 399.92 L 161.04 399.93 L 162.62 399.84 L 162.62 399.88 L 164.20 399.81 L 164.20 399.01 L 164.20 399.46 L 165.77 399.02 L 167.35 399.64 L 167.35 399.42 L 167.35 399.42 L 168.93 399.61 L 168.93 399.73 L 170.50 398.32 L 170.50 399.48 L 172.08 398.42 L 175.24 399.31 L 175.24 397.98 L 176.81 399.09 L 178.39 398.69 L 178.39 398.66 L 178.39 399.71 L 179.97 398.88 L 183.12 398.58 L 183.12 398.22 L 184.70 399.45 L 186.28 398.71 L 187.85 398.69 L 191.01 398.63 L 191.01 398.77 L 191.01 397.43 L 191.01 399.54 L 191.01 399.55 L 192.59 399.50 L 192.59 397.44 L 192.59 399.49 L 192.59 397.97 L 192.59 398.00 L 192.59 399.50 L 192.59 399.50 L 192.59 391.43 L 197.32 399.48 L 198.90 397.34 L 202.05 397.39 L 205.21 397.95 L 209.94 397.38 L 211.51 397.77 L 213.09 399.23 L 214.67 396.77 L 216.25 396.84 L 217.82 396.85 L 220.98 399.50 L 220.98 399.51 L 228.86 397.41 L 232.02 399.09 L 246.21 398.44 L 257.26 390.88 L 263.56 398.74 L 299.84 396.12 L 315.62 386.77 L 336.12 361.87 L 340.85 362.39 L 342.43 362.16 L 344.01 361.72 L 369.24 391.97 L 419.72 384.30 L 479.65 364.47 L 650.00 334.37" /><text  class="poloto_text poloto_legend_text" x="675" y="250" >rlf</text><line  class="poloto_line poloto_legend_icon poloto3stroke poloto3legend" stroke="black" x1="680" x2="730" y1="231.25" y2="231.25" /><path  class="poloto_line poloto3stroke" fill="none" stroke="black" d=" M 150.00 400.00 L 151.58 400.00 L 153.15 399.99 L 154.73 399.96 L 156.31 399.99 L 156.31 399.92 L 156.31 399.87 L 157.89 399.92 L 157.89 399.91 L 159.46 399.96 L 159.46 399.96 L 161.04 399.96 L 161.04 399.98 L 162.62 399.91 L 162.62 399.95 L 164.20 399.93 L 164.20 399.23 L 164.20 399.69 L 165.77 399.24 L 167.35 399.85 L 167.35 398.99 L 167.35 398.95 L 168.93 399.84 L 168.93 399.90 L 170.50 399.25 L 170.50 399.77 L 172.08 399.25 L 175.24 399.68 L 175.24 398.59 L 176.81 399.55 L 178.39 398.88 L 178.39 398.88 L 178.39 399.84 L 179.97 399.41 L 183.12 399.22 L 183.12 399.01 L 184.70 399.70 L 186.28 398.71 L 187.85 398.72 L 191.01 398.86 L 191.01 398.94 L 191.01 398.06 L 191.01 399.66 L 191.01 399.66 L 192.59 399.66 L 192.59 398.08 L 192.59 399.67 L 192.59 398.00 L 192.59 398.08 L 192.59 399.66 L 192.59 399.66 L 192.59 393.08 L 197.32 399.66 L 198.90 397.77 L 202.05 397.76 L 205.21 398.71 L 209.94 398.33 L 211.51 398.22 L 213.09 399.37 L 214.67 397.26 L 216.25 397.24 L 217.82 397.37 L 220.98 399.58 L 220.98 399.49 L 228.86 396.60 L 232.02 399.15 L 246.21 398.06 L 257.26 388.44 L 263.56 398.37 L 299.84 394.80 L 315.62 380.48 L 336.12 320.37 L 340.85 319.83 L 342.43 320.23 L 344.01 317.92 L 369.24 382.18 L 419.72 363.22 L 479.65 332.40 L 650.00 100.00" /><text  class="poloto_labels poloto_text poloto_title" x="400" y="37.5" >time vs naive</text><text  class="poloto_labels poloto_text poloto_xname" x="400" y="481.25" >naive colors</text><text  class="poloto_labels poloto_text poloto_yname" transform="rotate(-90,37.5,250)" x="37.5" y="250" >time (s)</text><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="start" x="150" y="70" ></text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="360.4523453069894" y2="360.4523453069894" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="360.4523453069894" >0.05</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="320.90410767295606" y2="320.90410767295606" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="320.90410767295606" >0.10</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="281.3558700389227" y2="281.3558700389227" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="281.3558700389227" >0.15</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="241.80763240488943" y2="241.80763240488943" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="241.80763240488943" >0.20</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="202.2593947708561" y2="202.2593947708561" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="202.2593947708561" >0.25</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="162.71115713682275" y2="162.71115713682275" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="162.71115713682275" >0.30</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="123.16291950278946" y2="123.16291950278946" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="123.16291950278946" >0.35</text><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="start" x="440.00000000000006" y="70" ></text><line  class="poloto_axis_lines" stroke="black" x1="222.55520504731862" x2="222.55520504731862" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="222.55520504731862" y="430" >50</text><line  class="poloto_axis_lines" stroke="black" x1="301.4195583596214" x2="301.4195583596214" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="301.4195583596214" y="430" >100</text><line  class="poloto_axis_lines" stroke="black" x1="380.2839116719243" x2="380.2839116719243" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="380.2839116719243" y="430" >150</text><line  class="poloto_axis_lines" stroke="black" x1="459.1482649842271" x2="459.1482649842271" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="459.1482649842271" y="430" >200</text><line  class="poloto_axis_lines" stroke="black" x1="538.01261829653" x2="538.01261829653" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="538.01261829653" y="430" >250</text><line  class="poloto_axis_lines" stroke="black" x1="616.8769716088327" x2="616.8769716088327" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="616.8769716088327" y="430" >300</text><path  stroke="black" fill="none" class="poloto_axis_lines" style="stroke-dasharray:7.886435331230284;stroke-dashoffset:-72.55520504731861;" d=" M 150 400 L 650 400" /><path  stroke="black" fill="none" class="poloto_axis_lines" style="stroke-dasharray:3.9548237634033327;stroke-dashoffset:-39.5476546930106;" d=" M 150 400 L 150 100.00000000000006" /></svg>

See the full results in `data/instances.tsv`.
