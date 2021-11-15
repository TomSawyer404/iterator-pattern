# Iterator 模式

迭代器设计模式（Iterator Pattern）用于顺序访问集合对象的元素，不需要知道及和对象的底层表示。在[《设计模式》](https://zh.wikipedia.org/wiki/设计模式：可复用面向对象软件的基础)建议合理的接口该要有：

```java
public interface Iterator
{
    public Object First();
    public Object Next();
    public boolean isDone();
    public Object CurrentItem();
}
```

而在 Rust 中， 语言自身已经提供了一个迭代器接口：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

迭代器有**外部迭代器（External Iterator）**和**内部迭代器（Internal Iterator）**两种。

外部迭代器独立于容器之外，通过容器提供的方法（比如，`next`方法就是所谓的游标）来迭代下一个元素，并需要考虑容器内可迭代的剩余数量来进行迭代。**外部迭代器的一个重要特点是，外部可以控制整个迭代进程**。Cpp 语言中的迭代器就是外部迭代器。

内部迭代器则通过迭代器自身来控制迭代下一个元素，外部无法干预。这意味着，只要调用了内部迭代器，并通过闭包传入了相关操作，就必须等待迭代器依次为其中的每个元素执行玩相关操作以后才可以停止遍历。Ruby 语言中的`each`迭代器就是典型的内部迭代器。

出于性能及各方面考虑，当今的迭代器以<u>外部迭代器</u>为主。

# 实战

假设你是西南地区的销售经理，在该地区我们有几家商店（集合）。

![shop](https://th.bing.com/th/id/OIP.V_1pI4vWBd0DyPFiFKUJuwEVDg?w=209&h=180&c=7&r=0&o=5&pid=1.7)

现在你希望了解各个商店的所有商品的价值总量，即你要遍历商店的商品价格（集合里的元素），但又不需要完全知道该商品的细节。对于你而已，你希望做的操作仅仅如下：

```rust
let mut store_iterator = store_chunxilu.iterator();
let mut total_price = 0;
while store_iterator.has_next() {
    let good = store_iterator.next();
    total_price += good.get_price();
}
```

我们定义一个`MyTrait`接口，并专门设置一个`StoreIterator`结构体去实现它：

```rust
trait MyIterator {
    type Object;
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Self::Object;
}

impl StoreIterator {
    fn new(store: Store) -> Self {
        StoreIterator { store, index: 0 }
    }
}

impl MyIterator for StoreIterator {
    type Object = Good;

    fn has_next(&self) -> bool {
        ...
    }

    fn next(&mut self) -> Self::Object {
       ...
    }
}
```

这样就能达到我们的目的了。 

# 参考资料

- [迭代器模式-Wikipedia](https://zh.wikipedia.org/zh-cn/%E8%BF%AD%E4%BB%A3%E5%99%A8%E6%A8%A1%E5%BC%8F)
- [lpxxn的示例代码](https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/iterator.rs)
- [yukihir0的示例代码](https://github.com/yukihir0/rust_design_pattern/blob/master/iterator/src/main.rs)
- [Rust 编程之道 6.3 节](http://product.dangdang.com/26475568.html)

---
