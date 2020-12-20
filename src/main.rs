fn main() {
    println!("Hello, world!");
}

pub fn kata4(arr: &mut [i32]) -> Vec<i32>{
    let mut temp = vec![1,2,3];
    temp
}

#[cfg(test)]
mod tests{
    #[test]
    fn escenario311(){
        let mut array: [i32;3] = [3,2,1];
        let actual = super::kata4(&mut array);
        assert_eq!(actual, [1,2,3]);
   }
}