#![feature(trait_alias)]

pub trait PriorityValue = Ord + PartialOrd + Eq;

struct PriorityItem<R, T: PriorityValue> {
  pub item: R,
  pub priority: T,
}

pub enum PriorityMode {
  MinimizeHead,
  MaximizeHead
}

pub struct PriorityQueue<R, T: PriorityValue> {
  data: Vec<PriorityItem<R, T>>,
  mode: PriorityMode,
}

impl <R, T: PriorityValue>PriorityQueue<R, T> {

  /** Construct a new priority queue with size_hint preallocated space */
  pub fn new(size_hint: usize, mode: PriorityMode) -> Self {
    Self {
      data: Vec::with_capacity(size_hint),
      mode: mode,
    }
  }

  /** The parent of a node is at index - 1 / 2, we're relying on integer division flooring */
  fn parent(&self, idx: usize) -> usize {
    (idx - 1) / 2
  }

  /** The child of a node is at (idx * 2) + 1 */
  fn children(&self, idx: usize) -> (usize, usize) {
    ((idx * 2) + 1, (idx * 2) + 2)
  }

  /** Returns the best child (if there are any) to swap when restoring the heap property for a parent */
  fn best_child(&self, idx: usize) -> Option<usize> {

    // Find the children of this node
    let (child1, child2) = self.children(idx);

    // Get the length of the data array
    let len = self.data.len();

    // Since child2 > child1, we know that if child1 doesn't exist then child2 doesn't exist
    if len <= child1 {
      // If there are no children then return none
      None
    } else if len <= child2 {
      // If there is only child1 then take child1
      Some(child1)
    } else {
      // If child1 would violate the heap property as the parent then take child2, otherwise take child 1
      if self.violates_heap_property(child1, child2) {
        Some(child2)
      } else {
        Some(child1)
      }
    }
  }

  /** Returns true if the current values for parent and child violate the heap property */
  fn violates_heap_property(&self, parent: usize, child: usize) -> bool {
    match &self.mode {
      PriorityMode::MinimizeHead => self.data[parent].priority > self.data[child].priority,
      PriorityMode::MaximizeHead => self.data[parent].priority < self.data[child].priority,
    }
  }

  /** Insert a new item into the priority queue */
  pub fn insert(&mut self, item: R, priority: T) {
    // First insert at the end
    self.data.push(PriorityItem { item, priority });

    // Next work up the tree swapping this value with it's parent if it violates the heap property
    let mut idx = self.data.len() - 1;

    while idx != 0 {
      let parent = self.parent(idx);
      if self.violates_heap_property(parent, idx) {
        // This parent child relationship violates the heap property
        // swap them and then make sure that the heap property is not violated at depth - 1 by repeating
        self.data.swap(parent, idx);
        idx = parent;
      } else {
        break; // We are done since the heap property is maintained
      }
    }
  }

  /** Take the highest priority item from the priority queue */
  pub fn take(&mut self) -> Option<R> {

    // Take the heap length
    let heap_len = self.data.len();

    // If the heap is empty then return nothing
    if heap_len == 0 {
      return None;
    }

    // First swap the item we want to remove with the last item in the heap
    self.data.swap(0, heap_len - 1);

    // Next, remove the item we want from the heap by popping
    let result_value = self.data.pop();

    // Now starting from the first node in the tree work down until the heap property is restored
    // by swapping any parent that violates the heap property with one of it's children
    let mut idx = 0;

    while idx < heap_len {

      // Find the best candidate for swapping (in a minimize heap the highest value child, in a maximize heap the lowest value child)
      if let Some(child) = self.best_child(idx) {

        // If it violates the heap property then swap it out
        if self.violates_heap_property(idx, child) {
          self.data.swap(idx, child);
          idx = child;
        } else {
          break;
        }

      } else {
        break;
      }
    }

    // Finally return our removed heap item
    result_value.map(|x| x.item)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::random;

  #[test]
  fn empty_queue_gives_none() {
    let mut queue = PriorityQueue::<usize, usize>::new(100, PriorityMode::MinimizeHead);
    assert!(queue.take().is_none());
  }

  #[test]
  fn simple_queue_maximize() {
    let mut queue = PriorityQueue::new(100, PriorityMode::MaximizeHead);
    queue.insert(1, 10);
    queue.insert(2, 20);
    queue.insert(3, 30);
    assert_eq!(queue.take().unwrap(), 3);
    assert_eq!(queue.take().unwrap(), 2);
    assert_eq!(queue.take().unwrap(), 1);
  }


  #[test]
  fn simple_queue_minimize() {
    let mut queue = PriorityQueue::new(100, PriorityMode::MinimizeHead);
    queue.insert(1, 10);
    queue.insert(2, 20);
    queue.insert(3, 30);
    assert_eq!(queue.take().unwrap(), 1);
    assert_eq!(queue.take().unwrap(), 2);
    assert_eq!(queue.take().unwrap(), 3);
  }

  #[test]
  fn large_queue_maximize() {
    let mut queue = PriorityQueue::new(100, PriorityMode::MaximizeHead);
    let size = 100000;

    for i in 0..size {
      queue.insert(i, i);
    }

    for i in 0..size {
      assert_eq!(queue.take().unwrap(), size - i - 1);
    }
  }

  #[test]
  fn large_queue_minimize() {
    let mut queue = PriorityQueue::new(100, PriorityMode::MinimizeHead);
    let size = 100000;

    for i in 0..size {
      queue.insert(i, i);
    }

    for i in 0..size {
      assert_eq!(queue.take().unwrap(), i);
    }
  }

  #[test]
  fn test_random_minimize() {
    let mut queue = PriorityQueue::new(100, PriorityMode::MinimizeHead);
    let size = 100000;

    for _ in 0..size {
      let rval: usize = random();
      queue.insert(rval, rval);
    }

    let mut head = queue.take().unwrap();

    for _ in 0..(size-1) {
      let nval = queue.take().unwrap();
      assert!(head <= nval);
      head = nval;
    }
  }

  #[test]
  fn test_random_maximize() {
    let mut queue = PriorityQueue::new(100, PriorityMode::MaximizeHead);
    let size = 100000;

    for _ in 0..size {
      let rval: usize = random();
      queue.insert(rval, rval);
    }

    let mut head = queue.take().unwrap();

    for _ in 0..(size-1) {
      let nval = queue.take().unwrap();
      assert!(head >= nval);
      head = nval;
    }
  }
}
