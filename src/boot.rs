use core::arch::asm;
use tock_registers::interfaces::Writeable;


use aarch64_cpu::{registers::{CNTHCTL_EL2, CNTVOFF_EL2, HCR_EL2, SPSR_EL2, ELR_EL2, SP_EL1}, asm};

use crate::{uart, println, print};

pub const STACK_SIZE: usize = 0x8000;

#[link_section = ".bss.stack"]
pub static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

#[inline(always)]
#[no_mangle]
unsafe fn _prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr: u64) {
    println!("stack: {:#X}", STACK.as_ptr() as usize);
    let tmp: usize;
    asm!(
        "mrs {tmp}, CurrentEL",
        tmp = out(reg) tmp
    );
    println!("currentel: {}", tmp >> 2);
    // // Enable timer counter registers for EL1.
    // CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // // No offset for reading the counters.
    // CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
    println!("Hello, world!");

    // Set up a simulated exception return.
    //
    // First, fake a saved program status where all interrupts were masked and SP_EL1 was used as a
    // stack pointer.
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // Second, let the link register point to kernel_init().
    ELR_EL2.set(_start_rust as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there
    // are no plans to ever return to EL2, just re-use the same stack.
    SP_EL1.set(phys_boot_core_stack_end_exclusive_addr);
    println!("sp: {:#x}", phys_boot_core_stack_end_exclusive_addr);
    println!("addr: {:#x}", _start_rust as *const () as u64);
    asm::eret();
}

/// 汇编入口函数
///
/// 分配栈 初始化页表信息 并调到rust入口函数
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {

    core::arch::asm!(
        // 1. 让主核心启动其他核心进入等待
        "
            mrs	x4, MPIDR_EL1
            and	x4, x4, {CONST_CORE_ID_MASK}
            mov x5, 0
            cmp	x4, x5
            b.ne	_other_core
        ",
        // 2. 设置栈指针
        "
            ldr	x0, ={boot_stack}
            add x4, x0, #{stack_size}
            mov	sp, x4

            add x0, x0, 0x1000
        ",
        // 3. 进入 os 内核
        "   
            bl	_prepare_el2_to_el1_transition
            # bl _start_rust
            ret
        ",
        stack_size = const STACK_SIZE,
        boot_stack = sym STACK,
        CONST_CORE_ID_MASK = const 0b11,
        options(noreturn),
    )
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub unsafe fn _start_rust(device_tree: usize) {
    println!("_start_rust");
    clear_bss();
    crate::main(device_tree);
}

#[no_mangle]
pub unsafe fn _other_core() {
    loop {
        asm!("wfe");
    }
}
