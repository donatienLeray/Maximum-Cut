use rand::Rng;

// run the local search algorithm return the resulting partition
pub fn maximum_cut(graph: &Vec<Vec<usize>>) -> Vec<usize> {

    let mut rng = rand::thread_rng(); // initialized thread-local random number generator
    let mut partition: Vec<usize> = (0..graph.len()).map(|_| rng.gen_range(0..2)).collect(); // initialize a random partition e.g. if graph.len()=5 [1,0,1,1,0]
    let mut cut = init_cut(graph, &partition); // calculate the cut size

    // run local search as long as an improved cut is found
    while improve_cut(graph, &mut partition, &mut cut){
        print!("local_search: {} \r",cut); // print current found cut to console
    } 

    print!("local_search: {} ",cut);
    partition // return the found partition

}

// flips the node in a partition wich leeds to the biggest cut improvement returns false if no such found
fn improve_cut(graph: &Vec<Vec<usize>>, partition: &mut Vec<usize>, cut: &mut usize)->bool {

    let mut best_cut = *cut; // best cutsize found. initalised with current cut size
    let mut best_node =0; // node leading to biggest improvement. initalised with 0
    let mut improved = false; // flag to signal if the cut was improved. initalised with false
    
    // iterrate other all nodes of the partition
    for i in 0..partition.len(){

        let new_cut = cut_size(graph,&partition,*cut as i32,i);//calculate cut if node i swapped partition

        // if new cut is an improvemnt update best_cut and best_node values and set improved to true
        if new_cut > best_cut{
            best_cut = new_cut;
            best_node = i;
            improved = true;
        }

    }

    // if an improvment was found flip the node that resulted in the biggest improvement and set cut to new found one
    if improved {
        partition[best_node] = 1 - partition[best_node];
        *cut = best_cut;
    }

    improved // return if improvment was found or not

}

// calculate the cut size for a given partition
// same funktion as in randomized
pub fn init_cut(graph: &Vec<Vec<usize>>, partition: &Vec<usize>) -> usize {

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
// same funktion as in randomized
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
    use crate::local_search::improve_cut;



    // test init_cut on empty partition
    #[test]
    fn test_improve_cut_best(){
        let graph: Vec<Vec<usize>> = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let mut partition: Vec<usize> = vec![0,1,0,1,1];
        let mut cut: usize = 5;
        assert!(!improve_cut(&graph, &mut partition,&mut cut));
    }

    #[test]
    fn test_improve_cut_worst(){
        let graph: Vec<Vec<usize>> = vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0],
        ];
        let mut partition: Vec<usize> = vec![0,0,0,0,0];
        let mut cut: usize = 5;
        assert!(improve_cut(&graph, &mut partition,&mut cut));
    }


}