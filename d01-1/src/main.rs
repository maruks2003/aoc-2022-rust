use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    let res: io::Result<usize> = Ok(0);
    let mut sum: usize = 0;
    let mut max: usize = 0;

    while res.is_ok() {
        //TODO error handling this res
        let _res = stdin.read_line(&mut input);
        if input.len() < 1{  
            if sum > max{
                max = sum;
            }
            sum = 0;
        }
        let num: usize = input.parse().unwrap();
        sum += num;
        print!("{}",input);
    }

    print!("{}", max);

    Ok(())
}
