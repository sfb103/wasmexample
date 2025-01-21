(module $withdraw.wasm
  (type (;0;) (func (param i32)))
  (type (;1;) (func))
  (type (;2;) (func (param f32 f32) (result f32)))
  (type (;3;) (func (result i32)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (type 0)))
  (func $__wasm_call_ctors (type 1))
  (func $_start (type 1)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        global.get $GOT.data.internal.__memory_base
        i32.const 1024
        i32.add
        i32.load
        br_if 0 (;@2;)
        global.get $GOT.data.internal.__memory_base
        i32.const 1024
        i32.add
        i32.const 1
        i32.store
        call $__wasm_call_ctors
        call $__original_main
        local.set 0
        call $__wasm_call_dtors
        local.get 0
        br_if 1 (;@1;)
        return
      end
      unreachable
    end
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $withdraw (type 2) (param f32 f32) (result f32)
    (local i32 i32 i32 f32 f32 i32 i32 i32 f32 f32 f32 f32 f32)
    global.get $__stack_pointer
    local.set 2
    i32.const 16
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    local.get 0
    f32.store offset=8
    local.get 4
    local.get 1
    f32.store offset=4
    local.get 4
    f32.load offset=4
    local.set 5
    local.get 4
    f32.load offset=8
    local.set 6
    local.get 5
    local.get 6
    f32.lt
    local.set 7
    i32.const 1
    local.set 8
    local.get 7
    local.get 8
    i32.and
    local.set 9
    block  ;; label = @1
      block  ;; label = @2
        local.get 9
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        f32.load offset=8
        local.set 10
        local.get 4
        f32.load offset=4
        local.set 11
        local.get 10
        local.get 11
        f32.sub
        local.set 12
        local.get 4
        local.get 12
        f32.store offset=12
        br 1 (;@1;)
      end
      local.get 4
      f32.load offset=8
      local.set 13
      local.get 4
      local.get 13
      f32.store offset=12
    end
    local.get 4
    f32.load offset=12
    local.set 14
    local.get 14
    return)
  (func $__original_main (type 3) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 0
    i32.const 16
    local.set 1
    local.get 0
    local.get 1
    i32.sub
    local.set 2
    i32.const 0
    local.set 3
    local.get 2
    local.get 3
    i32.store offset=12
    i32.const 0
    local.set 4
    local.get 4
    return)
  (func $__wasi_proc_exit (type 0) (param i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_proc_exit
    unreachable)
  (func $dummy (type 1))
  (func $__wasm_call_dtors (type 1)
    call $dummy
    call $dummy)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 2)
  (global $__stack_pointer (mut i32) (i32.const 66576))
  (global $GOT.data.internal.__memory_base i32 (i32.const 0))
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "withdraw" (func $withdraw)))
