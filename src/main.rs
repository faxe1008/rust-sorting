fn main() {
    let mut unsorted : [u16; 10] = [3,1,4,2,8,9,10,5,7,6];
    
    //bubble_sort(&mut unsorted);
    //insertion_sort(&mut unsorted);
    selection_sort(&mut unsorted);

    print_sorted_array(&unsorted);
}

fn bubble_sort(arr: &mut [u16]) {
    let mut sorted : bool = false;
    while(!sorted)
    {
        for x in 0..arr.len()-1{
            let r_elem = arr[x+1];

            if(arr[x]  > r_elem)
            {
                arr[x+1] = arr[x];
                arr[x] = r_elem;
                sorted = false;
                break;
            }
            sorted = true;
        }
    }
}

fn insertion_sort(arr: &mut [u16]){
    for x in 1..arr.len()
    {
        let mut index = x; 
        while arr[index] < arr[index-1] && index > 0
        {
            let buffer = arr[index];
            arr[index] = arr[index-1];
            arr[index-1] = buffer;
            index -= 1;
        }
    }
}

fn selection_sort(arr: &mut [u16]){
    for i in 0..arr.len(){
        let mut index = i;
        let mut min_index = i;
        for x in index..arr.len()
        {
            if arr[min_index] > arr[x]
            {
                min_index = x
            }
        }
        let buffer = arr[index];
        arr[index] = arr[min_index];
        arr[min_index] = buffer;
    }    
}


fn print_sorted_array(arr: &[u16])
{
    for x in arr {
        for len in 0..*x{
            print!("X");
        }
        print!("\n");
    }
}
