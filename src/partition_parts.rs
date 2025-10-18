/// Iterator over partitions of an integer with `PARTS` parts
#[repr(transparent)]
pub struct PartitionsParts {
    inner: Option<Vec<u8>>,
}

impl PartitionsParts {
    pub fn new(n_parts: usize, n: usize) -> Self {
        let inner = if n < n_parts || n_parts * 9 < n {
            None
        } else if n_parts == 0 {
            Some(Vec::new())
        } else {
            let mut parts = vec![1u8; n_parts];
            let mut r_idx = n_parts - 1;
            let mut sum = n_parts;
            loop {
                let rem = n - sum;
                if rem >= 8 {
                    parts[r_idx] = 9;
                    sum += 8;
                    if r_idx == 0 {
                        assert_eq!(sum, n);
                        break;
                    } else {
                        r_idx -= 1;
                    }
                } else {
                    parts[r_idx] = rem as u8 + 1;
                    break;
                }
            }
            Some(parts)
        };
        Self { inner }
    }
}

impl Iterator for PartitionsParts {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.inner.take()?;
        let parts = res.len();
        if parts < 2 {
            return Some(res);
        }
        let mut next = res.clone();
        let mut r_idx = parts - 1;
        let mut l_idx = r_idx - 1;
        loop {
            let l_val = next[l_idx];
            if l_val == 9 {
                if l_idx == 0 {
                    break;
                } else {
                    l_idx -= 1;
                    continue;
                }
            }
            let new_l_val = l_val + 1;
            let r_sum: usize =
                next[l_idx + 1..].iter().copied().map(usize::from).sum();
            let new_r_sum = r_sum - 1;
            let r_len = (parts - 1) - l_idx;
            let Some(mut r_sum_remaining) =
                new_r_sum.checked_sub(new_l_val as usize * r_len)
            else {
                if l_idx == 0 {
                    break;
                } else {
                    l_idx -= 1;
                    continue;
                }
            };
            for value in &mut next[l_idx..] {
                *value = new_l_val;
            }
            while r_sum_remaining > 0 {
                let incr = (9 - new_l_val as usize).min(r_sum_remaining);
                r_sum_remaining -= incr;
                next[r_idx] += incr as u8;
                r_idx -= 1;
            }
            self.inner = Some(next);
            break;
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::PartitionsParts;

    #[test]
    fn test_partitions_1_part() {
        let iter_0 = PartitionsParts::new(1, 0);
        assert!(iter_0.collect::<Vec<_>>().is_empty());
        let iter_1 = PartitionsParts::new(1, 1);
        assert_eq!(iter_1.collect::<Vec<_>>(), [[1u8]]);
        let iter_2 = PartitionsParts::new(1, 2);
        assert_eq!(iter_2.collect::<Vec<_>>(), [[2u8]]);
        let iter_9 = PartitionsParts::new(1, 9);
        assert_eq!(iter_9.collect::<Vec<_>>(), [[9u8]]);
        let iter_10 = PartitionsParts::new(1, 10);
        assert!(iter_10.collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn test_partitions_2_parts() {
        let iter_0 = PartitionsParts::new(2, 0);
        assert!(iter_0.collect::<Vec<_>>().is_empty());
        let iter_1 = PartitionsParts::new(2, 1);
        assert!(iter_1.collect::<Vec<_>>().is_empty());
        let iter_2 = PartitionsParts::new(2, 2);
        assert_eq!(iter_2.collect::<Vec<_>>(), [[1, 1]]);
        let iter_3 = PartitionsParts::new(2, 3);
        assert_eq!(iter_3.collect::<Vec<_>>(), [[1, 2]]);
        let iter_4 = PartitionsParts::new(2, 4);
        assert_eq!(iter_4.collect::<Vec<_>>(), [[1, 3], [2, 2]]);
        let iter_5 = PartitionsParts::new(2, 5);
        assert_eq!(iter_5.collect::<Vec<_>>(), [[1, 4], [2, 3]]);
        let iter_9 = PartitionsParts::new(2, 9);
        assert_eq!(
            iter_9.collect::<Vec<_>>(),
            [[1, 8], [2, 7], [3, 6], [4, 5]]
        );
        let iter_10 = PartitionsParts::new(2, 10);
        assert_eq!(
            iter_10.collect::<Vec<_>>(),
            [[1, 9], [2, 8], [3, 7], [4, 6], [5, 5]]
        );
        let iter_11 = PartitionsParts::new(2, 11);
        assert_eq!(
            iter_11.collect::<Vec<_>>(),
            [[2, 9], [3, 8], [4, 7], [5, 6]]
        );
        let iter_16 = PartitionsParts::new(2, 16);
        assert_eq!(iter_16.collect::<Vec<_>>(), [[7, 9], [8, 8]]);
        let iter_17 = PartitionsParts::new(2, 17);
        assert_eq!(iter_17.collect::<Vec<_>>(), [[8, 9]]);
        let iter_18 = PartitionsParts::new(2, 18);
        assert_eq!(iter_18.collect::<Vec<_>>(), [[9, 9]]);
        let iter_19 = PartitionsParts::new(2, 19);
        assert!(iter_19.collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn test_partitions_3_parts() {
        let iter_0 = PartitionsParts::new(3, 0);
        assert!(iter_0.collect::<Vec<_>>().is_empty());
        let iter_1 = PartitionsParts::new(3, 1);
        assert!(iter_1.collect::<Vec<_>>().is_empty());
        let iter_2 = PartitionsParts::new(3, 2);
        assert!(iter_2.collect::<Vec<_>>().is_empty());
        let iter_3 = PartitionsParts::new(3, 3);
        assert_eq!(iter_3.collect::<Vec<_>>(), [[1, 1, 1]]);
        let iter_5 = PartitionsParts::new(3, 5);
        assert_eq!(iter_5.collect::<Vec<_>>(), [[1, 1, 3], [1, 2, 2]]);
        let iter_7 = PartitionsParts::new(3, 7);
        assert_eq!(
            iter_7.collect::<Vec<_>>(),
            [[1, 1, 5], [1, 2, 4], [1, 3, 3], [2, 2, 3]]
        );
        let iter_25 = PartitionsParts::new(3, 25);
        assert_eq!(iter_25.collect::<Vec<_>>(), [[7, 9, 9], [8, 8, 9]]);
        let iter_26 = PartitionsParts::new(3, 26);
        assert_eq!(iter_26.collect::<Vec<_>>(), [[8, 9, 9]]);
        let iter_27 = PartitionsParts::new(3, 27);
        assert_eq!(iter_27.collect::<Vec<_>>(), [[9, 9, 9]]);
        let iter_28 = PartitionsParts::new(3, 28);
        assert!(iter_28.collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn test_partitions_4_parts() {
        for n in 0..4 {
            let iter = PartitionsParts::new(4, n);
            assert!(iter.collect::<Vec<_>>().is_empty());
        }
        let iter_4 = PartitionsParts::new(4, 4);
        assert_eq!(iter_4.collect::<Vec<_>>(), [[1, 1, 1, 1]]);
        let iter_8 = PartitionsParts::new(4, 8);
        assert_eq!(
            iter_8.collect::<Vec<_>>(),
            [
                [1, 1, 1, 5],
                [1, 1, 2, 4],
                [1, 1, 3, 3],
                [1, 2, 2, 3],
                [2, 2, 2, 2]
            ]
        );
        let iter_10 = PartitionsParts::new(4, 10);
        assert_eq!(
            iter_10.collect::<Vec<_>>(),
            [
                [1, 1, 1, 7],
                [1, 1, 2, 6],
                [1, 1, 3, 5],
                [1, 1, 4, 4],
                [1, 2, 2, 5],
                [1, 2, 3, 4],
                [1, 3, 3, 3],
                [2, 2, 2, 4],
                [2, 2, 3, 3],
            ]
        );
        let iter_12 = PartitionsParts::new(4, 12);
        assert_eq!(
            iter_12.collect::<Vec<_>>(),
            [
                [1, 1, 1, 9],
                [1, 1, 2, 8],
                [1, 1, 3, 7],
                [1, 1, 4, 6],
                [1, 1, 5, 5],
                [1, 2, 2, 7],
                [1, 2, 3, 6],
                [1, 2, 4, 5],
                [1, 3, 3, 5],
                [1, 3, 4, 4],
                [2, 2, 2, 6],
                [2, 2, 3, 5],
                [2, 2, 4, 4],
                [2, 3, 3, 4],
                [3, 3, 3, 3]
            ]
        );
        let iter_16 = PartitionsParts::new(4, 16);
        assert_eq!(
            iter_16.collect::<Vec<_>>(),
            [
                [1, 1, 5, 9],
                [1, 1, 6, 8],
                [1, 1, 7, 7],
                [1, 2, 4, 9],
                [1, 2, 5, 8],
                [1, 2, 6, 7],
                [1, 3, 3, 9],
                [1, 3, 4, 8],
                [1, 3, 5, 7],
                [1, 3, 6, 6],
                [1, 4, 4, 7],
                [1, 4, 5, 6],
                [1, 5, 5, 5],
                [2, 2, 3, 9],
                [2, 2, 4, 8],
                [2, 2, 5, 7],
                [2, 2, 6, 6],
                [2, 3, 3, 8],
                [2, 3, 4, 7],
                [2, 3, 5, 6],
                [2, 4, 4, 6],
                [2, 4, 5, 5],
                [3, 3, 3, 7],
                [3, 3, 4, 6],
                [3, 3, 5, 5],
                [3, 4, 4, 5],
                [4, 4, 4, 4]
            ]
        );
        let iter_20 = PartitionsParts::new(4, 20);
        assert_eq!(
            iter_20.collect::<Vec<_>>(),
            [
                [1, 1, 9, 9],
                [1, 2, 8, 9],
                [1, 3, 7, 9],
                [1, 3, 8, 8],
                [1, 4, 6, 9],
                [1, 4, 7, 8],
                [1, 5, 5, 9],
                [1, 5, 6, 8],
                [1, 5, 7, 7],
                [1, 6, 6, 7],
                [2, 2, 7, 9],
                [2, 2, 8, 8],
                [2, 3, 6, 9],
                [2, 3, 7, 8],
                [2, 4, 5, 9],
                [2, 4, 6, 8],
                [2, 4, 7, 7],
                [2, 5, 5, 8],
                [2, 5, 6, 7],
                [2, 6, 6, 6],
                [3, 3, 5, 9],
                [3, 3, 6, 8],
                [3, 3, 7, 7],
                [3, 4, 4, 9],
                [3, 4, 5, 8],
                [3, 4, 6, 7],
                [3, 5, 5, 7],
                [3, 5, 6, 6],
                [4, 4, 4, 8],
                [4, 4, 5, 7],
                [4, 4, 6, 6],
                [4, 5, 5, 6],
                [5, 5, 5, 5],
            ]
        );
        let iter_32 = PartitionsParts::new(4, 32);
        assert_eq!(
            iter_32.collect::<Vec<_>>(),
            [
                [5, 9, 9, 9],
                [6, 8, 9, 9],
                [7, 7, 9, 9],
                [7, 8, 8, 9],
                [8, 8, 8, 8]
            ]
        );
        let iter_36 = PartitionsParts::new(4, 36);
        assert_eq!(iter_36.collect::<Vec<_>>(), [[9, 9, 9, 9]]);
        let iter_37 = PartitionsParts::new(4, 37);
        assert!(iter_37.collect::<Vec<_>>().is_empty());
    }
}
