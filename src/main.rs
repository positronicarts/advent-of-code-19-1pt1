fn main() {
    let (mut total, content) = (0, std::fs::read_to_string("inputs.txt").unwrap());
    content.lines().collect::<Vec<&str>>().iter().for_each(|input| {total += input.clone().parse::<u64>().unwrap()/3 - 2; println!("{}", total);});   
}
