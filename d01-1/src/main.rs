use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut sum: usize = 0;
    let mut max: usize = 0;
    let mut lines_read: usize = 0;

    loop {
        let res = stdin.read_line(&mut input);
        lines_read+=1;

        if input.len() < 1{  
            if sum > max{
                max = sum;
            }
            sum = 0;
        };
        let num: usize;
        match input.trim().parse(){
            Ok(x) =>
            {
                num = x;
                sum += num;
            }
            Err(y) =>
            {
                match y.kind(){
                    std::num::IntErrorKind::Empty =>
                    {
                        if sum > max {
                            println!("{}", sum);
                            max = sum;
                        }
                        sum = 0;
                    }
                    _ =>
                        println!(
                            "[{:?}]{} while parsing {}",
                            y.kind(),
                            y.to_string(),
                            input),
                }
            }
        };
        input = "".to_string();
        match res{
            Ok(x) =>
            if x == 0{
                println!("Ending loop");
                break;
            },
            Err(y) =>
            { 
                println!(
                    "[{:?}]{} while reading {}",
                    y.kind(),
                    y.to_string(),
                    input);
                break;
            }
        }
    } 

    println!("max is {} from {} lines", max, lines_read);

    Ok(())
}
