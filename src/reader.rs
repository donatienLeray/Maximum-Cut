use std::{io::{BufReader, BufRead}, fs::File, path::Path};


pub fn get_adj_matrix(path: &Path)-> Result<Vec<Vec<usize>>, String > {

    // It opens the file at the given path and creates a new BufReader to read the file.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut n=0;// initalise n (number of edges) with 0
    
    //take the first line of the file
    if let Some(Ok(first_line)) = reader.lines().next() {

        //splits the line by spaces and collects the resulting substrings into a vector.
        let splitted: Vec<&str> = first_line.split(" ").collect();

        // set n to first valid integer found
        for i in splitted{
            match i.parse::<usize>() {
                Ok(pased_n) => {
                    n= pased_n;
                    break;
                },
                Err(_) => continue,
            }
            
        }

        // return error if no valid integer was found or n = 0
        if n == 0 {
            return Err("No nodes declared".to_string())
        }
    
    }else{
        // returns error if first line is not readable
        return Err("Wrong file format".to_string()); 
    }

    let mut matrix = vec![vec![0; n]; n]; // initalise n*n matrix filled with 0

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // iterates over the lines of the file (skipping the first one)
    for line in reader.lines().skip(1) {

        let line = line.unwrap();
        // split the line by spaces and collects the resulting substrings into a vector.
        let splitted: Vec<&str> = line.split(" ").collect();
        let (node_a,node_b);

        // check if the first element in the line is a valid integer, if not go to next line
        match splitted[0].parse::<usize>(){
            Ok(parsable_a) => node_a = parsable_a -1,
            Err(_) => continue,
        }

        // check if the second element in the line is a valid integer, if not go to next line
        match splitted[1].parse::<usize>(){
            Ok(parsable_b) => node_b = parsable_b -1,
            Err(_) => continue,
        }

        // if the nodes are the same (edge with itself) go to next line
        if node_a == node_b{
            continue;
        }

        // if both nodes are in the range of n: set the corresponding elements in the adj_matrix to 1
        if node_a < n && node_b <n {
            matrix[node_a][node_b] = 1;
            matrix[node_b][node_a] = 1;
        }
        else{
            // returns error that node index is out of range 
            return Err("Node index bigger then n".to_string());
        }
    }

    Ok(matrix) // if no errow occured return the adj_matrix

}



#[cfg(test)]
mod test{
    use std::path::Path;

    use crate::reader::get_adj_matrix;

    // test get_adj_matrix() on correctly formated file
    #[test]
    fn test_reader_1(){
        let result=[
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [0, 1, 0, 1, 1],
            [1, 0, 1, 0, 0],
            [0, 1, 1, 0, 0]
        ];
        let path= Path::new("./test_graphs/test.txt");
        assert_eq!(get_adj_matrix(&path).unwrap(), result);
    }

    // test get_adj_matrix() on correctly formated file
    #[test]
    fn test_reader_2(){
        let result=[
            [0, 1, 1, 0, 1, 0, 0], 
            [1, 0, 0, 1, 1, 0, 1], 
            [1, 0, 0, 1, 0, 1, 0], 
            [0, 1, 1, 0, 1, 1, 1], 
            [1, 1, 0, 1, 0, 1, 0], 
            [0, 0, 1, 1, 1, 0, 0], 
            [0, 1, 0, 1, 0, 0, 0]
        ];
        let path= Path::new("./test_graphs/test2.txt");
        assert_eq!(get_adj_matrix(&path).unwrap(), result);
    }

    // test get_adj_matrix() on wrong formated file
    #[test]
    #[should_panic]
    fn test_reader_3(){
        let path= Path::new("./test_graphs/test3.txt");
        assert!(get_adj_matrix(&path).unwrap().len() > 0);
    }

    // test get_adj_matrix() on wrong formated file
    #[test]
    #[should_panic]
    fn test_reader_4(){
        let path= Path::new("./test_graphs/test4.txt");
        assert!(get_adj_matrix(&path).unwrap().len() > 0);
    }

    // test get_adj_matrix() on wrong formated file
    #[test]
    #[should_panic]
    fn test_reader_5(){
        let path= Path::new("./test_graphs/test0.txt");
        assert!(get_adj_matrix(&path).unwrap().len() > 0);
    }

}

