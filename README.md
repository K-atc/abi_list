`abi_list`
====

> [!NOTE]
> This is a personal tool. No assurance for backward compatibility.


**`abi_list`** generates ABI list defined by LLVM DataFlowSanitizer, and currently aims to ignore functions defined external library. 
https://clang.llvm.org/docs/DataFlowSanitizer.html#id5


How to install
----
```shell
cargo install --git https://github.com/K-atc/abi_list.git --bins
```


How to use 
----
Dumps ABI list to stdout:
```
$ abi_list /usr/lib/x86_64-linux-gnu/libharfbuzz.so
fun:FT_Get_Advance=uninstrumented
fun:FT_Get_Advance=discard
fun:mprotect=uninstrumented
fun:mprotect=discard
fun:FT_Set_Transform=uninstrumented
fun:FT_Set_Transform=discard
fun:gr_slot_gid=uninstrumented
fun:gr_slot_gid=discard
fun:FT_Load_Sfnt_Table=uninstrumented
fun:FT_Load_Sfnt_Table=discard
[...]
```
