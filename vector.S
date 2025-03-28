    .arch armv8-a

    /* Place startup code and vector table in the .init section */
    .section .init, "ax", %progbits
    .global _start
    .type _start, %function
_start:
    /* Check current Exception Level */
    mrs   x1, CurrentEL         // Read CurrentEL; bits [3:2] contain EL
    lsr   x1, x1, #2            // Now x1 equals the current EL number
    cmp   x1, #1              // Compare with 1 (EL1)
    beq   el1_already         // If already in EL1, skip the EL2 branch

    /* We're in EL2. Print message and then branch to rust_main */
    bl    rust_print_el2      // Call Rust function that prints a message

    /* Prepare to drop from EL2 to EL1 */
    /* mov x2, #0x3C5           // SPSR_EL2: Set mode to EL1h (0b0101) and disable interrupts.
    msr SPSR_EL2, x2
    ldr x0, =rust_main       // Set entry point for EL1.
    msr ELR_EL2, x0
    eret                     // Return from exception to EL1 */

    /* Instead of proper EL2→EL1 drop using eret, simply branch */
    ldr   x0, =rust_main
    b     el1_already

el1_already:
    /* Set up the stack pointer using _stack_top */
    ldr   x0, =_stack_top
    mov   sp, x0

    /* Install the vector table by loading its address into VBAR_EL1 */
    ldr   x0, =vector_table
    msr   VBAR_EL1, x0

    /* Branch to the Rust entry point */
    bl    rust_main
    b     hang

hang:
    b hang

    /* -------------------------------------------------
       Exception Vector Table (aligned to 128 bytes)
       ------------------------------------------------- */
    .align 7
vector_table:
    /* EL1t: Exceptions taken using SP0 */
    b default_exception      /* Synchronous Exception */
    b irq_exception          /* IRQ Exception */
    b default_exception      /* FIQ Exception */
    b default_exception      /* SError Exception */

    /* EL1h: Exceptions taken using SP_EL1 */
    b default_exception      /* Synchronous Exception */
    b irq_exception          /* IRQ Exception */
    b default_exception      /* FIQ Exception */
    b default_exception      /* SError Exception */

    /* EL0 AArch64 exceptions */
    b default_exception      /* Synchronous Exception */
    b default_exception      /* IRQ Exception */
    b default_exception      /* FIQ Exception */
    b default_exception      /* SError Exception */

    /* EL0 AArch32 exceptions (unused in pure AArch64) */
    b default_exception      /* Synchronous Exception */
    b default_exception      /* IRQ Exception */
    b default_exception      /* FIQ Exception */
    b default_exception      /* SError Exception */

default_exception:
    b default_exception

irq_exception:
    /* Call the Rust IRQ handler. Instead of eret, branch to hang.
       (This is for debugging; a proper drop should use eret.) */
    bl rust_irq_handler
    b hang

    /* -------------------------------------------------
       Reserve space for the stack.
       Here we allocate 16 KB for the stack and export _stack_top.
       ------------------------------------------------- */
    .align 16
    .global _stack_top
_stack_top:
    .space 0x4000
