macro_rules! cache {
    ($key:ty, $value:ty) => {
        mod cache {
            #[repr(C, align(64))]
            struct Header {
                ptr: *const ($key, $value),
                len: usize,
                _pad: [u64; 6],
            }

            struct Retired {
                pub(super) list: ::std::cell::UnsafeCell<::std::collections::VecDeque<(*mut Header, u64)>>,
            }

            /// Safety: single-writer invariant
            unsafe impl Sync for Header {}
            /// Safety: single-writer invariant
            unsafe impl Send for Header {}
            /// Safety: single-writer invariant
            unsafe impl Sync for Retired {}
            /// Safety: single-writer invariant
            unsafe impl Send for Retired {}

            static CURRENT: ::std::sync::atomic::AtomicPtr<Header> = ::std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
            static GENERATION: ::std::sync::atomic::AtomicU64 = ::std::sync::atomic::AtomicU64::new(0);

            static RETIRED: ::std::sync::OnceLock<Retired> = ::std::sync::OnceLock::new();

            thread_local! {
                static TLS_PTR: ::std::cell::Cell<*const ($key, $value)> = const { ::std::cell::Cell::new(std::ptr::null_mut()) };
                static TLS_LEN: ::std::cell::Cell<usize> = const { ::std::cell::Cell::new(0) };
                static TLS_GEN: ::std::cell::Cell<u64> = const { ::std::cell::Cell::new(0) };
            }

            fn retired() -> &'static Retired {
                RETIRED.get_or_init(|| Retired {
                    list: ::std::cell::UnsafeCell::new(::std::collections::VecDeque::new()),
                })
            }

            #[inline]
            pub(super) fn with_entries<F: FnOnce(&[($key, $value)])>(f: F) {
                TLS_GEN.with(|g| {
                    let generation = GENERATION.load(::std::sync::atomic::Ordering::Acquire);
                    if g.get() != generation {
                        let h = CURRENT.load(::std::sync::atomic::Ordering::Acquire);
                        if !h.is_null() {
                            unsafe {
                                TLS_PTR.set((*h).ptr);
                                TLS_LEN.set((*h).len);
                                g.set(generation);
                            }
                        }
                    }
                    let ptr = TLS_PTR.get();
                    let len = TLS_LEN.get();
                    if !ptr.is_null() {
                        unsafe { f(std::slice::from_raw_parts(ptr, len)) }
                    }
                });
            }

            pub(super) fn publish_entries(mut data: Vec<($key, $value)>) {
                if data.capacity() < data.len() * 2 {
                    data.reserve(data.len());
                }

                let ptr = data.as_ptr();
                let len = data.len();
                std::mem::forget(data);
                let header = Box::into_raw(Box::new(Header {
                    ptr,
                    len,
                    _pad: [0; 6],
                }));
                let prev = CURRENT.swap(header, ::std::sync::atomic::Ordering::Release);
                let generation = GENERATION.fetch_add(1, ::std::sync::atomic::Ordering::Release) + 1;
                if !prev.is_null() {
                    unsafe {
                        let list = &mut *retired().list.get();
                        list.push_back((prev, generation));
                    }
                }
            }
            pub(super) fn reclaim_entries() {
                let min_gen = GENERATION.load(::std::sync::atomic::Ordering::Acquire);
                unsafe {
                    let list = &mut *retired().list.get();
                    while let Some(&(hdr, generation)) = list.front() {
                        if generation + 2 < min_gen {
                            let h = Box::from_raw(hdr);
                            let _ = Vec::from_raw_parts(h.ptr as *mut u64, h.len, h.len);
                            list.pop_front();
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    };
}
pub(crate) use cache;
