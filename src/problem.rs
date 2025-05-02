use std::time::Instant;

pub trait Problem {
    /**
    The core logic function. Must be implemented by each problem.

    NOTE: This method is intended for internal use by the `solve_and_report`
    method. Users of the trait should typically call `solve_and_report`.
     */
    #[doc(hidden)]
    fn run(&self) -> i128;

    /**
    Runs the problem's solution, measures execution time, and prints the result.
    This is the intended entry point for solving a problem.
    */
    fn solve_and_report(&self) {
        println!("Solving...");
        let start_time = Instant::now();
        let solution = self.run();
        let elapsed_time = start_time.elapsed();

        println!("--------------------");
        println!("Solution: {}", solution);
        println!("Time elapsed: {:?}", elapsed_time);
        println!("--------------------");
    }
}
