use std::fs;
use itertools::Itertools;

fn main() {
    let text = fs::read_to_string("data.txt").expect("Fixed input");
    let marker = get_marker(&text);
    println!("marker: {marker}");
}

fn get_marker(text: &str) -> usize {
    let mut window: [char; 4] = ['0', '0', '0', '0'];
    println!("{text}");
    text.chars().enumerate()
                .filter(|(i, c)| {
                    window[i % 4] = *c;
                    window.iter().unique().count().eq(&4) && (i >= &4)
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
        let marker = get_marker(input);

        assert_eq!(marker, 5);
    }


    #[test]
    fn two() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = get_marker(input);

        assert_eq!(marker, 6);
    }


    #[test]
    fn three() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = get_marker(input);

        assert_eq!(marker, 10);
    }


    #[test]
    fn four() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = get_marker(input);

        assert_eq!(marker, 11);
    }
}
