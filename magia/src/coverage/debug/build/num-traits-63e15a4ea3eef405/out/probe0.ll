; ModuleID = 'probe0.ab39f9eadf3822b0-cgu.0'
source_filename = "probe0.ab39f9eadf3822b0-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

@__llvm_profile_runtime = external hidden global i32

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #0 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { noinline }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}
!llvm.dbg.cu = !{!4}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = !{!"rustc version 1.77.2 (25ef9e3d8 2024-04-09)"}
!4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.77.2 (25ef9e3d8 2024-04-09))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!5 = !DIFile(filename: "probe0\\@\\probe0.ab39f9eadf3822b0-cgu.0", directory: "C:\\Users\\nahue\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\num-traits-0.2.19")
