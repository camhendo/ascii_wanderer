use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, WeightedIndex};
use yew_utils::vdom::*;
use yew::prelude::*;
use yew_utils::components::table::Table;


pub struct Arena {
    pub arena: Vec<String>,
}

impl Arena{
    pub fn new(fx: u32, fy: u32) -> Vec<String> {
        
        let symbols = [".  ", "Q", "V", "B", "$", "*", "X", "&", "%", "@", "C"];
        let weights = [950,5,5,5,5,5,5,5,1,5,5];
        
        let mut rng = thread_rng();
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut pa: Vec<String> = vec![];

        for _x in 0..fx {
            for _y in 0..fy {
                pa.push(String::from(symbols[dist.sample(&mut rng)]))
            }
        }
        
        pa.clone()
    }
}
