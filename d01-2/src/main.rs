use std::io;

fn insert_max(arr: &mut [usize], max: usize){
    println!("Inserting {} to {:?}", max, arr);
    arr[2] = max;
    arr[..].sort();    
    arr[..].reverse();
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut sum: usize = 0;
    let mut maxes = [0, 0, 0];
    let mut lines_read: usize = 0;

    loop {
        let res = stdin.read_line(&mut input);
        lines_read+=1;

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
                        if sum > maxes[2] {
                            println!("{}", sum);
                            insert_max(&mut maxes, sum);
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

    for max in maxes{
        sum += max;
    }
    println!("maxes are {:?} -sum: {} from {} lines", maxes, sum, lines_read);

    Ok(())
}
