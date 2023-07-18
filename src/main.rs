#![warn(unused_variables)]
#![warn(dead_code)]

////--------------------------------------------------------
use std::ops::Add;

pub struct Complex {
    rel: f64,
    im: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Self) -> Self {
        Self {
            rel: self.rel + other.rel,
            im: self.im + other.im,
        }
    }
}
////--------------------------------------------------------
pub struct Vep {
    stack: [u8; 10000000000000000000],
    index: usize,
}

pub struct Iter<'a> {
    borrow: &'a [u8; 10000000000000000000],
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.borrow.len() - 1 {
            return None;
        }
        let t = &self.borrow[self.index];
        self.index += 1;
        Some(t)
    }
}

impl Vep {
    pub fn new() -> Self {
        Vep {
            stack: [0; 10000000000000000000],
            index: 0,
        }
    }
    pub fn pop(&mut self) -> Option<u8> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;
        let i = self.index;
        return Some(self.stack[i]);
    }

    pub fn push(&mut self, item: u8) {
        self.index += 1;
        self.stack[self.index] = item;
    }
    pub fn iter(&self) -> Iter<'_> {
        return Iter {
            borrow: &self.stack,
            index: 0,
        };
    }
}
////--------------------------------------------------------
pub struct SingleLinkedList<T: Copy> {
    start: Box<SigleLinkedNode<T>>,
    //size: u8, //end: &mut SigleLinkedNode<T>,
}
struct SigleLinkedNode<T: Copy> {
    value: T,
    node: Option<Box<SigleLinkedNode<T>>>,
}

impl<T: Copy> SingleLinkedList<T> {
    pub fn read(&self, index: u8) -> Option<T> {
        return self.start.as_ref().read(index - 1);
    }
    pub fn add(&mut self, value: T) {
        self.start.as_mut().append(value);
    }
}

impl<T: Copy> SigleLinkedNode<T> {
    pub fn append(&mut self, value: T) {
        // match &mut self.node {
        //     None => {
        //         let new_node = Box::new(SigleLinkedNode {
        //             value: value,
        //             node: None,
        //         });
        //         self.node = Some(new_node);
        //     }
        //     Some(s) => s.append(value),
        // }
        if self.node.is_none() {
            let new_node = Box::new(SigleLinkedNode {
                value: value,
                node: None,
            });
            self.node = Some(new_node);
        } else {
            self.node.as_mut().unwrap().append(value);
        }
    }
    pub fn read(&self, index: u8) -> Option<T> {
        if index == 0 {
            return Some(self.value);
        }
        return self.node.as_ref().unwrap().read(index - 1);
    }
}
////--------------------------------------------------------
#[derive(Debug)]
pub enum BNode<T> {
    Empty,
    NOTempty {
        value: T,
        left: Box<BNode<T>>,
        right: Box<BNode<T>>,
    },
}

impl<'a, T: PartialOrd> BNode<T> {
    pub fn get_left_most(&self) -> &BNode<T> {
        match self {
            BNode::Empty => &self,
            BNode::NOTempty {
                value: _,
                left,
                right: _,
            } => match left.as_ref() {
                BNode::Empty => &self,
                BNode::NOTempty {
                    value: _,
                    left,
                    right: _,
                } => left.as_ref().get_left_most(),
            },
        }
    }

    pub fn insert(&'a mut self, val: T) {
        match self {
            BNode::NOTempty { value, left, right } => {
                if *value > val {
                    // *left = Box::new(BNode::NOTempty {
                    //     value: val,
                    //     left: Box::new(BNode::Empty),
                    //     right: Box::new(BNode::Empty),
                    // });
                    left.insert(val)
                } else {
                    // *right = Box::new(BNode::NOTempty {
                    //     value: val,
                    //     left: Box::new(BNode::Empty),
                    //     right: Box::new(BNode::Empty),
                    // })
                    right.insert(val)
                }
            }
            BNode::Empty => {
                *self = BNode::NOTempty {
                    value: val,
                    left: Box::new(BNode::Empty),
                    right: Box::new(BNode::Empty),
                }
            }
        }
    }
}

////--------------------------------------------------------

pub struct IntoIter<T> {
    arr: Vec<T>,
    index: usize,
}

impl<T: PartialOrd + Copy> std::iter::Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // match self.index {
        //     BNode::Empty => todo!(),
        //     BNode::NOTempty { value, left, right } => todo!(),
        // }
        if self.index == self.arr.len() {
            return None;
        }
        let t = self.arr[self.index];
        self.index += 1;
        Some(t)
        //     }
        // }
        // if (n.rightChild != NULL)
        //     RETURN getLeftMost(n.rightChild)
        //  ELSE
        // WHILE (n.parent != NULL AND n == n.parent.rightChild)
        //     n = n.parent;
        //     RETURN n.parent;
    }
}

impl<'a, T: Copy> BNode<T> {
    fn get_elements(&self) -> Vec<T> {
        match self {
            BNode::Empty => Vec::new(),
            BNode::NOTempty { value, left, right } => {
                let mut v = vec![*value];
                match left.as_ref() {
                    BNode::Empty => (),
                    BNode::NOTempty {
                        value: _,
                        left: _,
                        right: _,
                    } => v.append(&mut left.as_ref().get_elements()),
                }
                match right.as_ref() {
                    BNode::Empty => (),
                    BNode::NOTempty {
                        value: _,
                        left: _,
                        right: _,
                    } => v.append(&mut right.as_ref().get_elements()),
                }

                v
            }
        }
    }

    pub fn into_iter(&self) -> IntoIter<T> {
        let b = self.get_elements();
        IntoIter { index: 0, arr: b }
    }
}

////--------------------------------------------------------
fn main() {
    let mut x = BNode::NOTempty {
        value: 1,
        left: Box::new(BNode::Empty),
        right: Box::new(BNode::Empty),
    };
    x.insert(3);
    x.insert(5);
    x.insert(8);
    x.insert(6);
    x.insert(7);
    x.insert(2);
    println!("{x:#?}")
    // for val in x.into_iter() {
    //     println!("{val}");
    // }
}

#[test]
fn test_push() {
    let mut v = Vep::new();
    for i in 0..12 {
        v.push(i);
    }
}

#[test]
fn test_iter() {
    let mut v = Vep::new();
    for i in 0..12 {
        v.push(i);
    }
    for elt in v.iter() {
        println!("{elt:?}");
    }
}

#[test]
fn test_singlelinkedlist() {
    let mut list = SingleLinkedList {
        start: Box::new(SigleLinkedNode {
            value: "fas",
            node: None,
        }),
    };
    list.add("string");
    list.add("gafx");
    println!("{:?}", list.read(2).unwrap());
}
