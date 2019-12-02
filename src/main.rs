fn main() -> Result<(), std::io::Error> {
    let (mut total, content) = (0, std::fs::read_to_string("inputs.txt")?);
    let _ : Vec<&str> = content.lines().collect::<Vec<&str>>().iter().map(|input| {total += input.clone().parse::<u64>().unwrap()/3 - 2; *input}).collect();
    println!("{:?}", total);
    Ok(())
}
