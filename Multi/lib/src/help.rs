pub fn usage() {
    let script_version = "1.0.1"; 
println!(r#"♨️ Graphic circle {} - Mobile Legends and FPS Optimization Utility
   
Usage:
   Graphic circle

Option:
 -O          Which supports 120Hz choose this plus unlock Graphic
 -L           Which supports 90Hz choose this plus unlock Graphic
 -Q           Which supports 60Hz choose this plus unlock Graphic
 -h --help.   Show this help message and exit.
 
Discription:
  Graphic circle is a script module designed to open all Mobile Legends 
  graphic settings without requiring root access, and I also added 
  optimization code to help stabilize FPS in the game.
 
Notes Critical:
  Just open the graphics settings, the FPS problem depends 
  on how many Hz the device supports. And the choice in 
  that option is also not to open the device's Hz, but only to 
  release the FPS which is often only read as 60Hz even 
  though our device actually supports 120Hz or 90Hz.

Examples:
   Applay 120Hz if your device supports  
       adjust -O
   Applay 90Hz if your device supports  
       adjust -L
   Applay 60Hz if your device supports  
       adjust -Q

System Requirements:
  - Device must have access to the necessary configuration files
  - Only support on Mobile Legends 64bit 
  - Access to Android data folder is required
  - The device must support 'adb shell' and necessary shell utilities such as setprop, pm, and cmd.

For more information, visit:
Android Debugging Guide: https://developer.android.com/studio/command-line/adb
What is config  : https://segi-tekno.com/config-mobile-legends-lag/#Apa_Itu_Config
"#,
    script_version);
}