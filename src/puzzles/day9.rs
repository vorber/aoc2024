use std::fs;

pub fn solve() {
    let data = fs::read_to_string("../inputs/day9").expect("Should be able to read input");
    println!("P1: {p1}", p1 = part1(&data));
    println!("P2: {p2}", p2 = part2(&data));
}

fn part1(data:&String) -> usize {
    let (mut files, free) = parse_input_p1(data);
    let mut l = 0; let mut r = files.len()-1;
    while free[l].sector < files[r].sector {
        (0..free[l].size).for_each(|i| { files[r-i].sector = free[l].sector; });
        r -= free[l].size;
        l += 1;
    }
    checksum(&files)
}

fn part2(data:&String) -> usize {
    let (mut files, mut free) = parse_input_p2(data);
    files.iter_mut().rev().for_each(|f| {
        if let Some(ff) = free.iter_mut().find(|ff| ff.sector < f.sector && ff.size >= f.size) {
            f.sector = ff.sector;
            ff.sector += f.size;
            ff.size -= f.size;
        }
    });
    checksum(&files)
}

#[derive(Debug)]
struct Span {
    index: usize,
    sector: usize,
    size: usize
}

fn parse_input_p1(data:&String) -> (Vec<Span>, Vec<Span>) {
    parse_input(data, |f,i,p,s| f.extend((0..s).map(|k| Span {index: i, sector: p + k, size: 1})))
}

fn parse_input_p2(data:&String) -> (Vec<Span>, Vec<Span>) {
    parse_input(data, |f, i, p, s| f.push(Span {index:i, sector: p, size:s}))
}

fn parse_input(data:&String, extend: impl Fn(&mut Vec<Span>, usize, usize, usize) -> ()) -> (Vec<Span>, Vec<Span>) {
    let mut sector = 0;
    let mut files = Vec::new();
    let mut free = Vec::new();
    let mut r = vec![&mut files,&mut free];
    for (i,c) in data.chars().enumerate() {
        if let Some(v) = parse(c) {
            extend(r[i%2], i/2, sector, v as usize);
            sector += v as usize;
        }
    }
    (files, free)
}

fn parse(c:char) -> Option<u8> {
    let v = c as u8;
    if v >= b'0' {Some(v-b'0')} else { None }
}

fn checksum(files: &Vec<Span>) -> usize {
    files.iter().map(|f| (f.sector..f.sector+f.size).map(|s| f.index*s).sum::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let data = String::from("2333133121414131402");

        assert_eq!(part1(&data), 1928);
        assert_eq!(part2(&data), 2858);
    }
}
