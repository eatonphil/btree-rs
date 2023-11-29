struct BTreeNode<K: Ord + Clone + std::fmt::Debug, V: Ord + Clone + std::fmt::Debug> {
    node_size: usize,
    keys: Vec<K>,
    children: Vec<BTreeNode<K, V>>,
    values: Vec<V>,
}

impl<K: Ord + Clone + std::fmt::Debug, V: Ord + Clone + std::fmt::Debug> BTreeNode<K, V> {
    fn new(node_size: usize) -> BTreeNode<K, V> {
        return BTreeNode::<K, V> {
            node_size: node_size,
            // At the end of any operation there should only be
            // `node_size` keys left. However, we leave space for an
            // extra key for when we split.
            keys: Vec::<K>::with_capacity(node_size + 1),
            children: Vec::<BTreeNode<K, V>>::with_capacity(node_size + 1),
            // As with keys, values has extra space. At the end of an
            // operation there should be at most `node_size` values.
            values: Vec::<V>::with_capacity(node_size + 1),
        };
    }

    fn display(&self, depth: usize) {
        for (i, key) in self.keys.iter().enumerate() {
            if i < self.children.len() {
                self.children[i].display(depth + 1);
            }

            let k = key.clone();
            let v = self.values[i].clone();
            println!("{}{:?} = {:?}", " ".repeat(depth * 2), k, v);
        }

        if self.children.len() > self.keys.len() {
            self.children[self.children.len() - 1].display(depth + 1);
        }
    }

    fn next_index(keys: &Vec<K>, key: &K) -> i32 {
        let mut low: i32 = 0;
        let mut high = keys.len() as i32;
        while low != high {
            let mid = (low + high) / 2;
            let ordering = key.cmp(&keys[mid as usize]);
            if ordering == std::cmp::Ordering::Less {
                high = mid;
            } else if ordering == std::cmp::Ordering::Greater {
                low = mid + 1;
            } else {
                return -mid - 1;
            }
        }

        return low;
    }

    fn lookup_path(&self, key: &K) -> Vec<usize> {
        let mut stack = Vec::<usize>::new();
        let mut curnode = self;
        loop {
            let i = BTreeNode::<K, V>::next_index(&curnode.keys, key);
            if i < 0 {
                stack.push(-(i + 1) as usize);
                break;
            } else if (i as usize) < curnode.children.len() {
                curnode = &curnode.children[i as usize];
                stack.push(i as usize);
            } else {
                stack.clear();
                break;
            }
        }

        stack.reverse();
        return stack;
    }

    fn lookup(&self, key: &K) -> Option<V> {
        let mut path = self.lookup_path(key);
        let mut curnode = self;
        let mut valueindex = 0;
        while let Some(index) = path.pop() {
            if path.len() == 0 {
                valueindex = index;
                break;
            }

            curnode = &curnode.children[index];
        }

        if curnode.keys[valueindex] == *key {
            return Some(curnode.values[valueindex].clone());
        }

        return None;
    }

    fn split(&mut self) -> BTreeNode<K, V> {
        let mid = self.keys.len() / 2; // Will always be floor()ed;
        let mut new = BTreeNode::<K, V>::new(self.node_size);
        new.keys = self.keys[mid..].to_vec();
        new.values = self.values[mid..].to_vec();

        self.keys.drain(mid..);
        self.values.drain(mid..);

        return new;
    }

    fn insert_recursive(&mut self, key: K, value: V) -> Option<BTreeNode<K, V>> {
        let i = BTreeNode::<K, V>::next_index(&self.keys, &key);
        // Leaf node, add to values.
        if self.children.len() == 0 {
            let i = if i < 0 { -(i + 1) } else { i } as usize;
            self.keys.insert(i, key);
            self.values.insert(i, value);
        } else {
            let i = i as usize;
            let children = &mut self.children;
            assert!(i <= children.len() + 1);
            // Create space for a new right-most child.
            if i == children.len() {
                children.push(BTreeNode::<K, V>::new(self.node_size));
            }
            let new_node = children[i].insert_recursive(key, value);
            if let Some(mut n) = new_node {
                let new_key = n.keys.remove(0);
                let new_value = n.values.remove(0);
                children.insert(i + 1, n);
                self.keys.insert(i, new_key);
                self.values.insert(i, new_value);
            }
        }

        if self.keys.len() == self.node_size + 1 {
            return Some(self.split());
        }

        return None;
    }
}

struct BTree<K: Ord + Clone + std::fmt::Debug, V: Ord + Clone + std::fmt::Debug> {
    root: Box<BTreeNode<K, V>>,
}

impl<K: Ord + Clone + std::fmt::Debug, V: Ord + Clone + std::fmt::Debug> BTree<K, V> {
    fn new(node_size: usize) -> BTree<K, V> {
        return BTree::<K, V> {
            root: Box::new(BTreeNode::<K, V>::new(node_size)),
        };
    }

    fn lookup(&self, key: &K) -> Option<V> {
        return self.root.lookup(key);
    }

    fn insert(&mut self, key: K, value: V) {
        let mut overflow = if let Some(n) = self.root.insert_recursive(key, value) {
            n
        } else {
            return;
        };

        let newroot = Box::new(BTreeNode::<K, V>::new(self.root.node_size));
        let overflow_key = overflow.keys.remove(0);
        let overflow_value = overflow.values.remove(0);
        let old = std::mem::replace(&mut self.root, newroot);
        self.root.children.push(*old);

        self.root.keys.push(overflow_key);
        assert!(self.root.keys.len() == 1);
        self.root.values.push(overflow_value);
        assert!(self.root.values.len() == 1);

        self.root.children.push(overflow);
        assert!(self.root.children.len() == 2);
    }
}

fn debug_lookup(b: &BTree<i32, String>, key: i32) {
    let val = b.lookup(&key);
    if let Some(val) = val {
        println!("{}: {}", key, val);
    } else {
        println!("{}: Not found", key);
    }
}

fn main() {
    let b = &mut BTree::<i32, String>::new(2);
    let mut data = vec![
        (1, String::from("foo")),
        (3, String::from("abc")),
        (6, String::from("blub")),
        (8, String::from("bar")),
        (9, String::from("abe")),
        (10, String::from("bif")),
    ];
    data.reverse();
    for (i, (key, value)) in data.iter().enumerate() {
        println!("Inserting {} = {}.", key, value);
        b.insert(key.clone(), value.clone());
        for (key, _) in data[..i + 1].iter() {
            assert!(b.lookup(key).is_some());
        }
    }

    println!("\nDONE INSERTING. VALIDATING.\n");

    data.reverse();
    for (key, _) in data.iter() {
        assert!(b.lookup(key).is_some());
    }

    b.root.display(0);

    println!("\nVALIDATED. NOW FOR NOT FOUND.\n");

    // Not found
    debug_lookup(b, 0);
    debug_lookup(b, 2);
    debug_lookup(b, 7);
    debug_lookup(b, 100);
}
