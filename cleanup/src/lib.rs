pub fn included(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.contains(&b.start) && a.contains(&b.end)
}

pub fn overlap(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.contains(&b.start) || a.contains(&b.end)
}

pub fn to_range(pair: &str) -> Range<u32> {
    let mut bounds = pair.split('-');
    let lower_bound = bounds.next().expect("Exists").parse::<u32>().expect("Fixed input");
    let upper_bound = bounds.next().expect("Exists").parse::<u32>().expect("Fixed input");

    lower_bound..upper_bound
}

#[cfg(test)]
mod tests {
    super::*;
    
    #[test]
    fn overlap_one() {
        let str = "0,0-0,1";
        assert!()
    }
}