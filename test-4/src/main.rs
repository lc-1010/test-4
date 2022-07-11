fn main() {
       let x = [0,0];
       let abc= sum_u32(&x);
       println!("{:?}",abc);
    //实现一个打印图形面积的函数，
    //它接收一个可以计算面积的类型作为参数，
    //比如圆形，三角形，正方形，需要用到泛型和泛型约束

    //为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同，
    //实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
}


fn sum_u32(ulist:&[u32])  ->Option<u32> { 
    let list  =ulist;
    let max = list.iter().max().unwrap();
    if *max == 0 {
        return Some(0);
    }
    let mut x:u32 = 0;
    for i in ulist.iter() {
        if *i > 0 { 
           if  x.wrapping_add(*i) ==0 {
               return None;
           }else{
                
                 x= x.wrapping_add(*i);
           } 
        }else{
            continue;
        }
    }
    Some(x) 
}

#[test]
fn test(){
    let a= [1,3,4,5,6];
    let b = [u32::MIN,0];
    let c = [u32::MIN,1,u32::MAX];
     let d = [u32::MIN,u32::MAX];
    assert_eq!(sum_u32(&a), Some(19));
    assert_eq!(sum_u32(&b), Some(0));
     assert_eq!(sum_u32(&c), None);
      assert_eq!(sum_u32(&d), Some(u32::MAX));
}
