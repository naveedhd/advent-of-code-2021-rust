

pub fn solve(file_string: &str) {
    let data = parse(&file_string);

    println!("{:?}", solve1(&data));
    println!("{:?}", solve2(&data));
}

fn parse(file_string: &str) -> Vec<&str> {
    file_string.lines().collect()
}

fn solve1(bits_vector: &Vec<&str>) -> usize {
    let mcbs = most_common_bits(bits_vector);
    let lcbs: String = mcbs.chars().map(|c| if c == '0' { '1' } else { '0' }).collect();

    let num = usize::from_str_radix(&mcbs, 2).unwrap();
    let inverted_num = usize::from_str_radix(&lcbs, 2).unwrap();

    num * inverted_num
}

fn solve2(bits_vector: &Vec<&str>) -> usize {
    let bits_length = bits_vector[0].len();

    let mut oxygen_level = 0;
    let mut filtered: Vec<&str> = bits_vector.clone();
    for bit_place in 0..bits_length {
        let mcb = most_common_bit(&filtered, bit_place);

        filtered = filtered.into_iter().filter(|bits| {
            bits.chars().nth(bit_place).unwrap() == mcb
        }).collect();

        if filtered.len() == 1 {

            oxygen_level = usize::from_str_radix(&filtered[0], 2).unwrap();
            break;
        }
    }

    let mut carbon_level = 0;

    let mut filtered: Vec<&str> = bits_vector.clone();
    for bit_place in 0..bits_length {
        let lst = least_common_bit(&filtered, bit_place);
        filtered = filtered.into_iter().filter(|bits| { bits.chars().nth(bit_place).unwrap() == lst } ).collect();


        if filtered.len() == 1 {
            carbon_level = usize::from_str_radix(&filtered[0], 2).unwrap();
            break;
        }
    }

    oxygen_level * carbon_level
}

fn most_common_bits(bits_vector: &Vec<&str>) -> String {
    let bits_length = bits_vector[0].len();

    let mut zeros_count = vec![0; bits_length];
    for bits in bits_vector {
        for (i, c) in bits.chars().enumerate() {
            if c == '0' {
                zeros_count[i] += 1;
            }
        }
    }

    let bits_length = bits_vector.len();

    zeros_count.into_iter().map(|zero_count| {
        let one_count = bits_length - zero_count;
        if one_count < zero_count { '0' } else { '1' }
    }).collect()
}

fn most_common_bit(bits_vector: &Vec<&str>, place: usize) -> char {
    let zero_count = bits_vector.iter().filter(|bits| { bits.chars().nth(place).unwrap() == '0' }).count();

    let bits_count = bits_vector.len();
    let ones_count = bits_count - zero_count;

    if zero_count > ones_count { '0' } else { '1' }
}

fn least_common_bit(bits_vector: &Vec<&str>, place: usize) -> char {
    let zero_count = bits_vector.iter().filter(|bits| { bits.chars().nth(place).unwrap() == '0' }).count();

    let bits_count = bits_vector.len();
    let ones_count = bits_count - zero_count;

    if ones_count < zero_count { '1' } else { '0' }
}
