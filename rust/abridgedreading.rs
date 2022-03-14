mod fast_input;
use fast_input::FastInput;

use std::collections::HashSet;

#[derive(Default, Clone, Copy)]
struct Chapter {
    page_count: u32,
    previous: Option<usize>,
}

fn main() {
    let inp = FastInput::new();
    let (n, m) = inp.next_tuple();
    let mut culminating_chapters: HashSet<_> = (0..n).collect();
    let mut chapters = vec![Chapter::default(); n];
    let mut read_chapters = 0;

    while read_chapters < n {
        for pages in inp.next_as_iter() {
            chapters[read_chapters].page_count = pages;
            read_chapters += 1;
        }
    }

    for _ in 0..m {
        let (a, b): (usize, usize) = inp.next_tuple();
        let (a, b) = (a - 1, b - 1);
        culminating_chapters.remove(&a);
        chapters[b].previous = Some(a);
    }
    
    let culminating_chapters: Vec<_> = culminating_chapters.drain().collect();
    let mut best = core::u32::MAX;
    let mut visited = vec![false; n];
    for i in 0..culminating_chapters.len() {
        'case: for j in (i + 1)..culminating_chapters.len() {
            visited.iter_mut().for_each(|v| *v = false);
            let mut chapter = chapters[culminating_chapters[i]];

            let mut cost = chapter.page_count;

            while let Some(prev) = chapter.previous {
                let current_chapter = chapters[prev]; 
                visited[prev] = true;
                cost += current_chapter.page_count; 

                if cost > best {
                    continue 'case;
                }

                chapter = current_chapter;
            }

            chapter = chapters[culminating_chapters[j]];
            cost += chapter.page_count;
            while let Some(prev) = chapter.previous {
                let current_chapter = chapters[prev]; 
                if !visited[prev] {
                    cost += current_chapter.page_count; 
                    visited[prev] = true;

                    if cost > best {
                        continue 'case;
                    }
                }
                chapter = current_chapter;
            }
            best = best.min(cost);
        }
    }
    println!("{}", best);
}