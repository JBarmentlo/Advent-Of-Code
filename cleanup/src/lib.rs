use std::ops::RangeInclusive;


pub fn included(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.contains(&b.start()) && a.contains(&b.end())
}

pub fn overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    // println!("a: {:?}, b: {:?}", a, b);
    a.contains(&b.start()) || a.contains(&b.end())
}

pub fn to_range(pair: &str) -> RangeInclusive<u32> {
    let mut bounds = pair.split('-');
    let lower_bound = bounds.next().expect("Exists").parse::<u32>().expect("Fixed input");
    let upper_bound = bounds.next().expect("Exists").parse::<u32>().expect("Fixed input");

    RangeInclusive::new(lower_bound, upper_bound)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn overlap_one() {
        let a = to_range("0-0");
        let b = to_range("0-1");

        assert!(overlap(&a, &b) || overlap(&b, &a))
    }
    
    #[test]
    fn overlap_two() {
        let a = to_range("0-0");
        let b = to_range("0-0");

        assert!(overlap(&a, &b) || overlap(&b, &a))
    }

    #[test]
    fn overlap_three() {
        let a = to_range("10-15");
        let b = to_range("11-11");

        assert!(overlap(&a, &b) || overlap(&b, &a))
    }

    #[test]
    fn overlap_four() {
        let a = to_range("10-15");
        let b = to_range("9-16");

        assert!(overlap(&a, &b) || overlap(&b, &a))
    }

    #[test]
    fn no_overlap_one() {
        let a = to_range("0-0");
        let b = to_range("1-1");

        assert!(!(overlap(&a, &b) || overlap(&b, &a)))
    }

    #[test]
    fn included_one() {
        let a = to_range("0-1");
        let b = to_range("1-1");

        assert!((included(&a, &b) || included(&b, &a)))
    }

    #[test]
    fn included_two() {
        let a = to_range("0-0");
        let b = to_range("1-1");

        assert!(!(included(&a, &b) || included(&b, &a)))
    }
}