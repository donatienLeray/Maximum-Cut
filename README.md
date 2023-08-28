> [!NOTE]  
> The Implementation is mostly done\
> but the experiments and there evaluation is not!

# ExamLeray - Maximum Cut

## Dependencies

**Cargo**\
Since this is a cargo build, cargo will be needed to compile and run the code properly. How to install it is described [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

**Good_lp**\
The [good_lp](https://crates.io/crates/good_lp) crate allows us to utilize the mixed integer linear programming solver, [Coin-or Cbc](https://www.coin-or.org/Cbc/), within Rust.

In order to use it, Coin-or Cbc **must be installed** as outlined in its [README](https://github.com/coin-or/Cbc/blob/master/README.md).\
**note:** a working C compiler is also required for Coin-or Cbc to function properly.

**hyperfine**\
hyperfine is a command-line benchmarking tool used in the scripts for the experiments.
It is easily installable per command line as described in its projects [README](https://github.com/sharkdp/hyperfine) under *Installation*.


## Compiling
before running the code you need to compile it with the following command:
```bash
cd <Path_to_this_folder>
cargo build --release
```

## Running 
to run an algorithm use the following command:
```bash
./target/release/main <flag> <input file>
```
e.g.
```bash
./target/release/main -l /home/user/mac/rudy/g05_60.0
```

### flags
You must use one of the four possible flags to specify which algorithm to run:\
**-r** (Randomized) &emsp;&emsp;&emsp;&emsp;&emsp;-> heuristic\
**-l** (Local search)&emsp; &emsp;&emsp;&emsp; &emsp;-> approximation\
**-g** (Goemans-Williamson) &ensp; -> approximation\
**-i** (ILP formulation) &emsp; &emsp; &emsp; -> exact

### Input

As Input Graphs, it is recommended to use Graphs from the [pace challage 2019](https://pacechallenge.org/2019/vc/vc_exact/).\
You can download them by clicking [here](https://pacechallenge.org/files/pace2019-vc-exact-public-v2.tar.bz2) [tar.bz2-file] (10.2 MiB).

For smaller graphs with a Max-Cut result [documentation](https://biqmac.aau.at/biqmaclib.pdf) it's recommended to use Graphs from the [Biq Mac Library](https://biqmac.aau.at/biqmaclib.html).\
You can download them by clicking [here](https://biqmac.aau.at/library/tar_files/biqmac_all.tar.gz) [tar.gz-file] (9.6 MB).

If you want to use Graphs from other sources make sure they are structured as follow: 
```
xy 50   // the first line all Strings get ignored the first integer here gives the number of nodes n.  
1 2     // for each edge give the id of two nodes connected by it  
1 37    // all node ids must be in the range from 1 to n
1 8
2 2     // self connections get ignored since they don't influence the cut
2 48
2 40 
2 47 -5 // everything after the first two integers in a line gets ignored
2 46 15 
3 29 18
3 9 
// ...
47 49
48 50  
        // file should end with a newline
```
## Module test
to run all module use the following command:
```bash
cargo test
```

to run only the test for one specific algorithm run
```bash
cargo test <name corresspomding .rs>
```
e.g. run all the test for the local search algorithm
```bash
cargo test local_search
```

# Experiments
The experiments utilized graphs from the [Biq Mac Library](./README.md/#Input) Library, specifically those from the mac/rudy/ folder. Their optimal solution can be obtained from the library's [documentation](https://biqmac.aau.at/biqmaclib.pdf). 
