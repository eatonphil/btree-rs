struct BTree<K: Ord + Clone + std::fmt::Debug + std::fmt::Display, V: Ord + Clone + std::fmt::Debug + std::fmt::Display> {
    node_size: usize,
    keys: Vec<K>,
    children: Vec<BTree<K, V>>,
    values: Vec<V>,
}

impl<K: Ord + Clone + std::fmt::Debug + std::fmt::Display, V: Ord + Clone + std::fmt::Debug + std::fmt::Display> BTree<K, V> {
    fn new(node_size: usize) -> BTree<K, V> {
	return BTree::<K, V>{
	    node_size: node_size,
	    keys: Vec::<K>::with_capacity(node_size),
	    children: Vec::<BTree<K, V>>::with_capacity(node_size + 1),
	    values: Vec::<V>::with_capacity(node_size + 1),
	};
    }

    fn find_index(keys: &Vec<K>, key: &K) -> i32 {
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

    fn find_node_path(&self, key: &K) -> Vec<usize> {
	let mut stack = Vec::<usize>::new();
	let mut curnode = self;
	loop {
	    let i = BTree::<K, V>::find_index(&curnode.keys, key);
	    if i < 0 {
		stack.push(-(i+1) as usize);
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
	let mut path = self.find_node_path(key);
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

    // fn insert_maybe_split(node: &BTree<K, V>, key: &K, value: V) -> Option<(K, V)> {
    // 	if node.keys.len() == node.node_size {
    // 	    // Find center.
    // 	    return Some((key, value));
    // 	}

    // 	return None;
    // }

    // fn insert(&mut self, key: K, value: V) {
    // 	let mut node_path = self.find_node(&key);
    // 	let mut currentkey = &key;
    // 	let mut currentval = value;
    // 	let mut leftover = true;
    // 	loop {
    // 	    let node = if let Some(n) = node_path.pop() { n } else { break; };
    // 	    let maybe_split_kv = BTree::insert_maybe_split(node, currentkey, currentval);
    // 	    if let Some((splitkey, splitval)) = maybe_split_kv {
    // 		currentkey = splitkey;
    // 		currentval = splitval;
    // 	    } else {
    // 		leftover = false;
    // 		break;
    // 	    }
    // 	}

    // 	if leftover {
    // 	    // TODO: replace root.
    // 	}
    // }
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
    let b = &BTree::<i32, String>{
	node_size: 2,
	keys: vec![3, 8],
	values: vec![String::from("abc"), String::from("def")],
	children: vec![
	    BTree::<i32, String>{
		node_size: 2,
		keys: vec![1],
		values: vec![String::from("123")],
		children: vec![],
	    },
	    BTree::<i32, String>{
		node_size: 2,
		keys: vec![6],
		values: vec![String::from("bar")],
		children: vec![],
	    },
	    BTree::<i32, String>{
		node_size: 2,
		keys: vec![9, 10],
		values: vec![String::from("foo"), String::from("blub")],
		children: vec![],
	    },
	],
    };

    debug_lookup(b, 1);
    debug_lookup(b, 3);
    debug_lookup(b, 6);
    debug_lookup(b, 8);
    debug_lookup(b, 9);
    debug_lookup(b, 10);

    // Not found
    debug_lookup(b, 0);
    debug_lookup(b, 2);
    debug_lookup(b, 7);
    debug_lookup(b, 100);
}
