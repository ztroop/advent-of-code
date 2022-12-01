mod input;

fn main() {
    if let Ok(lines) = input::read_lines("./input.file") {
        let mut elves = Vec::<usize>::new();
        elves.push(0);

        let mut elf_index: usize = 0;

        for line in lines {
            let value = line.unwrap();
            if value == "" {
                elf_index = elf_index + 1;
                elves.push(0);
                continue
            }
            let value = value.trim().parse::<usize>().unwrap();
            elves[elf_index] = elves[elf_index] + value;
        }

        let p1_answer = elves.iter().max().unwrap();

        println!("Part one answer is: {:?}", p1_answer);

        elves.sort();
        let p2_answer = &elves[(elf_index - 2)..].iter().sum::<usize>();

        println!("Part two answer is: {:?}", p2_answer);
    }
}
