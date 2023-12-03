pub fn de_snafu(num_str: &str) -> isize {
    num_str
        .chars()
        .rev()
        .map(|c| match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("invalid char: {c}"),
        })
        .enumerate()
        .fold(0, |acc, (power, count)| {
            acc + count * 5isize.pow(power as u32)
        })
}

pub fn snafu(mut n: isize) -> String {
    let mut snafu = String::new();
    let mut factors = vec![];
    let mut i = 20;
    while i >= 0 {
        let factor = n / 5isize.pow(i as u32);
        if !factors.is_empty() || factor > 0 {
            factors.push(factor);
        }
        n = n % 5isize.pow(i as u32);
        i -= 1;
    }

    dbg!(&factors);

    let mut carry = false;
    for factor in factors.into_iter().rev() {
        match factor {
            0 => snafu.push(if carry { '1' } else { '0' }),
            1 => snafu.push(if carry { '2' } else { '1' }),
            2 => snafu.push(if carry { '=' } else { '2' }),
            3 => snafu.push(if carry { '-' } else { '=' }),
            4 => snafu.push(if carry { '0' } else { '-' }),
            _ => panic!(),
        }
        carry = factor >= 3 || carry && factor >= 2;
    }
    if carry {
        snafu.push('1');
    }

    snafu.chars().rev().collect()
}
/*
* 24
* 44
* 10-
*
* - carry
* 0 carry
* 1 nocarry
*/
