#[allow(dead_code)]

// struct Node(i32, Node);
struct Node<'a> {
    data: i32,
    next: &'a mut Node<'a>,
    // next: Option<&'a mut Node<'a>>,
}

impl Node<'_> {

    fn new(d : i32) -> Self{
        Node { data: d, next: None, }
    }

    fn print_node(&self) {
        print!("{} ", self.data);
    }
    fn print_node_with_next(&self) {
        print!(
            "present node : {} and next node : {}",
            self.data, self.next.data
        );
    }
}

fn main() {
    println!("Singly Linked List in the Rust Programming language");
    let &mut head: Node = 
}
