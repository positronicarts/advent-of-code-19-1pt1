fn main() {
    let (mut total, content) = (0, std::fs::read_to_string("inputs.txt").unwrap());
    let _ : Vec<&str> = content.lines().collect::<Vec<&str>>().iter().map(|input| {total += input.clone().parse::<u64>().unwrap()/3 - 2; println!("{}", total); *input}).collect();   
}
