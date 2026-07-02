use std::io;

use rand::Rng;

fn main() {
    let mut input = String::new();
    let mut answer = String::new();
    let mut lives = 5;
    let mut number = 0;
    let word = rand::random_range(1..=5);
    if word == 1 {
        answer = String::from("apple");
        let answer_in_tuple: (&str, &str, &str, &str, &str) = ("a", "p", "p", "l", "e");
        let mut answered_A = false;
        let mut answered_P = false;
        let mut answered_L = false;
        let mut answered_E = false;
        loop {
            if lives == 0 {
                println!("Game over! The correct answer was {answer}.");
                break;
            } else if  answered_A == true && answered_P == true && answered_L == true && answered_E == true {
                println!("Congratulations! You have guessed the word {answer} correctly!");
                break;
            }
            if answered_A == true {
                print!("A");
            } else {
                print!("_ ");
            }
            if answered_P == true {
                print!("P");
            } else {
                print!("_ ");
            }
            if answered_P == true {
                print!("P");
            } else {
                print!("_ ");
            }
            if answered_L == true {
                print!("L");
            } else {
                print!("_ ");
            }
            if answered_E == true {
                print!("E");
            } else {
                print!("_ ");
            }
            println!("You have {lives} lives left. Please enter a letter: ");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            loop {
                if input.trim() == answer_in_tuple.0 {
                    if answered_A == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_A = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.1 {
                    if answered_P == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_P = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.2 {
                    if answered_P == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_P = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.3 {
                    if answered_L == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_L = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.4 {
                    if answered_E == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_E = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else {
                    lives -= 1;
                    println!("Incorrect! You have {lives} lives left.");
                    input.clear();
                    break;
                }
            }
        }
    } else if word == 2 {
        answer = String::from("banana");
        let answer_in_tuple: (&str, &str, &str, &str, &str, &str) = ("b", "a", "n", "a", "n", "a");
        let mut answered_B = false;
        let mut answered_A = false;
        let mut answered_N = false;
        loop {

            if lives == 0 {
                println!("Game over! The correct answer was {answer}.");
                break;
            } else if answered_B == true && answered_A == true && answered_N == true {
                println!("Congratulations! You have guessed the word {answer} correctly!");
                break;
            }
            if answered_B == true {
                print!("B");
            } else {
                print!("_ ");
            }
            if answered_A == true {
                print!("A");
            } else {
                print!("_ ");
            }
            if answered_N == true {
                print!("N");
            } else {
                print!("_ ");
            }
            if answered_A == true {
                print!("A");
            } else {
                print!("_ ");
            }
            if answered_N == true {
                print!("N");
            } else {
                print!("_ ");
            }
            if answered_A == true {
                print!("A");
            } else {
                print!("_ ");
            }
            println!("You have {lives} lives left. Please enter a letter: ");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            
            loop {
                if input.trim() == answer_in_tuple.0 {
                    if answered_B == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_B = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.1 {
                    if answered_A == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_A = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.2 {
                    if answered_N == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_N = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.3 {
                    if answered_A == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_A = true;
                        println!("Correct!");
                    }
                    break;
                } else if input.trim() == answer_in_tuple.4 {
                    if answered_N == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_N = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.5 {
                    if answered_A == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_A = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else {
                    lives -= 1;
                    println!("Incorrect! You have {lives} lives left.");
                    input.clear();
                    break;
                }
            }
        }
    } else if word == 3 {
        answer = String::from("cherry");
        let answer_in_tuple: (&str, &str, &str, &str, &str, &str) = ("c", "h", "e", "r", "r", "y");
        let mut answered_C = false;
        let mut answered_H = false;
        let mut answered_E = false;
        let mut answered_R = false;
        let mut answered_Y = false;
        loop {
            if lives == 0 {
                println!("Game over! The correct answer was {answer}.");
                break;
            } else if answered_C == true && answered_H == true && answered_E == true && answered_R == true && answered_Y == true {
                println!("Congratulations! You have guessed the word {answer} correctly!");
                break;
            }
            if answered_C == true {
                print!("C");
            } else {
                print!("_ ");
            }
            if answered_H == true {
                print!("H");
            } else {
                print!("_ ");
            }
            if answered_E == true {
                print!("E");
            } else {
                print!("_ ");
            }
            if answered_R == true {
                print!("R");
            } else {
                print!("_ ");
            }
            if answered_R == true {
                print!("R");
            } else {
                print!("_ ");
            }
            if answered_Y == true {
                print!("Y");
            } else {
                print!("_ ");
            }
            println!("You have {lives} lives left. Please enter a letter: ");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");

            loop {
                if input.trim() == answer_in_tuple.0 {
                    if answered_C == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_C = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.1 {
                    if answered_H == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_H = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.2 {
                    if answered_E == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_E = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.3 {
                    if answered_R == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_R = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.4 {
                    if answered_R == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_R = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.5 {
                    if answered_Y == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_Y = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else {
                    lives -= 1;
                    println!("Incorrect! You have {lives} lives left.");
                    input.clear();
                    break;
                }
            }
        }
    } else if word == 4 {
        answer = String::from("date");
        let answer_in_tuple: (&str, &str, &str, &str) = ("d", "a", "t", "e");
        let mut answered_D = false;
        let mut answered_A = false;
        let mut answered_T = false;
        let mut answered_E = false;
        loop {
            if lives == 0 {
                println!("Game over! The correct answer was {answer}.");
                break;
            } else if answered_D == true && answered_A == true && answered_T == true && answered_E == true {
                println!("Congratulations! You have guessed the word {answer} correctly!");
                break;
            }
            if answered_D == true {
                print!("D");
            } else {
                print!("_ ");
            }
            if answered_A == true {
                print!("A");
            } else {
                print!("_ ");
            }
            if answered_T == true {
                print!("T");
            } else {
                print!("_ ");
            }
            if answered_E == true {
                print!("E");
            } else {
                print!("_ ");
            }
            println!("You have {lives} lives left. Please enter a letter: ");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");

            loop {
                if input.trim() == answer_in_tuple.0 {
                    if answered_D == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_D = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.1 {
                    if answered_A == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_A = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.2 {
                    if answered_T == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_T = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.3 {
                    if answered_E == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_E = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else {
                    lives -= 1;
                    println!("Incorrect! You have {lives} lives left.");
                    input.clear();
                    break;
                }
            }
        }
    } else if word == 5 {
        answer = String::from("berry");
        let answer_in_tuple: (&str, &str, &str, &str, &str) = ("b", "e", "r", "r", "y");
        let mut answered_B = false;
        let mut answered_E = false;
        let mut answered_R = false;
        let mut answered_Y = false;
        loop {
            if lives == 0 {
                println!("Game over! The correct answer was {answer}.");
                break;
            } else if answered_B == true && answered_E == true && answered_R == true && answered_R == true && answered_Y == true {
                println!("Congratulations! You have guessed the word {answer} correctly!");
                break;
            }
            if answered_B == true {
                print!("B");
            } else {
                print!("_ ");
            }
            if answered_E == true {
                print!("E");
            } else {
                print!("_ ");
            }
            if answered_R == true {
                print!("R");
            } else {
                print!("_ ");
            }
            if answered_R == true {
                print!("R");
            } else {
                print!("_ ");
            }
            if answered_Y == true {
                print!("Y");
            } else {
                print!("_ ");
            }
            println!("You have {lives} lives left. Please enter a letter: ");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");

            loop {
                if input.trim() == answer_in_tuple.0 {
                    if answered_B == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_B = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.1 {
                    if answered_E == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_E = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.2 {
                    if answered_R == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_R = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.3 {
                    if answered_R == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_R = true;
                        println!("Correct!");
                    }
                    input.clear();
                    break;
                } else if input.trim() == answer_in_tuple.4 {
                    if answered_Y == true {
                        println!("You have already answered this letter.");
                    } else {
                        answered_Y = true;
                         println!("Correct!");
                    }
                    input.clear();
                    break;
                } else {
                    lives -= 1;
                    println!("Incorrect! You have {lives} lives left.");
                    input.clear();
                    break;
                    }
                }
            }
        }
    }
