use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    
    let mut paper = 0;
    let mut ribbon = 0;
    
    for line in input.lines() {
        // Parse and sort dimensions: [a, b, c] where a <= b <= c
        let mut d: Vec<u32> = line.split('x').map(|s| s.parse().unwrap()).collect();
        d.sort();
        let [a, b, c] = [d[0], d[1], d[2]];
        
        // Part 1: surface area + smallest side
        paper += 2 * (a*b + b*c + a*c) + a*b;
        
        // Part 2: perimeter of two smallest + volume
        ribbon += 2 * (a + b) + a*b*c;
    }
    
    println!("Part 1: {}", paper);
    println!("Part 2: {}", ribbon);
    
    Ok(())
}

// use std::fs;

// /// Calculates the paper area needed for a single gift
// fn calculate_gift_area(line: &str) -> u32 {
//     // Parse the dimensions
//     let dimensions: Vec<u32> = line
//         .split('x')
//         .map(|s| s.parse::<u32>().expect("Invalid dimension"))
//         .collect();
    
//     let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
    
//     // Side areas
//     let side1 = l * w;
//     let side2 = w * h;
//     let side3 = h * l;
    
//     // Total surface area
//     let surface_area = 2 * side1 + 2 * side2 + 2 * side3;
    
//     // Smallest side (for overlap)
//     let smallest_side = side1.min(side2).min(side3);
    
//     surface_area + smallest_side
// }

// /// Calculates the ribbon needed for a single gift
// fn calculate_gift_ribbon(line: &str) -> u32 {
//     let dimensions: Vec<u32> = line
//         .split('x')
//         .map(|s| s.parse::<u32>().expect("Invalid dimension"))
//         .collect();
    
//     let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
    
//     // Sort to get the two smallest
//     let mut sides = [l, w, h];
//     sides.sort();
    
//     // Perimeter of smallest side (2 * (smallest + second_smallest))
//     let perimeter = 2 * (sides[0] + sides[1]);
    
//     // Volume (for the bow)
//     let volume = l * w * h;
    
//     perimeter + volume
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let input = fs::read_to_string("input.txt")?;
//     let input = input.trim();
    
//     // ========== PART 1 ==========
//     let total_paper: u32 = input
//         .lines()
//         .map(|line| calculate_gift_area(line))
//         .sum();
    
//     println!("Part 1: {}", total_paper);
    
//     // ========== PART 2 ==========
//     let total_ribbon: u32 = input
//         .lines()
//         .map(|line| calculate_gift_ribbon(line))
//         .sum();
    
//     println!("Part 2: {}", total_ribbon);
    
//     Ok(())
// }
