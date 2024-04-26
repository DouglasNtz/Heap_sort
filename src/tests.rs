use super::MaxHeap::{self, *};
use super::TreeNode;
use super::{binary_tree, build_max_heap_tree, build_max_heap, heap_sort};
#[test]
fn vector_to_binary_tree() {

    let mut elements = vec![5,9,10,12,1,0,9,8,1,2,3,14];

    let tree = binary_tree(&elements);

    let result_tree = NotEmpty(Box::new(TreeNode { element: 5,
        left: NotEmpty(Box::new(TreeNode { element: 9,
            left: NotEmpty(Box::new(TreeNode { element: 12,
                left: NotEmpty(Box::new(TreeNode { element: 8,
                    left: Empty, right: Empty })), right: NotEmpty(Box::new(TreeNode { element: 1, left: Empty, right: Empty })) })),
            right: NotEmpty(Box::new(TreeNode { element: 1,
                left: NotEmpty(Box::new(TreeNode { element: 2, left: Empty, right: Empty })),
                right: NotEmpty(Box::new(TreeNode { element: 3, left: Empty, right: Empty })) })) })),
        right: NotEmpty(Box::new(TreeNode { element: 10,
            left: NotEmpty(Box::new(TreeNode { element: 0,
                left: NotEmpty(Box::new(TreeNode { element: 14,
                    left: Empty, right: Empty })), right: Empty })),
            right: NotEmpty(Box::new(TreeNode { element: 9, left: Empty, right: Empty })) })) }));

    assert_eq!(tree, result_tree);
}

#[test]
fn vector_to_max_heap_tree() {

    let mut elements = vec![5,9,10,12,1,0,9,8,1,2,3,14];

    let tree = build_max_heap_tree(&mut elements);

    let result_tree = NotEmpty(Box::new(TreeNode { element: 14,
        left: NotEmpty(Box::new(TreeNode { element: 12,
            left: NotEmpty(Box::new(TreeNode { element: 9,
                left: NotEmpty(Box::new(TreeNode { element: 8,
                    left: Empty, right: Empty })),
                right: NotEmpty(Box::new(TreeNode { element: 1,
                    left: Empty, right: Empty })) })),
            right: NotEmpty(Box::new(TreeNode { element: 3,
                left: NotEmpty(Box::new(TreeNode { element: 2,
                    left: Empty, right: Empty })),
                right: NotEmpty(Box::new(TreeNode { element: 1, left: Empty, right: Empty })) })) })),
        right: NotEmpty(Box::new(TreeNode { element: 10,
            left: NotEmpty(Box::new(TreeNode { element: 5,
                left: NotEmpty(Box::new(TreeNode { element: 0,
                    left: Empty, right: Empty })), right: Empty })),
            right: NotEmpty(Box::new(TreeNode { element: 9, left: Empty, right: Empty })) })) }));


    assert_eq!(tree, result_tree);
}

#[test]
fn test_build_max_heap() {

    let mut elements = vec![5, 9, 10, 12, 1, 0, 9, 8, 1, 2, 3, 14];

    build_max_heap(&mut elements);

    let result = vec![14,12,10,9,3,5,9,8,1,2,1,0];

    assert_eq!(elements, result);
}

#[test]
fn test_heap_sort() {

    let mut elements = vec![5, 9, 10, 12, 1, 0, 9, 8, 1, 2, 3, 14];

    heap_sort(&mut elements);

    let result = vec![0, 1, 1, 2, 3, 5, 8, 9, 9, 10, 12, 14];

    assert_eq!(elements, result);
}