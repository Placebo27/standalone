use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("rustsbi-machine.ld");

    std::fs::write(ld, LINKER_SCRIPT).unwrap();

    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

const LINKER_SCRIPT: &[u8] = b"OUTPUT_ARCH(riscv)
ENTRY(_start) 
SECTIONS {
    .text : ALIGN(8) { 
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : ALIGN(8) { 
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(8);  
        erodata = .;
    } 
    .data : ALIGN(8) { 
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(8); 
        edata = .;
    }
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(8) {  
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } 
    /DISCARD/ : {
        *(.eh_frame)
    }
}";
