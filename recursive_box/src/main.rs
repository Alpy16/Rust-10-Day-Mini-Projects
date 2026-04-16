struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn add(&mut self, value: i32) {
        let old_head = self.head.take();
        let new_node = Box::new(Node {
            next: old_head,
            value: value, //apparently in rust you can just do, "value," instead of assigning it manually
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(old_node) => {
                self.head = old_node.next;

                Some(old_node.value)
            }
            None => None,
        }
    }
}
// I found it makes kinda more sense if you model it like a bank-desk/counter with a number sign, except the handoff part its a perfect analogy
fn main() {
    // 1. Open the bank (Create the manager)
    let mut bank_line = LinkedList::new();

    // 2. Customers arrive (The line-jumpers)
    println!("Adding customers 10, 20, and 30...");
    bank_line.add(10);
    bank_line.add(20);
    bank_line.add(30);

    // 3. The Teller starts calling numbers
    // We expect 30 to come out first because they jumped to the front!
    println!("First out: {:?}", bank_line.pop()); // Should be Some(30)
    println!("Second out: {:?}", bank_line.pop()); // Should be Some(20)

    // 4. Add one more late-comer
    bank_line.add(40);
    println!("Third out: {:?}", bank_line.pop()); // Should be Some(40)
    println!("Fourth out: {:?}", bank_line.pop()); // Should be Some(10)

    // 5. The line is now empty
    println!("Fifth out: {:?}", bank_line.pop()); // Should be None
}

// this one did take a bit of mental gymnastic ngl
