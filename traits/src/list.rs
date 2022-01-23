use std::{
    collections::LinkedList,
    ops::{Deref, DerefMut, Index},
};
struct List<T>(LinkedList<T>);

impl<T> Deref for List<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Index<isize> for List<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let len = self.len() as isize;
        // 我们需要先对负数，以及 index 超出范围的数字进行处理
        // 使其落在 0..len 之间
        let n = (len + index % len) % len;
        let iter = self.iter();
        // 由于 n 一定小于 len 所以这里可以 skip n 取 next
        // 此时一定有值，所以可以放心 unwrap
        iter.skip(n as usize).next().unwrap()

        //////////////////////////////////////////////
        // let len = self.len();
        // //标准库的checked_rem_euclid方法, 如果len=0 则返回None
        // //这里直接把i进行unwrap, 如果链表长度不为0, 则i一定在0..len范围内, 可以放心使用,
        // //如果长度为零, 意味这对一个空链表进行索引, 那么我panic应该也是合情合理的吧
        // let i = (index as usize).checked_rem_euclid(len).unwrap();
        // &self.iter().nth(i).unwrap()
    }
}

#[test]
fn it_works() {
    let mut list: List<u32> = List::default();
    for i in 0..16 {
        list.push_back(i);
    }

    assert_eq!(list[0], 0);
    assert_eq!(list[5], 5);
    assert_eq!(list[15], 15);
    assert_eq!(list[16], 0);
    assert_eq!(list[-1], 15);
    assert_eq!(list[128], 0);
    assert_eq!(list[-128], 0);
}

fn main() {}
