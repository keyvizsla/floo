fn main() {
    // Make users that have not read the docs aware of how to proceed
    println!("cargo:warning= ");
    println!("cargo:warning=  +-----------------------------------------------------------+");
    println!("cargo:warning=  |  🎉 SUCCESS! INSTALLATION COMPLETE                        |");
    println!("cargo:warning=  +-----------------------------------------------------------+");
    println!("cargo:warning=  |                                                           |");
    println!("cargo:warning=  |  To activate, add this to your ~/.bashrc or ~/.zshrc:     |");
    println!("cargo:warning=  |  eval \"$(floo-bin init)\"                                  |");
    println!("cargo:warning=  |  and restart your terminal or source the altered rc-file  |");
    println!("cargo:warning=  |                                                           |");
    println!("cargo:warning=  +-----------------------------------------------------------+");
    println!("cargo:warning= ");

    println!("cargo:rerun-if-changed=build.rs");
}
