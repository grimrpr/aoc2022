use std::{
    borrow::Borrow, cmp::max, collections::HashSet, fs::File, io::BufRead, io::BufReader, u32,
};

#[derive(Clone, Copy, Debug)]
struct Tree {
    id: usize,
    height: u32,
}

impl From<(usize, u32)> for Tree {
    fn from((id, height): (usize, u32)) -> Self {
        Tree {
            id: id,
            height: height,
        }
    }
}

fn check_tree<'a>(
    (max_height, hidden_set): (u32, &'a mut HashSet<usize>),
    tree: &Tree,
) -> (u32, &'a mut HashSet<usize>) {
    if max_height >= tree.height {
        (max_height, hidden_set)
    } else {
        hidden_set.remove(&tree.id);
        (tree.height, hidden_set)
    }
}

fn check_line(tree_line: &[Tree], hidden_set: &mut HashSet<usize>) {
    let set_updated = tree_line.iter().fold((0u32, hidden_set), check_tree).1;
    tree_line.iter().rev().fold((0u32, set_updated), check_tree);
}

fn parse_lines<B>(input_reader: B) -> Vec<Vec<Tree>>
where
    B: BufRead,
{
    let mut tree_ids = 1..;
    input_reader
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            tree_ids
                .by_ref()
                .zip(l.chars().map(|c| c.to_digit(10).unwrap() + 1))
                .map(Tree::from)
                .collect::<Vec<Tree>>()
        })
        .collect()
}

fn count_visible_trees(trees: &Vec<Vec<Tree>>) -> usize {
    let num_cols = trees.first().unwrap().len();
    let num_trees = num_cols * trees.len();
    let mut hidden_trees: HashSet<usize> = HashSet::from_iter(trees.iter().flatten().map(|t| t.id));

    // check horizontal treelines
    trees
        .iter()
        .for_each(|line| check_line(line, &mut hidden_trees));

    for k in 0..num_cols {
        let col = trees
            .iter()
            .flatten()
            .skip(k)
            .step_by(num_cols)
            .copied()
            .collect::<Vec<Tree>>();
        check_line(&col, &mut hidden_trees)
    }

    println!("Id set size after checks: {}", hidden_trees.len());
    num_trees - hidden_trees.len()
}

fn scenic_score<I>(trees: I, height: u32) -> usize
where
    I: IntoIterator,
    I::Item: Borrow<Tree>,
{
    trees
        .into_iter()
        .fold((0usize, 0u32), |(score, last_height), tree| {
            if last_height >= height {
                (score, last_height)
            } else {
                (score + 1, tree.borrow().height)
            }
        }).0
}

fn highest_scenic_score(trees: &Vec<Vec<Tree>>) -> usize {
    let mut trees_cols: Vec<Vec<Tree>> = Vec::new();
    let num_cols = trees.first().unwrap().len();
    for k in 0..num_cols {
        trees_cols.push(
            trees
                .iter()
                .flatten()
                .skip(k)
                .step_by(num_cols)
                .copied()
                .collect::<Vec<Tree>>(),
        );
    }

    let mut max_score = 0;
    for r in 0..trees.len() {
        for c in 0..trees_cols.len() {
            let left = scenic_score(trees[r][..c].iter().rev(), trees[r][c].height);
            let right = scenic_score(trees[r][c + 1..].iter(), trees[r][c].height);
            let up = scenic_score(trees_cols[c][..r].iter().rev(), trees[r][c].height);
            let down = scenic_score(trees_cols[c][r + 1..].iter(), trees[r][c].height);
            max_score = max(left * right * up * down, max_score);
        }
    }

    max_score
}

pub fn print_answer() {
    let buf_read = BufReader::new(File::open("data/input_day8").unwrap());
    let trees = parse_lines(buf_read);
    println!("Visible trees: {}", count_visible_trees(&trees));
    println!("Max scenic score: {}", highest_scenic_score(&trees));
}
