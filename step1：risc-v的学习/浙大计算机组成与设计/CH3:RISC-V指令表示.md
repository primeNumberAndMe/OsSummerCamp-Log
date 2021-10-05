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

  

#### 3.2I型指令和S型指令

##### 1. I型指令之和立即数有关的指令

> 相较于R型指令，不需要在两个寄存器之间运算只需一个寄存器和一个立即数，所以我们的rs3字段就不需要了，但假如只使用5bits有符号数来表示立即数，那仅可以表示[-16, 15]，太小了。所以我们再去掉funct7这个字段，7+5共12个比特有符号用来表示立即数，则可以表示[-2048, 2047]，若立即数所需位数超过12bits，后续再说明怎样去表示。**12bits的立即数放到寄存器中需使用符号扩展**

* I型指令的opcode是001 0011
* 在I型指令中，区别是什么具体指令，**仅看funct3字段** **(例外： shift right logical和shift right arithmetic看的是immediate的前7bits)**

* I型指令立即数型举例：addi x15, x1, -50

  opcode: 001 0011

  rd: 01110

  funct3: 000

  rs1: 00001

  immed: 111111 001110 (-50的12位补码表示)

* 立即数运算I型指令：

  | operation | funct3(3bits) | immediate(12bits)                               |
  | --------- | ------------- | ----------------------------------------------- |
  | addi      | 000           | 12位有符号立即数                                |
  | slti      | 010           | 12位有符号立即数                                |
  | sltiu     | 011           | 12位有符号立即数                                |
  | xori      | 100           | 12位有符号立即数                                |
  | ori       | 110           | 12位有符号立即数                                |
  | andi      | 111           | 12位有符号立即数                                |
  | slli      | 001           | 高7位(25-31)：0000000 低5位：shift amount移位量 |
  | srli      | **101**       | 高7位：0000000 低5位：shift amount移位量        |
  | srai      | **101**       | 高7位：0100000  低5位：shift amount移位量       |

##### 2. I型指令之load指令

> I型load指令和R型指令或立即数相关的I型指令不同，不需要3个寄存器，不需要2个寄存器，仅需要1个寄存器，也就是rd字段保留，rs1和rs2通通去掉。我们需要一个基址和偏移量，所以rs1用作基址，12位的immediate用作偏移量。**基址只是用rs1指代，真正的基址是rs1寄存器里的地址**

* I型load指令举例：

  lw x14, 8(x2)

  opcode: 001 0011

  rd: 01110

  funct3: 010 代表lw

  rs1(base)：00010

  immediate：000000001000 

* load型I型指令：

  | operation                         | funct3 | immediate        |
  | --------------------------------- | ------ | ---------------- |
  | lb (load byte sign-extension)     | 000    | 12位有符号偏移量 |
  | lh(load halfword sign-extension)  | 010    | 12位有符号偏移量 |
  | lw                                | 011    | 12位有符号偏移量 |
  | lbu(load byte zero-extension)     | 100    | 12位有符号偏移量 |
  | lhu(load halfword zero-extension) | 110    | 12位有符号偏移量 |

  

##### 3. S型指令

> S型指令仅需要一个源寄存器，一个base，一个offset，由于要和load对应，所以我们的offset也要是12bits，然后s1在I型load指令中用作base，所以S型沿用。但我们又还需要一个源寄存器，所以rs2不用做offset而用作源寄存器，funct7仍用作offset，但又不够12bits阿，所以rd字段取代了rs2作为offset的低5位，在S型指令表示中，offset这12个比特被分成了funct7+rd两段。

* S型指令举例

  sw x14, 8(x2)

  opcode:  0100011

  rd: 01000 低5位offset

  funct3: 010 代表sw

  rs1: 00010 base

  rs2: 01110

  funct7: 000 0000 高7位offset

* S型指令：

  | funct7       | rs2             | rs1  | funct3 | rd          | opcode  | operatino |
  | ------------ | --------------- | ---- | ------ | ----------- | ------- | --------- |
  | offset[11:5] | source register | base | 000    | offset[4:0] | 0100011 | sb        |
  | offset[11:5] | source register | base | 001    | offset[4:0] | 0100011 | sh        |
  | offset[11:5] | source register | base | 010    | offset[4:0] | 0100011 | sw        |

  

