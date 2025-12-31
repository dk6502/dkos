export CC := if os_family() == "windows" {"clang.exe"} else {""}
export AR := if os_family() == "windows" {"ar.exe"} else {""}
xorriso := if os_family() == "windows" {"tools/xorriso/xorriso.exe"} else {"xorriso"}
windows := if os_family() == "windows" {"true"} else {"false"}

build:
    #!/usr/bin/env sh
    if windows; then just xorriso-win; fi
    if cargo build; then
    if test -a limine;
    then echo Limine is already cloned
    else git clone https://github.com/limine-bootloader/limine.git --branch=v9.x-binary --depth=1 tools/limine
    fi
    make -C tools/limine
    rm -rf iso_root
    mkdir -p iso_root/boot
    cp -v target/x86_64-unknown-none/debug/kernel iso_root/boot/
    mkdir -p iso_root/boot/limine
    cp -v limine.conf iso_root/boot/limine
    mkdir -p iso_root/EFI/BOOT
    cp -v tools/limine/limine-bios.sys \
    tools/limine/limine-bios-cd.bin \
    tools/limine/limine-uefi-cd.bin \
    iso_root/boot/limine/
    cp -v tools/limine/BOOTX64.EFI iso_root/EFI/BOOT
    {{xorriso}} -as mkisofs -b boot/limine/limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot boot/limine/limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o dkos-x86_64.iso
    ./tools/limine/limine bios-install dkos-x86_64.iso
    rm -rf iso_root
    fi

clean:
    rm -rf limine
    rm -f dkos-*.iso
    cargo clean

run:
    #!/usr/bin/env sh
      if just build; then
      qemu-system-x86_64 dkos-x86_64.iso
      fi

[windows]
xorriso-win:
    #!/usr/bin/env sh
      if test -a tools/xorriso;
      then :
      else
      git clone https://github.com/Cavaler/xorriso-exe-for-windows/ --depth=1 tools/xorriso
      fi
