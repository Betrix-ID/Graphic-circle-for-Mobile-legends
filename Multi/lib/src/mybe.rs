/* 
 notes critical user :
 - You may not change, modify or even put this script into your personal project without written permission from the script creator (official author) 
 
 Telegarm    : @UnixeID | Chenel : @Yeye_PID
 Github      : Betrix-ID
 Consultasi  : betrix322@gmail.com
 
 the date this script was written : 21 - Mei - 2025
*/

use std::{
     path::Path,
     process::Command,
     thread::sleep,
     time::Duration,
};


fn source(path: &str) -> bool {
    Path::new(path).exists()
}

fn shell(message: &str, fps: i32) {
    let cmd = format!(
         " cmd notification post -I /sdcard/Multi/IFS.png -S bigtext -t '♨️ Unlimited Graphic Mobile legends' 'Tag' '{} {} Hz' > /dev/null 2>&1",
         message, fps
        );
    let _ = Command::new("sh").arg("-c").arg(cmd).status();
}

fn slow(delay: u64) {
    sleep(Duration::from_secs(delay));
}

fn inject(fps: i32) {
      println!(
         "Description
           This function optimizes graphics settings and frame rate for 
           Mobile Legends. It configures display settings, adjusts refresh rate, 
           and ensures optimal performance by managing 
           processes and resources."
        );
        
      println!(
         "Set to {} Hz", 
      fps);
    
         if source("/sdcard/Multi/unlimited/com.mobile.legends") {
         let _ = Command::new("sh").arg("-c")
                  .arg("cp -r /sdcard/Multi/unlimited/com.mobile.legends /sdcard/Android/data > /dev/null 2>&1")
                 .status();
                 
     let props = format!(
            "cmd display clear-user-preferred-display-mode 0 && 
             cmd display set-user-preferred-display-mode $(wm size | awk -F '[ x]+' '{{print $3}}') $(wm size | awk -F '[ x]+' '{{print $4}}') {} 0 && 
             settings put system peak_refresh_rate {} && 
             settings put system user_refresh_rate {} && 
             settings put system min_refresh_rate {} && 
             settings put secure user_refresh_rate {} && 
             cmd package compile -m quicken -f com.mobile.legends >/dev/null 2>&1 && 
             cmd deviceidle except-idle-whitelist +com.mobile.legends > /dev/null 2>&1 && 
             cmd activity clear-watch-heap com.mobile.legends && 
             cmd activity clear-exit-info --user 0 com.mobile.legends && 
             cmd deviceidle disable && 
             cmd activity set-stop-user-on-switch false >/dev/null 2>&1 && 
             sync", fps, fps, fps, fps, fps
         );
     let _ = Command::new("sh").arg("-c").arg(&props).status();
        } else {
            println!("⚠️ Source folder not found !");
               return;
          }
          
         let _ = Command::new("sh").arg("-c")
                  .arg("cmd activity start -n com.mobile.legends/com.moba.unityplugin.MobaGameUnityActivity > /dev/null 2>&1")
                  .status();            
         slow(27);
         
         let _ = Command::new("sh").arg("-c")
                 .arg("cmd activity force-stop --user 0 com.mobile.legends >/dev/null 2>&1")
                 .status();
                 
         if source("/sdcard/Multi/default/com.mobile.legends") {
         let _ = Command::new("sh").arg("-c")
                   .arg("cp -r /sdcard/Multi/default/com.mobile.legends /sdcard/Android/data > /dev/null 2>&1")
                   .status();
        } else {
           println!("⚠️ Source folder not found !");
        }
      
      slow(1);
      shell("Success: Apply settings ", fps);
}
      
pub fn custem_hz_120() {
    inject(120);
}
    
pub fn custem_hz_90() {
     inject(90);
}

pub fn custem_hz_60() {
    inject(60);
}