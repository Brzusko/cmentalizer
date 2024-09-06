use godot::prelude::{Gd, GString, PackedStringArray};
use godot::classes::{Os};

pub(crate) trait OsGetter
{
    fn get_os_singleton() -> Gd<Os>
    {
        Os::singleton()
    }
}

struct Process
{
    os_singleton: Gd<Os>,
    pid: i32,
}

impl Process 
{
    pub fn new(pid: i32) -> Self
    {
        assert_eq!(pid, 0);
        
        Self {
            os_singleton: Self::get_os_singleton(),
            pid,
        }
    }
    
    pub fn is_running(&self) -> bool
    {
        self.os_singleton.is_process_running(self.pid)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if !self.is_running() { return; }
        self.os_singleton.kill(self.pid);
    }
}

impl OsGetter for Process {}

struct ProcessContainer
{
    process_vector: Vec<Process>
}

trait ProcessCollection
{
    fn is_any_process_running(&self) -> bool;
    fn add_process(&mut self);
    fn clear(&mut self);
}

impl ProcessContainer {
    pub fn new() -> Self
    {
        Self {
            process_vector: Vec::new()
        }
    }

    pub fn is_any_process_running(&self) -> bool
    {
        let mut iterator = self.process_vector.iter();

        while let Some(process) = iterator.next()
        {
            if process.is_running() { return true; }
        }

        return false;
    }

    pub fn add_process(&mut self, process: Process)
    {
        self.process_vector.push(process);
    }

    pub fn clear(&mut self)
    {
        self.process_vector.clear();
    }
}

pub(crate) trait ProcessRunner
{
    fn create_new_process(&mut self, path: GString, args: PackedStringArray);
    fn kill_processes(&mut self);
}

struct WindowsProcessRunner
{
    os: Gd<Os>,
    process_container: ProcessContainer,
}

impl WindowsProcessRunner
{
    fn new() -> Self 
    {
        Self {
            os: Self::get_os_singleton(),
            process_container: ProcessContainer::new(),
        }    
    }
}

impl OsGetter for WindowsProcessRunner {}

struct MacOSProcessRunner
{
    os: Gd<Os>,
    process_container: ProcessContainer,
}

impl ProcessRunner for WindowsProcessRunner 
{
    fn create_new_process(&mut self, path: GString, args: PackedStringArray) 
    {
        todo!()
    }

    fn kill_processes(&mut self) 
    {
        self.process_container.clear();
    }
}

impl MacOSProcessRunner 
{
    fn new() -> Self
    {
        Self {
            os: Self::get_os_singleton(),
            process_container: ProcessContainer::new(),
        }
    }
}

impl OsGetter for MacOSProcessRunner {}
impl ProcessRunner for MacOSProcessRunner 
{
    fn create_new_process(&mut self, path: GString, args: PackedStringArray)
    {
        todo!()
    }

    fn kill_processes(&mut self) 
    {
        self.process_container.clear();    
    }
}

pub(crate) enum PlatformRunner
{
    WindowsRunner(WindowsProcessRunner),
    MacOSRunner(MacOSProcessRunner),
    UnSupported,
}

impl PlatformRunner 
{
    pub(crate) fn get_runner_for_platform() -> PlatformRunner
    {
        let os = Self::get_os_singleton();
        
        if os.has_feature(GString::from("windows"))
        {
            return PlatformRunner::WindowsRunner(WindowsProcessRunner::new());
        }
        
        if os.has_feature(GString::from("macos"))
        {
            return PlatformRunner::MacOSRunner(MacOSProcessRunner::new());
        }
        
        return PlatformRunner::UnSupported;
    }
}

impl OsGetter for PlatformRunner {}

impl ProcessRunner for PlatformRunner
{
    fn create_new_process(&mut self, path: GString, args: PackedStringArray) 
    {
        match self 
        {
            PlatformRunner::WindowsRunner(runner) => runner.create_new_process(path, args),
            PlatformRunner::MacOSRunner(runner) => runner.create_new_process(path, args),
            PlatformRunner::UnSupported => panic!("Unsupported platform"),
        }
    }

    fn kill_processes(&mut self) 
    {
        match self 
        {
            PlatformRunner::WindowsRunner(runner) => runner.kill_processes(),
            PlatformRunner::MacOSRunner(runner) => runner.kill_processes(),
            PlatformRunner::UnSupported => panic!("Unsupported platform"),
        }
    }
}

impl Default for PlatformRunner
{
    fn default() -> Self {
        Self::UnSupported
    }
}