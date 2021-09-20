use std::io::Read;

struct Node {
    x: i32,
    y: i32,
    size: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub type Treap = Option<Box<Node>>;

pub fn merge(t1: &mut Treap, t2: Treap) {
    match (t1.take(), t2) {
        (Some(mut tl), Some(mut tr)) => {
            if tl.y > tr.y {
                merge(&mut tl.right, Some(tr));
                tl.update();
                *t1 = Some(tl);
            } else {
                let mut new_tree = Some(tl);
                merge(&mut new_tree, tr.left.take());
                tr.left = new_tree;
                tr.update();
                *t1 = Some(tr);
            }
        }

        (t, None) | (None, t) => {
            *t1 = t;
        }
    }
}

pub fn split(t: &mut Treap, k: i32) -> Treap {
    return match t.take() {
        Some(mut node) => {
            let ret;

            if node.x <= k {
                ret = split(&mut node.right, k);
                node.update();
                *t = Some(node);
            } else {
                let res = split(&mut node.left, k);
                *t = node.left.take();
                node.left = res;
                node.update();
                ret = Some(node);
            }

            ret
        }

        None => None
    };
}

fn push(t: &mut Treap, x: i32, y: i32) {
    let node = Some(Box::new(
        Node {
            x,
            y,
            size: 1,
            left: None,
            right: None,
        }
    ));

    let right = split(t, x);
    merge(t, node);
    merge(t, right);
}

fn get(t: &Treap, k: usize) -> i32 {
    return match t.as_ref() {
        None => 0,
        Some(node) => {
            let mut sz: usize = 1;
            if let Some(t) = node.right.as_ref() {
                sz += t.size;
            }

            if sz < k {
                return get(&node.left, k - sz);
            }

            if sz == k {
                return node.x;
            }

            return get(&node.right, k);
        }
    };
}

fn del(t: &mut Treap, k: i32) {
    match t.take() {
        None => {}
        Some(mut node) => {
            if node.x == k {
                merge(&mut node.left, node.right);
                *t = node.left;
                return;
            }

            if node.x < k {
                del(&mut node.right, k);
            } else {
                del(&mut node.left, k);
            }

            node.update();
            *t = Some(node)
        }
    };
}


impl Node {
    fn update(&mut self) {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Some(tl), Some(tr)) => {
                self.size = 1 + tl.size + tr.size;
            }

            (Some(t), None) | (None, Some(t)) => {
                self.size = 1 + t.size;
            }

            _ => {
                self.size = 1;
            }
        }
    }
}

fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn read() -> i32 { get_word().parse().ok().unwrap() }

const A: i64 = 1366;
const C: i64 = 150889;
const M: i64 = 714025;

struct Random {
    seed: i64
}

impl Random {
    fn new() -> Self {
        let cur_time = 0x114514;
        let seed = cur_time % M;
        return Random { seed };
    }

    fn next(&mut self) -> i32 {
        self.seed = (A * self.seed + C) % M;
        self.seed as i32
    }
}

fn main() {
    let mut random = Random::new();
    random.next();

    let mut treap = None;
    let mut n: i32 = read();

    for i in 0..n {
        let op: i32 = read();
        let val: i32 = read();

        match op {
            1 => push(&mut treap, val, random.next()),
            0 => println!("{}", get(&treap, val as usize)),
            _ => del(&mut treap, val),
        }
    }
}