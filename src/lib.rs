use std::mem::MaybeUninit;

enum WindowState<T: Iterator, const N: usize> {
    Full([T::Item;N]),
    Partial(Vec<T::Item>)
}
struct WindowIterator<T: Iterator, const N: usize> {
    
    iter: T,
    // window: Option<[T::Item;N]>,
    window: WindowState<T, N>
}

impl<I: Clone, T: Iterator<Item = I>, const N: usize> Iterator for WindowIterator<T,N>
{
    type Item = [T::Item;N];
    fn next(&mut self) -> Option<Self::Item> {
        match self.window {
            WindowState::<T, N>::Full(ref mut window) => {
                let mut new_item = self.iter.next()?;
                
                for i in (0..N).rev() {
                    std::mem::swap(&mut new_item, &mut window[i]);
                }
                Some(window.clone())
            },
            WindowState::<T, N>::Partial(ref mut window) => {
                while window.len() != N {
                    window.push(self.iter.next()?);
                }
                unsafe {
                    let mut arr: [MaybeUninit<T::Item>;N] = MaybeUninit::uninit().assume_init();
                    for i in (0..N).rev() {
                        arr[i].write(window.pop().unwrap());
                    }
                    let arr: [T::Item;N] = (&arr as *const _ as *const [T::Item; N]).read();
                    self.window = WindowState::Full(arr.clone());
                    Some(arr)
                }
            }
        }
    }
}

trait IteratorExt: Iterator {
    fn window<const N: usize>(self) -> WindowIterator<Self, N> where Self: Sized;
}

impl<I: Clone, T: Iterator<Item = I>> IteratorExt for T {
    fn window<const N: usize>(self) -> WindowIterator<Self, N> {
        WindowIterator {iter: self, window: WindowState::Partial(Vec::with_capacity(N))}  
    }
}
