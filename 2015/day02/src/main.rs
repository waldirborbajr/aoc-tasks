use std::fs;

/// Parseia uma linha "LxWxH" em um array ordenado [menor, meio, maior]
fn parsear_dimensoes(input: &str) -> [u32; 3] {
    let mut dims: [u32; 3] = input
        .split('x')
        .map(|s| s.parse().expect("Wrong dimension"))
        .collect::<Vec<u32>>()
        .try_into()
        .expect("Expected  exatamente 3 dimensões");
    
    dims.sort();
    dims
}

fn calcular_area_presente(input: &str) -> u32 {
    let [a, b, c] = parsear_dimensoes(input); // a <= b <= c
    
    // Áreas dos lados (a*b é sempre o menor)
    let lado1 = a * b;  // Menor lado
    let lado2 = b * c;
    let lado3 = a * c;
    
    // Superfície + menor lado
    2 * (lado1 + lado2 + lado3) + lado1
}

fn calcular_fita_presente(linha: &str) -> u32 {
    let [a, b, c] = parsear_dimensoes(linha); // a <= b <= c
    
    // Perímetro dos dois menores lados + volume
    2 * (a + b) + (a * b * c)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.trim();
    
    let total_papel: u32 = input.lines().map(calcular_area_presente).sum();
    let total_fita: u32 = input.lines().map(calcular_fita_presente).sum();
    
    println!("Parte 1: {}", total_papel);
    println!("Parte 2: {}", total_fita);
    
    Ok(())
}
