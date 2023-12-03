fn main(){
    //initialize a mutable tuple
    let mut moutain_heights = ("Everest", 8848, "Fishtail",6993);

    println!("Original tuple = {:?}",moutain_heights);

    //change 3rd and 4th element of a muttable tuple
    moutain_heights.2 = "Lhotse";
    moutain_heights.3 = 8516;

    println!("Changed tuple = {:?}",moutain_heights );
}