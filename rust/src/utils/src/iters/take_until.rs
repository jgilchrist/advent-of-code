// The following implementation is taken from the 'take_until' crate.
// Once take_until is available in the standard library, this can be removed:
// https://github.com/rust-lang/libs-team/issues/142

// Copyright 2019 Hannes De Valkeneer

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// `TakeUntilExt` is an extension trait for iterators.
/// It adds the `take_until` method.
pub trait TakeUntilExt<P>
where
    Self: Sized,
{
    fn take_until(self, predicate: P) -> TakeUntil<Self, P>;
}

impl<I, P> TakeUntilExt<P> for I
where
    I: Sized + Iterator,
    P: FnMut(&I::Item) -> bool,
{
    fn take_until(self, predicate: P) -> TakeUntil<Self, P> {
        TakeUntil {
            iter: self,
            flag: false,
            predicate,
        }
    }
}

/// `TakeUntil` is similar to the `TakeWhile` iterator,
/// but takes items until the predicate is true,
/// including the item that made the predicate true.
pub struct TakeUntil<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> Iterator for TakeUntil<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            self.iter.next().map(|x| {
                if (self.predicate)(&x) {
                    self.flag = true;
                }
                x
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.flag {
            (0, Some(0))
        } else {
            let (_, upper) = self.iter.size_hint();
            (0, upper) // can't know a lower bound, due to the predicate
        }
    }
}
