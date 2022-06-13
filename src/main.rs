#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(genso_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use genso_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let s = r#"
                          || Welcome to Genso OS ||
________________________________________________________________________________

       .88888.                                          .88888.  .d88888b  
      d8'   `88                                        d8'   `8b 88.    "' 
      88        .d8888b. 88d888b. .d8888b. .d8888b.    88     88 `Y88888b. 
      88   YP88 88ooood8 88'  `88 Y8ooooo. 88'  `88    88     88       `8b 
      Y8.   .88 88.  ... 88    88       88 88.  .88    Y8.   .8P d8'   .8P 
       `88888'  `88888P' dP    dP `88888P' `88888P'     `8888P'   Y88888P  

________________________________________________________________________________





                                                           ___               
                                                           | |  Now Loading...  
    "#;
    println!("{}", s);

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    genso_os::test_panic_handler(info)
}
