
# Ultrapacker

Bitpack a sequence of integers more efficiently by combining groups of them together into bundles.

# Why?

While working on some vertex compression I was trying to figure out how you could use the wasted space
that was still present even with bitpacking. For example if I need a value between 0-18, I need 5 bits to
represent this, but I'm not using the values from 19-32. I figured out that combining multiple values 
allows you to save bits by changing the range. Which is kind of similar to linearizing an multidimensional 
array: x + y * 18 + z * 18 * 18 gives you a range of 0-5832, which only requires 13 bits.

Later I found this: `https://save-buffer.github.io/ultrapack.html`
Which is basically an abstraction of that concept for values that all have the same possible values.

This might also be called radix encoding or factoradic representation?

# Limitations
This doesn't currently let you use different possible values for each value in the bundle. Ideally you should
able to send something like [18, 18, 18, 59] so the values have 3 with 18 possible values and 1 with 59.

This also is currently limited by `u64::MAX_VALUE`. But unless you have a lot of very high possible values, you 
probably won't hit this limit. `u128` support might be worthwhile.
