(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $manga_list_request (type 0) (param i32 i32) (result i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    i32.const 12
    i32.add)
  (memory (;0;) 16)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "manga_list_request" (func $manga_list_request))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
