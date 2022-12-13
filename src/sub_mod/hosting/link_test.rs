enum Link {
    Empty,
    More(Box<Node>),
}

impl Default for Link {
    fn default() -> Self {
        Link::Empty
    }
}

struct Node {
    data: u8,
    next: Link,
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        return List {
            head: Link::default(),
        };
    }

    pub fn push(&mut self, data: u8) {
        let new_node = Link::More(Box::new(Node {
            data,
            next: std::mem::take(&mut self.head),
        }));

        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<u8> {
        match std::mem::take(&mut self.head) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn print(&self) {
        print!("head -> ");
        let mut head = &self.head;
        loop {
            match head {
                Link::Empty => {
                    print!("None");
                    break;
                }
                Link::More(node) => {
                    print!("{} -> ", node.data);
                    head = &node.next;
                }
            }
        }

        println!();
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_link() {
        let mut z = List::new();

        z.print();

        z.push(0);
        z.push(1);
        z.push(2);

        z.print();

        println!("pop {}", z.pop().unwrap());
        println!("pop {}", z.pop().unwrap());

        z.print();
    }
}
