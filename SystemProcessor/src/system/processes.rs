extern crate psutil;

extern crate log;
extern crate simple_logger;

use std::collections::HashMap;
use psutil::process::Process;

struct Component
{
    _name: String,
    _pid: i32,
    _alive: bool,
}

pub struct Processes
{
    component_map: HashMap<u16, Component>,
    id_key:u16,
}

impl Processes
{
    pub fn new() -> Processes
    {
        Processes {component_map: HashMap::new(), id_key: 0}
    }

    fn join(&mut self, component: Component)
    {
        warn!("Adding found process to map");
        self.component_map.insert(self.id_key, component);  // inserting moves `node`
        self.id_key += 1;
    }


    pub fn ps_list(&mut self)
    {
        warn!(
            "{:>5} {:^5} {:>8} {:>8} {:.100}",
            "PID", "STATE", "UTIME", "STIME", "CMD"
        );

        for p in &psutil::process::all().unwrap()
        {
            warn!(
                "{:>5} {:^5} {:>8.2} {:>8.2} {:.100}",
                p.pid,
                p.state.to_string(),
                p.utime,
                p.stime,
                p.cmdline()
                    .unwrap()
                    .unwrap_or_else(|| format!("[{}]", p.comm))
            );
        }
    }

    pub fn ps_find(&mut self, component:&str) -> u16
    {
        let mut amount_found:u16 = 0;

        for p in &psutil::process::all().unwrap()
        {
            let mut cmd = p.cmdline().unwrap().unwrap_or_else(|| format!("[{}]", p.comm));
            if(cmd.contains(component))
            {
                warn!("Found program and listing details");
                warn!("{:>5} {:^5} {:>8.2} {:>8.2} {:.100}",
                       "PID", "STATE", "UTIME", "STIME", "CMD");
                warn!(
                    "{:>5} {:^5} {:>8.2} {:>8.2} {:.100}",
                    p.pid,
                    p.state.to_string(),
                    p.utime,
                    p.stime,
                    p.cmdline()
                        .unwrap()
                        .unwrap_or_else(|| format!("[{}]", p.comm))
                );
                let mut this_pid = p.pid;
                let mut this_alive = true;
                let mut new = component;
                let mut inputted = Component
                {   _name:String::from(new),
                    _pid:this_pid,
                    _alive:this_alive,
                };
                self.join(inputted);
                amount_found += 1;
            }
        }
        return amount_found;
    }

    pub fn kill_duplicate_component(&mut self, component:&str)
    {
        let mut pid:i32 = 0;
        for (key, val) in self.component_map.iter()
        {
            warn!("key: {} val name: {} val pid: {} val alive: {}", key, val._name, val._pid, val._alive);
            if val._name == "python3 SYPSim.py"
            {
                pid = val._pid;
                warn!("Found Sim");
            }
        }
        if pid != 0
        {
            warn!("Killing sim");
            self.ps_kill_component(pid);
        }
    }

    pub fn ps_kill_component(&mut self, component:i32)
    {
        let process = Process::new(component).unwrap();

        if let Err(error) = process.kill()
        {
            println!("Failed to kill process: {}.", error);
        };
    }
}
