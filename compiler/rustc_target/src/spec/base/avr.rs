use object::elf;

/// Resolve the value of the EF_AVR_ARCH field for AVR ELF files, given the
/// name of the target CPU / MCU.
///
/// In ELF files using the AVR architecture, the lower 7 bits of the e_flags
/// field is a code that identifies the "ISA revision" of the object code.
///
/// This flag is generally set by AVR compilers in their output ELF files,
/// and linkers like avr-ld check this flag in all of their input files to
/// make sure they are compiled with the same ISA revision.
pub fn ef_avr_arch(target_cpu: &str) -> u32 {
    // Adapted from llvm-project/llvm/lib/target/AVR/AVRDevices.td
    match target_cpu {
        // Generic MCUs
        "avr1" => elf::EF_AVR_ARCH_AVR1,
        "avr2" => elf::EF_AVR_ARCH_AVR2,
        "avr25" => elf::EF_AVR_ARCH_AVR25,
        "avr3" => elf::EF_AVR_ARCH_AVR3,
        "avr31" => elf::EF_AVR_ARCH_AVR31,
        "avr35" => elf::EF_AVR_ARCH_AVR35,
        "avr4" => elf::EF_AVR_ARCH_AVR4,
        "avr5" => elf::EF_AVR_ARCH_AVR5,
        "avr51" => elf::EF_AVR_ARCH_AVR51,
        "avr6" => elf::EF_AVR_ARCH_AVR6,
        "avrxmega1" => elf::EF_AVR_ARCH_XMEGA1,
        "avrxmega2" => elf::EF_AVR_ARCH_XMEGA2,
        "avrxmega3" => elf::EF_AVR_ARCH_XMEGA3,
        "avrxmega4" => elf::EF_AVR_ARCH_XMEGA4,
        "avrxmega5" => elf::EF_AVR_ARCH_XMEGA5,
        "avrxmega6" => elf::EF_AVR_ARCH_XMEGA6,
        "avrxmega7" => elf::EF_AVR_ARCH_XMEGA7,
        "avrtiny" => elf::EF_AVR_ARCH_AVRTINY,

        // Specific MCUs
        "at90s1200" => elf::EF_AVR_ARCH_AVR1,
        "attiny11" => elf::EF_AVR_ARCH_AVR1,
        "attiny12" => elf::EF_AVR_ARCH_AVR1,
        "attiny15" => elf::EF_AVR_ARCH_AVR1,
        "attiny28" => elf::EF_AVR_ARCH_AVR1,
        "at90s2313" => elf::EF_AVR_ARCH_AVR2,
        "at90s2323" => elf::EF_AVR_ARCH_AVR2,
        "at90s2333" => elf::EF_AVR_ARCH_AVR2,
        "at90s2343" => elf::EF_AVR_ARCH_AVR2,
        "attiny22" => elf::EF_AVR_ARCH_AVR2,
        "attiny26" => elf::EF_AVR_ARCH_AVR2,
        "at86rf401" => elf::EF_AVR_ARCH_AVR25,
        "at90s4414" => elf::EF_AVR_ARCH_AVR2,
        "at90s4433" => elf::EF_AVR_ARCH_AVR2,
        "at90s4434" => elf::EF_AVR_ARCH_AVR2,
        "at90s8515" => elf::EF_AVR_ARCH_AVR2,
        "at90c8534" => elf::EF_AVR_ARCH_AVR2,
        "at90s8535" => elf::EF_AVR_ARCH_AVR2,
        "ata5272" => elf::EF_AVR_ARCH_AVR25,
        "ata6616c" => elf::EF_AVR_ARCH_AVR25,
        "attiny13" => elf::EF_AVR_ARCH_AVR25,
        "attiny13a" => elf::EF_AVR_ARCH_AVR25,
        "attiny2313" => elf::EF_AVR_ARCH_AVR25,
        "attiny2313a" => elf::EF_AVR_ARCH_AVR25,
        "attiny24" => elf::EF_AVR_ARCH_AVR25,
        "attiny24a" => elf::EF_AVR_ARCH_AVR25,
        "attiny4313" => elf::EF_AVR_ARCH_AVR25,
        "attiny44" => elf::EF_AVR_ARCH_AVR25,
        "attiny44a" => elf::EF_AVR_ARCH_AVR25,
        "attiny84" => elf::EF_AVR_ARCH_AVR25,
        "attiny84a" => elf::EF_AVR_ARCH_AVR25,
        "attiny25" => elf::EF_AVR_ARCH_AVR25,
        "attiny45" => elf::EF_AVR_ARCH_AVR25,
        "attiny85" => elf::EF_AVR_ARCH_AVR25,
        "attiny261" => elf::EF_AVR_ARCH_AVR25,
        "attiny261a" => elf::EF_AVR_ARCH_AVR25,
        "attiny441" => elf::EF_AVR_ARCH_AVR25,
        "attiny461" => elf::EF_AVR_ARCH_AVR25,
        "attiny461a" => elf::EF_AVR_ARCH_AVR25,
        "attiny841" => elf::EF_AVR_ARCH_AVR25,
        "attiny861" => elf::EF_AVR_ARCH_AVR25,
        "attiny861a" => elf::EF_AVR_ARCH_AVR25,
        "attiny87" => elf::EF_AVR_ARCH_AVR25,
        "attiny43u" => elf::EF_AVR_ARCH_AVR25,
        "attiny48" => elf::EF_AVR_ARCH_AVR25,
        "attiny88" => elf::EF_AVR_ARCH_AVR25,
        "attiny828" => elf::EF_AVR_ARCH_AVR25,
        "at43usb355" => elf::EF_AVR_ARCH_AVR3,
        "at76c711" => elf::EF_AVR_ARCH_AVR3,
        "atmega103" => elf::EF_AVR_ARCH_AVR31,
        "at43usb320" => elf::EF_AVR_ARCH_AVR31,
        "attiny167" => elf::EF_AVR_ARCH_AVR35,
        "at90usb82" => elf::EF_AVR_ARCH_AVR35,
        "at90usb162" => elf::EF_AVR_ARCH_AVR35,
        "ata5505" => elf::EF_AVR_ARCH_AVR35,
        "ata6617c" => elf::EF_AVR_ARCH_AVR35,
        "ata664251" => elf::EF_AVR_ARCH_AVR35,
        "atmega8u2" => elf::EF_AVR_ARCH_AVR35,
        "atmega16u2" => elf::EF_AVR_ARCH_AVR35,
        "atmega32u2" => elf::EF_AVR_ARCH_AVR35,
        "attiny1634" => elf::EF_AVR_ARCH_AVR35,
        "atmega8" => elf::EF_AVR_ARCH_AVR4,
        "ata6289" => elf::EF_AVR_ARCH_AVR4,
        "atmega8a" => elf::EF_AVR_ARCH_AVR4,
        "ata6285" => elf::EF_AVR_ARCH_AVR4,
        "ata6286" => elf::EF_AVR_ARCH_AVR4,
        "ata6612c" => elf::EF_AVR_ARCH_AVR4,
        "atmega48" => elf::EF_AVR_ARCH_AVR4,
        "atmega48a" => elf::EF_AVR_ARCH_AVR4,
        "atmega48pa" => elf::EF_AVR_ARCH_AVR4,
        "atmega48pb" => elf::EF_AVR_ARCH_AVR4,
        "atmega48p" => elf::EF_AVR_ARCH_AVR4,
        "atmega88" => elf::EF_AVR_ARCH_AVR4,
        "atmega88a" => elf::EF_AVR_ARCH_AVR4,
        "atmega88p" => elf::EF_AVR_ARCH_AVR4,
        "atmega88pa" => elf::EF_AVR_ARCH_AVR4,
        "atmega88pb" => elf::EF_AVR_ARCH_AVR4,
        "atmega8515" => elf::EF_AVR_ARCH_AVR4,
        "atmega8535" => elf::EF_AVR_ARCH_AVR4,
        "atmega8hva" => elf::EF_AVR_ARCH_AVR4,
        "at90pwm1" => elf::EF_AVR_ARCH_AVR4,
        "at90pwm2" => elf::EF_AVR_ARCH_AVR4,
        "at90pwm2b" => elf::EF_AVR_ARCH_AVR4,
        "at90pwm3" => elf::EF_AVR_ARCH_AVR4,
        "at90pwm3b" => elf::EF_AVR_ARCH_AVR4,
        "at90pwm81" => elf::EF_AVR_ARCH_AVR4,
        "ata5702m322" => elf::EF_AVR_ARCH_AVR5,
        "ata5782" => elf::EF_AVR_ARCH_AVR5,
        "ata5790" => elf::EF_AVR_ARCH_AVR5,
        "ata5790n" => elf::EF_AVR_ARCH_AVR5,
        "ata5791" => elf::EF_AVR_ARCH_AVR5,
        "ata5795" => elf::EF_AVR_ARCH_AVR5,
        "ata5831" => elf::EF_AVR_ARCH_AVR5,
        "ata6613c" => elf::EF_AVR_ARCH_AVR5,
        "ata6614q" => elf::EF_AVR_ARCH_AVR5,
        "ata8210" => elf::EF_AVR_ARCH_AVR5,
        "ata8510" => elf::EF_AVR_ARCH_AVR5,
        "atmega16" => elf::EF_AVR_ARCH_AVR5,
        "atmega16a" => elf::EF_AVR_ARCH_AVR5,
        "atmega161" => elf::EF_AVR_ARCH_AVR5,
        "atmega162" => elf::EF_AVR_ARCH_AVR5,
        "atmega163" => elf::EF_AVR_ARCH_AVR5,
        "atmega164a" => elf::EF_AVR_ARCH_AVR5,
        "atmega164p" => elf::EF_AVR_ARCH_AVR5,
        "atmega164pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega165" => elf::EF_AVR_ARCH_AVR5,
        "atmega165a" => elf::EF_AVR_ARCH_AVR5,
        "atmega165p" => elf::EF_AVR_ARCH_AVR5,
        "atmega165pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega168" => elf::EF_AVR_ARCH_AVR5,
        "atmega168a" => elf::EF_AVR_ARCH_AVR5,
        "atmega168p" => elf::EF_AVR_ARCH_AVR5,
        "atmega168pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega168pb" => elf::EF_AVR_ARCH_AVR5,
        "atmega169" => elf::EF_AVR_ARCH_AVR5,
        "atmega169a" => elf::EF_AVR_ARCH_AVR5,
        "atmega169p" => elf::EF_AVR_ARCH_AVR5,
        "atmega169pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega32" => elf::EF_AVR_ARCH_AVR5,
        "atmega32a" => elf::EF_AVR_ARCH_AVR5,
        "atmega323" => elf::EF_AVR_ARCH_AVR5,
        "atmega324a" => elf::EF_AVR_ARCH_AVR5,
        "atmega324p" => elf::EF_AVR_ARCH_AVR5,
        "atmega324pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega324pb" => elf::EF_AVR_ARCH_AVR5,
        "atmega325" => elf::EF_AVR_ARCH_AVR5,
        "atmega325a" => elf::EF_AVR_ARCH_AVR5,
        "atmega325p" => elf::EF_AVR_ARCH_AVR5,
        "atmega325pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega3250" => elf::EF_AVR_ARCH_AVR5,
        "atmega3250a" => elf::EF_AVR_ARCH_AVR5,
        "atmega3250p" => elf::EF_AVR_ARCH_AVR5,
        "atmega3250pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega328" => elf::EF_AVR_ARCH_AVR5,
        "atmega328p" => elf::EF_AVR_ARCH_AVR5,
        "atmega328pb" => elf::EF_AVR_ARCH_AVR5,
        "atmega329" => elf::EF_AVR_ARCH_AVR5,
        "atmega329a" => elf::EF_AVR_ARCH_AVR5,
        "atmega329p" => elf::EF_AVR_ARCH_AVR5,
        "atmega329pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega3290" => elf::EF_AVR_ARCH_AVR5,
        "atmega3290a" => elf::EF_AVR_ARCH_AVR5,
        "atmega3290p" => elf::EF_AVR_ARCH_AVR5,
        "atmega3290pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega406" => elf::EF_AVR_ARCH_AVR5,
        "atmega64" => elf::EF_AVR_ARCH_AVR5,
        "atmega64a" => elf::EF_AVR_ARCH_AVR5,
        "atmega640" => elf::EF_AVR_ARCH_AVR5,
        "atmega644" => elf::EF_AVR_ARCH_AVR5,
        "atmega644a" => elf::EF_AVR_ARCH_AVR5,
        "atmega644p" => elf::EF_AVR_ARCH_AVR5,
        "atmega644pa" => elf::EF_AVR_ARCH_AVR5,
        "atmega645" => elf::EF_AVR_ARCH_AVR5,
        "atmega645a" => elf::EF_AVR_ARCH_AVR5,
        "atmega645p" => elf::EF_AVR_ARCH_AVR5,
        "atmega649" => elf::EF_AVR_ARCH_AVR5,
        "atmega649a" => elf::EF_AVR_ARCH_AVR5,
        "atmega649p" => elf::EF_AVR_ARCH_AVR5,
        "atmega6450" => elf::EF_AVR_ARCH_AVR5,
        "atmega6450a" => elf::EF_AVR_ARCH_AVR5,
        "atmega6450p" => elf::EF_AVR_ARCH_AVR5,
        "atmega6490" => elf::EF_AVR_ARCH_AVR5,
        "atmega6490a" => elf::EF_AVR_ARCH_AVR5,
        "atmega6490p" => elf::EF_AVR_ARCH_AVR5,
        "atmega64rfr2" => elf::EF_AVR_ARCH_AVR5,
        "atmega644rfr2" => elf::EF_AVR_ARCH_AVR5,
        "atmega16hva" => elf::EF_AVR_ARCH_AVR5,
        "atmega16hva2" => elf::EF_AVR_ARCH_AVR5,
        "atmega16hvb" => elf::EF_AVR_ARCH_AVR5,
        "atmega16hvbrevb" => elf::EF_AVR_ARCH_AVR5,
        "atmega32hvb" => elf::EF_AVR_ARCH_AVR5,
        "atmega32hvbrevb" => elf::EF_AVR_ARCH_AVR5,
        "atmega64hve" => elf::EF_AVR_ARCH_AVR5,
        "atmega64hve2" => elf::EF_AVR_ARCH_AVR5,
        "at90can32" => elf::EF_AVR_ARCH_AVR5,
        "at90can64" => elf::EF_AVR_ARCH_AVR5,
        "at90pwm161" => elf::EF_AVR_ARCH_AVR5,
        "at90pwm216" => elf::EF_AVR_ARCH_AVR5,
        "at90pwm316" => elf::EF_AVR_ARCH_AVR5,
        "atmega32c1" => elf::EF_AVR_ARCH_AVR5,
        "atmega64c1" => elf::EF_AVR_ARCH_AVR5,
        "atmega16m1" => elf::EF_AVR_ARCH_AVR5,
        "atmega32m1" => elf::EF_AVR_ARCH_AVR5,
        "atmega64m1" => elf::EF_AVR_ARCH_AVR5,
        "atmega16u4" => elf::EF_AVR_ARCH_AVR5,
        "atmega32u4" => elf::EF_AVR_ARCH_AVR5,
        "atmega32u6" => elf::EF_AVR_ARCH_AVR5,
        "at90usb646" => elf::EF_AVR_ARCH_AVR5,
        "at90usb647" => elf::EF_AVR_ARCH_AVR5,
        "at90scr100" => elf::EF_AVR_ARCH_AVR5,
        "at94k" => elf::EF_AVR_ARCH_AVR5,
        "m3000" => elf::EF_AVR_ARCH_AVR5,
        "atmega128" => elf::EF_AVR_ARCH_AVR51,
        "atmega128a" => elf::EF_AVR_ARCH_AVR51,
        "atmega1280" => elf::EF_AVR_ARCH_AVR51,
        "atmega1281" => elf::EF_AVR_ARCH_AVR51,
        "atmega1284" => elf::EF_AVR_ARCH_AVR51,
        "atmega1284p" => elf::EF_AVR_ARCH_AVR51,
        "atmega128rfa1" => elf::EF_AVR_ARCH_AVR51,
        "atmega128rfr2" => elf::EF_AVR_ARCH_AVR51,
        "atmega1284rfr2" => elf::EF_AVR_ARCH_AVR51,
        "at90can128" => elf::EF_AVR_ARCH_AVR51,
        "at90usb1286" => elf::EF_AVR_ARCH_AVR51,
        "at90usb1287" => elf::EF_AVR_ARCH_AVR51,
        "atmega2560" => elf::EF_AVR_ARCH_AVR6,
        "atmega2561" => elf::EF_AVR_ARCH_AVR6,
        "atmega256rfr2" => elf::EF_AVR_ARCH_AVR6,
        "atmega2564rfr2" => elf::EF_AVR_ARCH_AVR6,
        "atxmega16a4" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega16a4u" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega16c4" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega16d4" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega32a4" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega32a4u" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega32c3" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega32c4" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega32d3" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega32d4" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega32e5" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega16e5" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega8e5" => elf::EF_AVR_ARCH_XMEGA2,
        "atxmega64a3" => elf::EF_AVR_ARCH_XMEGA4,
        "atxmega64a3u" => elf::EF_AVR_ARCH_XMEGA4,
        "atxmega64a4u" => elf::EF_AVR_ARCH_XMEGA4,
        "atxmega64b1" => elf::EF_AVR_ARCH_XMEGA4,
        "atxmega64b3" => elf::EF_AVR_ARCH_XMEGA4,
        "atxmega64c3" => elf::EF_AVR_ARCH_XMEGA4,
        "atxmega64d3" => elf::EF_AVR_ARCH_XMEGA4,
        "atxmega64d4" => elf::EF_AVR_ARCH_XMEGA4,
        "atxmega64a1" => elf::EF_AVR_ARCH_XMEGA5,
        "atxmega64a1u" => elf::EF_AVR_ARCH_XMEGA5,
        "atxmega128a3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega128a3u" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega128b1" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega128b3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega128c3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega128d3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega128d4" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega192a3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega192a3u" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega192c3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega192d3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega256a3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega256a3u" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega256a3b" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega256a3bu" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega256c3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega256d3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega384c3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega384d3" => elf::EF_AVR_ARCH_XMEGA6,
        "atxmega128a1" => elf::EF_AVR_ARCH_XMEGA7,
        "atxmega128a1u" => elf::EF_AVR_ARCH_XMEGA7,
        "atxmega128a4u" => elf::EF_AVR_ARCH_XMEGA7,
        "attiny4" => elf::EF_AVR_ARCH_AVRTINY,
        "attiny5" => elf::EF_AVR_ARCH_AVRTINY,
        "attiny9" => elf::EF_AVR_ARCH_AVRTINY,
        "attiny10" => elf::EF_AVR_ARCH_AVRTINY,
        "attiny20" => elf::EF_AVR_ARCH_AVRTINY,
        "attiny40" => elf::EF_AVR_ARCH_AVRTINY,
        "attiny102" => elf::EF_AVR_ARCH_AVRTINY,
        "attiny104" => elf::EF_AVR_ARCH_AVRTINY,
        "attiny202" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny402" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny204" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny404" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny804" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1604" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny406" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny806" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1606" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny807" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1607" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny212" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny412" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny214" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny414" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny814" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1614" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny416" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny816" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1616" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny3216" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny417" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny817" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1617" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny3217" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1624" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1626" => elf::EF_AVR_ARCH_XMEGA3,
        "attiny1627" => elf::EF_AVR_ARCH_XMEGA3,
        "atmega808" => elf::EF_AVR_ARCH_XMEGA3,
        "atmega809" => elf::EF_AVR_ARCH_XMEGA3,
        "atmega1608" => elf::EF_AVR_ARCH_XMEGA3,
        "atmega1609" => elf::EF_AVR_ARCH_XMEGA3,
        "atmega3208" => elf::EF_AVR_ARCH_XMEGA3,
        "atmega3209" => elf::EF_AVR_ARCH_XMEGA3,
        "atmega4808" => elf::EF_AVR_ARCH_XMEGA3,
        "atmega4809" => elf::EF_AVR_ARCH_XMEGA3,

        // Unknown target CPU => Unspecified/generic code
        _ => 0,
    }
}
