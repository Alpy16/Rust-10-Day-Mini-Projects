use std::sync::{Arc, Mutex};
use std::thread::{self};

/*
  DAY 5: Shared Ownership with Arc and Mutex
  -------------------------------------------------------------------------
  ANALOGY:
  - THE TABLE (String): The shared destination for all information.
  - THE TABLE-NUMBER (Arc): Each Dish (Thread) needs the coordinates to find the
    same Table. We clone the NUMBER, but there is only ever one Table.
  - THE SERVICE LOCK (Mutex): Only one Dish can be "placed" on the Table
    at a time to avoid a mess. The thread must wait for the Table to be
    clear before it can update the state.
  - THE SERVICE BILLS (JoinHandles): Main() holds these receipts. We cannot
    close the restaurant (exit) until every bill is settled (joined).

  LOGIC:
  1. Wrap the String (Table) in Arc<Mutex<...>>.
  2. For each thread: Clone the TABLE-NUMBER, move the Dish in, lock the Table,
     and update the "Menu" or "Status".
  3. Join all handles at the end to ensure the last Dish is served.
*/

struct Users {
    name: String,
    room: Arc<Mutex<String>>,
}

fn main() {
    // INITIALIZE THE TABLE: We wrap the string so multiple people can "find" it (Arc)
    // and only one can "touch" it at a time (Mutex).
    let thread_room = Arc::new(Mutex::new(String::from("Room 1")));

    // PREPARING THE DISHES: Each user gets a clone of the Arc (The Table Number).
    // im using clone very liberally here, because cloning an arc is very cheap,
    // the data is not cloned but the pointer is so its n x 8 bytes
    let dish1 = Users {
        name: String::from("Alice"),
        room: Arc::clone(&thread_room),
    };
    let dish2 = Users {
        name: String::from("Bob"),
        room: Arc::clone(&thread_room),
    };
    let dish3 = Users {
        name: String::from("Charlie"),
        room: Arc::clone(&thread_room),
    };

    // THE RECEIPT FOLDER: We need this to store the JoinHandles so main() doesn't quit early.
    let mut bills: Vec<thread::JoinHandle<()>> = vec![];

    // SERVICE START - DISH 1:
    // we use 'move' so 'dish1' is physically carried into the thread.
    let handle1 = thread::spawn(move || {
        // Grabbing the lock (The Service Lock). If someone else is at the table, we wait here.
        let mut room = dish1.room.lock().unwrap();
        // Dereferencing (*) to change the actual String on the Table.
        *room = format!("{} is in {}", dish1.name, *room);
    });
    bills.push(handle1); // Save the bill.

    // SERVICE START - DISH 2:
    let handle2 = thread::spawn(move || {
        let mut room = dish2.room.lock().unwrap();
        *room = format!("{} is in {}", dish2.name, *room);
    });
    bills.push(handle2);

    // SERVICE START - DISH 3:
    let handle3 = thread::spawn(move || {
        let mut room = dish3.room.lock().unwrap();
        *room = format!("{} is in {}", dish3.name, *room);
    });
    bills.push(handle3);

    // THE SETTLEMENT: Main() goes through the folder and waits for every thread to finish.
    for bill in bills {
        bill.join().unwrap();
    }

    // FINAL INSPECTION: Everyone is done. We lock one last time to read the final result.
    println!("Final Table State: {}", *thread_room.lock().unwrap());
}
