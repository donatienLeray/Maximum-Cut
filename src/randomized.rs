use rand::Rng;

// run the local search algorithm return the resulting partition
pub fn maximum_cut(graph: &Vec<Vec<usize>>) -> Vec<usize> {

    let mut rng = rand::thread_rng(); // initialized thread-local random number generator
    let mut partition: Vec<usize> = (0..graph.len()).map(|_| rng.gen_range(0..2)).collect(); // initialize a random partition e.g. if graph.len()=5 [1,0,1,1,0]
    let mut cut = init_cut(graph, &partition); // calculate the cut size

    // look n time at a random node and flip it if it improoves the cut
    for _ in 0..partition.len(){

        let node= rng.gen_range(0..partition.len()); // choose random node
        let new_cut=cut_size(graph, &partition, cut as i32, node); // calculate cut if node change partition
        
        // make the flip if it makes an improovement
        if  new_cut  > cut{
            partition[node] = 1- partition[node];
            cut= new_cut;
            print!("randomized: {} \r",cut); // print current found cut to console
        }
    }

    print!("randomized: {} ",cut);
    partition // return the found partition
}

// calculate the cut size for a given partition
// same funktion as in local_search
fn init_cut(graph: &Vec<Vec<usize>>, partition: &Vec<usize>) -> usize {

    let mut cut = 0; // initalise cut size with 0

    // itterate though the adj_matrix
    for i in 0..partition.len() {
        for j in (i+1)..partition.len() {
            // if an edge is found that is not in the partition add one to the cut size
            if graph[i][j] == 1 && partition[i] != partition[j] {
                cut += 1;
            }
        }
    }

    cut // return the resulting cut size 
}

// calculate the cut size for a given partition if node i swapped partitions
// same funktion as in local_search
fn cut_size(graph: &Vec<Vec<usize>>, partition: &Vec<usize>,cut: i32,node: usize) -> usize {

    let mut same_p=0; // count of neighbors in the same partition
    let mut other_p=0; // count of neighbors in the other partition 

    // itterate though each node of the partition
    for i in 0..partition.len(){
        // if node is neighbour and in the other partition increase the other_p counter  
        if graph[node][i]==1 && partition[node] != partition[i]{
            other_p += 1;
        }
        // else if node is neighbour increase the same_p counter
        else if graph[node][i]==1{ 
            same_p += 1;
        }
    }

    // calculate the cut that will result if node is flipped (init_cut +(neighbors in the same partition - neighbors in the other partition))
    let result =cut + (same_p - other_p);
    result as usize // return the result
}

#[cfg(test)]
mod test{
    use crate:: randomized::{init_cut, cut_size};


    // test init_cut on empty partition
    #[test]
    fn test_init_cut_empty(){
        let graph: Vec<Vec<usize>> = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let partition = vec![0,0,0,0,0];
        assert_eq!(init_cut(&graph, &partition),0);
    }

    #[test]
    fn test_init_cut_full(){
        let graph: Vec<Vec<usize>> = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let partition = vec![1,1,1,1,1];
        assert_eq!(init_cut(&graph, &partition),0);
    }

    #[test]
    fn test_init_cut_max(){
        let graph: Vec<Vec<usize>> = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let partition = vec![0,1,0,1,1];
        assert_eq!(init_cut(&graph, &partition),5);
    }

    #[test]
    fn test_cut_size_max(){
        let graph: Vec<Vec<usize>> = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let partition = vec![0,1,0,1,1];
        for i in 0..partition.len(){
            assert!(cut_size(&graph, &partition,5,i)<= 5);
        } 
    }

    #[test]
    fn test_cut_size_min(){
        let graph: Vec<Vec<usize>> = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let partition = vec![1,0,0,1,0];
        for i in 0..partition.len(){
            assert!(cut_size(&graph, &partition,2,i)>=1);
        } 
    }

    #[test]
    fn test_randomized_not_zero(){
        let graph: Vec<Vec<usize>> = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let result_1:Vec<usize> = vec![0,0,0,0,0];
        let result_2:Vec<usize> = vec![1,1,1,1,1];
        assert_ne!(super::maximum_cut(&graph),result_1);
        assert_ne!(super::maximum_cut(&graph),result_2);  
    }
}