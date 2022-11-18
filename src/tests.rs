use std::mem; 

pub struct List {
    head: Option<Box<Node>>
}

struct Node {
    elem: i32, 
    next: Option<Box<Node>>
}

impl List {
    pub fn new() -> Self {
        List { head : None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem, 
            next: mem::replace(&mut self.head, None)
        }); 

        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => {
                return None;
            }
            Some(node) => {
                self.head = node.next;
                return Some(node.elem);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List; 
    
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves 
        assert_eq!(list.pop(), None); 
        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
