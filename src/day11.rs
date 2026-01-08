use std::collections::{HashMap, VecDeque};
use std::io;

fn parse(line: &str) -> (String, Vec<String>) {
    let (source, dest) = line.split_once(":").unwrap();
    let dests: Vec<String> = dest
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect();
    (source.to_string(), dests)
}

fn bfs(graph: &HashMap<String, Vec<String>>) -> u64 {
    let mut q: VecDeque<String> = VecDeque::new();
    let mut tot: u64 = 0;
    q.push_back("you".to_string());
    while let Some(source) = q.pop_front() {
        let to_visit = graph.get(&source).unwrap();
        for v in to_visit {
            if v != "out" {
                q.push_back(v.clone());
            } else {
                tot += 1;
            }
        }
    }
    tot
}

fn dfs(
    current: &String,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if current == "out" {
        return 1;
    }
    if cache.contains_key(current) {
        return *cache.get(current).unwrap();
    }

    let mut tot: u64 = 0;
    let to_visit = graph.get(current).unwrap();
    for node in to_visit {
        tot += dfs(node, graph, cache);
    }
    cache.insert(current.clone(), tot);
    tot
}

fn dfs_part2(
    current: &str,
    has_dac: bool,
    has_fft: bool,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    if current == "out" {
        return if has_dac && has_fft { 1 } else { 0 };
    }
    let key = (current.to_string(), has_dac, has_fft);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    let mut tot = 0;
    if let Some(to_visit) = graph.get(current) {
        for node in to_visit {
            let new_has_dac = has_dac || node == "dac";
            let new_has_fft = has_fft || node == "fft";
            tot += dfs_part2(node, new_has_dac, new_has_fft, graph, cache);
        }
    }
    cache.insert(key, tot);
    tot
}

pub fn solve(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let line = line?;
        let (source, dests) = parse(&line);
        graph.insert(source, dests);
    }

    let ans = dfs(&String::from("you"), &graph, &mut HashMap::new());
    let bfs_ans = bfs(&graph);
    println!("bfs: {}, part1: {}", bfs_ans, ans);
    let ans = dfs_part2("svr", false, false, &graph, &mut HashMap::new());
    Ok(ans as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_str = "
            aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out";
        let input: Vec<Result<String, _>> = input_str
            .lines()
            .map(|line| Ok(line.trim().to_string())) // Trim each line
            .filter(|line| !line.as_ref().unwrap().is_empty()) // Skip empty lines
            .collect();

        let result = solve(input.into_iter()).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_2() {
        let input_str = "
            svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out";
        let input: Vec<Result<String, _>> = input_str
            .lines()
            .map(|line| Ok(line.trim().to_string())) // Trim each line
            .filter(|line| !line.as_ref().unwrap().is_empty()) // Skip empty lines
            .collect();

        let result = solve(input.into_iter()).unwrap();
        assert_eq!(result, 2);
    }
}
