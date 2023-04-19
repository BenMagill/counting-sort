// TODO: allow any unsigned integer types

fn find_max(inp: &Vec<u32>) -> u32 {
    let mut max = inp.first().expect("Vector must have elements");

    for value in inp.iter().skip(1) {
        if value > max {
            max = value;
        }
    }
    *max
}

fn counting_sort(input: &Vec<u32>) -> Vec<u32> {

    let mut totals = vec![0; find_max(input) as usize + 1];
    
    let mut output = vec![0; input.len()];
    
    // Count occurences of each value
    for num in input {
        let v = totals.get_mut(*num as usize)
            .unwrap();
        
        *v = *v + 1;
    }
    
    // Convert counts into the position the first of each item should go in the output array
    for i in 0..totals.len() {
        if i == 0 {
            continue;
        }
    
        let values = totals.get_mut(i-1..=i).unwrap(); 
        values[1] = values[0] + values[1];
    }
    
    // in reverse order add each item to correct position
    for i in input.iter().rev() {
        let value_total = totals.get_mut(*i as usize).unwrap();
        let output_pos = output.get_mut(*value_total - 1).unwrap();
    
        *output_pos = *i;
    
        // Next time item will go in slot below
        *value_total = *value_total -1;
    }
    
    output
}

fn main() {
    let input = vec![0, 1, 2, 5, 2, 5, 7, 1, 3, 5];
    
    let output = counting_sort(&input);
    
    for i in &output {
        println!("{}", i);
    }
}
