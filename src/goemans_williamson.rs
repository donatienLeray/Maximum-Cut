use std::process::Command;

pub fn maximum_cut(file: &String)-> Vec<usize> {
    
    // executes the Python script and capture its output
    // ($ python3 src/Goemans-Williamson.py -f <file> )
    let output = Command::new("python3")
        .arg("src/Goemans-Williamson.py")
        .arg("-f")
        .arg(file)
        .output()
        .expect("Failed to execute command");

    // check if the Python script ran successfully
    if !output.status.success() {
        eprintln!("Command failed: {:?}", output);
        std::process::exit(1);
    }

    // convert the output bytes to a string
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    // delete unnesasarry chars
    let stripped = output_str.trim_matches(|c| c == '[' || c == ']' || c == '\n');
    // make it iterable
    let items = stripped.split(", ");
    //make it to an usize Vector
    let partition = items.map(|x| x.parse::<usize>().unwrap()).collect();

    partition // return the found partition
}

#[cfg(test)]
mod test{

    #[test]
    fn test_stuff(){
        assert_eq!(45,45);
    }

}