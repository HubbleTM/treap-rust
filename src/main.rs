use rand::{Rng, random};

#[derive(Clone)]
enum NodeValue where {
    Node(i32, i32, usize, Box<NodeValue>, Box<NodeValue>),
    Nil,
}

impl NodeValue {
    fn update_size(&mut self) -> usize {
        return match self {
            NodeValue::Nil => {
                0
            }

            NodeValue::Node(_, _, size, left, right) => {
                *size = 1 + left.size() + right.size();
                *size
            }
        };
    }

    fn size(&self) -> usize {
        return match self {
            NodeValue::Nil => 0,
            NodeValue::Node(_, _, sz, ..) => *sz
        };
    }

    pub fn new(x: i32, y: i32, left: Box<NodeValue>, right: Box<NodeValue>) -> NodeValue {
        let mut t = NodeValue::Node(x, y, 1, left, right);
        t.update_size();
        t
    }
}

struct Tree {
    root: Box<NodeValue>
}

impl Tree {
    fn split(t: Box<NodeValue>, x: i32) -> (Box<NodeValue>, Box<NodeValue>) {
        return match t.as_ref() {
            NodeValue::Nil => (Box::new(NodeValue::Nil), Box::new(NodeValue::Nil)),

            NodeValue::Node(tx, ty, size, left, right) => if *tx <= x {
                let (l, r) = Tree::split(right.clone(), x);
                let t = NodeValue::new(*tx, *ty, left.clone(), l);
                (Box::new(t), r)
            } else {
                let (l, r) = Tree::split(left.clone(), x);
                let t = NodeValue::new(*tx, *ty, r, right.clone());
                (l, Box::new(t))
            }
        };
    }


    fn merge(t1: Box<NodeValue>, t2: Box<NodeValue>) -> Box<NodeValue> {
        let tr1 = t1.as_ref();
        let tr2 = t2.as_ref();

        if let NodeValue::Nil = tr1 {
            return t2;
        }

        if let NodeValue::Nil = tr2 {
            return t1;
        }

        if let (
            NodeValue::Node(x1, y1, _, l1, r1),
            NodeValue::Node(x2, y2, _, l2, r2)
        ) = (tr1, tr2) {
            return if y1 > y2 {
                let mut t = NodeValue::new(*x1, *y1, l1.clone(), Tree::merge(r1.clone(), t2));
                Box::new(t)
            } else {
                let mut t = NodeValue::new(*x2, *y2, Tree::merge(t1, l2.clone()), r2.clone());
                Box::new(t)
            };
        }

        return Box::new(NodeValue::Nil);
    }

    fn new() -> Tree {
        Tree {
            root: Box::new(NodeValue::Nil),
        }
    }

    fn push(&mut self, x: i32) {
        let node = NodeValue::new(x, random(), Box::new(NodeValue::Nil), Box::new(NodeValue::Nil));

        match self.root.as_ref() {
            NodeValue::Nil => self.root = Box::from(node),
            _ => {
                let (l, r) = Tree::split(self.root.clone(), x);
                let l = Tree::merge(l, Box::new(node));
                self.root = Tree::merge(l, r);
            }
        }
    }

    fn get(&self, k: usize) -> Box<NodeValue> {
        return Tree::get_rec(&self.root, k);
    }

    fn get_rec(t: &Box<NodeValue>, k: usize) -> Box<NodeValue> {
        if let NodeValue::Node(.., l, r) = t.as_ref() {
            if t.size() < k {
                return Box::new(NodeValue::Nil);
            }

            if l.size() + 1 < k {
                return Tree::get_rec(r, k);
            }

            if l.size() + 1 == k {
                return t.clone();
            }

            return Tree::get_rec(l, k);
        }

        return Box::new(NodeValue::Nil);
    }

    fn del(&mut self, x: i32) {
        let (l, r) = Tree::split(self.root.clone(), x);
        self.root = Tree::merge(Tree::del_rec(&l, x), r);
    }

    fn del_rec(t: &Box<NodeValue>, x: i32) -> Box<NodeValue> {
        if let NodeValue::Node(tx, ty, _, l, r) = t.as_ref() {
            if *tx == x {
                return l.clone();
            }

            return Box::new(NodeValue::new(*tx, *ty, l.clone(), Tree::del_rec(r, x)));
        }
        return Box::new(NodeValue::Nil);
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.push(10);
    tree.push(5);
    tree.push(9);

    println!("{}", tree.root.size());
    tree.del(9);
    println!("{}", tree.root.size());

    let res = tree.get(1);
    match res.as_ref() {
        NodeValue::Node(x, ..) => println!("{}", x),
        _ => println!("Oops!"),
    }
}
