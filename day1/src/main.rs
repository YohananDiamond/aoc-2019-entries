// https://adventofcode.com/2019/day/1
// Includes code for part 1 and part 2

fn main() {
    // part_1();
    part_2();
}

fn part_1() {
    let masses: Vec<&str> = PUZZLE_INPUT.split(" ").collect();
    let mut fuel_sum: f64 = 0.0;
    for number_str in masses {
        let fuel_required = do_the_maths(number_str.parse::<f64>().unwrap());
        fuel_sum += fuel_required;
    }
    println!("{}", fuel_sum);
}

fn part_2() {
    let masses: Vec<&str> = PUZZLE_INPUT.split(" ").collect();
    let mut fuel_requeriments = 0.0;
    for mass_str in masses {
        let mass = mass_str.parse::<f64>().unwrap();
        fuel_requeriments += module_mass_and_fuel(mass);
    }
    println!("{}", fuel_requeriments);
}

fn do_the_maths(mass: f64) -> f64 {
    /*
     * Take its mass,
     * divide by three,
     * round down,
     * subtract 2
     */
    let calculation = (((mass / 3.0) as i64) - 2) as f64;
    if calculation > 0.0 { calculation } else { 0.0f64 }
}

fn module_mass_and_fuel(mass: f64) -> f64 {
    // let mut vec = Vec::new();
    let mut calc = do_the_maths(mass);
    let mut sum = calc; // calc is on stack memory, so we can simply call it here.
    while calc != 0.0 {
        // vec.push(calc);
        calc = do_the_maths(calc);
        sum += calc;
    }
    // vec
    sum
}

const PUZZLE_INPUT: &str = "88397 140448 79229 122289 143507 71642 145178 149729 104257 109287 136937 131253 88847 143302 104210 56054 137178 134861 117151 103772 135590 64319 53682 101137 52772 142235 88312 146564 131670 74925 126276 109028 95438 56083 77649 135414 52079 83883 92754 69122 77489 142896 126195 78749 133146 107841 75897 70156 128501 113859 64823 147935 72855 139576 125827 57409 113492 85048 89204 68744 120464 118813 102856 117750 130545 65139 77010 139609 88580 104355 99680 82451 141198 142489 121556 66616 121318 149517 135978 126001 70211 73221 52727 82621 143301 64186 75382 130742 135248 129867 78189 148444 95969 106317 147315 81697 131555 56152 105759 117769";
