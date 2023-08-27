use std::{path::Path, env};

mod reader;
mod randomized;
mod local_search;
mod goemans_williamson;
mod ilp_formulation;


fn main() {

    // get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // check if there are enough arguments
    if args.len() != 3 {
        // Display an error message and exit the program
        eprintln!("please use two argumetns: cargo run -- <falg> <input file>");
        std::process::exit(1);
    }

    // make first argument to flag and second to file
    let flag = &args[1];
    let file = Path::new(&args[2]);

    // read file and get adj matrix
    print!("reading File ");
    let graph = reader::get_adj_matrix(file).unwrap();
    println!("✔"); // finished reading

    // initalise resulting partition
    let mut result: Vec<usize> = Vec::new();

    // run programm algorithm corresponding to the flag
    match flag.as_str() {
        "-r" => {
            // Heuristic (randomized)
            result = randomized::maximum_cut(&graph);
        },
        "-l" => {
            // Aproximation (local search)
            result = local_search::maximum_cut(&graph);
        },
        "-g" => {
            // Aproximation (goemans-williamson)
            println!("goemans-williamson:");
            result = goemans_williamson::maximum_cut(&args[2]);
            print!("                 ↳  {} ",local_search::init_cut(&graph, &result));
        },
        "-i" => {
            // Exact (ILP formulation)
            result = ilp_formulation::maximum_cut(&graph);
            print!("olp formulation: {}",local_search::init_cut(&graph, &result));
        },
        _ => {
            // display an error message and exit the program
            eprintln!("Invalid flag: {} \nFlag must be:\n -h (heuristic)\n -a (approximation)\n -e (exact)", flag);
            std::process::exit(1);
        },
    }
    println!("result:\n{:?}",result);
    if result.len()>0{println!("✔")} // double check if result was found and signal that finished
}


#[cfg(test)]
mod test{

    use assert_cli::Assert;

    // test if all flags works
    #[test]
    fn test_main_all_flags() {
        let flags = ["-r","-l","-g","-i"];
        for i in flags{
            Assert::command(&["cargo", "run", "--", i , "./test_graphs/test.txt"])
            .succeeds()
            .unwrap()
        }
    }

    // test if wrong flag fails
    #[test]
    fn test_main_wrong_flag() {
        Assert::command(&["cargo", "run", "--", "-w", "./test_graphs/test2.txt"])
            .fails()
            .unwrap()
    }

    // test if to many arguments fails
    #[test]
    fn test_main_to_many_arg() {
        Assert::command(&["cargo", "run", "--", "-r", "-l", "./test_graphs/test.txt"])
            .fails()
            .unwrap()
    }

    // test if wrong file fails
    #[test]
    fn test_main_not_a_file() {
        Assert::command(&["cargo", "run", "--", "-r", "./test_graphs/test0.txt"])
            .fails()
            .unwrap()
    }

}