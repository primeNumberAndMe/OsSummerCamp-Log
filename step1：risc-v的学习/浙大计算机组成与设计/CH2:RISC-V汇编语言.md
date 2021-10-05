#### 2.1算术指令

* risc-v有32个寄存器，分别叫做x0-x31，RV32中每个寄存器都为32bits，RV64中每个寄存器都为64bits。其中x0永远存储0值，为什么单独拿一个寄存器放0值，是因为设计者发现0用的次数很多。
  0在这里也是一个立即数，所有试图对x0寄存器写的指令都是无效的。

* 加法汇编： add x1, x2, x3 => x1 = x2 + x3

* 减法汇编： sub  x1, x2, x3 =>  x1 = x2 - x3

* riscv汇编指令的格式： one two three four

  one是操作名，two是得到操作结果的寄存器destination，three和four是2个source。

  > 这种指令格式是比较死板的，为什么要这样设计，是为了使其硬件设计更加简单。

* riscv汇编的注释使用#

* 立即数(immediate)：即出现在汇编指令中的无需寻址的数值常量

* riscv对于包含立即数的指令：

  * add: addi x1, x2, 10 => x1 = x2+10 

  * sub: riscv没有对立即数减法操作的指令，因为其可以通过加法完成

    x1 = x2 - 10 => addi x1, x2, -10

#### 2.2访存指令

> 讲解寄存器和内存的数据交换。汇编指令例子以64bit计算机为例

1. 以机器字长为单位的数据交换（lw: load word|sw: store word）

   * lw x1, 8(x15): x15是内存基址，8是偏移量，也就是从x15+8开始，读取8个字节到x1寄存器中
   * sw x1, 8(x15): 将x1寄存器的8个字节放到x15+8开始的8个字节中

2. 以单字节长为单位的数据交换(lb|sw|lbu)

   * lb x1, 8(x15): 将x15+8这个字节放到数据表示的低地址中，并在前面7个字节进行**符号扩展**
   * sb x1, 8(x15): 将x1寄存器数据表示的低地址字节放到x15+8字节中
   * lbu: 用法和lb一样，但扩展时**不是进行符号扩展**，而是**0扩展**，即前面7字节补0

   > riscv并没有sbu的指令，因为save byte to mem并不会发生扩展，就是存放到内存的那个字节中

   > riscv这个访问内存采用基址+偏移量的机制中，偏移量可以是负数。



#### 2.3条件判断分支转移指令

> 条件判断分支转移指令用来实现和高级语言中if语句相类似的效果。

* beq指令：beq register1, register2, Label1

  意思是判断寄存器1和寄存器2中的值是否相同，如果相同则跳转到Label1处

  beq指代的是branch equal

* bne指令：bne register1, register2, Label1

  判断寄存器1和寄存器2中的值是否相同，若不同，则跳转到Label1处

  而bne指的是branch not equal

  

* 条件判断转移指令和无条件转移指令：

  * 条件判断转移指令: 指的是这条指令会在跳转前进行条件判断，若判断成立，才跳转，如上面提到的beq，bne以及blt，bgt，bltu
  * 无条件转移指令: 直接跳转无条件判断，如j(jump)指令

* j指令：j label 直接跳转到label

* jr指令：jr register 直接跳转到寄存器register中的地址执行

* blt指令： blt register1, register2, Label1

  将寄存器中的值看待为**有符号数**，并判断寄存器1的值是否少于寄存器2中的值，若是，则跳转label1

  blt即branch less than

* bge 指令：bge register1, register2, Label1

  将寄存器中的值看作**有符号数**，若寄存器1的值大于或等于寄存器2的值，则跳转label1

* bltu和bgeu指令：和blt/bge用法相同，不过将寄存器中的值看待为**无符号数**



#### 2.4逻辑运算指令

> bit-wise的逻辑运算。

> RISC-V中的逻辑运算包含：与，或，异或，左移，右移。并没有非，因为非可以通过和FFFFFFFFFFFFFFFF进行异或得到。

* 与/and：and register, register1, register2

  将寄存器1/2逐bit进行比特与运算，并将结果放到寄存器中

  等价于高级语言 register=register1 & register2

  > 带立即数版本：andi register, register1, immediate
  >
  > 将立即数看作有机器字长的有符号数进行与操作(存疑，溢出的话是直接截断吗)

  > and操作常用来取其中的某个字节，比如取其最高字节，和0xff00000000000000进行and操作

* 或/or： or egister, register1, register2

  > 带立即数版本：ori register, register1, immediate

