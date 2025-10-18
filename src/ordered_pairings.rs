use arrayvec::ArrayVec;
use lender::{Lend, Lender, Lending};

struct Inner {
    acc: ArrayVec<u8, 10>,
    ys: ArrayVec<u8, 9>,
}

/// Iterate over all possible sorted pairings in lexicographic order
pub struct OrderedPairings {
    inner: Vec<Inner>,
    /// Buffer used to store results.
    /// The first element of the pairs are xs. The second elements are
    /// the ys of the previous iterator value, or zeros upon initialization.
    res_buf: ArrayVec<(u8, u8), 10>,
}

impl OrderedPairings {
    /// Both inputs must be sorted and have equal length
    pub fn new(xs: &[u8], ys: &[u8]) -> Self {
        let n_ys = ys.len();
        let inner = if n_ys == 0 {
            Vec::new()
        } else {
            (0..n_ys)
                .rev()
                .filter_map(|idx| {
                    let y = ys[idx];
                    if idx != 0 && ys[idx - 1] == y {
                        return None;
                    }
                    let mut inner_ys = ArrayVec::new();
                    #[allow(clippy::needless_range_loop)]
                    for ys_idx in 0..n_ys {
                        if ys_idx != idx {
                            inner_ys.push(ys[ys_idx]);
                        }
                    }
                    let mut acc = ArrayVec::new();
                    acc.push(y);
                    Some(Inner { acc, ys: inner_ys })
                })
                .collect()
        };
        let res_buf = xs.iter().map(|x| (*x, 0)).collect();
        Self { inner, res_buf }
    }
}

impl<'lend> Lending<'lend> for OrderedPairings {
    type Lend = &'lend [(u8, u8)];
}

impl Lender for OrderedPairings {
    fn next(&mut self) -> Option<Lend<'_, Self>> {
        loop {
            let inner = self.inner.pop()?;
            let n_inner_ys = inner.ys.len();
            if n_inner_ys == 0 {
                for (idx, y) in inner.acc.into_iter().enumerate() {
                    self.res_buf[idx].1 = y;
                }
                return Some(&self.res_buf);
            } else {
                let acc_len = inner.acc.len();
                if acc_len != 0
                    && self.res_buf[acc_len - 1].0 == self.res_buf[acc_len].0
                {
                    'inner: for idx in (0..n_inner_ys).rev() {
                        let y = inner.ys[idx];
                        if idx != 0 && inner.ys[idx - 1] == y {
                            continue 'inner;
                        }
                        if y >= inner.acc[acc_len - 1] {
                            let mut ys = ArrayVec::new();
                            for ys_idx in 0..n_inner_ys {
                                if ys_idx != idx {
                                    ys.push(inner.ys[ys_idx]);
                                }
                            }
                            let mut acc = inner.acc.clone();
                            acc.push(y);
                            let inner = Inner { acc, ys };
                            self.inner.push(inner);
                        }
                    }
                } else {
                    'inner: for idx in (0..n_inner_ys).rev() {
                        let y = inner.ys[idx];
                        if idx != 0 && inner.ys[idx - 1] == y {
                            continue 'inner;
                        }
                        let mut ys = ArrayVec::new();
                        for ys_idx in 0..n_inner_ys {
                            if ys_idx != idx {
                                ys.push(inner.ys[ys_idx]);
                            }
                        }
                        let mut acc = inner.acc.clone();
                        acc.push(y);
                        let inner = Inner { acc, ys };
                        self.inner.push(inner);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use lender::Lender;

    use super::OrderedPairings;

    #[test]
    fn test_ordered_pairings() {
        let iter = OrderedPairings::new(&[1, 1, 2], &[3, 3, 4]);
        let mut res = Vec::new();
        iter.for_each(|pairing| res.push(pairing.to_owned()));
        assert_eq!(res, [[(1, 3), (1, 3), (2, 4)], [(1, 3), (1, 4), (2, 3)]]);
        let iter = OrderedPairings::new(&[1, 2, 2, 3], &[4, 5, 5, 6]);
        res.clear();
        iter.for_each(|pairing| res.push(pairing.to_owned()));
        assert_eq!(
            res,
            [
                [(1, 4), (2, 5), (2, 5), (3, 6)],
                [(1, 4), (2, 5), (2, 6), (3, 5)],
                [(1, 5), (2, 4), (2, 5), (3, 6)],
                [(1, 5), (2, 4), (2, 6), (3, 5)],
                [(1, 5), (2, 5), (2, 6), (3, 4)],
                [(1, 6), (2, 4), (2, 5), (3, 5)],
                [(1, 6), (2, 5), (2, 5), (3, 4)],
            ]
        );
    }
}
