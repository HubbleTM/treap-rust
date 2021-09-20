use std::cmp::max;
use std::io::Read;

enum Treap where {
    Node(i32, i32, usize, Box<Treap>, Box<Treap>),
    Nil,
}

impl Treap {
    pub fn new(x: i32, y: i32, left: Box<Treap>, right: Box<Treap>) -> Treap {
        let mut t = Treap::Node(x, y, 1, left, right);
        t.update_size();
        t
    }

    fn size(&self) -> usize {
        return match self {
            Treap::Nil => 0,
            Treap::Node(_, _, sz, ..) => *sz
        };
    }

    fn update_size(&mut self) -> usize {
        return match self {
            Treap::Nil => {
                0
            }

            Treap::Node(_, _, size, left, right) => {
                *size = 1 + left.size() + right.size();
                *size
            }
        };
    }

    fn split(self, k: i32) -> (Self, Self) {
        use Treap::*;

        return match self {
            Nil => (Nil, Nil),
            Node(x, y, _, left, right) => {
                if x <= k {
                    let (l, r) = right.split(k);
                    (Treap::new(x, y, left, Box::new(l)), r)
                } else {
                    let (l, r) = left.split(k);
                    (l, Treap::new(x, y, Box::new(r), right))
                }
            }
        };
    }

    fn merge(t1: Self, t2: Self) -> Self {
        use Treap::*;

        return match (t1, t2) {
            (Nil, Nil) => Nil,
            (t, Nil) => t,
            (Nil, t) => t,
            (Node(lx, ly, _, ll, lr), Node(rx, ry, _, rl, rr)) => {
                if ly > ry {
                    let t2 = Treap::new(rx, ry, rl, rr);
                    Treap::new(lx, ly, ll, Box::new(Treap::merge(*lr, t2)))
                } else {
                    let t1 = Treap::new(lx, ly, ll, lr);
                    Treap::new(rx, ry, Box::new(Treap::merge(t1, *rl)), rr)
                }
            }
        };
    }

    fn push(self, x: i32, y: i32) -> Self {
        use Treap::*;

        let node = Treap::new(x, y, Box::new(Treap::Nil), Box::new(Treap::Nil));
        return match self {
            Nil => node,
            _ => {
                let (l, r) = self.split(x);
                let left = Treap::merge(l, node);
                Treap::merge(left, r)
            }
        };
    }

    fn get(&self, k: usize) -> i32 {
        use Treap::*;

        return match self {
            Nil => 0,
            Node(x, _, _, l, r) => {
                if r.size() + 1 < k {
                    return l.get(k - r.size() - 1);
                }

                if r.size() + 1 == k {
                    return *x;
                }

                return r.get(k);
            }
        };
    }

    fn del(self, k: i32) -> Self {
        use Treap::*;

        return match self {
            Nil => Nil,
            Node(x, y, sz, l, r) => {
                if x == k {
                    return Treap::merge(*l, *r);
                }

                if x < k {
                    return Treap::new(x, y, l, Box::new(r.del(k)));
                }

                return Treap::new(x, y, Box::new(l.del(k)), r);
            }
        };
    }

    fn deep(&self) -> i32 {
        use Treap::*;

        return match self {
            Nil => 0,
            Node(.., l, r) => {
                max(l.deep(), r.deep()) + 1
            }
        };
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

fn get() -> i32 { get_word().parse().ok().unwrap() }

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

    let mut treap = Treap::Nil;
    let mut n: i32 = get();
    for i in 0..n {
        let op: i32 = get();
        let val: i32 = get();

        match op {
            1 => treap = treap.push(val, random.next()),
            0 => println!("{}", treap.get(val as usize)),
            _ => treap = treap.del(val),
        }
    }
}