* 左移：

  * 逻辑左移：sll(shift left logical)

* 右移：

  * 逻辑右移：srl 左边补0 (shift right logical)
  * 算术右移：sra左边补符号位 (shift right arithmetic)

  > 没看到课上提供sll/srl/sra的例子，值给了立即数版本的例子，slli/srli/srai
  >
  > slli x1， x2，10 将x2的值逻辑左移10位，赋给x1寄存器。需要注意，immediate可以给不同进制的数，而移动的位数以立即数十进制的结果进行计算。



#### 2.5函数调用

> 讲和函数调用相关的一些汇编指令。

* symbolic register names: a0-a7指x10-x17寄存器，用来存储函数参数和返回值。a0-a1还可以放返回值

* (伪指令)汇编语法糖：

  * 将一个寄存器赋给另一个寄存器，以前我们这么写 ```add x1, x2, x0```，现在我们有 ```mv x1 x2``` 

  * 将一个立即数赋给一个寄存器，以前 ```addi x1, x0, immediate```，现在 ```li x1 immediate```

    li即load immediate

* riscv是定长指令集架构，其PC指针每次执行完一条指令+4/+8，遇到跳转除外。

* ra寄存器(return address)存放函数执行完的返回值，其为x1寄存器

* jr指令：jr register跳转到某个寄存器的地址，常用在函数返回时，ra寄存器存放了函数的返回地址，所以函数执行完 ```jr ra``` 。

* jal指令(jump and link)：也是一种语法糖吧，``` jal sum``` 的操作是将词条指令的地址+4/+8赋给ra寄存器，然后``` j sum``` 。比如 ```jal sum``` 这条指令的地址是1000，32bit的机器，ra的值就被赋为1004，然后跳转sum。

  > ppt中写到，按jal的语义，它应该改名叫link and jump即laj



#### 2.6栈的使用

> 讲和函数调用相关的一些栈知识

* 函数的参数需要存放到a0到a7寄存器，这些寄存器中可能有一些老旧的值，需要先将这些老旧的值给保存下来，等寄存器中的函数参数使用完成，再恢复旧值。而这些老旧的值就是要放到栈中。所以在函数调用前，每次栈指针sp都要减少一些来存放一些值，这就是为什么递归(非尾递归)会一直增大栈，使其overflow。

* 栈帧(stack frame)：栈在函数调用会增大来保存调用函数返回地址(这个马上要运行的函数叫做被调用函数)，参数及局部变量，它每次增大都是增一定大小，也就是一块内存，我们管这一块内存叫做栈帧。

  所以栈里存放的就是一个接一个的栈帧。

* caller/callee：调用函数和被调用函数

* 当callee执行完毕回到caller时，caller需要知道哪些寄存器可能会被更改，那些寄存器一定未被更改

* riscv将寄存器分为两类：

  1. 一定不会被更改的，caller可以相信的：sp，gp，tp，saved registers s0-s11，fp就是s0

     > sp的值不会变的，因为存帧又去帧，减小又增大。

  2. 可能会被更改的：函数参数和返回值寄存器a0-a7，返回地址寄存器ra，临时寄存器t0-t6

  > 上面给的寄存器的名字都是在汇编里面写的对人类友好的符号名，而不是x0-x31物理硬件寄存器名

* riscv的符号寄存器名：

  | Register | ABI name | description                                                  | Saver(没懂这一栏的意思) |
  | -------- | -------- | ------------------------------------------------------------ | ----------------------- |
  | x0       | zero     | hard-wired zero(hard-wired指的是用电路硬编码的)              | -                       |
  | x1       | ra       | return address                                               | caller                  |
  | x2       | sp       | stack pointer                                                | caller                  |
  | x3       | gp       | global pointer                                               | -                       |
  | x4       | tp       | thread pointer                                               | -                       |
  | x5       | t0       | temporaty/alternate link register                            | caller                  |
  | x6-x7    | t1-t2    | temporaries                                                  | caller                  |
  | x8       | s0/fp    | saved register/frame pointer                                 | callee                  |
  | x9       | s1       | saved register                                               | callee                  |
  | x10-x11  | a0-a1    | function arguments/return values(这两个寄存器何时是参数，何时是返回值) | caller                  |
  | x12-x17  | a2-a7    | function arguments                                           | caller                  |
  | x18-x27  | s2-s11   | saved registers                                              | callee                  |
  | x28-x31  | t3-t6    | temporaries                                                  | caller                  |

* RV32/RV64/RV128有不同的memory layout

