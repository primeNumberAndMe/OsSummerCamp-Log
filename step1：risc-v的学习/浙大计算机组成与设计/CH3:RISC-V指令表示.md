### CH3：RISC-V指令表示

> RISC-V是定长指令的ISA，均为32bits，RV32/RV64/RV128也是这样。**这章介绍汇编指令的机器码表示。**
>
> 冯诺伊曼提出了stored-program的概念，即程序(数据和指令是要存储在主存中的)，EDSAC是第一个采用这种思想的电子计算机

#### 3.1R型指令

* RISCV的指令是32bits，把32bits划分为不同的field段，每个字段都会告诉处理器一些信息。

* 按照段的不同划分格式，也按不同指令的功能，RISCV将指令分为了6种格式，分别是：

  1. R型指令：寄存器与寄存器之间的算术运算
  2. I型指令：寄存器和立即数之间的算术运算/从内存load到寄存器
  3. S型指令：从寄存器store到内存
  4. B型指令：branch分支指令，是S型指令的小变体，以前叫SB型指令
  5. U型指令：20bits upper immediate instructions (不懂什么是upper immediate)
  6. J型指令：jump指令，U型指令的小变体，以前叫UJ指令

* R指令的比特布局：

  0-6：opcode operation code部分描述了指令是什么，**是用来说明是什么类型的指令吗**

  7-11：rd destination register 目标寄存器，即接受运算结果的寄存器

  12-14：funct**3** 部分描述指令是什么 **（funct3 就是3bits）**

  15-19：rs1 source register1 源操作数1号寄存器

  20-24：rs2 source register2 源操作数2号

  25-31：funct**7** 部分描述指令是什么**（funct7 就是7个bits）**

  > opcode funct3和funct7一起共同描述指令是什么
  >
  > 对于R型指令，opcode均为0110011
  >
  > rd rs1 rs2都是5比特，可存放0-31，分别指代x0-x31号寄存器

* R型指令例子：add x18, x19, x10

  opcode: 0110011

  rd: 10010

  funct3: 000

  rs1: 10011

  rs2: 01010

  funct7： 0000000

* riscv的所有R型指令，及相关funct字段的值，**opcode均为01100111**

  | OPERATION | funct3 | funct7   |
  | --------- | ------ | -------- |
  | add       | 000    | 000 0000 |
  | sub       | 000    | 010 0000 |
  | sll       | 001    | 000 0000 |
  | slt       | 010    | 000 0000 |
  | sltu      | 011    | 000 0000 |
  | xor       | 100    | 000 0000 |
  | srl       | 101    | 000 0000 |
  | sra       | 101    | 010 0000 |
  | or        | 110    | 000 0000 |
  | and       | 111    | 000 0000 |

  
