# 🐄 Farm Record Keeper

This is a small Rust project I’m using to practice what I’ve learned so far about **structs**, **functions** and **data organization**.  
It’s simple, think of it as a digital notebook that helps a farmer keep track of animals, how many they have and their condition.

---

## 🌾 What It Does

The program allows you to:
- Create and store basic animal information (name, quantity, condition)
- Add new animals to your farm record
- View your current list of animals in a clean, readable format

Basically, it helps you keep a simple record of what’s in your farm and it's written in pure Rust.

---

## 💡 What I’m Practicing

This project is part of my **Rust learning journey**.  
Here I’m focusing on:
- Structs and how to model real-world data
- Functions and how they make the code cleaner
- Ownership and borrowing (not very clear yet but I'm gradually getting there 😅)
- Thinking in Rust, one step at a time

---

## 🧠 My Thought Process (in plain words)

I wanted to simulate how a farmer might store their animal records —  
so I created a “basket” (a `Vec`) to hold all the animal info.  
Each animal is defined using a struct and I wrote a function to add new animals to the record.

That’s basically how this project started —  
a mix of curiosity, confusion and excitement to see how Rust actually handles data.

---

## ⚙️ How to Run It

If you want to try it out, kindly do this👇:
```bash
# clone the repo
git clone https://github.com/heismint/farm-record.git
          
# navigate into it
cd farm-record

# run the project
cargo run
