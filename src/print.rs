pub fn run() {
  // Basic formatting
  println!("{} is so {}", "Dylan", "Weird");
  
  // Positional Arguments
  println!(
    "{0} is {1} and {0} is {2}",
    "Dylan", "weird", "strange"
  );

  // Named Arguments
  println!(
    "{name} likes to play {activity}",
    name = "Dylan",
    activity = "guitar"
  );

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));
}