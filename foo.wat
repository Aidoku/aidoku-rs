(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (param i32) (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32) (result i64)))
  (type (;5;) (func (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32)))
  (type (;8;) (func (param i32 i32 i32)))
  (type (;9;) (func))
  (type (;10;) (func (param i32 i32 i32 i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (import "std" "typeof" (func $_ZN14aidoku_imports3std10value_kind17ha3880610cd52b218E (type 1)))
  (import "std" "array_len" (func $_ZN14aidoku_imports3std9array_len17h798b55f560da32c6E (type 1)))
  (import "std" "destroy" (func $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E (type 2)))
  (import "std" "array_get" (func $_ZN14aidoku_imports3std9array_get17h3b6b59cea62c295eE (type 0)))
  (import "std" "object_get" (func $_ZN14aidoku_imports3std10object_get17hc7f0d5cb2f875986E (type 3)))
  (import "std" "read_int" (func $_ZN14aidoku_imports3std8read_int17h13f11c1613649589E (type 4)))
  (import "std" "create_array" (func $_ZN14aidoku_imports3std12create_array17h7056aec6721f851bE (type 5)))
  (import "aidoku" "create_manga" (func $_ZN6aidoku7structs12create_manga17h35470fb75f4211c0E (type 6)))
  (import "std" "array_append" (func $_ZN14aidoku_imports3std12array_append17hfa5c3707320db518E (type 7)))
  (import "aidoku" "create_manga_result" (func $_ZN6aidoku7structs19create_manga_result17hbe04b8bd9055a540E (type 0)))
  (import "std" "string_len" (func $_ZN14aidoku_imports3std10string_len17h6acc294070917a02E (type 1)))
  (import "std" "read_string" (func $_ZN14aidoku_imports3std11read_string17h3c1f102f7e811cc2E (type 8)))
  (func $get_manga_list (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i64.const 0
    i64.store offset=4 align=4
    local.get 2
    i32.const 0
    i32.load offset=1048644
    local.tee 3
    i32.store
    i32.const 0
    local.set 4
    i32.const 0
    local.set 5
    local.get 3
    local.set 6
    block  ;; label = @1
      local.get 0
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        call $_ZN14aidoku_imports3std10value_kind17ha3880610cd52b218E
        i32.const 5
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        local.get 2
        i32.const 40
        i32.add
        local.set 7
        local.get 3
        local.set 6
        i32.const 0
        local.set 8
        loop  ;; label = @3
          block  ;; label = @4
            local.get 8
            local.get 0
            call $_ZN14aidoku_imports3std9array_len17h798b55f560da32c6E
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
            local.get 0
            call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
            local.get 2
            i32.load offset=4
            local.set 5
            br 3 (;@1;)
          end
          local.get 0
          local.get 8
          call $_ZN14aidoku_imports3std9array_get17h3b6b59cea62c295eE
          local.tee 9
          call $_ZN14aidoku_imports3std10value_kind17ha3880610cd52b218E
          local.set 5
          local.get 9
          call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
          block  ;; label = @4
            local.get 5
            i32.const 6
            i32.ne
            br_if 0 (;@4;)
            local.get 9
            i32.const 1048626
            i32.const 4
            call $_ZN14aidoku_imports3std10object_get17hc7f0d5cb2f875986E
            local.tee 5
            call $_ZN14aidoku_imports3std10value_kind17ha3880610cd52b218E
            local.set 10
            local.get 5
            call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
            block  ;; label = @5
              local.get 10
              i32.const 3
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              local.set 11
              block  ;; label = @6
                local.get 9
                i32.const 1048630
                i32.const 4
                call $_ZN14aidoku_imports3std10object_get17hc7f0d5cb2f875986E
                local.tee 10
                call $_ZN14aidoku_imports3std10value_kind17ha3880610cd52b218E
                i32.const 1
                i32.ne
                br_if 0 (;@6;)
                local.get 10
                call $_ZN14aidoku_imports3std8read_int17h13f11c1613649589E
                i64.const -1
                i64.add
                local.tee 12
                i64.const 8
                i64.gt_u
                br_if 0 (;@6;)
                local.get 12
                i32.wrap_i64
                i32.const 1
                i32.add
                local.set 11
              end
              local.get 2
              i32.const 32
              i32.add
              local.get 5
              call $_ZN14aidoku_imports3std9StringRef4read17h286cc716a486f574E
              local.get 9
              i32.const 1048634
              i32.const 5
              call $_ZN14aidoku_imports3std10object_get17hc7f0d5cb2f875986E
              local.set 13
              local.get 10
              call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
              block  ;; label = @6
                local.get 4
                local.get 2
                i32.load offset=4
                i32.ne
                br_if 0 (;@6;)
                local.get 2
                local.get 4
                call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h836c765c11a45bceE
                local.get 2
                i32.load
                local.set 6
                local.get 2
                i32.load offset=8
                local.set 4
              end
              local.get 8
              i32.const 1
              i32.add
              local.set 8
              local.get 6
              local.get 4
              i32.const 20
              i32.mul
              i32.add
              local.tee 5
              local.get 11
              i32.store
              local.get 5
              local.get 2
              i64.load offset=32
              i64.store offset=4 align=4
              local.get 5
              i32.const 12
              i32.add
              local.get 7
              i32.load
              i32.store
              local.get 5
              local.get 13
              i32.store offset=16
              local.get 2
              local.get 2
              i32.load offset=8
              i32.const 1
              i32.add
              local.tee 4
              i32.store offset=8
              local.get 9
              call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
              br 2 (;@3;)
            end
            local.get 9
            call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
          end
          local.get 8
          i32.const 1
          i32.add
          local.set 8
          br 0 (;@3;)
        end
      end
      local.get 0
      call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
      i32.const 0
      local.set 4
      i32.const 0
      local.set 5
      local.get 3
      local.set 6
    end
    local.get 2
    i64.const 0
    i64.store offset=36 align=4
    local.get 2
    local.get 3
    i32.store offset=32
    local.get 2
    i32.const 32
    i32.add
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17haba155738b5aee6dE
    local.get 2
    i32.load offset=32
    local.get 2
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    i32.load
    local.tee 9
    i32.const 80
    i32.mul
    i32.add
    local.tee 8
    local.get 3
    i32.store offset=56
    local.get 8
    i32.const 1048700
    i32.store offset=48
    local.get 8
    i32.const 1048615
    i32.store offset=40
    local.get 8
    i32.const 1048700
    i32.store offset=32
    local.get 8
    i32.const 1048609
    i32.store offset=24
    local.get 8
    i32.const 1048604
    i32.store offset=16
    local.get 8
    i32.const 1048577
    i32.store offset=8
    local.get 8
    i32.const 1
    i32.store offset=4
    local.get 8
    i32.const 1048576
    i32.store
    local.get 8
    i32.const 60
    i32.add
    i64.const 0
    i64.store align=4
    local.get 8
    i32.const 52
    i32.add
    i32.const 0
    i32.store
    local.get 8
    i32.const 44
    i32.add
    i32.const 11
    i32.store
    local.get 8
    i32.const 36
    i32.add
    i32.const 0
    i32.store
    local.get 8
    i32.const 28
    i32.add
    i32.const 6
    i32.store
    local.get 8
    i32.const 20
    i32.add
    i32.const 5
    i32.store
    local.get 8
    i32.const 12
    i32.add
    i32.const 27
    i32.store
    local.get 8
    i32.const 68
    i32.add
    i64.const 0
    i64.store align=4
    local.get 8
    i32.const 76
    i32.add
    i32.const 0
    i32.store
    local.get 2
    i32.const 16
    i32.add
    i32.const 8
    i32.add
    local.get 9
    i32.const 1
    i32.add
    i32.store
    local.get 2
    local.get 2
    i64.load offset=32
    i64.store offset=16
    block  ;; label = @1
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.const 20
      i32.mul
      local.set 9
      local.get 6
      i32.const 16
      i32.add
      local.set 8
      loop  ;; label = @2
        block  ;; label = @3
          local.get 8
          i32.const -8
          i32.add
          i32.load
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const -12
          i32.add
          i32.load
          local.get 0
          call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
        end
        local.get 8
        i32.load
        call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
        local.get 8
        i32.const 20
        i32.add
        local.set 8
        local.get 9
        i32.const -20
        i32.add
        local.tee 9
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.const 20
      i32.mul
      local.tee 8
      i32.eqz
      br_if 0 (;@1;)
      local.get 6
      local.get 8
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
    end
    local.get 2
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 2
    local.get 2
    i64.load offset=16
    i64.store offset=32
    local.get 2
    i32.const 0
    i32.store8 offset=44
    local.get 2
    i32.const 32
    i32.add
    call $_ZN6aidoku7structs15MangaPageResult6create17h7a62178f9d7d9cb7E
    local.set 8
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 8)
  (func $_ZN14aidoku_imports3std9StringRef4read17h286cc716a486f574E (type 7) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call $_ZN14aidoku_imports3std10string_len17h6acc294070917a02E
        local.tee 2
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        local.get 2
        br_if 1 (;@1;)
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        i32.const 1
        i32.store
        local.get 1
        call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
        return
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h10b26fa3afebc0c9E
      unreachable
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 1
        call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h864bff3da949edfcE
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        local.get 3
        i32.store
        i32.const 0
        local.set 0
        loop  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 128
              i32.add
              local.tee 5
              local.get 2
              i32.lt_u
              local.tee 6
              br_if 0 (;@5;)
              local.get 4
              local.set 7
              local.get 0
              i32.eqz
              br_if 1 (;@4;)
              local.get 3
              i32.const 0
              local.get 0
              i32.const 0
              call $_ZN4core3str16slice_error_fail17h1e2d4ca974a24a43E
              unreachable
            end
            i32.const 128
            local.set 7
            local.get 0
            local.get 5
            i32.or
            br_if 3 (;@1;)
          end
          local.get 1
          local.get 3
          local.get 0
          i32.add
          local.get 7
          call $_ZN14aidoku_imports3std11read_string17h3c1f102f7e811cc2E
          local.get 4
          i32.const -128
          i32.add
          local.set 4
          local.get 5
          local.set 0
          local.get 6
          br_if 0 (;@3;)
        end
        local.get 1
        call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
        return
      end
      unreachable
      unreachable
    end
    local.get 3
    i32.const 0
    local.get 0
    local.get 0
    i32.const 128
    i32.add
    call $_ZN4core3str16slice_error_fail17h1e2d4ca974a24a43E
    unreachable)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h836c765c11a45bceE (type 7) (param i32 i32)
    (local i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.add
      local.tee 3
      local.get 1
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      i32.const 4
      i32.add
      local.tee 4
      i32.load
      local.tee 1
      i32.const 1
      i32.shl
      local.tee 5
      local.get 3
      local.get 5
      local.get 3
      i32.gt_u
      select
      local.tee 3
      i32.const 4
      local.get 3
      i32.const 4
      i32.gt_u
      select
      local.tee 3
      i64.extend_i32_u
      i64.const 20
      i64.mul
      local.tee 6
      i32.wrap_i64
      local.get 6
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      i32.eqz
      i32.const 2
      i32.shl
      local.get 0
      i32.load
      i32.const 0
      local.get 1
      select
      local.get 1
      i32.const 20
      i32.mul
      i32.const 4
      call $_ZN5alloc7raw_vec11finish_grow17h1daab74553f4f9a3E
      block  ;; label = @2
        local.get 2
        i32.load
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        i32.const 8
        i32.add
        i32.load
        i32.eqz
        br_if 1 (;@1;)
        unreachable
        unreachable
      end
      local.get 2
      i32.load offset=4
      local.set 1
      local.get 4
      local.get 3
      i32.store
      local.get 0
      local.get 1
      i32.store
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17h10b26fa3afebc0c9E
    unreachable)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17haba155738b5aee6dE (type 2) (param i32)
    (local i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    i32.const 4
    i32.add
    local.tee 2
    i32.load
    local.tee 3
    i32.const 1
    i32.shl
    local.tee 4
    i32.const 1
    local.get 4
    select
    local.tee 4
    i32.const 4
    local.get 4
    i32.const 4
    i32.gt_u
    select
    local.tee 4
    i64.extend_i32_u
    i64.const 80
    i64.mul
    local.tee 5
    i32.wrap_i64
    local.get 5
    i64.const 32
    i64.shr_u
    i32.wrap_i64
    i32.eqz
    i32.const 2
    i32.shl
    local.get 0
    i32.load
    i32.const 0
    local.get 3
    select
    local.get 3
    i32.const 80
    i32.mul
    i32.const 4
    call $_ZN5alloc7raw_vec11finish_grow17h1daab74553f4f9a3E
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 8
        i32.add
        i32.load
        i32.eqz
        br_if 1 (;@1;)
        unreachable
        unreachable
      end
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 2
      local.get 4
      i32.store
      local.get 0
      local.get 3
      i32.store
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17h10b26fa3afebc0c9E
    unreachable)
  (func $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E (type 7) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 3
            i32.add
            i32.const 2
            i32.shr_u
            i32.const -1
            i32.add
            local.tee 3
            i32.const 255
            i32.le_u
            br_if 0 (;@4;)
            local.get 0
            i32.const 0
            i32.store
            local.get 0
            i32.const -8
            i32.add
            local.tee 1
            local.get 1
            i32.load
            local.tee 4
            i32.const -2
            i32.and
            local.tee 5
            i32.store
            i32.const 0
            i32.load offset=1048724
            local.set 3
            i32.const 1048700
            call $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h6b2bdc3dffd01582E
            i32.eqz
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const -4
                i32.add
                local.tee 6
                i32.load
                i32.const -4
                i32.and
                local.tee 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 7
                i32.load
                local.tee 8
                i32.const 1
                i32.and
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.const -4
                      i32.and
                      local.tee 9
                      br_if 0 (;@9;)
                      local.get 7
                      local.set 0
                      br 1 (;@8;)
                    end
                    local.get 7
                    local.set 0
                    i32.const 0
                    local.get 9
                    local.get 4
                    i32.const 2
                    i32.and
                    select
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 4
                    i32.load offset=4
                    i32.const 3
                    i32.and
                    local.get 7
                    i32.or
                    i32.store offset=4
                    local.get 6
                    i32.load
                    local.tee 4
                    i32.const -4
                    i32.and
                    local.tee 0
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 0
                    i32.load
                    local.set 8
                    local.get 1
                    i32.load
                    local.set 5
                  end
                  local.get 0
                  local.get 8
                  i32.const 3
                  i32.and
                  local.get 5
                  i32.const -4
                  i32.and
                  i32.or
                  i32.store
                  local.get 6
                  i32.load
                  local.set 4
                end
                local.get 6
                local.get 4
                i32.const 3
                i32.and
                i32.store
                local.get 1
                local.get 1
                i32.load
                local.tee 0
                i32.const 3
                i32.and
                i32.store
                local.get 0
                i32.const 2
                i32.and
                i32.eqz
                br_if 1 (;@5;)
                local.get 7
                local.get 7
                i32.load
                i32.const 2
                i32.or
                i32.store
                br 1 (;@5;)
              end
              local.get 4
              i32.const -4
              i32.and
              local.tee 7
              i32.eqz
              br_if 2 (;@3;)
              i32.const 0
              local.get 7
              local.get 4
              i32.const 2
              i32.and
              select
              local.tee 4
              i32.eqz
              br_if 2 (;@3;)
              local.get 4
              i32.load8_u
              i32.const 1
              i32.and
              br_if 2 (;@3;)
              local.get 0
              local.get 4
              i32.load offset=8
              i32.const -4
              i32.and
              i32.store
              local.get 4
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=8
            end
            local.get 3
            local.set 1
            br 2 (;@2;)
          end
          local.get 0
          i32.const 0
          i32.store
          local.get 0
          i32.const -8
          i32.add
          local.tee 1
          local.get 1
          i32.load
          local.tee 7
          i32.const -2
          i32.and
          local.tee 6
          i32.store
          local.get 3
          i32.const 2
          i32.shl
          i32.const 1048728
          i32.add
          local.tee 4
          i32.load
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 12
              i32.add
              call $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h88e6a0d1d794c40dE
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const -4
                  i32.add
                  local.tee 8
                  i32.load
                  i32.const -4
                  i32.and
                  local.tee 5
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.load
                  local.tee 9
                  i32.const 1
                  i32.and
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 7
                        i32.const -4
                        i32.and
                        local.tee 10
                        br_if 0 (;@10;)
                        local.get 5
                        local.set 0
                        br 1 (;@9;)
                      end
                      local.get 5
                      local.set 0
                      i32.const 0
                      local.get 10
                      local.get 7
                      i32.const 2
                      i32.and
                      select
                      local.tee 7
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 7
                      local.get 7
                      i32.load offset=4
                      i32.const 3
                      i32.and
                      local.get 5
                      i32.or
                      i32.store offset=4
                      local.get 8
                      i32.load
                      local.tee 7
                      i32.const -4
                      i32.and
                      local.tee 0
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 0
                      i32.load
                      local.set 9
                      local.get 1
                      i32.load
                      local.set 6
                    end
                    local.get 0
                    local.get 9
                    i32.const 3
                    i32.and
                    local.get 6
                    i32.const -4
                    i32.and
                    i32.or
                    i32.store
                    local.get 8
                    i32.load
                    local.set 7
                  end
                  local.get 8
                  local.get 7
                  i32.const 3
                  i32.and
                  i32.store
                  local.get 1
                  local.get 1
                  i32.load
                  local.tee 0
                  i32.const 3
                  i32.and
                  i32.store
                  local.get 0
                  i32.const 2
                  i32.and
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 5
                  local.get 5
                  i32.load
                  i32.const 2
                  i32.or
                  i32.store
                  br 1 (;@6;)
                end
                local.get 7
                i32.const -4
                i32.and
                local.tee 5
                i32.eqz
                br_if 1 (;@5;)
                i32.const 0
                local.get 5
                local.get 7
                i32.const 2
                i32.and
                select
                local.tee 7
                i32.eqz
                br_if 1 (;@5;)
                local.get 7
                i32.load8_u
                i32.const 1
                i32.and
                br_if 1 (;@5;)
                local.get 0
                local.get 7
                i32.load offset=8
                i32.const -4
                i32.and
                i32.store
                local.get 7
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=8
              end
              local.get 3
              local.set 1
              br 1 (;@4;)
            end
            local.get 0
            local.get 3
            i32.store
          end
          local.get 4
          local.get 1
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        i32.store
      end
      i32.const 0
      local.get 1
      i32.store offset=1048724
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN6aidoku7structs15MangaPageResult6create17h7a62178f9d7d9cb7E (type 1) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    call $_ZN14aidoku_imports3std12create_array17h7056aec6721f851bE
    local.set 1
    local.get 0
    i32.load
    local.tee 2
    local.get 0
    i32.load offset=8
    local.tee 3
    i32.const 80
    i32.mul
    i32.add
    local.set 4
    local.get 0
    i32.load offset=4
    local.set 5
    local.get 2
    local.set 6
    block  ;; label = @1
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.set 6
      block  ;; label = @2
        loop  ;; label = @3
          local.get 6
          local.tee 3
          i32.const 80
          i32.add
          local.set 6
          local.get 3
          i32.load offset=72
          local.tee 7
          i32.const 3
          i32.eq
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.const 64
                i32.add
                i32.load
                local.tee 8
                i32.const 1073741823
                i32.and
                local.get 8
                i32.ne
                br_if 0 (;@6;)
                local.get 8
                i32.const 2
                i32.shl
                local.tee 9
                i32.const 0
                i32.lt_s
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=56
                local.set 10
                local.get 3
                i32.const 60
                i32.add
                i32.load
                local.set 11
                local.get 3
                i32.const 52
                i32.add
                i32.load
                local.set 12
                local.get 3
                i32.const 44
                i32.add
                i32.load
                local.set 13
                local.get 3
                i32.const 36
                i32.add
                i32.load
                local.set 14
                local.get 3
                i32.const 28
                i32.add
                i32.load
                local.set 15
                local.get 3
                i32.const 20
                i32.add
                i32.load
                local.set 16
                local.get 3
                i32.const 12
                i32.add
                i32.load
                local.set 17
                local.get 3
                i32.load offset=68
                local.set 18
                local.get 3
                i32.load offset=48
                local.set 19
                local.get 3
                i32.load offset=40
                local.set 20
                local.get 3
                i32.load offset=32
                local.set 21
                local.get 3
                i32.load offset=24
                local.set 22
                local.get 3
                i32.load offset=16
                local.set 23
                local.get 3
                i32.load offset=8
                local.set 24
                local.get 3
                i32.load offset=4
                local.set 25
                local.get 3
                i32.load
                local.set 26
                local.get 3
                i32.load offset=76
                local.set 27
                local.get 9
                br_if 1 (;@5;)
                i32.const 4
                local.set 28
                br 2 (;@4;)
              end
              call $_ZN5alloc7raw_vec17capacity_overflow17h10b26fa3afebc0c9E
              unreachable
            end
            local.get 9
            i32.const 4
            call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h864bff3da949edfcE
            local.tee 28
            i32.eqz
            br_if 2 (;@2;)
          end
          block  ;; label = @4
            local.get 10
            local.get 10
            local.get 8
            i32.const 3
            i32.shl
            local.tee 29
            i32.add
            local.tee 30
            i32.eq
            local.tee 31
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 29
                i32.const -8
                i32.add
                local.tee 32
                i32.const 3
                i32.shr_u
                i32.const 1
                i32.add
                i32.const 7
                i32.and
                local.tee 3
                br_if 0 (;@6;)
                local.get 28
                local.set 3
                local.get 10
                local.set 33
                br 1 (;@5;)
              end
              i32.const 0
              local.get 3
              i32.sub
              local.set 34
              local.get 28
              local.set 3
              local.get 10
              local.set 33
              loop  ;; label = @6
                local.get 3
                local.get 33
                i32.load
                i32.store
                local.get 3
                i32.const 4
                i32.add
                local.set 3
                local.get 33
                i32.const 8
                i32.add
                local.set 33
                local.get 34
                i32.const 1
                i32.add
                local.tee 35
                local.get 34
                i32.ge_u
                local.set 36
                local.get 35
                local.set 34
                local.get 36
                br_if 0 (;@6;)
              end
            end
            local.get 32
            i32.const 56
            i32.lt_u
            br_if 0 (;@4;)
            loop  ;; label = @5
              local.get 3
              local.get 33
              i32.load
              i32.store
              local.get 3
              i32.const 4
              i32.add
              local.get 33
              i32.const 8
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 8
              i32.add
              local.get 33
              i32.const 16
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 12
              i32.add
              local.get 33
              i32.const 24
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 16
              i32.add
              local.get 33
              i32.const 32
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 20
              i32.add
              local.get 33
              i32.const 40
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 24
              i32.add
              local.get 33
              i32.const 48
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 28
              i32.add
              local.get 33
              i32.const 56
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 32
              i32.add
              local.set 3
              local.get 33
              i32.const 64
              i32.add
              local.tee 33
              local.get 30
              i32.ne
              br_if 0 (;@5;)
            end
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 9
              br_if 0 (;@5;)
              i32.const 4
              local.set 32
              br 1 (;@4;)
            end
            local.get 9
            i32.const 4
            call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h864bff3da949edfcE
            local.tee 32
            i32.eqz
            br_if 2 (;@2;)
          end
          block  ;; label = @4
            local.get 31
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 29
                i32.const -8
                i32.add
                local.tee 29
                i32.const 3
                i32.shr_u
                i32.const 1
                i32.add
                i32.const 7
                i32.and
                local.tee 3
                br_if 0 (;@6;)
                local.get 32
                local.set 3
                local.get 10
                local.set 33
                br 1 (;@5;)
              end
              i32.const 0
              local.get 3
              i32.sub
              local.set 34
              local.get 32
              local.set 3
              local.get 10
              local.set 33
              loop  ;; label = @6
                local.get 3
                local.get 33
                i32.const 4
                i32.add
                i32.load
                i32.store
                local.get 3
                i32.const 4
                i32.add
                local.set 3
                local.get 33
                i32.const 8
                i32.add
                local.set 33
                local.get 34
                i32.const 1
                i32.add
                local.tee 35
                local.get 34
                i32.ge_u
                local.set 36
                local.get 35
                local.set 34
                local.get 36
                br_if 0 (;@6;)
              end
            end
            local.get 29
            i32.const 56
            i32.lt_u
            br_if 0 (;@4;)
            local.get 33
            i32.const 60
            i32.add
            local.set 33
            loop  ;; label = @5
              local.get 3
              local.get 33
              i32.const -56
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 4
              i32.add
              local.get 33
              i32.const -48
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 8
              i32.add
              local.get 33
              i32.const -40
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 12
              i32.add
              local.get 33
              i32.const -32
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 16
              i32.add
              local.get 33
              i32.const -24
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 20
              i32.add
              local.get 33
              i32.const -16
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 24
              i32.add
              local.get 33
              i32.const -8
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 28
              i32.add
              local.get 33
              i32.load
              i32.store
              local.get 3
              i32.const 32
              i32.add
              local.set 3
              local.get 33
              i32.const 4
              i32.add
              local.set 34
              local.get 33
              i32.const 64
              i32.add
              local.set 33
              local.get 34
              local.get 30
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 26
          local.get 25
          local.get 24
          local.get 17
          local.get 23
          local.get 16
          local.get 22
          local.get 15
          local.get 21
          local.get 14
          local.get 20
          local.get 13
          local.get 19
          local.get 12
          local.get 28
          local.get 32
          local.get 8
          local.get 18
          local.get 7
          local.get 27
          call $_ZN6aidoku7structs12create_manga17h35470fb75f4211c0E
          local.set 3
          block  ;; label = @4
            local.get 8
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 9
              i32.eqz
              br_if 0 (;@5;)
              local.get 32
              local.get 9
              call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
            end
            local.get 9
            i32.eqz
            br_if 0 (;@4;)
            local.get 28
            local.get 9
            call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
          end
          block  ;; label = @4
            local.get 11
            i32.eqz
            br_if 0 (;@4;)
            local.get 11
            i32.const 3
            i32.shl
            local.tee 33
            i32.eqz
            br_if 0 (;@4;)
            local.get 10
            local.get 33
            call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
          end
          local.get 1
          local.get 3
          call $_ZN14aidoku_imports3std12array_append17hfa5c3707320db518E
          local.get 3
          call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
          local.get 6
          local.get 4
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 4
        local.set 6
        br 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 4
    local.get 6
    i32.sub
    local.tee 3
    i32.const 80
    i32.div_s
    local.set 33
    block  ;; label = @1
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 33
      i32.const 80
      i32.mul
      local.set 33
      local.get 6
      i32.const 60
      i32.add
      local.set 3
      loop  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.load
          local.tee 34
          i32.eqz
          br_if 0 (;@3;)
          local.get 34
          i32.const 3
          i32.shl
          local.tee 34
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const -4
          i32.add
          i32.load
          local.get 34
          call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
        end
        local.get 3
        i32.const 80
        i32.add
        local.set 3
        local.get 33
        i32.const -80
        i32.add
        local.tee 33
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.const 80
      i32.mul
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
    end
    local.get 1
    local.get 0
    i32.load8_u offset=12
    call $_ZN6aidoku7structs19create_manga_result17hbe04b8bd9055a540E
    local.set 3
    local.get 1
    call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
    local.get 3)
  (func $get_manga_listing (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.const 1048626
    i32.const 4
    call $_ZN14aidoku_imports3std10object_get17hc7f0d5cb2f875986E
    local.tee 3
    call $_ZN14aidoku_imports3std10value_kind17ha3880610cd52b218E
    local.set 4
    local.get 3
    call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
    local.get 0
    call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
    i32.const -1
    local.set 0
    block  ;; label = @1
      local.get 4
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      call $_ZN14aidoku_imports3std9StringRef4read17h286cc716a486f574E
      i32.const 0
      i32.load offset=1048644
      local.set 0
      block  ;; label = @2
        local.get 2
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load
        local.get 3
        call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
      end
      local.get 2
      i32.const 28
      i32.add
      i32.const 0
      i32.store8
      local.get 2
      i64.const 0
      i64.store offset=20 align=4
      local.get 2
      local.get 0
      i32.store offset=16
      local.get 2
      i32.const 16
      i32.add
      call $_ZN6aidoku7structs15MangaPageResult6create17h7a62178f9d7d9cb7E
      local.set 0
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $get_chapter_list (type 1) (param i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i32.const 1048639
    i32.const 2
    call $_ZN14aidoku_imports3std10object_get17hc7f0d5cb2f875986E
    local.tee 2
    call $_ZN14aidoku_imports3std10value_kind17ha3880610cd52b218E
    local.set 3
    local.get 2
    call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
    local.get 0
    call $_ZN14aidoku_imports3std7destroy17hd98058e13f4f51c9E
    block  ;; label = @1
      local.get 3
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      local.get 2
      call $_ZN14aidoku_imports3std9StringRef4read17h286cc716a486f574E
      local.get 1
      i32.load offset=4
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load
      local.get 0
      call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    i32.const -1)
  (func $_ZN5alloc7raw_vec17capacity_overflow17h10b26fa3afebc0c9E (type 9)
    call $_ZN4core9panicking9panic_fmt17h4b75645fa717413aE
    unreachable)
  (func $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h864bff3da949edfcE (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 3
        i32.add
        local.tee 3
        i32.const 2
        i32.shr_u
        local.tee 4
        i32.const -1
        i32.add
        local.tee 0
        i32.const 256
        i32.lt_u
        br_if 0 (;@2;)
        local.get 2
        i32.const 0
        i32.load offset=1048724
        i32.store offset=12
        block  ;; label = @3
          local.get 4
          local.get 1
          local.get 2
          i32.const 12
          i32.add
          i32.const 1048700
          i32.const 1048652
          call $_ZN9wee_alloc15alloc_first_fit17hbb7b1077be7a3a77E
          local.tee 0
          br_if 0 (;@3;)
          i32.const 0
          local.set 0
          local.get 3
          i32.const -4
          i32.and
          local.tee 3
          local.get 1
          i32.const 3
          i32.shl
          i32.const 16384
          i32.add
          local.tee 5
          local.get 3
          local.get 5
          i32.gt_u
          select
          i32.const 65543
          i32.add
          local.tee 3
          i32.const 16
          i32.shr_u
          memory.grow
          local.tee 5
          i32.const -1
          i32.eq
          br_if 0 (;@3;)
          local.get 5
          i32.const 16
          i32.shl
          local.tee 0
          i32.const 0
          i32.store offset=4
          local.get 0
          local.get 2
          i32.load offset=12
          i32.store offset=8
          local.get 0
          local.get 0
          local.get 3
          i32.const -65536
          i32.and
          i32.add
          i32.const 2
          i32.or
          i32.store
          local.get 2
          local.get 0
          i32.store offset=12
          local.get 4
          local.get 1
          local.get 2
          i32.const 12
          i32.add
          i32.const 1048700
          i32.const 1048652
          call $_ZN9wee_alloc15alloc_first_fit17hbb7b1077be7a3a77E
          local.set 0
        end
        i32.const 0
        local.get 2
        i32.load offset=12
        i32.store offset=1048724
        br 1 (;@1;)
      end
      local.get 2
      i32.const 1048724
      i32.store offset=8
      local.get 2
      local.get 0
      i32.const 2
      i32.shl
      i32.const 1048728
      i32.add
      local.tee 3
      i32.load
      i32.store offset=12
      block  ;; label = @2
        local.get 4
        local.get 1
        local.get 2
        i32.const 12
        i32.add
        local.get 2
        i32.const 8
        i32.add
        i32.const 1048676
        call $_ZN9wee_alloc15alloc_first_fit17hbb7b1077be7a3a77E
        local.tee 0
        br_if 0 (;@2;)
        local.get 2
        local.get 2
        i32.const 8
        i32.add
        local.get 4
        local.get 1
        call $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h8826442f813fbd58E
        i32.const 0
        local.set 0
        local.get 2
        i32.load
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=4
        local.tee 0
        local.get 2
        i32.load offset=12
        i32.store offset=8
        local.get 2
        local.get 0
        i32.store offset=12
        local.get 4
        local.get 1
        local.get 2
        i32.const 12
        i32.add
        local.get 2
        i32.const 8
        i32.add
        i32.const 1048676
        call $_ZN9wee_alloc15alloc_first_fit17hbb7b1077be7a3a77E
        local.set 0
      end
      local.get 3
      local.get 2
      i32.load offset=12
      i32.store
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3str16slice_error_fail17h1e2d4ca974a24a43E (type 10) (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 257
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 4
              i32.add
              local.tee 5
              i32.const 256
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 0 (;@5;)
              local.get 4
              i32.const 256
              i32.add
              local.set 5
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 5
              i32.const 255
              i32.add
              i32.load8_s
              i32.const -65
              i32.le_s
              br_if 0 (;@5;)
              local.get 4
              i32.const 255
              i32.add
              local.set 5
              br 3 (;@2;)
            end
            local.get 5
            i32.const 254
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            br_if 1 (;@3;)
            block  ;; label = @5
              local.get 5
              i32.const 253
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              br_if 0 (;@5;)
              local.get 4
              i32.const -4
              i32.add
              local.tee 4
              i32.const -256
              i32.ne
              br_if 1 (;@4;)
              br 4 (;@1;)
            end
          end
          local.get 4
          i32.const 253
          i32.add
          local.set 5
          br 1 (;@2;)
        end
        local.get 4
        i32.const 254
        i32.add
        local.set 5
      end
      local.get 5
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 0
      local.get 5
      call $_ZN4core3str16slice_error_fail17h1e2d4ca974a24a43E
      unreachable
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        local.get 1
        i32.gt_u
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        i32.gt_u
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        i32.gt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 2
                local.get 1
                i32.lt_u
                br_if 0 (;@6;)
                local.get 1
                local.get 2
                i32.eq
                br_if 1 (;@5;)
                br 2 (;@4;)
              end
              local.get 0
              local.get 2
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 1 (;@4;)
            end
            i32.const 0
            local.set 5
            local.get 3
            local.set 2
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
          end
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                local.get 1
                i32.lt_u
                br_if 0 (;@6;)
                local.get 1
                local.get 2
                i32.eq
                br_if 5 (;@1;)
                br 1 (;@5;)
              end
              local.get 0
              local.get 2
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 0 (;@5;)
              local.get 2
              local.set 5
              br 2 (;@3;)
            end
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            br_if 0 (;@4;)
          end
          i32.const 0
          local.set 5
        end
        local.get 5
        local.get 1
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        local.get 5
        i32.add
        local.tee 4
        i32.load8_s
        local.tee 5
        i32.const -1
        i32.gt_s
        br_if 0 (;@2;)
        local.get 5
        i32.const 255
        i32.and
        local.tee 2
        i32.const 224
        i32.lt_u
        br_if 0 (;@2;)
        local.get 5
        i32.const 255
        i32.and
        i32.const 240
        i32.lt_u
        br_if 0 (;@2;)
        local.get 4
        i32.load8_u offset=1
        i32.const 63
        i32.and
        i32.const 12
        i32.shl
        local.get 4
        i32.load8_u offset=2
        i32.const 63
        i32.and
        i32.const 6
        i32.shl
        i32.or
        local.get 4
        i32.load8_u offset=3
        i32.const 63
        i32.and
        i32.or
        local.get 2
        i32.const 18
        i32.shl
        i32.const 1835008
        i32.and
        i32.or
        i32.const 1114112
        i32.eq
        br_if 1 (;@1;)
      end
      call $_ZN4core9panicking9panic_fmt17h4b75645fa717413aE
      unreachable
    end
    call $_ZN4core9panicking5panic17h165ed0f21cd8b0f3E
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17h4b75645fa717413aE (type 9)
    unreachable
    unreachable)
  (func $_ZN4core9panicking5panic17h165ed0f21cd8b0f3E (type 9)
    call $_ZN4core9panicking9panic_fmt17h4b75645fa717413aE
    unreachable)
  (func $_ZN5alloc7raw_vec11finish_grow17h1daab74553f4f9a3E (type 11) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 2
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 1
                      local.set 6
                      local.get 1
                      i32.const 0
                      i32.lt_s
                      br_if 1 (;@8;)
                      local.get 3
                      i32.eqz
                      br_if 3 (;@6;)
                      local.get 4
                      br_if 2 (;@7;)
                      local.get 1
                      br_if 4 (;@5;)
                      br 6 (;@3;)
                    end
                    local.get 0
                    local.get 1
                    i32.store offset=4
                    i32.const 1
                    local.set 6
                  end
                  i32.const 0
                  local.set 1
                  br 6 (;@1;)
                end
                local.get 1
                local.get 2
                call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h864bff3da949edfcE
                local.tee 7
                i32.eqz
                br_if 2 (;@4;)
                local.get 7
                local.get 3
                local.get 4
                call $memcpy
                drop
                local.get 3
                local.get 4
                call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17hf7adefdd18c1cd53E
                br 4 (;@2;)
              end
              local.get 1
              i32.eqz
              br_if 2 (;@3;)
            end
            local.get 1
            local.get 2
            call $_ZN72_$LT$wee_alloc..WeeAlloc$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h864bff3da949edfcE
            local.tee 7
            br_if 2 (;@2;)
          end
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 2
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.set 7
      end
      local.get 0
      local.get 7
      i32.store offset=4
      i32.const 0
      local.set 6
    end
    local.get 0
    local.get 6
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i32.store)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h2c18175a112e7f96E (type 10) (param i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 2
        i32.shl
        local.tee 2
        local.get 3
        i32.const 3
        i32.shl
        i32.const 16384
        i32.add
        local.tee 3
        local.get 2
        local.get 3
        i32.gt_u
        select
        i32.const 65543
        i32.add
        local.tee 2
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 3
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 1
        local.set 2
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 3
      i32.const 16
      i32.shl
      local.tee 3
      i64.const 0
      i64.store offset=4 align=4
      local.get 3
      local.get 3
      local.get 2
      i32.const -65536
      i32.and
      i32.add
      i32.const 2
      i32.or
      i32.store
      i32.const 0
      local.set 2
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17h1375bd35a706d143E (type 0) (param i32 i32) (result i32)
    i32.const 512)
  (func $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h6b2bdc3dffd01582E (type 1) (param i32) (result i32)
    i32.const 1)
  (func $_ZN9wee_alloc15alloc_first_fit17hbb7b1077be7a3a77E (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.load
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -1
      i32.add
      local.set 6
      local.get 0
      i32.const 2
      i32.shl
      local.set 7
      i32.const 0
      local.get 1
      i32.sub
      local.set 8
      loop  ;; label = @2
        local.get 5
        i32.const 8
        i32.add
        local.set 9
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.load offset=8
            local.tee 10
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 5
            local.set 1
            br 1 (;@3;)
          end
          loop  ;; label = @4
            local.get 9
            local.get 10
            i32.const -2
            i32.and
            i32.store
            block  ;; label = @5
              block  ;; label = @6
                local.get 5
                i32.load offset=4
                local.tee 10
                i32.const -4
                i32.and
                local.tee 9
                br_if 0 (;@6;)
                i32.const 0
                local.set 1
                br 1 (;@5;)
              end
              i32.const 0
              local.get 9
              local.get 9
              i32.load8_u
              i32.const 1
              i32.and
              select
              local.set 1
            end
            block  ;; label = @5
              local.get 5
              i32.load
              local.tee 11
              i32.const -4
              i32.and
              local.tee 12
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 12
              local.get 11
              i32.const 2
              i32.and
              select
              local.tee 11
              i32.eqz
              br_if 0 (;@5;)
              local.get 11
              local.get 11
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 9
              i32.or
              i32.store offset=4
              local.get 5
              i32.load offset=4
              local.tee 10
              i32.const -4
              i32.and
              local.set 9
            end
            block  ;; label = @5
              local.get 9
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              local.get 9
              i32.load
              i32.const 3
              i32.and
              local.get 5
              i32.load
              i32.const -4
              i32.and
              i32.or
              i32.store
              local.get 5
              i32.load offset=4
              local.set 10
            end
            local.get 5
            local.get 10
            i32.const 3
            i32.and
            i32.store offset=4
            local.get 5
            local.get 5
            i32.load
            local.tee 9
            i32.const 3
            i32.and
            i32.store
            block  ;; label = @5
              local.get 9
              i32.const 2
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 1
              i32.load
              i32.const 2
              i32.or
              i32.store
            end
            local.get 2
            local.get 1
            i32.store
            local.get 1
            i32.const 8
            i32.add
            local.set 9
            local.get 1
            local.set 5
            local.get 1
            i32.load offset=8
            local.tee 10
            i32.const 1
            i32.and
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 1
          i32.load
          i32.const -4
          i32.and
          local.tee 10
          local.get 1
          i32.const 8
          i32.add
          local.tee 5
          i32.sub
          local.get 7
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              local.get 3
              local.get 0
              local.get 4
              i32.load offset=16
              call_indirect (type 0)
              i32.const 2
              i32.shl
              i32.add
              i32.const 8
              i32.add
              local.get 10
              local.get 7
              i32.sub
              local.get 8
              i32.and
              local.tee 10
              i32.le_u
              br_if 0 (;@5;)
              local.get 6
              local.get 5
              i32.and
              br_if 2 (;@3;)
              local.get 2
              local.get 9
              i32.load
              i32.const -4
              i32.and
              i32.store
              local.get 1
              local.get 1
              i32.load
              i32.const 1
              i32.or
              i32.store
              local.get 1
              local.set 5
              br 1 (;@4;)
            end
            local.get 10
            i32.const 0
            i32.store
            local.get 10
            i32.const -8
            i32.add
            local.tee 5
            i64.const 0
            i64.store align=4
            local.get 5
            local.get 1
            i32.load
            i32.const -4
            i32.and
            i32.store
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 10
              i32.const -4
              i32.and
              local.tee 11
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.get 11
              local.get 10
              i32.const 2
              i32.and
              select
              local.tee 10
              i32.eqz
              br_if 0 (;@5;)
              local.get 10
              local.get 10
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 5
              i32.or
              i32.store offset=4
            end
            local.get 5
            local.get 5
            i32.load offset=4
            i32.const 3
            i32.and
            local.get 1
            i32.or
            i32.store offset=4
            local.get 9
            local.get 9
            i32.load
            i32.const -2
            i32.and
            i32.store
            local.get 1
            local.get 1
            i32.load
            local.tee 9
            i32.const 3
            i32.and
            local.get 5
            i32.or
            local.tee 10
            i32.store
            block  ;; label = @5
              block  ;; label = @6
                local.get 9
                i32.const 2
                i32.and
                br_if 0 (;@6;)
                local.get 5
                i32.load
                local.set 1
                br 1 (;@5;)
              end
              local.get 1
              local.get 10
              i32.const -3
              i32.and
              i32.store
              local.get 5
              local.get 5
              i32.load
              i32.const 2
              i32.or
              local.tee 1
              i32.store
            end
            local.get 5
            local.get 1
            i32.const 1
            i32.or
            i32.store
          end
          local.get 5
          i32.const 8
          i32.add
          return
        end
        local.get 2
        local.get 1
        i32.load offset=8
        local.tee 5
        i32.store
        local.get 5
        br_if 0 (;@2;)
      end
    end
    i32.const 0)
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h8826442f813fbd58E (type 10) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 1
    i32.load
    local.tee 5
    i32.load
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 2
          i32.add
          local.tee 2
          local.get 2
          i32.mul
          local.tee 2
          i32.const 2048
          local.get 2
          i32.const 2048
          i32.gt_u
          select
          local.tee 1
          i32.const 4
          local.get 4
          i32.const 12
          i32.add
          i32.const 1048700
          i32.const 1048700
          call $_ZN9wee_alloc15alloc_first_fit17hbb7b1077be7a3a77E
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 4
          i32.load offset=12
          i32.store
          local.get 1
          i32.const 2
          i32.shl
          local.set 6
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 2
            i32.shl
            local.tee 6
            i32.const 16416
            local.get 6
            i32.const 16416
            i32.gt_u
            select
            i32.const 65543
            i32.add
            local.tee 7
            i32.const 16
            i32.shr_u
            memory.grow
            local.tee 2
            i32.const -1
            i32.eq
            br_if 0 (;@4;)
            local.get 2
            i32.const 16
            i32.shl
            local.tee 2
            i32.const 0
            i32.store offset=4
            local.get 2
            local.get 4
            i32.load offset=12
            i32.store offset=8
            local.get 2
            local.get 2
            local.get 7
            i32.const -65536
            i32.and
            i32.add
            i32.const 2
            i32.or
            i32.store
            local.get 4
            local.get 2
            i32.store offset=12
            local.get 1
            i32.const 4
            local.get 4
            i32.const 12
            i32.add
            i32.const 1048700
            i32.const 1048700
            call $_ZN9wee_alloc15alloc_first_fit17hbb7b1077be7a3a77E
            local.set 2
            local.get 5
            local.get 4
            i32.load offset=12
            i32.store
            local.get 2
            br_if 2 (;@2;)
            br 1 (;@3;)
          end
          local.get 5
          local.get 4
          i32.load offset=12
          i32.store
        end
        i32.const 1
        local.set 1
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 2
      i64.const 0
      i64.store offset=4 align=4
      local.get 2
      local.get 2
      local.get 6
      i32.add
      i32.const 2
      i32.or
      i32.store
      i32.const 0
      local.set 1
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN4core3ptr48drop_in_place$LT$wee_alloc..LargeAllocPolicy$GT$17hdd7c19aa90fdc9dbE (type 2) (param i32))
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h88e6a0d1d794c40dE (type 1) (param i32) (result i32)
    i32.const 0)
  (func $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17h5db092c15115b2e0E (type 0) (param i32 i32) (result i32)
    local.get 1)
  (func $memcpy (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 15
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.set 5
      block  ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        local.get 1
        local.set 6
        loop  ;; label = @3
          local.get 3
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 7
      i32.const -4
      i32.and
      local.tee 8
      i32.add
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 9
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 9
          i32.const 3
          i32.shl
          local.tee 1
          i32.const 24
          i32.and
          local.set 2
          i32.const 0
          local.get 1
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 9
          i32.const -4
          i32.and
          local.tee 6
          i32.const 4
          i32.add
          local.set 1
          local.get 6
          i32.load
          local.set 6
          loop  ;; label = @4
            local.get 5
            local.get 6
            local.get 2
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 8
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 9
        local.set 1
        loop  ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 7
      i32.const 3
      i32.and
      local.set 2
      local.get 9
      local.get 8
      i32.add
      local.set 1
    end
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.add
      local.set 5
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 5
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (table (;0;) 8 8 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1049752))
  (global (;2;) i32 (i32.const 1049760))
  (export "memory" (memory 0))
  (export "get_manga_list" (func $get_manga_list))
  (export "get_manga_listing" (func $get_manga_listing))
  (export "get_chapter_list" (func $get_chapter_list))
  (export "get_manga_details" (func $get_chapter_list))
  (export "get_page_list" (func $get_chapter_list))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr48drop_in_place$LT$wee_alloc..LargeAllocPolicy$GT$17hdd7c19aa90fdc9dbE $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h2c18175a112e7f96E $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17h1375bd35a706d143E $_ZN70_$LT$wee_alloc..LargeAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h6b2bdc3dffd01582E $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$22new_cell_for_free_list17h8826442f813fbd58E $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$13min_cell_size17h5db092c15115b2e0E $_ZN88_$LT$wee_alloc..size_classes..SizeClassAllocPolicy$u20$as$u20$wee_alloc..AllocPolicy$GT$32should_merge_adjacent_free_cells17h88e6a0d1d794c40dE)
  (data $.rodata (i32.const 1048576) "1https://skitty.xyz/icon.pngTitleAuthorDescriptionnametypevalueid\00\00\00\04\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\01\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00"))
