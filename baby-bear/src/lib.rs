#![no_std]

extern crate alloc;

mod baby_bear;
mod extension;

pub use baby_bear::*;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod aarch64_neon;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub use aarch64_neon::*;

// Type your code here, or load an example.
const P: u32 = 15 * (1 << 27) + 1;

/// The modulus of the field as a u64.
const P_U64: u64 = P as u64;

// montgomery form constants
const M: u32 = 0x88000001;

const NBETA: u32 = P - 11;

pub fn mul(lhs: u32, rhs: u32) -> u32 {
    (((lhs as u64) * (rhs as u64)) % P_U64) as u32
}

pub fn ext_mul(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [
        a[0] * b[0] + NBETA * (a[1] * b[3] + a[2] * b[2] + a[3] * b[1]),
        a[0] * b[1] + a[1] * b[0] + NBETA * (a[2] * b[3] + a[3] * b[2]),
        a[0] * b[2] + a[1] * b[1] + a[2] * b[0] + NBETA * (a[3] * b[3]),
        a[0] * b[3] + a[1] * b[2] + a[2] * b[1] + a[3] * b[0],
    ]
}

// example::ext_mul_canonical:
//         push    rbp
//         push    r15
//         push    r14
//         push    r13
//         push    r12
//         push    rbx
//         mov     qword ptr [rsp - 8], rdi
//         mov     ecx, dword ptr [rsi]
//         mov     r11d, dword ptr [rsi + 4]
//         mov     r9d, dword ptr [rdx]
//         mov     edi, dword ptr [rdx + 4]
//         mov     eax, dword ptr [rdx + 12]
//         mov     ebx, eax
//         imul    ebx, r11d
//         mov     r10d, dword ptr [rsi + 8]
//         mov     edx, dword ptr [rdx + 8]
//         mov     ebp, edx
//         imul    ebp, r10d
//         add     ebp, ebx
//         mov     esi, dword ptr [rsi + 12]
//         mov     ebx, edi
//         imul    ebx, esi
//         add     ebx, ebp
//         mov     ebp, edi
//         mov     r14d, esi
//         imul    r14d, edx
//         mov     r15d, edx
//         mov     r12d, edi
//         imul    r12d, r11d
//         imul    edx, r11d
//         imul    r11d, r9d
//         mov     r13d, r10d
//         imul    edi, r10d
//         imul    r10d, r9d
//         mov     r8d, eax
//         imul    r8d, esi
//         imul    esi, r9d
//         imul    r9d, ecx
//         imul    ebx, ebx, 2013265910
//         add     ebx, r9d
//         imul    ebp, ecx
//         add     r11d, ebp
//         imul    r13d, eax
//         add     r14d, r13d
//         imul    r9d, r14d, 2013265910
//         add     r9d, r11d
//         imul    r15d, ecx
//         add     r10d, r15d
//         add     r10d, r12d
//         imul    r8d, r8d, 2013265910
//         add     r8d, r10d
//         imul    eax, ecx
//         add     edx, eax
//         add     esi, edi
//         add     esi, edx
//         mov     rax, qword ptr [rsp - 8]
//         mov     dword ptr [rax], ebx
//         mov     dword ptr [rax + 4], r9d
//         mov     dword ptr [rax + 8], r8d
//         mov     dword ptr [rax + 12], esi
//         pop     rbx
//         pop     r12
//         pop     r13
//         pop     r14
//         pop     r15
//         pop     rbp
//         ret

// example::ext_mul:
//         push    rbp
//         push    r15
//         push    r14
//         push    r13
//         push    r12
//         push    rbx
//         mov     qword ptr [rsp - 8], rdi
//         mov     ecx, dword ptr [rsi]
//         mov     r11d, dword ptr [rsi + 4]
//         mov     r9d, dword ptr [rdx]
//         mov     edi, dword ptr [rdx + 4]
//         mov     eax, dword ptr [rdx + 12]
//         mov     ebx, eax
//         imul    ebx, r11d
//         mov     r10d, dword ptr [rsi + 8]
//         mov     edx, dword ptr [rdx + 8]
//         mov     ebp, edx
//         imul    ebp, r10d
//         add     ebp, ebx
//         mov     esi, dword ptr [rsi + 12]
//         mov     ebx, edi
//         imul    ebx, esi
//         add     ebx, ebp
//         mov     ebp, edi
//         mov     r14d, esi
//         imul    r14d, edx
//         mov     r15d, edx
//         mov     r12d, edi
//         imul    r12d, r11d
//         imul    edx, r11d
//         imul    r11d, r9d
//         mov     r13d, r10d
//         imul    edi, r10d
//         imul    r10d, r9d
//         mov     r8d, eax
//         imul    r8d, esi
//         imul    esi, r9d
//         imul    r9d, ecx
//         imul    ebx, ebx, 2013265910
//         add     ebx, r9d
//         imul    ebp, ecx
//         add     r11d, ebp
//         imul    r13d, eax
//         add     r14d, r13d
//         imul    r9d, r14d, 2013265910
//         add     r9d, r11d
//         imul    r15d, ecx
//         add     r10d, r15d
//         add     r10d, r12d
//         imul    r8d, r8d, 2013265910
//         add     r8d, r10d
//         imul    eax, ecx
//         add     edx, eax
//         add     esi, edi
//         add     esi, edx
//         mov     rax, qword ptr [rsp - 8]
//         mov     dword ptr [rax], ebx
//         mov     dword ptr [rax + 4], r9d
//         mov     dword ptr [rax + 8], r8d
//         mov     dword ptr [rax + 12], esi
//         pop     rbx
//         pop     r12
//         pop     r13
//         pop     r14
//         pop     r15
//         pop     rbp
//         ret
