/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: PartialOrd>(array: &mut [T]) {
    if array.len() <= 1 {
        return; // 如果数组长度小于等于1，直接返回
    }

    let pivot_index = partition(array); // 获取基准元素的索引
    let (left, right) = array.split_at_mut(pivot_index); // 将数组分为两部分
    sort(left); // 对左半部分递归排序
    sort(&mut right[1..]); // 对右半部分递归排序（跳过基准元素）
}

fn partition<T: PartialOrd>(array: &mut [T]) -> usize {
    let pivot_index = array.len() - 1; // 选择最后一个元素作为基准
    let mut i = 0;

    for j in 0..pivot_index {
        if array[j] <= array[pivot_index] {
            array.swap(i, j); // 将小于等于基准的元素交换到左侧
            i += 1;
        }
    }

    array.swap(i, pivot_index); // 将基准元素放到正确的位置
    i // 返回基准元素的索引
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}