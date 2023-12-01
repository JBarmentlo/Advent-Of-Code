use std::fs;
use itertools::Itertools;

fn main() {
    let text = fs::read_to_string("data.txt").expect("Fixed input");
    let marker = get_marker(&text, 4);
    println!("marker: {marker}");
}

fn get_marker(text: &str, n_distinct: usize) -> usize {
    let mut window = Vec::from(['0', '0', '0', '0']);

    println!("{text}");
    text.chars().enumerate()
                .filter(|(i, c)| {
                    window[i % n_distinct] = *c;
                    window.iter().unique().count().eq(&n_distinct) && (i >= &n_distinct)
                })
                .map(|(i, _)| i + 1)
                .next()
                .expect("It exists")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = get_marker(input, 4);

        assert_eq!(marker, 5);
    }


    #[test]
    fn two() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = get_marker(input, 4);

        assert_eq!(marker, 6);
    }


    #[test]
    fn three() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = get_marker(input, 4);

        assert_eq!(marker, 10);
    }


    #[test]
    fn four() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = get_marker(input, 4);

        assert_eq!(marker, 11);
    }
}
