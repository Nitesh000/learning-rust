#### Arrays

---

- Arrays have fixed length that can store items of the same type. They deonted by [T, N], where `T` is the type and the `N` is the number of array elements.

#### Tuples

---

- Tuples differ from arrays in the way that elements of an array have to be of the same type,
  while items in a tuple can be a mix of types. They are heterogeneous collections and are
  useful for storing distinct types together. They can also be used when returning multiple
  values from a function.

#### Vectors

---

- Vectors are like arrays, except that their content or length doesn't need to be known in
  advance and can grow on demand. They are allocated on the heap. They can be created by
  either calling the Vec::new constructor or by using the vec![] macro

#### Hashmaps

---

- Rust also provides us with maps, which can be used to store key-value data. They come
  from the std::collections module and are named HashMap. They are created with the
  HashMap::new constructor function
