面向对象语言通常包含的特性：命名对象、封装、继承。

对象包含数据和行为 rust满足
封装实现细节 rust满足
作为类型系统和代码共享机制的继承 rust通过trait实现
    多态：指代所有能够适应多种数据类型的代码。
    对于继承而言，【类型】就是子类。
    
**trait对象会执行动态派发**
静态派发：rust编译器会在范型使用trait约束时执行单态化，即对范型的每种具体类型生成对应的非范型实现，这种通过单态化生成代码会执行 _静态派发_ 。
动态派发：动态派发下的编译器无法在编译过程中确定调用的是哪个方法。编译器会生成一些额外代码以便在运行时找出我们希望调用的方法。
rust执行trait时会执行动态派发。


**trait对象必须保证对象安全**  ！！！重要
如果一个trait中定义的所有方法满足如下两个规则，那么这个trait就是对象安全的：
（1）方法的返回类型不是Self（Self是别名，指向实现当前trait或方法的具体类型）
（2）方法中不包含任何范型参数
trait对象必须是对象安全的，因为rust无法在我们使用trait对象时确定实现这个trait的具体类型是什么。
由于trait对象忘了及Self的具体类型，所有编译器无法在trait方法返回Self时使用原来的具体类型。
同理，对于trait方法中的范型参数而言，会在使用时将具体类型填入范型所处位置，由于trait对象忘记类型信息，所以我们无法确定被填入范型参数处的类型是哪个。

使用面向对象的状态模式实现了博客
动作：发布  提交审核   审核
状态： 草稿    待审核    已发布
示例：cp17_oop::blog::blog_test()  内部有很多实现细节，很难懂

不采用状态模式可以很简单实现，示例：cp17_oop::blog_v2::test() 且可以把任何问题暴露在编译器。