build:
    #!/usr/bin/env sh
    cargo build
    if test -a limine;
    then echo Limine is already cloned
    else git clone https://github.com/limine-bootloader/limine.git --branch=v9.x-binary --depth=1
    fi
    make -C limine
    rm -rf iso_root
    mkdir -p iso_root/boot
    cp -v target/x86_64-unknown-none/debug/kernel iso_root/boot/
    mkdir -p iso_root/boot/limine
    cp -v limine.conf iso_root/boot/limine
    mkdir -p iso_root/EFI/BOOT
    cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
    cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT
    xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot boot/limine/limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o dkos-x86_64.iso
    ./limine/limine bios-install dkos-x86_64.iso
    rm -rf iso_root

clean:
    rm -rf limine

run:
    just build
    qemu-system-x86_64 dkos-x86_64.iso
