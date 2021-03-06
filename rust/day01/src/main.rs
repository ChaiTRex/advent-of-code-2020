use std::fs::File;
use std::io::Read;

fn main() {
    let values = {
        let mut contents = String::new();
        let _ = File::open("../01.txt")
            .unwrap()
            .read_to_string(&mut contents);

        let mut values = contents
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        values.sort();
        values
    };

    'two: for (i, x) in values.iter().copied().enumerate() {
        if x >= 2020 {
            break;
        }
        for y in values[i + 1..].iter().copied() {
            let x_plus_y = x + y;
            if x_plus_y == 2020 {
                println!("{}", x * y);
                break 'two;
            } else if x_plus_y > 2020 {
                break;
            }
        }
    }

    'three: for (i, x) in values.iter().copied().enumerate() {
        if x >= 2020 {
            break;
        }
        let values = &values[i + 1..];
        for (j, y) in values.iter().copied().enumerate() {
            let x_plus_y = x + y;
            if x_plus_y + y > 2020 {
                break;
            }
            let values = &values[j + 1..];
            for z in values.iter().copied() {
                let x_plus_y_plus_z = x_plus_y + z;
                if x_plus_y_plus_z == 2020 {
                    println!("{}", x * y * z);
                    break 'three;
                } else if x_plus_y_plus_z > 2020 {
                    break;
                }
            }
        }
    }
}
