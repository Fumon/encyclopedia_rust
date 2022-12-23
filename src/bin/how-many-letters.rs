// I forget how many letters there are sometimes...

fn main() {
    println!(
        "There are {} letters in the English language.",
        ('a'..='z').map(|_| 1).sum::<u8>()
    );
}
