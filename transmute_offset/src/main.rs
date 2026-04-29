struct ByteInspector {
    address: *const u8,
}

impl ByteInspector {
    fn peek(&self, i: isize) -> u8 {
        let byte = unsafe { *self.address.offset(i) };
        byte
    }
}

fn main() {
    let my_number = 12345678;
    let raw_address = &raw const my_number as *const u8;
    // a raw adress is just a location in the RAM where our variable is
    // and we ask the pc here basically to forget what the type is and just give us the 8bits of raw data where our variable begins
    let inspector = ByteInspector {
        address: raw_address,
    };
    for i in 0..4 {
        let byte = inspector.peek(i as isize);
        println!("Byte {}: {:02x}", i, byte);
    }
    // this means from 0..4 run this loop 4 times, because we need to look at the building blocks of a 32bit integer which is 4 x 8 bits (1byte)

    let my_string = String::from("Whats Up");
    let components = unsafe {
        std::mem::transmute::<String, [usize; 3]>(my_string)
        //the transmute here needs to convert a type to another type with the exact same size so, a string (24 bytes) needs usize (8bytes) x 3
        // its breaks down a

        // Also when we called std::mem::transmute::<String, [usize; 3]>(my_string), we effectively kidnapped that string
        // now that its an array of numbers rust doesnt know its a string anymore and wont call the proper drop logic, which is essentially a memory leak
    };
    println!("Pointer:  0x{:x}", components[0]); // Where the text is in RAM
    println!("Capacity: {}", components[1]); // Total space reserved
    println!("Length:   {}", components[2]); // Bytes actually used

    let heap_pointer = components[1] as *const u8;
    // the raw pointer we took (Where the text is in RAM)

    let heap_inspector = ByteInspector {
        address: heap_pointer,
    };
    // we initialize the struct to tell the inspector to look at the specific adress where the data lives

    println!("peeking at the heap");
    for i in 0..components[0] {
        //the 'Length' we found
        let byte = heap_inspector.peek(i as isize);
        println!("Byte {}: {:02x} ('{}')", i, byte, byte as char);
        // print the char itself so we can see it
    }

    // To avoid the memory leak i mentioned before we can do this
    // essentially what we did with transmute before but in reverse
    let _reconstructed_string = unsafe { std::mem::transmute::<[usize; 3], String>(components) };

    // we don't do anything else.
    // when 'main' ends, Rust sees '_reconstructed_string' is a String
    // and calls the proper 'drop()' logic to clean up the Heap.
}

/*

On this machine/compiler version, the String layout was not [Ptr, Cap, Len].
Based on the terminal output and the resulting Segfault:

1. components[0] = 0x8 (This was actually the LENGTH of "Whats Up")
2. components[1] = 95351191829264 (This was the actual HEAP POINTER)
3. components[2] = 8 (This was the CAPACITY)

I ran into a Segfault because I tried to use 0x8 as a memory address.
By pointing the ByteInspector at components[1] instead, I successfully
accessed the heap.


**********************************************************************

If you dont wanna be like me and find out the hard way you can just use #[repr(C)] to recieve the data in the order of C programing language
EDIT: nevermind apparently this only works for structs you define not for standart library stuff

**********************************************************************
Also i verified my unsafe code using Miri and discovered an 'Undefined Behavior' error related to Pointer Provenance. While the code executes correctly on hardware,
Miri correctly identified that transmuting a pointer to a usize strips its metadata. But for todays lesson i will just call it a day


*/
