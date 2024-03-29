//! #OS
//! 
//! A library for my OS class
pub use self::os::Process;

pub mod os{
    use std::{error::Error, io, process};
    #[derive(Debug)]
    pub struct Process{
        id:u32,
        burst_time:u32,
        turn_around_time:u32,
        wait_time:u32
    }
    impl Process {
        pub fn build (burst_time:u32, id:u32)-> Process {
            Process{
                id,
                burst_time,
                turn_around_time: 0,
                wait_time:0 
            }
        }
        pub fn assign_waiting_turn(&mut self, wait_time: u32){
            self.wait_time = wait_time;
            self.turn_around_time = self.wait_time + self.burst_time;
    
        }
    }
    /// Creates a vector of  process with a length of length variable
    /// 
    /// #Example
    /// 
    /// let length = 2;
    /// let mut processes:`Vec<Process>` = Vec::new();
    /// create_processes(length, &mut processes);
    ///  
    pub fn create_processes(length: u32, processes: &mut Vec<Process>) -> Result< &Vec<Process>, Box<dyn Error>> {
        for i in 0..length{
            let mut burst_time = String::new();
            println!("Enter burst time for process {} : ", i);
            io::stdin().read_line(&mut burst_time)?;        
            let burst_time = burst_time.trim().parse::<u32>()?;
            processes.push(Process::build(burst_time, i));
        }
        Ok(processes)
    }
    pub fn  process_waiting_turn(processes: &mut Vec<Process>){
        let mut  wait_time = 0;
        processes.iter_mut().for_each(|p| {
            p.assign_waiting_turn(wait_time);
            wait_time += p.burst_time;
        });
        processes.iter().for_each(|p| println!("{:?}", p));
    }
    pub fn process_short_job_first(processes: &mut Vec<Process>){
        let mut wait_time = 0;
        processes.sort_by_key(|p| p.burst_time);
        processes.iter_mut().for_each(|p|{
            p.assign_waiting_turn(wait_time);
            wait_time+=p.burst_time;
        });
    }
    pub fn calculate_averages(processes: &Vec<Process>) -> (f32, f32){
        let mut overall_waiting: f32 = 0.0;
        let mut overall_turnaround: f32 = 0.0;
        processes.iter().for_each(|p|{
            overall_waiting+=p.wait_time as f32;
            overall_turnaround+=p.turn_around_time as f32;
        });
        let overall_waiting = overall_waiting / processes.len() as f32; 
        let overall_turnaround = overall_turnaround / processes.len() as f32;
        (overall_waiting, overall_turnaround)
    }
    pub fn print_results(processes: &Vec<Process>, average_tup: (f32, f32)){
        println!("Process\t\t\tBurst Time\t\t\tWaiting Time\t\t\tTurnaround Time");
        processes.iter().for_each(|p| {
            println!("P:{}\t\t\t{}\t\t\t\t\t{}\t\t\t\t\t{}", p.id, p.burst_time, p.wait_time, p.turn_around_time);
        });
        let (average_wait, average_turn) = average_tup;
        println!("Average Waiting Time: {}", average_wait);
        println!("Average Turnaround Time: {}", average_turn);
    }
}