fn main() {
    println!("Hello, world!");
}

pub fn kata4(arr: &mut [i32]) -> Vec<i32>{
    let len = arr.len();
    for left in 0..len{
        let mut smallest = left;
        for right in (left+1)..len{
            if arr[right]<arr[smallest]{
                smallest = right;
            }
        }
        arr.swap(smallest,left);
    }
    let mut temp = Vec::new();
    for i in 0..len-1{
        if arr[i] != arr[i+1]{
            temp.push(arr[i]);
        }
    }
    temp.push(arr[len-1]); 
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
   #[test]
   fn escenario312(){
       let mut array: [i32;5] = [4,5,3,2,1];
       let actual = super::kata4(&mut array);
       assert_eq!(actual, [1,2,3,4,5]);
  }
  #[test]
  fn escenario313(){
      let mut array: [i32;6] = [3,3,2,2,1,1];
      let actual = super::kata4(&mut array);
      assert_eq!(actual, [1,2,3]);
 }
}