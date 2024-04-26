#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
struct TreeNode<T: PartialOrd + Copy> {
    element: T,
    left: MaxHeap<T>,
    right: MaxHeap<T>
}

#[derive(Debug, PartialEq)]
pub enum MaxHeap<T: PartialOrd + Copy> {
    Empty,
    NotEmpty(Box<TreeNode<T>>)
}

use MaxHeap::*;

pub fn binary_tree<T: PartialOrd + Copy>(elements: &[T]) -> MaxHeap<T> {

    let mut nodes = Vec::new();

    for element in elements {
        nodes.push(Some(TreeNode { element: *element, left: Empty, right: Empty }));
    }

    let mut indices = 2..=nodes.len();

    for i in indices.rev() {

        let children = nodes[i - 1].take().unwrap();

        let parent = nodes[i/2 - 1].as_mut().unwrap();

        if i % 2 == 0 {
            parent.left = NotEmpty(Box::new(children));
        } else {
            parent.right = NotEmpty(Box::new(children));
        }
    }

    let root = nodes[0].take().unwrap();

    NotEmpty(Box::new(root))
}

fn max_heapify<T: PartialOrd + Copy>(i: usize, elements: &mut [T]) {

    let parent = elements.get(i - 1).unwrap();

    let left_children = elements.get(2*i - 1);

    let rigth_children = elements.get(2*i);

    match (left_children, rigth_children) {
        (None, None) => {
            return;
        }
        (Some(left), None) => {
            if parent >= left {
                return;
            } else {
                elements.swap(i - 1, 2*i - 1);
                max_heapify(2*i, elements);
            }
        }
        (Some(left), Some(right)) => {
            if parent >= left && parent >= right {
                return;
            } else {
                if left >= right {
                    elements.swap(i - 1, 2*i - 1);
                    max_heapify(2*i, elements);
                } else {
                    elements.swap(i - 1, 2*i);
                    max_heapify(2*i + 1, elements)
                }
            }
        }
        _ => {panic!("Isso nunca vai acontecer!!!");}
    }
}
pub fn build_max_heap_tree<T: PartialOrd + Copy>(elements: &mut [T]) -> MaxHeap<T> {

    let mut indices = 1..=(elements.len()/2);

    for i in indices.rev() {

        max_heapify(i, elements)

    }

    binary_tree(elements)
}

pub fn build_max_heap<T: PartialOrd + Copy>(elements: &mut [T]) {

    let mut indices = 1..=(elements.len()/2);

    for i in indices.rev() {

        max_heapify(i, elements)
    }
}

pub fn heap_sort<T: PartialOrd + Copy>(elements: &mut [T]) {

    let mut last = elements.len() - 1;

    build_max_heap(elements);

    loop {

        elements.swap(0, last);

        last -= 1;

        if last == 0 {
            break;
        }

        max_heapify(1, &mut elements[..=last]);

    }
}