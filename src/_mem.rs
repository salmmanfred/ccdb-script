pub fn get_size(find_size: Vec<String>) ->u64{
    let mut col: usize = 0;
    for x in find_size{
        col +=  x.chars().collect::<Vec<char>>().len();
        
    }
    col as u64
}