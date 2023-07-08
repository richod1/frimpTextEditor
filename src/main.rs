use std::io::{self,stdout,Read};
use termion::raw::IntoRawMode;


// control function to exit program
fn to_ctrl_byte(c:char)->u8{
    let byte=c as u8;
    byte & 0b0001_1111
}

// error handling from paninickinkg
fn die(e:std::io::Error){
    panic!("{}",e);
}
fn main() {
let _stdout=stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes(){
        // let b=b.unwrap();
        // let c=b as char;
        // // println!("{}",c);
        // if c.is_control(){
        //     println!("{:?}\r",b);
        // }else{
        //     println!("{:?} ({})\r",b,c);
        // }
        // // replace with ctrl function
        // // to exit program ctrl q

        // if b ==to_ctrl_byte('q'){
        //     break;
        // }
    match b{
        Ok(b) if b== to_ctrl_byte('q') => {
            break;
        }
        Ok(_b) => {}
        Err(err)=>die(err),
       
    }
   
    }
}
