#[allow(dead_code)]

// struct Node(i32, Node);
struct Node<'a> {
    data: i32,
    next_node: &'a mut Node<'a>,
    // next: Option<&'a mut Node<'a>>,
}

impl Node<'_> {
    fn print_node(&self) {
        print!("{} ", self.data);
    }
    fn print_node_with_next(&self) {
        print!(
            "present node : {} and next node : {}",
            self.data, self.next_node.data
        );
    }
}

// struct CircularLinkedList {
//     tail: Node<'a>,
//     node_count: u32,
// }

// impl CircularLinkedList {
//     fn print_list(&self) {
//         print!("{} ", self.tail);
//         //
//    }
// }

fn main() {
    println!("Circular Linked List in the Rust Programming language");
}
