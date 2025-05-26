use rust_web::bitwisef2linalg::Mat2;

fn main(){
    let matrix = Mat2::id(2);
    println!("{}", matrix);
    let matrix2 = Mat2::from_u8(vec![
        vec![1, 1],
        vec![0, 1],
    ]);
    println!("{}", matrix2);
    let matrix3 = matrix * matrix2;
    println!("{}", matrix3);

}


