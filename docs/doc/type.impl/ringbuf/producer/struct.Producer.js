(function() {
    var type_impls = Object.fromEntries([["ringbuf",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Producer%3CT,+R%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#174-202\">source</a><a href=\"#impl-Producer%3CT,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, R: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbRef.html\" title=\"trait ringbuf::ring_buffer::RbRef\">RbRef</a>&gt; <a class=\"struct\" href=\"ringbuf/producer/struct.Producer.html\" title=\"struct ringbuf::producer::Producer\">Producer</a>&lt;T, R&gt;<div class=\"where\">where\n    R::<a class=\"associatedtype\" href=\"ringbuf/ring_buffer/trait.RbRef.html#associatedtype.Rb\" title=\"type ringbuf::ring_buffer::RbRef::Rb\">Rb</a>: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbWrite.html\" title=\"trait ringbuf::ring_buffer::RbWrite\">RbWrite</a>&lt;T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#182-201\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.push_slice\" class=\"fn\">push_slice</a>(&amp;mut self, elems: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.slice.html\">[T]</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Appends items from slice to the ring buffer.\nElements must be <a href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\"><code>Copy</code></a>.</p>\n<p>Returns count of items been appended to the ring buffer.</p>\n</div></details></div></details>",0,"ringbuf::alias::StaticProducer","ringbuf::alias::HeapProducer","ringbuf::producer::PostponedProducer"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Producer%3CT,+R%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#32-172\">source</a><a href=\"#impl-Producer%3CT,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, R: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbRef.html\" title=\"trait ringbuf::ring_buffer::RbRef\">RbRef</a>&gt; <a class=\"struct\" href=\"ringbuf/producer/struct.Producer.html\" title=\"struct ringbuf::producer::Producer\">Producer</a>&lt;T, R&gt;<div class=\"where\">where\n    R::<a class=\"associatedtype\" href=\"ringbuf/ring_buffer/trait.RbRef.html#associatedtype.Rb\" title=\"type ringbuf::ring_buffer::RbRef::Rb\">Rb</a>: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbWrite.html\" title=\"trait ringbuf::ring_buffer::RbWrite\">RbWrite</a>&lt;T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#41-46\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.new\" class=\"fn\">new</a>(target: R) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates producer from the ring buffer reference.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p>There must be only one producer containing the same ring buffer reference.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.rb\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#50-52\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.rb\" class=\"fn\">rb</a>(&amp;self) -&gt; &amp;R::<a class=\"associatedtype\" href=\"ringbuf/ring_buffer/trait.RbRef.html#associatedtype.Rb\" title=\"type ringbuf::ring_buffer::RbRef::Rb\">Rb</a></h4></section></summary><div class=\"docblock\"><p>Returns reference to the underlying ring buffer.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_rb_ref\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#55-57\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.into_rb_ref\" class=\"fn\">into_rb_ref</a>(self) -&gt; R</h4></section></summary><div class=\"docblock\"><p>Consumes <code>self</code> and returns underlying ring buffer reference.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.postponed\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#60-62\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.postponed\" class=\"fn\">postponed</a>(&amp;mut self) -&gt; <a class=\"type\" href=\"ringbuf/producer/type.PostponedProducer.html\" title=\"type ringbuf::producer::PostponedProducer\">PostponedProducer</a>&lt;T, &amp;R::<a class=\"associatedtype\" href=\"ringbuf/ring_buffer/trait.RbRef.html#associatedtype.Rb\" title=\"type ringbuf::ring_buffer::RbRef::Rb\">Rb</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Returns postponed producer that borrows <a href=\"ringbuf/producer/struct.Producer.html\" title=\"struct ringbuf::producer::Producer\"><code>Self</code></a>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_postponed\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#65-67\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.into_postponed\" class=\"fn\">into_postponed</a>(self) -&gt; <a class=\"type\" href=\"ringbuf/producer/type.PostponedProducer.html\" title=\"type ringbuf::producer::PostponedProducer\">PostponedProducer</a>&lt;T, R&gt;</h4></section></summary><div class=\"docblock\"><p>Transforms <a href=\"ringbuf/producer/struct.Producer.html\" title=\"struct ringbuf::producer::Producer\"><code>Self</code></a> into postponed producer.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.capacity\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#73-75\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.capacity\" class=\"fn\">capacity</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Returns capacity of the ring buffer.</p>\n<p>The capacity of the buffer is constant.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_empty\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#79-81\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.is_empty\" class=\"fn\">is_empty</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Checks if the ring buffer is empty.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_full\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#87-89\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.is_full\" class=\"fn\">is_full</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Checks if the ring buffer is full.</p>\n<p><em>The result may become irrelevant at any time because of concurring consumer activity.</em></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.len\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#95-97\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.len\" class=\"fn\">len</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>The number of items stored in the buffer.</p>\n<p><em>Actual number may be less than the returned value because of concurring consumer activity.</em></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.free_len\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#103-105\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.free_len\" class=\"fn\">free_len</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>The number of remaining free places in the buffer.</p>\n<p><em>Actual number may be greater than the returning value because of concurring consumer activity.</em></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.free_space_as_slices\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#118-122\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.free_space_as_slices\" class=\"fn\">free_space_as_slices</a>(\n    &amp;mut self,\n) -&gt; (&amp;mut [<a class=\"union\" href=\"https://doc.rust-lang.org/1.83.0/core/mem/maybe_uninit/union.MaybeUninit.html\" title=\"union core::mem::maybe_uninit::MaybeUninit\">MaybeUninit</a>&lt;T&gt;], &amp;mut [<a class=\"union\" href=\"https://doc.rust-lang.org/1.83.0/core/mem/maybe_uninit/union.MaybeUninit.html\" title=\"union core::mem::maybe_uninit::MaybeUninit\">MaybeUninit</a>&lt;T&gt;])</h4></section></summary><div class=\"docblock\"><p>Provides a direct access to the ring buffer vacant memory.\nReturns a pair of slices of uninitialized memory, the second one may be empty.</p>\n<h5 id=\"safety-1\"><a class=\"doc-anchor\" href=\"#safety-1\">§</a>Safety</h5>\n<p>Vacant memory is uninitialized. Initialized items must be put starting from the beginning of first slice.\nWhen first slice is fully filled then items must be put to the beginning of the second slice.</p>\n<p><em>This method must be followed by <code>Self::advance</code> call with the number of items being put previously as argument.</em>\n<em>No other mutating calls allowed before that.</em></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.advance\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#130-132\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.advance\" class=\"fn\">advance</a>(&amp;mut self, count: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a>)</h4></section></summary><div class=\"docblock\"><p>Moves <code>tail</code> counter by <code>count</code> places.</p>\n<h5 id=\"safety-2\"><a class=\"doc-anchor\" href=\"#safety-2\">§</a>Safety</h5>\n<p>First <code>count</code> items in free space must be initialized.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#137-150\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.push\" class=\"fn\">push</a>(&amp;mut self, elem: T) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.unit.html\">()</a>, T&gt;</h4></section></summary><div class=\"docblock\"><p>Appends an item to the ring buffer.</p>\n<p>On failure returns an <code>Err</code> containing the item that hasn’t been appended.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_iter\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#159-171\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.push_iter\" class=\"fn\">push_iter</a>&lt;I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = T&gt;&gt;(&amp;mut self, iter: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.reference.html\">&amp;mut I</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Appends items from an iterator to the ring buffer.\nElements that haven’t been added to the ring buffer remain in the iterator.</p>\n<p>Returns count of items been appended to the ring buffer.</p>\n<p><em>Inserted items are committed to the ring buffer all at once in the end,</em>\n<em>e.g. when buffer is full or iterator has ended.</em></p>\n</div></details></div></details>",0,"ringbuf::alias::StaticProducer","ringbuf::alias::HeapProducer","ringbuf::producer::PostponedProducer"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Producer%3Cu8,+R%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#239-265\">source</a><a href=\"#impl-Producer%3Cu8,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbRef.html\" title=\"trait ringbuf::ring_buffer::RbRef\">RbRef</a>&gt; <a class=\"struct\" href=\"ringbuf/producer/struct.Producer.html\" title=\"struct ringbuf::producer::Producer\">Producer</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.u8.html\">u8</a>, R&gt;<div class=\"where\">where\n    R::<a class=\"associatedtype\" href=\"ringbuf/ring_buffer/trait.RbRef.html#associatedtype.Rb\" title=\"type ringbuf::ring_buffer::RbRef::Rb\">Rb</a>: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbWrite.html\" title=\"trait ringbuf::ring_buffer::RbWrite\">RbWrite</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.u8.html\">u8</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.read_from\" class=\"method\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#251-264\">source</a><h4 class=\"code-header\">pub fn <a href=\"ringbuf/producer/struct.Producer.html#tymethod.read_from\" class=\"fn\">read_from</a>&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt;(\n    &amp;mut self,\n    reader: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.reference.html\">&amp;mut P</a>,\n    count: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a>&gt;,\n) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.83.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Reads at most <code>count</code> bytes from <code>Read</code> instance and appends them to the ring buffer.\nIf <code>count</code> is <code>None</code> then as much as possible bytes will be read.</p>\n<p>Returns <code>Ok(n)</code> if <code>read</code> succeeded. <code>n</code> is number of bytes been read.\n<code>n == 0</code> means that either <code>read</code> returned zero or ring buffer is full.</p>\n<p>If <code>read</code> is failed then original error is returned. In this case it is guaranteed that no items was read from the reader.\nTo achieve this we read only one contiguous slice at once. So this call may read less than <code>remaining</code> items in the buffer even if the reader is ready to provide more.</p>\n</div></details></div></details>",0,"ringbuf::alias::StaticProducer","ringbuf::alias::HeapProducer","ringbuf::producer::PostponedProducer"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Write-for-Producer%3Cu8,+R%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#268-284\">source</a><a href=\"#impl-Write-for-Producer%3Cu8,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbRef.html\" title=\"trait ringbuf::ring_buffer::RbRef\">RbRef</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"ringbuf/producer/struct.Producer.html\" title=\"struct ringbuf::producer::Producer\">Producer</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.u8.html\">u8</a>, R&gt;<div class=\"where\">where\n    R::<a class=\"associatedtype\" href=\"ringbuf/ring_buffer/trait.RbRef.html#associatedtype.Rb\" title=\"type ringbuf::ring_buffer::RbRef::Rb\">Rb</a>: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbWrite.html\" title=\"trait ringbuf::ring_buffer::RbWrite\">RbWrite</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.u8.html\">u8</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.write\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#272-279\">source</a><a href=\"#method.write\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#tymethod.write\" class=\"fn\">write</a>(&amp;mut self, buffer: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.83.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class='docblock'>Writes a buffer into this writer, returning how many bytes were written. <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#tymethod.write\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.flush\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#281-283\">source</a><a href=\"#method.flush\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#tymethod.flush\" class=\"fn\">flush</a>(&amp;mut self) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.83.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.unit.html\">()</a>&gt;</h4></section></summary><div class='docblock'>Flushes this output stream, ensuring that all intermediately buffered\ncontents reach their destination. <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#tymethod.flush\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_vectored\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.36.0\">1.36.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/std/io/mod.rs.html#1626\">source</a></span><a href=\"#method.write_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_vectored\" class=\"fn\">write_vectored</a>(&amp;mut self, bufs: &amp;[<a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/std/io/struct.IoSlice.html\" title=\"struct std::io::IoSlice\">IoSlice</a>&lt;'_&gt;]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.usize.html\">usize</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Like <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#tymethod.write\" title=\"method std::io::Write::write\"><code>write</code></a>, except that it writes from a slice of buffers. <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_write_vectored\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.83.0/src/std/io/mod.rs.html#1641\">source</a><a href=\"#method.is_write_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.is_write_vectored\" class=\"fn\">is_write_vectored</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.bool.html\">bool</a></h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>can_vector</code>)</span></div></span><div class='docblock'>Determines if this <code>Write</code>r has an efficient <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_vectored\" title=\"method std::io::Write::write_vectored\"><code>write_vectored</code></a>\nimplementation. <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.is_write_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_all\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/std/io/mod.rs.html#1703\">source</a></span><a href=\"#method.write_all\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_all\" class=\"fn\">write_all</a>(&amp;mut self, buf: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Attempts to write an entire buffer into this writer. <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_all\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_all_vectored\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.83.0/src/std/io/mod.rs.html#1765\">source</a><a href=\"#method.write_all_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_all_vectored\" class=\"fn\">write_all_vectored</a>(&amp;mut self, bufs: &amp;mut [<a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/std/io/struct.IoSlice.html\" title=\"struct std::io::IoSlice\">IoSlice</a>&lt;'_&gt;]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>write_all_vectored</code>)</span></div></span><div class='docblock'>Attempts to write multiple buffers into this writer. <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_all_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_fmt\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/std/io/mod.rs.html#1818\">source</a></span><a href=\"#method.write_fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_fmt\" class=\"fn\">write_fmt</a>(&amp;mut self, fmt: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/struct.Arguments.html\" title=\"struct core::fmt::Arguments\">Arguments</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Writes a formatted string into this writer, returning any error\nencountered. <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.write_fmt\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.by_ref\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/std/io/mod.rs.html#1878-1880\">source</a></span><a href=\"#method.by_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.by_ref\" class=\"fn\">by_ref</a>(&amp;mut self) -&gt; &amp;mut Self<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Creates a “by reference” adapter for this instance of <code>Write</code>. <a href=\"https://doc.rust-lang.org/1.83.0/std/io/trait.Write.html#method.by_ref\">Read more</a></div></details></div></details>","Write","ringbuf::alias::StaticProducer","ringbuf::alias::HeapProducer","ringbuf::producer::PostponedProducer"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Write-for-Producer%3Cu8,+R%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#286-298\">source</a><a href=\"#impl-Write-for-Producer%3Cu8,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbRef.html\" title=\"trait ringbuf::ring_buffer::RbRef\">RbRef</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Write.html\" title=\"trait core::fmt::Write\">Write</a> for <a class=\"struct\" href=\"ringbuf/producer/struct.Producer.html\" title=\"struct ringbuf::producer::Producer\">Producer</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.u8.html\">u8</a>, R&gt;<div class=\"where\">where\n    R::<a class=\"associatedtype\" href=\"ringbuf/ring_buffer/trait.RbRef.html#associatedtype.Rb\" title=\"type ringbuf::ring_buffer::RbRef::Rb\">Rb</a>: <a class=\"trait\" href=\"ringbuf/ring_buffer/trait.RbWrite.html\" title=\"trait ringbuf::ring_buffer::RbWrite\">RbWrite</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.u8.html\">u8</a>&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_str\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ringbuf/producer.rs.html#290-297\">source</a><a href=\"#method.write_str\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Write.html#tymethod.write_str\" class=\"fn\">write_str</a>(&amp;mut self, s: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.str.html\">str</a>) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Writes a string slice into this writer, returning whether the write\nsucceeded. <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Write.html#tymethod.write_str\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_char\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.1.0\">1.1.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/core/fmt/mod.rs.html#174\">source</a></span><a href=\"#method.write_char\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Write.html#method.write_char\" class=\"fn\">write_char</a>(&amp;mut self, c: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.char.html\">char</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Writes a <a href=\"https://doc.rust-lang.org/1.83.0/std/primitive.char.html\" title=\"primitive char\"><code>char</code></a> into this writer, returning whether the write succeeded. <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Write.html#method.write_char\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_fmt\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/core/fmt/mod.rs.html#202\">source</a></span><a href=\"#method.write_fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Write.html#method.write_fmt\" class=\"fn\">write_fmt</a>(&amp;mut self, args: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/struct.Arguments.html\" title=\"struct core::fmt::Arguments\">Arguments</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Glue for usage of the <a href=\"https://doc.rust-lang.org/1.83.0/core/macro.write.html\" title=\"macro core::write\"><code>write!</code></a> macro with implementors of this trait. <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Write.html#method.write_fmt\">Read more</a></div></details></div></details>","Write","ringbuf::alias::StaticProducer","ringbuf::alias::HeapProducer","ringbuf::producer::PostponedProducer"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[32722]}