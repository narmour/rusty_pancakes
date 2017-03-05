use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone)]
struct State{
	pan_cakes: Vec<i32>,
	prev_state: Option<Box<State>>,
	// f(n)
	cost: i32,
	// f(n) + g(n)
	heuristic: i32
}



impl Ord for State{
	fn cmp(&self,other: &State) -> Ordering{
		self.heuristic.cmp(&other.heuristic)
	}
}

impl PartialOrd for State{
	fn partial_cmp(&self,other: &State) -> Option<Ordering>{
		Some(self.cmp(other))
	}
}

impl PartialEq for State{
	fn eq(&self,other: &State) -> bool{
		self.heuristic == other.heuristic
	}
}

impl Eq for State{}

fn num_breakpoints(list: &Vec<i32>) -> i32{
	let mut bp: i32 = 0;

	let mut i =0;
	while (i+1) < list.len(){
		if(list[i] - list[i+1]).abs() >1{
			bp+=1;
		}
	i+=1;

	}
	println!("num breakpoints : {}",bp);

	return bp;
}

fn flip(list:&Vec<i32>,i:usize,j:usize)->Vec<i32>{
	let mut ret: Vec<i32> = Vec::new(); 
	let mut x: usize =0;
	// copy vals before i
	while x <i{
		ret.push(list[x]);
		x+=1;
	}
	// copy list[i] - list[j] in reverse
	x = j;
	while x >=i{
		ret.push(list[x]);
        if x ==0{
            break;
        }
		x-=1;
	}
	//copy rest of list
	x = j+1;
	while x < list.len(){
		ret.push(list[x]);
		x+=1;
	}
	
	for p in &ret{
		print!("{} ",p);
	}
	println!(" ");


	return ret;
		

}

fn a_star(list: &Vec<i32>) {
    // create priority queue
	let mut p_queue: BinaryHeap<State> = BinaryHeap::new();
    // copy vector passed in
    let mut start_list = list.to_vec();
    // push start into pqueue
    let start = State{pan_cakes: start_list,
                      prev_state: None,
                      cost: 0,
                      heuristic: num_breakpoints(&list)};
    p_queue.push(start);

    
    while p_queue.len() > 0{
        let mut current = p_queue.pop().unwrap();

        if num_breakpoints(&current.pan_cakes) == 0{
            break;
        }

        // push all children into open
        let mut i =0;
        let mut j =1;
        while i < current.pan_cakes.len() -1{
            while j <=current.pan_cakes.len(){
                let mut l = flip(&current.pan_cakes,i,j);
                let mut c = current.cost;
                let mut child = State{pan_cakes: l.clone(),
                                      prev_state: Some(Box::new(current.clone())),
                                      cost:c+1,
                                      heuristic: (c+1) + num_breakpoints(&l)/2};
                p_queue.push(child);
                j+=1;
            }
            i+=1;
            j = i+1;

        }


    }
}


fn main(){
    //vec to hold input
    let mut pan_cakes: Vec<i32> = Vec::new();

    // read input file into vector
	let mut input_string = String::new();
	let mut f = File::open("input.txt").expect("cant open");
	f.read_to_string(&mut input_string);
	for c in input_string.trim().chars(){
		match c.to_digit(10){
			Some(c) => {
				print!("{}",c);
				pan_cakes.push(c as i32);
			},
			None =>println!("tried to cast {}",c) ,
		}
			 
	}
	println!(" ");
	num_breakpoints(&pan_cakes);
	a_star(&pan_cakes);
	

	


}
