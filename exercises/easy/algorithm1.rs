use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        let mut current = self.start;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return unsafe { Some(&(*node.as_ptr()).val) };
            }
            current = unsafe { (*node.as_ptr()).next };
            current_index += 1;
        }
        None
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: PartialOrd,
    {
        let mut merged_list = LinkedList::new();

        unsafe {
            let (mut a_ptr, mut b_ptr) = (list_a.start, list_b.start);

            while let (Some(mut a), Some(mut b)) = (a_ptr, b_ptr) {
                if (*a.as_ptr()).val <= (*b.as_ptr()).val {
                    merged_list.push_node(a);
                    a_ptr = (*a.as_ptr()).next;
                } else {
                    merged_list.push_node(b);
                    b_ptr = (*b.as_ptr()).next;
                }
            }

            // Append remaining nodes
            let mut remaining = if a_ptr.is_some() { a_ptr } else { b_ptr };
            while let Some(node) = remaining {
                merged_list.push_node(node);
                remaining = (*node.as_ptr()).next;
            }
        }

        // Prevent original lists from deallocating nodes
        list_a.start = None;
        list_a.end = None;
        list_b.start = None;
        list_b.end = None;

        merged_list
    }

    // Helper to take ownership of existing nodes
    unsafe fn push_node(&mut self, node: NonNull<Node<T>>) {
        let node_ptr = Some(node);
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => (*end_ptr.as_ptr()).next = node_ptr,
        }
        self.end = node_ptr;
        self.length += 1;
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(node) = current {
            unsafe {
                current = (*node.as_ptr()).next;
                let _ = Box::from_raw(node.as_ptr());
            }
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut first = true;
        while let Some(node) = current {
            unsafe {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", (*node.as_ptr()).val)?;
                first = false;
                current = (*node.as_ptr()).next;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::new();
        let mut list_b = LinkedList::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &num in &vec_a {
            list_a.add(num);
        }
        for &num in &vec_b {
            list_b.add(num);
        }

        let list_c = LinkedList::merge(list_a, list_b);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(list_c.get(i as i32), Some(&expected));
        }
    }
}