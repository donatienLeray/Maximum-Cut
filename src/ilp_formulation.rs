use good_lp::*;

// run the local search algorithm return the resulting partition
pub fn maximum_cut(graph: &Vec<Vec<usize>>) -> Vec<usize> {

    // create an new Vec to hold the edges
    let mut edges:Vec<(usize,usize)> = Vec::new();
    //for each unique edge in the graph make a toupple holding it nodes 
    for i in 0..graph.len()-1{
        for j in i+1..graph.len(){
            if graph[i][j]==1{
                edges.push((i,j));
            }
        }
    }

    // create Vec for the variables
    let mut problem = ProblemVariables::new();
    // create binary variables for every edge (edge_vars_(i,j)∈[0,1] for each edge (i,j)∈edges)
    let edge_vars: Vec<Variable> = problem.add_vector(variable().binary(), edges.len());
    // create binary variables for every node (edge_vars_(v)∈[0,1] for each vertex v∈V.)
    let node_vars: Vec<Variable> = problem.add_vector(variable().binary(), graph.len());

    // define the objective (   ∑     y_(i,j) )
    //                      ((i,j)∈E         )
    let objective: Expression = edge_vars.iter().sum();
   

    // create the problem and maxmise the objective
    let mut ccp = problem.maximise(objective).using(default_solver);

    // add two constrains that logicaly represent (∀(i,j)∈E (xi ∨ xj) ∧ ¬(xi ∧ xj))
    for n in 0..edge_vars.len(){
        let i = edges[n].0;
        let j = edges[n].1;
        // define constrain (edge_vars_(i,j)≤ edge_vars_(i) + edge_vars_(j)    ∀(i,j)∈E)
        ccp.add_constraint(constraint!(edge_vars[n].into_expression() <= node_vars[i].into_expression() + node_vars[j].into_expression() ));
        // define constrain (edge_vars_(i,j)≤ 2 - edge_vars_(i) - edge_vars_(j)    ∀(i,j)∈E)
        ccp.add_constraint(constraint!(edge_vars[n].into_expression() <= 2 - node_vars[i].into_expression() - node_vars[j].into_expression() ));
   }

    // run the solver on the problem and get the solution
    let solution = ccp.solve().unwrap();

   // make the resulting partition to a usize Vec
    let partition: Vec<usize> = node_vars.iter().map(|i| solution.value(*i) as usize).collect();

    partition

} 
