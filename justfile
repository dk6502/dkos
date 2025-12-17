build karch:
    rm -rf limine
    git clone https://github.com/limine-bootloader/limine.git --branch=v9.x-binary --depth=1
    make -C limine
    just kernel/ build {{ karch }}
    @echo 'dkos-'{{ karch }}
    rm -rf iso_root
    mkdir -p iso_root/boot
    mkdir -p iso_root/boot/limine
    cp limine.conf iso_root/boot/limine
    cp \
      limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin \
      iso_root/boot/limine
    mkdir -p iso_root/EFI/BOOT
    cp limine/BOOTX64.EFI iso_root/EFI/BOOT
    cp kernel/target/{{ karch }}-unknown-none/debug/kernel iso_root/boot/kernel
    xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
      -no-emul-boot -boot-load-size 4 -boot-info-table \
      --efi-boot boot/limine/limine-uefi-cd.bin \
      -efi-boot-part --efi-boot-image --protective-msdos-label \
      iso_root -o dkos-{{ karch }}.iso
    ./limine/limine bios-install dkos-{{ karch }}.iso
    rm -r iso_root

clean:
    just kernel/ clean
    rm -f dkos-*.iso
