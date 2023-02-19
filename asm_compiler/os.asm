
FNC os_coursor

LDM 1 10000 ; coursor x
LDM 2 10001

LDM 3 10002 ; coursor y
LDM 4 10003

LDM 0 65527
SBV 0 128
CCF

JPNZ 2 0 ; up arrow
DEC 4
SBC 3

DEC 0
CCF

JPNZ 2 0 ; down arrow
INC 4
ADC 3

DEC 0
CCF

JPNZ 2 0 ; left arrow
DEC 2
SBC 1

DEC 0
CCF

JPNZ 2 0 ; right arrow
INC 2
ADC 1
CCF

STM 1 10000 ; coursor x
STM 2 10001

STM 3 10002 ; coursor y
STM 4 10003

STVM 20 0
STVM 20 1

MDRWR 0 0 0 10000 0 10002 0 0 0 255

RET

FNC os_home_loop



RET


FNC main_os_loop
CLS

; background
STVM 0 0
STVM 0 1

LDM 0 65535
SBV 0 100
STM 0 3
LDM 0 65534
SBC 0
CCF
STM 0 2

LDM 0 65532
LDM 1 65533
LDV 2 2
DIV 0 1 3 2 0 1
SBV 1 190
STM 1 5
SBC 0
CCF
STM 0 4

MDRWR 65532 65534 0 0 0 0 0 0 150 255 
MDRWR 65532 2 0 0 50 0 150 150 150 255

MWRTL Y-Bot\sOS\sCopyright\s(C)2023\sTobias\sOffermann 20 0 4 30 0 255 255 255 255

LDM 0 65533
SBV 0 10
STM 0 3
LDM 0 65532 ; subtract 10 from screen x and store it at mem addres 2, 3
SBC 0
CCF
STM 0 2

MDRWL 130 0 60 0 0 2 60 0 2 0 0 150 255

LDM 0 65535
SBV 0 60
STM 0 5
LDM 0 65534 ; subtract 10 from screen y and store it at mem addres 4, 5
SBC 0
CCF
STM 0 4

MDRWL 130 0 0 4 0 2 0 4 2 0 0 150 255

MDRWL 130 0 60 0 130 0 0 4 2 0 0 150 255
MDRWL 0 2 60 0 0 2 0 4 2 0 0 150 255

;CAL os_home_loop

STVM 0 0
STVM 0 1

DRWR 120 40 0 50 0 0 150 255
WRTL Home 30 30 75 255 255 255 255

WRTL Settings 30 10 125 0 0 150 255

RET

CAL main_os_loop
JMP -3
