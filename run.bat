cargo build --target x86_64-unknown-uefi
copy /Y target\x86_64-unknown-uefi\debug\rust-uefi.efi esp\efi\boot\bootx64.efi
qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd -drive format=raw,file=fat:rw:esp
